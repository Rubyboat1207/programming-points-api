use rocket::http::ext::IntoCollection;
use serde::Serialize;
use serde_json::to_string;
use crate::{GLOBAL_SHEET_CLIENT, SHEET_ID};
use crate::account::Account;
use crate::sheet_helper::get_sheets;

#[derive(Serialize, Clone)]
struct LeaderboardEntry {
    points: i32,
    username: String
}

#[get("/leaderboard")]
pub async fn leaderboard() -> String {
    let sheets = get_sheets(&GLOBAL_SHEET_CLIENT, SHEET_ID).await.unwrap();

    let mut account_names: Vec<String> = vec![];

    for sheet in sheets.iter() {
        let title = sheet.properties.as_ref().unwrap().title.clone().unwrap();
        if title.ends_with("_ACCOUNT") {
            account_names.push(title.to_string())
        }
    }

    let mut accounts = vec![];

    for name in account_names.iter() {
        accounts.push(Account::get_account(&GLOBAL_SHEET_CLIENT, &name[..(name.len() - 8)]).await.unwrap());
    }

    let mut entries = vec![];

    for acc in accounts {
        entries.push(LeaderboardEntry {points: acc.points, username: acc.name.to_string()});
    }

    return to_string(&entries).unwrap().to_string();
}