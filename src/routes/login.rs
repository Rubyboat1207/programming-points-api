use crate::{account::Account, sheet_helper::sheet_exists, GLOBAL_SHEET_CLIENT, SHEET_ID};

use rocket::serde::{Deserialize, json::Json};
extern crate rocket;

#[derive(Deserialize)]
pub struct LoginData {
    pub password: String,
    pub username: String
}

#[post("/login", data = "<input>")]
pub async fn login(input: Json<LoginData>) -> String {
    let res = sheet_exists(&GLOBAL_SHEET_CLIENT, SHEET_ID, &(input.username.clone() + "_ACCOUNT")).await;

    if res.is_err() {
        return "{\"message\": \"Failed\"}".to_string();
    }

    let exists = res.unwrap();

    if !exists {
        return "{\"message\": \"Username or password was incorrect\"}".to_string();
    }

    let mut acc = Account::get_account(&GLOBAL_SHEET_CLIENT, &input.username).await.unwrap();


    if acc.pass != input.password {
        return "{\"message\": \"Username or password was incorrect\"}".to_string();
    }

    acc.pass = "".to_string();
    acc.history.reverse();

    return serde_json::to_string(&acc).unwrap();
}