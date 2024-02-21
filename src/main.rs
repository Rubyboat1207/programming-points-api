mod account;
mod sheet_helper;
mod sheet_serialize;
mod routes;
mod cors;

use account::Account;
use once_cell::sync::OnceCell;
use sheet_helper::GlobalSheetClient;
use sheet_serialize::{SerializeArgs, SheetSerializable};
use sheets4::oauth2::ServiceAccountAuthenticator;
use tokio::sync::Mutex;
extern crate google_sheets4 as sheets4;
use sheets4::{hyper, hyper_rustls, Sheets};
pub static SHEET_ID: &'static str = "1hrM6YUzPFeDgIw2NWM_hESaJMEo5kgifgifcXCmB278";
pub static GLOBAL_SHEET_CLIENT: GlobalSheetClient = OnceCell::new();

async fn initialize_global_sheet_client() {
    let sa_key = yup_oauth2::read_service_account_key("./key.json")
        .await
        .unwrap();

    let auth = ServiceAccountAuthenticator::builder(sa_key)
        .build()
        .await
        .unwrap();

    let client = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    );

    // Initialize the OnceCell with a Mutex containing None if not already initialized
    let cell = GLOBAL_SHEET_CLIENT.get_or_init(|| Mutex::new(None));
    let mut global_client = cell.lock().await;
    *global_client = Some(client);
}

#[macro_use]
extern crate rocket;
std::thread_local! {}

#[put("/test")]
async fn test() -> String {
    // let acct = Account {
    //     name: "Michael_Wu".to_string(),
    //     points: 20,
    //     currency: "Programming Points".to_string(),
    //     pass: "Password".to_string(),
    //     history: vec![HistoryEntry {
    //         event_name: "acct_create".to_string(),
    //         difference: 20,
    //         prev_bal: 0,
    //         post_bal: 20,
    //     }],
    // };

    // let res = acct
    //     .write_to_gsheet(&GLOBAL_SHEET_CLIENT, SerializeArgs::new())
    //     .await;

    // if res.is_err() {
    //     println!("{}", res.unwrap_err());
    //     return "Failed".to_string();
    // }

    let mut dsarg = SerializeArgs::new();

    dsarg.sheet = Some("Michael_Wu_ACCOUNT".to_string());

    let res = Account::read_from_gsheet(&GLOBAL_SHEET_CLIENT, dsarg).await;

    let json;

    match &res {
        Ok(v) => json = serde_json::to_string(v.as_ref()),
        Err(e) => {
            println!("{}", &e);
            return "Failed".to_string();
        },
    }

    if res.is_err() {
        return "Failed2".to_string();
    }

    match json {
        Ok(v) => return v,
        Err(e) => println!("{}", &e),
    }

    return "failed".to_string();
}

#[launch]
async fn rocket() -> _ {
    let _ = GLOBAL_SHEET_CLIENT.set(Mutex::new(None));
    initialize_global_sheet_client().await;
    rocket::build().attach(cors::CORS).mount("/", routes![test, crate::routes::login::login, crate::routes::transfer::transfer, crate::routes::leaderboard::leaderboard])
}
