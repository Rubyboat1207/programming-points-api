use crate::{account::Account, sheet_helper::sheet_exists, GLOBAL_SHEET_CLIENT, SHEET_ID};

use rocket::serde::{Deserialize, json::Json};
use crate::account::HistoryEntry;
use crate::sheet_serialize::{SerializeArgs, SheetSerializable};

use super::login::LoginData;
extern crate rocket;

#[derive(Deserialize)]
pub struct TransferData {
    amount: f32,
    destination: String,
    authentication: LoginData
}
#[post("/transfer", data = "<input>")]
pub async fn transfer(input: Json<TransferData>) -> String {
    if input.amount <= 0. {
        return "{\"message\": \"Transfer amount cannot be below 0! Nice try.\"}".to_string()
    }

    if input.amount < 0.01 {
        return "{\"message\": \"Transfer amount cannot be below 0.01!\"}".to_string()
    }

    let account_exists = sheet_exists(&GLOBAL_SHEET_CLIENT, SHEET_ID, &(input.authentication.username.clone() + "_ACCOUNT")).await;

    if account_exists.is_err() {
        return "{\"message\": \"Failed\"}".to_string();
    }

    let exists = account_exists.unwrap();
    if !exists {
        return "{\"message\": \"Username or password was incorrect\"}".to_string();
    }

    let mut acc = Account::get_account(&GLOBAL_SHEET_CLIENT, &input.authentication.username).await.unwrap();
    if acc.pass != input.authentication.password {
        return "{\"message\": \"Username or password was incorrect\"}".to_string();
    }

    // Authentication Complete
    let destination_account_exists = sheet_exists(&GLOBAL_SHEET_CLIENT, SHEET_ID, &(input.destination.to_string())).await;

    if destination_account_exists.is_err() {
        return "{\"message\": \"There is an issue with google sheets, please try again in a few moments!\"}".to_string();
    }

    let dest_exists = destination_account_exists.unwrap();

    if !dest_exists {
        return "{\"message\": \"The destination is not valid\"}".to_string();
    }

    if input.destination.ends_with("_ACCOUNT") {
        // this is an account currency transfer

        let mut dest_acc = Account::get_account(&GLOBAL_SHEET_CLIENT, &input.destination.as_str()[..input.destination.len() - 8]).await.unwrap();

        let prev_bal = acc.points;
        let dest_prev_bal = dest_acc.points;

        dest_acc.points += input.amount;
        if acc.name != "rudy" {
            acc.points -= input.amount;
        }

        dest_acc.history.push(HistoryEntry {event_name: "transfer_receive".to_string(), prev_bal: dest_prev_bal, post_bal: dest_acc.points, difference: input.amount});
        acc.history.push(HistoryEntry {event_name: "transfer_send".to_string(), prev_bal, post_bal: acc.points, difference: -input.amount});

        // todo: error handling, and hopefully retry
        let _ = acc.write_to_gsheet(&GLOBAL_SHEET_CLIENT, SerializeArgs::new()).await;
        let _ = dest_acc.write_to_gsheet(&GLOBAL_SHEET_CLIENT, SerializeArgs::new()).await;
    }

    acc.pass = "".to_string();
    acc.history.reverse();

    return serde_json::to_string(&acc).unwrap();
}