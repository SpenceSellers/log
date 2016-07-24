use std::fmt;
use std::fmt::Write;


extern crate time;
use time::Tm;

extern crate regex;
use regex::Regex;


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
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
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
    // let e = Entry {
    //     date: time::now(),
    //     group: "general".to_string(),
    //     content: "U sux forever".to_string()
    // };
    //
    // e.write(&mut std::io::stdout());

    let journal = Journal::from_str(
        "**| 2016-05-20 @prog
        U SUX
        **| 2016-07-10 @general ALSO SUX").unwrap();

    println!("{}", journal.encoded());
}
