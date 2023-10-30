use tracing::{debug, error, info, trace, warn};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::fmt::{self};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
/// init log
#[inline]
pub(crate) fn init_logger(name: &str, verbose_level: u8) {
    let env_layer = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or(VerboseLevel(verbose_level).to_string()),
    );

    let format_log = rolling::daily("logs", format!("{}.log", name));
    let (non_blocking_appender, _guard) = non_blocking(format_log);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);
    tracing_subscriber::registry()
        .with(env_layer)
        .with(file_layer)
        .with(fmt::layer())
        .init();
    #[cfg(debug_assertions)]
    print_log();
}
#[allow(dead_code)]
fn print_log() {
    info!("info");
    debug!("debug");
    error!("error");
    warn!("warn");
    trace!(tracing = "trace");
}
struct VerboseLevel(u8);
impl ToString for VerboseLevel {
    fn to_string(&self) -> String {
        match self {
            VerboseLevel(x) if *x == 1 => "error".to_owned(),
            VerboseLevel(x) if *x == 2 => "warn".to_owned(),
            VerboseLevel(x) if *x == 3 => "info".to_owned(),
            VerboseLevel(x) if *x == 4 => "debug".to_owned(),
            VerboseLevel(x) if *x == 5 => "trace".to_owned(),
            _ => "info".to_owned(),
        }
    }
}
