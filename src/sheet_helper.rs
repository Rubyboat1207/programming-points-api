use once_cell::sync::OnceCell;
use tokio::sync::Mutex;
use sheets4::{api::ValueRange, hyper::client::HttpConnector, hyper_rustls::HttpsConnector, Error};
use sheets4::Sheets;

pub type GlobalSheetClient = OnceCell<Mutex<Option<Sheets<HttpsConnector<HttpConnector>>>>>;


pub async fn write_cell(global_sheet_client: &GlobalSheetClient, id: &str, range: &str, value: String) {
    let mut req: ValueRange = ValueRange::default();
    
    req.values = Some(vec![
        vec![serde_json::value::Value::String(value)]
    ]);


    if let Some(mutex) = global_sheet_client.get() {
        let mut guard = mutex.lock().await;
        if let Some(client) = guard.as_mut() {
            let res = client
                .spreadsheets()
                .values_update(req, id, range)
                .value_input_option("USER_ENTERED")
                .doit()
                .await;

            match res {
                Err(e) => match e {
                    // The Error enum provides details about what exactly happened.
                    // You can also just use its `Debug`, `Display` or `Error` traits
                    Error::HttpError(_)
                    | Error::Io(_)
                    | Error::MissingAPIKey
                    | Error::MissingToken(_)
                    | Error::Cancelled
                    | Error::UploadSizeLimitExceeded(_, _)
                    | Error::Failure(_)
                    | Error::BadRequest(_)
                    | Error::FieldClash(_)
                    | Error::JsonDecodeError(_, _) => println!("{}", e),
                },
                Ok(res) => println!("Success: {:?}", res),
            }
        } else {
            println!("Client is not yet initalized!");
        }
    } else {
        println!("OneCell is not yet initalized!");
    }
}