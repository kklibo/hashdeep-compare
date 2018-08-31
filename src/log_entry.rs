use std::fmt;

#[derive(Eq, PartialEq, Hash)]
pub struct LogEntry {
    pub hashes: String,
    pub filename: String,
}

impl LogEntry {

    const HASHCOUNT: usize = 3;

    pub fn from_str(s: &str) -> Option<LogEntry> {

        let sections: Vec<&str> = s.split(",").collect();

        if sections.len() < LogEntry::HASHCOUNT + 1 {return None;}

        //todo: replace with split?
        let hashes = sections[..LogEntry::HASHCOUNT].join(",");
        let filename = sections[LogEntry::HASHCOUNT..].join(",");

        Some(LogEntry{hashes, filename})
    }
}

impl fmt::Display for LogEntry {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.hashes, self.filename)
    }
}