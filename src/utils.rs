use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::FmtSubscriber;

pub fn clean_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn configure_logger() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("./logs", "prefix.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Impossible de configurer le subscriber global de tracing");
    guard
}
