use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("解析出现了点问题 {}", err);
        process::exit(1);
    });


    if let Err(e) = minigrep::run(config) {
        eprintln!("程序错误, {}", e);

        process::exit(1);
    }
}