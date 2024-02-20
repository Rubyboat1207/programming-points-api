use once_cell::sync::OnceCell;
use sheets4::api::{Sheet, Spreadsheet};
use tokio::sync::Mutex;
use sheets4::{api::ValueRange, hyper::client::HttpConnector, hyper_rustls::HttpsConnector};
use sheets4::Sheets;

pub type GlobalSheetClient = OnceCell<Mutex<Option<Sheets<HttpsConnector<HttpConnector>>>>>;
pub type SheetsResult<T> = Result<T, anyhow::Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AppError {
    NotInitialized,
    NoValues,
}

impl std::error::Error for AppError {}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub fn to_string_vec(json_vec: Vec<Vec<serde_json::Value>>) -> Vec<Vec<String>> {
    json_vec.iter()
    .map(|row|
        row.iter()
           .map(|cell| cell.as_str().unwrap_or_default().to_string())
           .collect::<Vec<String>>()
    )
    .collect::<Vec<Vec<String>>>()
}

pub fn to_json_vec(string_vec: Vec<Vec<String>>) -> Vec<Vec<serde_json::Value>> {
    string_vec.iter()
    .map(|row|
        row.iter()
           .map(|cell| serde_json::Value::from(cell.clone()))
           .collect::<Vec<serde_json::Value>>()
    )
    .collect::<Vec<Vec<serde_json::Value>>>()
}

// pub async fn write_cell(global_sheet_client: &GlobalSheetClient, id: &str, range: &str, value: String) -> SheetsResult<()> {
//     let mut req: ValueRange = ValueRange::default();
//     req.values = Some(vec![vec![serde_json::value::Value::String(value)]]);

//     let client = global_sheet_client.get().ok_or(AppError::NotInitialized)?;
//     let mut guard = client.lock().await;
//     let client = guard.as_mut().ok_or(AppError::NotInitialized)?;

//     client
//         .spreadsheets()
//         .values_update(req, id, range)
//         .value_input_option("USER_ENTERED")
//         .doit()
//         .await
//         .map_err(|e| Box::new(e))?;

//     Ok(())
// }

pub async fn write_range(global_sheet_client: &GlobalSheetClient, id: &str, range: &str, value: Vec<Vec<String>>) -> SheetsResult<()> {
    let mut req: ValueRange = ValueRange::default();
    req.values = Some(to_json_vec(value));

    let client = global_sheet_client.get().ok_or(AppError::NotInitialized)?;
    let mut guard = client.lock().await;
    let client = guard.as_mut().ok_or(AppError::NotInitialized)?;

    client
        .spreadsheets()
        .values_update(req, id, range)
        .value_input_option("USER_ENTERED")
        .doit()
        .await
        .map_err(|e| Box::new(e))?;

    Ok(())
}

// pub async fn read_cell(global_sheet_client: &GlobalSheetClient, id: &str, range: &str) -> SheetsResult<String> {

//     let client = global_sheet_client.get().ok_or(AppError::NotInitialized)?;
//     let mut guard = client.lock().await;
//     let client = guard.as_mut().ok_or(AppError::NotInitialized)?;

//     let value = client
//         .spreadsheets()
//         .values_get(id, range)
//         .doit().await?;

//     let values = value.1.values.ok_or(AppError::NoValues)?;

//     let string_values = to_string_vec(values);

//     Ok(string_values.get(0).ok_or("Value not found").unwrap().get(0).ok_or("Value not found").unwrap().clone())
// }

pub async fn read_range(global_sheet_client: &GlobalSheetClient, id: &str, range: &str) -> SheetsResult<Vec<Vec<String>>> {

    let client = global_sheet_client.get().ok_or(AppError::NotInitialized)?;
    let mut guard = client.lock().await;
    let client = guard.as_mut().ok_or(AppError::NotInitialized)?;

    let value = client
        .spreadsheets()
        .values_get(id, range)
        .doit().await?;

    let values = value.1.values.ok_or(AppError::NoValues)?;

    let string_values = to_string_vec(values);

    Ok(string_values)
}

pub async fn get_info(global_sheet_client: &GlobalSheetClient, id: &str) -> SheetsResult<Spreadsheet> {
    let client = global_sheet_client.get().ok_or(AppError::NotInitialized)?;
    let mut guard = client.lock().await;
    let client = guard.as_mut().ok_or(AppError::NotInitialized)?;

    let value = client
        .spreadsheets()
        .get(id)
        .include_grid_data(false)
        .doit().await
        .map_err(|e| Box::new(e))?;

    return Ok(value.1);
}

pub async fn get_sheets(global_sheet_client: &GlobalSheetClient, id: &str) -> SheetsResult<Vec<Sheet>> {
    let info = get_info(global_sheet_client, id).await;

    return Ok(info.unwrap().sheets.unwrap()); // todo unsafe
}

pub async fn sheet_exists(global_sheet_client: &GlobalSheetClient, id: &str, sheet_name: &str) -> SheetsResult<bool> {
    let sheets = get_sheets(global_sheet_client, id).await.unwrap(); // todo unsafe

    for sheet in sheets.iter() {
        if sheet.properties.clone().unwrap().title.unwrap().to_string() == sheet_name {
            return Ok(true);
        }
    }

    return Ok(false);
}