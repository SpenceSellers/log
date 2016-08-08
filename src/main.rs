use std::io::{self, Read, Seek};
use std::fs::{OpenOptions};

extern crate time;
use time::Tm;

extern crate clap;
use clap::{Arg, App, ArgMatches};

mod journal;
use journal::*;



const JOURNAL_FILE: &'static str =  "journal.txt";
const JOURNAL_BACKUP: &'static str = "journal.txt.bak";
const DEFAULT_GROUP: &'static str = "general";

fn shallow_copy<'a, T> (source: &'a Vec<T>) -> Vec<&'a T> {
    let mut v = Vec::new();
    for item in source {
        v.push(item);
    }
    return v;
}

fn parse_group(s: &str) -> String {
    if s.starts_with("@") {
        let (_, rest) = s.split_at(1);
        rest.to_string()
    } else {
        s.to_string()
    }
}


fn selected_entries<'a> (args: &ArgMatches, journal: &'a Journal) -> Vec<&'a Entry> {
    let mut entries = shallow_copy(&journal.entries);

    if let Some(group_str) = args.value_of("group") {
        let group = parse_group(group_str);
        entries.retain(|e| e.group == group);
    }

    return entries;
}

fn compose_entry_content() -> String {
    println!("Type your entry below: ");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    return buffer;
}

fn compose_entry(group: Option<String>, date: Option<Tm>) -> Entry {
    let group = group.unwrap_or_else(|| DEFAULT_GROUP.to_string());
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
            .append(false)
            .open(JOURNAL_FILE).expect("Error opening journal file!");

    let mut journal = {
        let mut jstring = String::new();
        file.read_to_string(&mut jstring).expect("Error reading journal file!");
        Journal::from_str(&jstring).expect("Error parsing journal!")
    };

    // If you don't seek to the beginning, new writes could be appended on the end.
    file.seek(std::io::SeekFrom::Start(0));


    let matches = App::new("Log")
        .setting(clap::AppSettings::TrailingVarArg)
        .arg(Arg::with_name("list")
                 .short("l")
                 .long("list")
                 .help("Lists previous entries"))
        .arg(Arg::with_name("group")
                 .short("g")
                 .long("group")
                 .takes_value(true))
        .arg(Arg::with_name("rest")
                 .index(1)
                 .multiple(true))
        .get_matches();

    

    if matches.is_present("list") {

        let entries = selected_entries(&matches, &journal);
        for entry in &entries {
            entry.encode(&mut io::stdout()).unwrap();
        }
        return;
    }

    //let mut group: Option<String> = None;

    let new_entry = if let Some(trailing) = matches.values_of("rest") {
        // Command line has at least some of the message.
        let mut trailing = trailing.peekable();
        if trailing.peek().unwrap().starts_with("@") {
            // Args start with group name
            let group = parse_group(trailing.next().unwrap());
            println!("Group is {}", group);

            if trailing.peek().is_none() {
                println!("Rest is empty");
                compose_entry(Some(group), None)
            } else {
                unimplemented!()
            }

        } else {
            unimplemented!()
        }

    } else {
        compose_entry(None, None)
    };

    
    journal.entries.push(new_entry);
    println!("Entries after this op: {}", journal.entries.len());

    journal.encode(&mut file).expect("Error writing file!");
}
