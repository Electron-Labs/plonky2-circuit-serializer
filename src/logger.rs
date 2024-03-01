use tracing_appender::non_blocking::WorkerGuard;

pub fn initialize_logger() -> WorkerGuard{
    let file_appender = tracing_appender::rolling::daily("./log", "circuit_serializer.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .json()
            .pretty()
            .init();
    _guard
}