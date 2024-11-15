use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use log::LevelFilter;
use env_logger;
use env_logger::{Builder, Target};


pub fn init_file_logger() {
    let log_file = File::create("./logs.txt").unwrap();
    let log_file = Arc::new(Mutex::new(log_file));

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout)
        .format(move |buf, record| {
            let mut file = log_file.lock().unwrap();
            writeln!(file, "{} - {}", record.level(), record.args())?;  // log to file
            writeln!(buf, "{} - {}", record.level(), record.args())     // log to stdout
        })
        .filter(None, LevelFilter::Info)
        .init();
}