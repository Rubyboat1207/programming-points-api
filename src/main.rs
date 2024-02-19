mod account;
mod sheet_serialize;
mod sheet_helper;

use once_cell::sync::OnceCell;
use sheet_helper::{write_cell, GlobalSheetClient};
use sheets4::oauth2::ServiceAccountAuthenticator;
use tokio::sync::Mutex;
extern crate google_sheets4 as sheets4;
use sheets4::{hyper, hyper_rustls, Sheets};

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

#[get("/set/<range>/<value>")]
async fn index(range: String, value: String) -> &'static str {
    write_cell(&GLOBAL_SHEET_CLIENT, "1hrM6YUzPFeDgIw2NWM_hESaJMEo5kgifgifcXCmB278", &range, value).await;

    "Done did it!"
}

#[launch]
async fn rocket() -> _ {
    let _ = GLOBAL_SHEET_CLIENT.set(Mutex::new(None));
    initialize_global_sheet_client().await;
    rocket::build().mount("/", routes![index])
}
