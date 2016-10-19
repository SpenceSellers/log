#![allow(dead_code)]

use std::io::{self, Read, Seek};
use std::fs::{OpenOptions, File};

extern crate time;
use time::Tm;

extern crate clap;
use clap::{Arg, App, ArgMatches};

extern crate tempfile;

mod journal;
use journal::*;

mod composers;

mod selection;

mod util;

const JOURNAL_FILE: &'static str =  "journal.txt";
const JOURNAL_BACKUP: &'static str = "journal.txt.bak";
const DEFAULT_GROUP: &'static str = "general";

fn parse_group(s: &str) -> String {
    if s.starts_with("@") {
        let (_, rest) = s.split_at(1);
        rest.to_string()
    } else {
        s.to_string()
    }
}

fn compose_entry<C>(group: Option<String>, date: Option<Tm>, composer: &C) -> Entry 
    where C: composers::Composer {
    let group = group.unwrap_or_else(|| DEFAULT_GROUP.to_string());
    let date = date.unwrap_or_else(|| time::now());
    let content = composer.compose().expect("Error composing entry");
    
    Entry {
        group: group,
        date: date,
        content: content
    }
}

fn selected_entries<'a> (args: &ArgMatches, journal: &'a Journal) -> Vec<&'a Entry> {
    let mut entries = util::shallow_copy(&journal.entries);

    if let Some(group_str) = args.value_of("group") {
        let group = parse_group(group_str);
        entries.retain(|e| e.group == group);
    }

    if args.is_present("select_last") {
        let last = entries.pop();
        entries.clear();
        if let Some(last) = last {
            entries.push(last);
        }
    }

    return entries;
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
    file.seek(std::io::SeekFrom::Start(0)).unwrap();


    let matches = App::new("Log")
        .setting(clap::AppSettings::TrailingVarArg)
        .arg(Arg::with_name("show")
                 .short("s")
                 .long("show")
                 .help("Show previous entries"))
        .arg(Arg::with_name("group")
                 .short("g")
                 .long("group")
                 .takes_value(true))
        .arg(Arg::with_name("select_last")
                 .long("last"))
        .arg(Arg::with_name("rest")
                 .index(1)
                 .multiple(true))
        .get_matches();

    

    if matches.is_present("show") {

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

            if trailing.peek().is_none() {
                // They've only supplied the group name, they need to compose the rest.
                compose_entry(Some(group), None, &composers::StdIn)
            } else {
                // They've supplied both the group name and the content.
                let words: Vec<&str> = trailing.collect();
                Entry::new_now(group, words.join(" "))
            }

        } else {
            let words: Vec<&str> = trailing.collect();
            Entry::new_now(parse_group(DEFAULT_GROUP), words.join(" "))
        }

    } else {
        compose_entry(None, None, &composers::StdIn)
        
    };

    
    journal.entries.push(new_entry);

    journal.encode(&mut file).expect("Error writing file!");
    println!("[Entry saved successfully]");
}
