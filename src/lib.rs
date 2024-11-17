use std::error::Error;
use journald::JournalEntry;
use journald::reader::{JournalFiles, JournalReader, JournalReaderConfig, JournalSeek};

pub struct JournalStream {
    reader: JournalReader,
}

impl JournalStream {
    pub fn for_each<F>(&mut self, f: F)
    where
        F: Fn(JournalEntry) -> (),
    {
        for entry in self.reader.as_blocking_iter() {
            if let Ok(entry) = entry {
                f(entry)
            }
        }
    }
}

pub fn journal_stream_args(
    mut args: impl Iterator<Item=String>,
) -> Result<JournalStream, Box<dyn Error>> {
    let args = args.skip(1);

    let mut files = JournalFiles::All;
    let mut only_local = true;
    let mut only_volatile = false;

    for arg in args {
        match arg.as_str() {
            "--files=current-user" => files = JournalFiles::CurrentUser,
            "--files=system" => files = JournalFiles::System,
            "--only-local=false" => only_local = false,
            "--only_volatile=true" => only_volatile = true,
            _ => {}
        }
    }

    journal_stream(
        files,
        only_local,
        only_volatile,
    )
}

pub fn journal_stream(
    files: JournalFiles,
    only_local: bool,
    only_volatile: bool,
) -> Result<JournalStream, Box<dyn Error>> {
    let mut journal_reader = JournalReader::open(&JournalReaderConfig {
        files,
        only_local,
        only_volatile,
    })?;

    journal_reader.seek(JournalSeek::Tail)?;

    Ok(JournalStream {
        reader: journal_reader
    })
}
