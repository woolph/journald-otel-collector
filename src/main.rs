use std::process;
use journald::reader::{JournalFiles, JournalReader, JournalReaderConfig, JournalSeek};

const TRANSPORT: &str = "_TRANSPORT";
const REALTIME_TIMESTAMP: &str = "__REALTIME_TIMESTAMP";
const SOURCE_REALTIME_TIMESTAMP: &str = "_SOURCE_REALTIME_TIMESTAMP";
fn main() {
    let mut journal_reader = JournalReader::open(&JournalReaderConfig{
        files: JournalFiles::All,
        only_local: true,
        only_volatile: false,
    }).unwrap_or_else(|err| {
        eprintln!("Couldn't open journal: {err}");
        process::exit(1);
    });

    journal_reader.seek(JournalSeek::Tail).unwrap_or_else(|err| {
        eprintln!("Couldn't move to tail fof journal: {err}");
        process::exit(1);
    });
    
    for entry in journal_reader.as_blocking_iter() {
        if let Ok(entry) = entry {
            let message = entry.get_message().or(Some("<no-message>")).unwrap();
            let fields = entry.get_fields();
            let transport = entry.get_field(TRANSPORT).or(Some("<none>")).unwrap();
            let timestamp = entry.get_field(SOURCE_REALTIME_TIMESTAMP).or(
                entry.get_field(REALTIME_TIMESTAMP)
            ).expect("should have at least one timestamp");
            println!("{:<12} {:<12} {} {:?}", timestamp, transport, message, fields);
        }
    }
}
