use crate::sheet_serialize::SheetSerializable;

struct Account {
    points: i32,
    currency: String,
    pass: String,
    history: Vec<HistoryEntry>
}

struct HistoryEntry {
    event_name: String,
    difference: i32,
    prev_bal: i32,
    post_bal: i32,
}