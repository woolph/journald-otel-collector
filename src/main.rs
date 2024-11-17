use std::{env, process};

const TRANSPORT: &str = "_TRANSPORT";
const REALTIME_TIMESTAMP: &str = "__REALTIME_TIMESTAMP";
const SOURCE_REALTIME_TIMESTAMP: &str = "_SOURCE_REALTIME_TIMESTAMP";

fn main() {
    let mut journal_stream = rust_journald::journal_stream_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Couldn't create journal iterator: {err}");
        process::exit(1);
    });

    journal_stream.for_each(|entry| {
        let message = entry.get_message().unwrap_or("<no-message>");
        let fields = entry.get_fields();
        let transport = entry.get_field(TRANSPORT).unwrap_or("<none>");
        let timestamp = entry.get_field(SOURCE_REALTIME_TIMESTAMP).or(
            entry.get_field(REALTIME_TIMESTAMP)
        ).expect("should have at least one timestamp");
        let timestamp : i64 = timestamp.parse().expect("should be parsable as i64");
        let timestamp = chrono::DateTime::from_timestamp_micros(timestamp).expect("should be convertable to a DateTime");
        let timestamp = timestamp.format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();
        println!("{} {:<12} {} {:?}", timestamp, transport, message, fields);
    });
}
