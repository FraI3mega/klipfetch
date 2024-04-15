use color_eyre::Result;
use ureq::get;

#[derive(Debug, PartialEq)]
pub enum PrinterState {
    Ready,
    Shutdown,
    Startup,
    Error,
    CanNotReach,
}

pub fn get_status(printer_url: &str) -> Result<PrinterState> {
    match get(format!("{}/printer/info", printer_url).as_str()).call() {
        Ok(data) => {
            match data.into_json::<serde_json::Value>()?["result"]["state"]
                .as_str()
                .unwrap()
            {
                "ready" => Ok(PrinterState::Ready),
                "error" => Ok(PrinterState::Error),
                "shutdown" => Ok(PrinterState::Shutdown),
                "startup" => Ok(PrinterState::Startup),
                _ => unreachable!("This is unreachable"),
            }
        }
        Err(_) => Ok(PrinterState::CanNotReach),
    }
}
