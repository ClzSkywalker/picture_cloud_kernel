use chrono::Local;

use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::fmt::{self, time::FormatTime};
use tracing_subscriber::layer::SubscriberExt;

#[derive(Debug)]
struct MyTimeFormat;

impl FormatTime for MyTimeFormat {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        write!(w, "{}", now.to_string())
    }
}

/**
 * @Author         : ClzSkywalker
 * @Date           : 2023-04-28
 * @Description    : 初始化 Log
 * @param           {*} path
 * @return          {*}
 */
pub fn init_log(path: &str) -> WorkerGuard {
    let err_file = rolling::daily(path, "error").with_max_level(Level::ERROR);

    let info_file = rolling::daily(path, "info")
        .with_min_level(Level::WARN)
        .with_max_level(Level::INFO);

    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
    let non_blocking = non_blocking
        .with_min_level(Level::ERROR)
        .with_max_level(Level::DEBUG);

    let subscriber = tracing_subscriber::registry()
        .with(
            fmt::Layer::new()
                .with_timer(MyTimeFormat)
                .with_line_number(true)
                .with_test_writer()
                .with_line_number(true)
                // .json()
                .with_writer(non_blocking),
        )
        .with(
            fmt::Layer::new()
                .json()
                .with_timer(MyTimeFormat)
                .with_line_number(true)
                .with_span_list(true)
                .with_writer(info_file),
        )
        .with(
            fmt::Layer::new()
                .json()
                .with_timer(MyTimeFormat)
                .with_line_number(true)
                .with_span_list(true)
                .with_writer(err_file),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global collector");
    guard
}
