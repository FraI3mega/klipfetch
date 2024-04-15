mod moonraker;

use std::thread;

use clap::Parser;
use color_eyre::{owo_colors::OwoColorize, Result};
use colored::Colorize;
use serde_json::Value;
use ureq::get;

use crate::moonraker::{get_status, PrinterState};

#[derive(Parser)]
struct Cli {
    json: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let printer_url = "http://192.168.1.113";
    loop {
        let status = get_status(printer_url)?;
        if status == PrinterState::Startup {
            thread::sleep(std::time::Duration::from_secs(2));
            continue;
        }
        break;
    }
    dbg!(get_status(printer_url)?);
    if get_status(printer_url)? == PrinterState::Ready {
        let data = &get(format!("{}/printer/objects/query?extruder=target,temperature&print_stats&heater_bed=target,temperature", printer_url).as_str()).call()?.into_json::<Value>()?["result"]["status"];
        dbg!(data);
        let print_stats = &data["print_stats"];
        let hotend = &data["extruder"];
        let heatbed = &data["heater_bed"];
        println!("Status: {}", print_stats["state"].as_str().unwrap().cyan());
        println!(
            "Hotend: {}/{}\nHeater bed: {}/{}",
            hotend["temperature"].cyan(),
            hotend["target"].cyan(),
            heatbed["temperature"].cyan(),
            heatbed["target"].cyan()
        );
        if print_stats["state"] == "printing" {
            println!(
                "Filename: {}\nLayer {}/{}",
                print_stats["filename"].as_str().unwrap(),
                print_stats["info"]["current_layer"].as_str().unwrap(),
                print_stats["info"]["total_layer"].as_str().unwrap()
            );
        } else if print_stats["state"] == "error" {
            println!("Error: {}", print_stats["message"].as_str().unwrap().red())
        }
    }
    Ok(())
}
