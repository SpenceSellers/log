use super::journal;

extern crate time;
use time::Tm;

trait DecideEntry {
    fn included(&self, &journal::Entry) -> bool;
}

enum Selection {
    All(Vec<Selection>),
    OneOf(Vec<Selection>),
    Sel(Selector)
}

enum Selector {
    Date(DateSelection),
    Content(ContentSelection),
    Group(GroupSelection)
}

enum DateSelection {
    Before(Tm),
    After(Tm),
    On(Tm)
}

enum ContentSelection {
    Contains(String),
}

impl DecideEntry for ContentSelection {
    fn included(&self, entry: &journal::Entry) -> bool {
        match self {
            &ContentSelection::Contains(ref contains) => entry.content.contains(contains)
        }
    }

}

enum GroupSelection {
    Is(String),
}

impl DecideEntry for GroupSelection {
    fn included(&self, entry: &journal::Entry) -> bool {
        match self {
            &GroupSelection::Is(ref group) => entry.group == *group

        }
    }

}
