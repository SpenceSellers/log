use std::io::{self, Read};
use std::fs::{OpenOptions};

extern crate time;
use time::Tm;

extern crate clap;
use clap::{Arg, App, ArgMatches};

mod journal;
use journal::*;



const JOURNAL_FILE: &'static str =  "journal.txt";
const JOURNAL_BACKUP: &'static str = "journal.txt.bak";


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
