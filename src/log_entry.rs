

pub struct LogEntry {
    hashes: String,
    filename: String,
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