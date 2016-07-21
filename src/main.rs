use std::io::Write;
use std::fmt;

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
    fn write<W: Write> (&self, sink: &mut W) -> Result<(), std::io::Error> {
        let time_s = time::strftime("%F", &self.date).expect("Failed date conversion???");
        write!(sink, "**| {} @{}:\n{}\n\n", time_s, self.group, self.content)
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        self.write(&mut s);
        write!(f, "{}", s)
    }
}

fn parse_date(s: &str) -> Option<Tm> {
    time::strptime(s, "%F").ok()
}

fn parse_entries(s: &str) -> Option<Vec<Entry>> {
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

    return Some(entries);

}


fn main() {
    // let e = Entry {
    //     date: time::now(),
    //     group: "general".to_string(),
    //     content: "U sux forever".to_string()
    // };
    //
    // e.write(&mut std::io::stdout());

    parse_entries(
        "**| 2016-05-20 @prog
        U SUX
        **| 2016-07-10 @general ALSO SUX");
}
