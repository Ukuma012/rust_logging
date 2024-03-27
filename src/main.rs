use log_sample::{fibonacci, tribonacci};
use std::fs::File;

fn init_logger() {
    // env_loggerは環境変数で出力するログレベルを変更できる
    // 出力先は標準エラーに固定されているが、簡単に導入できる
    // env_logger::init();

    // simplelogでは書き出し先を制御できる
    // 実行すると、ターミナルにはWarn以上、simple.logにはInfo以上のログが書き出される
    // simplelog::CombinedLogger::init(vec![
    //     simplelog::TermLogger::new(
    //         simplelog::LevelFilter::Warn,
    //         simplelog::Config::default(),
    //         simplelog::TerminalMode::Mixed,
    //     ),
    //     // ファイルsimplelog.logにはInfo以上を表示する
    //     simplelog::WriteLogger::new(
    //         simplelog::LevelFilter::Info,
    //         simplelog::Config::default(),
    //         File::create("simplelog.log").unwrap(),
    //     ),
    // ])
    // .unwrap();

    // ログのフォーマットを自由に設定したり、モジュールごとのログレベルを柔軟に設定したい場合はfern
    // 自分が書いたコードのログは詳しくみたいが、OSSのログはWarn以上で良い場合に便利
    let base_config = fern::Dispatch::new();

    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Warn)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(fern::log_file("fern.log").unwrap());

    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .level_for("fibonacci", log::LevelFilter::Trace)
        .level_for("tribonacci", log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .unwrap();
}

fn main() {
    // ロガーを初期化
    init_logger();

    // 5種類のエラーレベル
    log::trace!("Some trace log");
    log::debug!("Some debug log");
    log::info!("Some infomation log");
    log::warn!("Some waring log");
    log::error!("Some error log");

    let n = 4;
    log::info!("try to calculate fibonacci({})", n);
    let fib = fibonacci::fibonacci(n);
    println!("fib[{}] = {}", n, fib);
    log::info!("try to calculate tribonacci({})", n);
    let trib = tribonacci::tribonacci(n);
    println!("trib[{}] = {}", n, trib);
}
