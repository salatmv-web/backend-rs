use std::path::Path;
use chrono::Local;

pub fn setup_logger() -> Result<(), fern::InitError> {
    let logs_dir = Path::new("logs");

    if !logs_dir.exists() {
        std::fs::create_dir(logs_dir)?;
    }

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d] [%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(format!("logs/{}.log", Local::now().format("%Y-%m-%d")))?)
        .apply()?;
    Ok(())
}