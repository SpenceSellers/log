extern crate time;
extern crate regex;

use std::fmt;
use std::io::{self, Read, Write};

use time::Tm;

use self::regex::Regex;

pub struct Entry {
    pub date: Tm,
    pub group: String,
    pub content: String,
}

impl Entry {
    pub fn encode<W: Write>(&self, mut sink: &mut W) -> io::Result<()> {
        write!(&mut sink, "**| {} @{}:\n{}\n\n", string_date(&self.date), self.group, self.content)
    }

    pub fn new_now(group: String, content: String) -> Self {
        Entry {
            date: time::now(),
            group: group,
            content: content
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bytes = Vec::new();
        self.encode(&mut bytes).unwrap();

        let s = String::from_utf8(bytes).unwrap();

        write!(f, "{}", s)
    }
}

pub struct Journal {
    pub entries: Vec<Entry>
}

impl Journal {
    pub fn from_entries(v: Vec<Entry>) -> Self {
        Journal {entries: v}
    }

    pub fn from_str(s: &str) -> Option<Journal> {
        let re = Regex::new(r"\*\*\|\s+(\d{4}-\d{2}-\d{2})\s+@(\w+)\s*:\s+(.*)").unwrap();
        let caps = re.captures_iter(s);

        let mut entries = Vec::new();
        for cap in caps {
            let entry = Entry {
                date: parse_date(cap.at(1).unwrap()).expect("Bad date!"),
                group: cap.at(2).unwrap().to_string(),
                content: cap.at(3).unwrap().to_string()
            };
            entries.push(entry);
        }
        return Some(Journal::from_entries(entries));
    }

    pub fn encode<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        for entry in &self.entries {
            try!(entry.encode(sink));
        }
        return Ok(());
    }
}

fn parse_date(s: &str) -> Option<Tm> {
    time::strptime(s, "%F").ok()
}

fn string_date(date: &Tm) -> String {
    time::strftime("%F", date).expect("Failed date conversion???")
}
