use std::fmt;
use std::fmt::Write as FmtWrite;
use std::env;
use std::io::{self, Read, Write};
use std::fs::{OpenOptions};


extern crate time;
use time::Tm;

extern crate regex;
use regex::Regex;

extern crate getopts;
use getopts::Options;

const JOURNAL_FILE: &'static str =  "journal.txt";
const JOURNAL_BACKUP: &'static str = "journal.txt.bak";

struct Entry {
    date: Tm,
    group: String,
    content: String,
}

impl Entry {
    fn encoded(&self) -> String {
        let time_s = time::strftime("%F", &self.date).expect("Failed date conversion???");
        let mut sink = String::new();
        write!(&mut sink, "**| {} @{}:\n{}\n\n", time_s, self.group, self.content).unwrap();
        return sink;
    }

    fn new_now(group: String, content: String) -> Self {
        Entry {
            date: time::now(),
            group: group,
            content: content
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.encoded())
    }
}

struct Journal {
    entries: Vec<Entry>
}

impl Journal {
    fn from_entries(v: Vec<Entry>) -> Self {
        Journal {entries: v}
    }

    fn from_str(s: &str) -> Option<Journal> {
        let re = Regex::new(r"\*\*\|\s+(\d{4}-\d{2}-\d{2})\s+@(\w+)\s+(.*)").unwrap();
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

    fn encoded(&self) -> String {
        let mut s = String::new();
        for entry in &self.entries {
            s.push_str(&entry.encoded());
        }
        return s;
    }
}

fn parse_date(s: &str) -> Option<Tm> {
    time::strptime(s, "%F").ok()
}


fn main() {

    let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(JOURNAL_FILE).expect("Error opening journal file!");

    let mut journal = {
        let mut jstring = String::new();
        file.read_to_string(&mut jstring).expect("Error reading journal file!");
        Journal::from_str(&jstring).expect("Error parsing journal!")
    };



    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("l", "list", "Show log entries");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("l") {
        for entry in &journal.entries {
            
        }
        return;
    }

    



    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let group = "general".to_string();
    let content = buffer;
    let new_entry = Entry::new_now(group, content);

    journal.entries.push(new_entry);

    file.write(journal.encoded().as_bytes());
}
