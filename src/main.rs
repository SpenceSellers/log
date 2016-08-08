use std::fmt;
//use std::fmt::Write as FmtWrite;
use std::io::{self, Read, Write};
use std::fs::{OpenOptions};


extern crate time;
use time::Tm;

extern crate regex;
use regex::Regex;

extern crate clap;
use clap::{Arg, App, ArgMatches};



const JOURNAL_FILE: &'static str =  "journal.txt";
const JOURNAL_BACKUP: &'static str = "journal.txt.bak";

struct Entry {
    date: Tm,
    group: String,
    content: String,
}

impl Entry {
    fn encode<W: Write>(&self, mut sink: &mut W) -> std::io::Result<()> {
        write!(&mut sink, "**| {} @{}:\n{}\n\n", string_date(&self.date), self.group, self.content)
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
        let mut bytes = Vec::new();
        self.encode(&mut bytes).unwrap();

        let s = String::from_utf8(bytes).unwrap();

        write!(f, "{}", s)
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

    fn encode<W: Write>(&self, sink: &mut W) -> std::io::Result<()> {
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

fn shallow_copy<'a, T> (source: &'a Vec<T>) -> Vec<&'a T> {
    let mut v = Vec::new();
    for item in source {
        v.push(item);
    }
    return v;
}


fn selected_entries<'a> (args: &ArgMatches, journal: &'a Journal) -> Vec<&'a Entry> {
    let mut entries = shallow_copy(&journal.entries);

    if let Some(group_str) = args.value_of("group") {
        entries.retain(|e| e.group == group_str);
    }

    return entries;
}

fn compose_entry_content() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    return buffer;
}

fn compose_entry(group: Option<String>, date: Option<Tm>) -> Entry {
    let group = group.unwrap_or_else(|| "general".to_string());
    let date = date.unwrap_or_else(|| time::now());
    let content = compose_entry_content();
    
    Entry {
        group: group,
        date: date,
        content: content
    }
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



    let matches = App::new("Log")
        .arg(Arg::with_name("list")
                 .short("l")
                 .long("list")
                 .help("Lists previous entries"))
        .arg(Arg::with_name("group")
                 .short("g")
                 .long("group")
                 .takes_value(true))
        .get_matches();

    

    if matches.is_present("list") {

        let entries = selected_entries(&matches, &journal);
        for entry in &entries {
            entry.encode(&mut io::stdout()).unwrap();
        }
        return;
    }

    

    let new_entry = compose_entry(None, None);

    journal.entries.push(new_entry);

    journal.encode(&mut file).expect("Error writing file!");
}
