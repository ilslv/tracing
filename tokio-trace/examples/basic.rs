#[macro_use]
extern crate tokio_trace;
extern crate tokio_trace_log;
extern crate env_logger;

use tokio_trace::Level;

fn main() {
    env_logger::Builder::new().parse("info").init();
    let subscriber = tokio_trace_log::TraceLogger::new();

    tokio_trace::Dispatch::to(subscriber).with(|| {
        let foo = 3;
        event!(Level::Info, { foo = foo, bar = "bar" }, "hello world");

        let span = span!("my_great_span", foo = 4, baz = 5);
        span.enter(|| {
            event!(Level::Info, { yak_shaved = true }, "hi from inside my span");
        });
    });
}
