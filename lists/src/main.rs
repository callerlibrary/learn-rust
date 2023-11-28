use chrono::Local;
use fern::Dispatch;
use log::{debug, error, info, log_enabled, warn, Level, LevelFilter};
use std::env;

fn main() {
    // 设置启动环境变量LOG_MODE，选择日志库
    let log_mode = env::var("LOG_MODE").unwrap_or_else(|_| String::from("default"));
    setup(&log_mode);
}

fn setup(mode: &str) {
    match mode {
        "env_logger" => setup_env_logger(),
        "fern" => {
            setup_fern().expect("failed to setup logger");
            info!("this is an info message");
            warn!("this is a warning message");
            error!("this is an error message");
        }
        _ => println!("LOG_MODE is Illegal parameter"),
    }
}

// 初始化 env_logger
fn setup_env_logger() {
    use env_logger::Env;
    // windos 下运行
    // $env:RUST_LOG="info";cargo run
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    debug!("this is a debug {}", "message");
    warn!("this is a warn message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}

// 初始化 fern
fn setup_fern() -> Result<(), fern::InitError> {
    let log_file = format!("{}.log", Local::now().format("%Y-%m-%d"));

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(fern::log_file(log_file)?)
        .apply()?;
    Ok(())
}
