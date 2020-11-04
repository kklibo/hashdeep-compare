use std::fmt;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct LogEntry {
    pub hashes: String,
    pub filename: String,
}

impl LogEntry {

    const HASHCOUNT: usize = 3;

    pub fn from_str(s: &str) -> Option<LogEntry> {

        let sections: Vec<&str> = s.split(',').collect();
        if sections.len() < LogEntry::HASHCOUNT + 1 {return None;}

        let (hashes_sections, filename_sections) = sections.split_at(LogEntry::HASHCOUNT);
        if hashes_sections.contains(&"") {return None;}

        let hashes = hashes_sections.join(",");
        let filename = filename_sections.join(",");
        if filename.is_empty() {return None;}

        Some(LogEntry{hashes, filename})
    }
}

impl fmt::Display for LogEntry {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.hashes, self.filename)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_string_test() {
        let hashes = "1,aaaaa,bbbbbbb".to_owned();
        let filename = "theDir/theFile.ext".to_owned();
        let le = LogEntry{hashes, filename};
        assert_eq!(le.to_string(), "1,aaaaa,bbbbbbb,theDir/theFile.ext".to_owned());
    }

    #[test]
    fn from_str_test() {
        let random_chars = "[l]425[o24h8j5ffp983h4f";
        assert_eq!(LogEntry::from_str(random_chars), None);

        let not_enough_commas = "4692d489b0638e49682df4f46dacd3c3,0c47cda934d53d7ca29d822a59531dcf6d36cbd9740a4fd0b867a0343910a715,hashdeepComp/345.txt";
        assert_eq!(LogEntry::from_str(not_enough_commas), None);

        let no_size = ",4692d489b0638e49682df4f46dacd3c3,0c47cda934d53d7ca29d822a59531dcf6d36cbd9740a4fd0b867a0343910a715,hashdeepComp/345.txt";
        assert_eq!(LogEntry::from_str(no_size), None);

        let empty_filename = "4,4692d489b0638e49682df4f46dacd3c3,0c47cda934d53d7ca29d822a59531dcf6d36cbd9740a4fd0b867a0343910a715,";
        assert_eq!(LogEntry::from_str(empty_filename), None);

        let just_commas = ",,,";
        assert_eq!(LogEntry::from_str(just_commas), None);

        let hashes_str = "4,4692d489b0638e49682df4f46dacd3c3,0c47cda934d53d7ca29d822a59531dcf6d36cbd9740a4fd0b867a0343910a715";

        let normal_entry = "4,4692d489b0638e49682df4f46dacd3c3,0c47cda934d53d7ca29d822a59531dcf6d36cbd9740a4fd0b867a0343910a715,hashdeepComp/345.txt";
        assert_eq!(LogEntry::from_str(normal_entry),
                   Some(LogEntry{hashes: hashes_str.to_owned(), filename: "hashdeepComp/345.txt".to_owned()}));

        let non_ascii_filename = "4,4692d489b0638e49682df4f46dacd3c3,0c47cda934d53d7ca29d822a59531dcf6d36cbd9740a4fd0b867a0343910a715,hashdeepComp/Γεια σου.txt";
        assert_eq!(LogEntry::from_str( non_ascii_filename),
                   Some(LogEntry{hashes: hashes_str.to_owned(), filename: "hashdeepComp/Γεια σου.txt".to_owned()}));

        let commas_in_filename = "4,4692d489b0638e49682df4f46dacd3c3,0c47cda934d53d7ca29d822a59531dcf6d36cbd9740a4fd0b867a0343910a715,hashdeepComp/3,4,,5.txt,";
        assert_eq!(LogEntry::from_str(commas_in_filename),
                   Some(LogEntry{hashes: hashes_str.to_owned(), filename: "hashdeepComp/3,4,,5.txt,".to_owned()}));
    }
}