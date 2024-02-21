use std::vec;

use crate::{sheet_helper::{read_range, write_range}, sheet_serialize::{SerializeArgs, SheetSerializable}, SHEET_ID};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub name: String,
    pub points: i32,
    pub currency: String,
    pub pass: String,
    pub history: Vec<HistoryEntry>
}

impl Account {
    pub async fn get_account(client: &crate::sheet_helper::GlobalSheetClient, name: &str) -> Result<Box<Account>, &'static str> {
        Account::read_from_gsheet(client, SerializeArgs {sheet: Some((name.to_string() + "_ACCOUNT").to_string()), start_idx: None}).await
    }
}

#[async_trait]
impl SheetSerializable for Account {
    async fn write_to_gsheet(&self, client: &crate::sheet_helper::GlobalSheetClient, _args: SerializeArgs) -> Result<(), &'static str> {
        let val = write_range(client, SHEET_ID, (self.name.clone() + "_ACCOUNT!A:E").as_str(), vec![
            vec![self.points.to_string()],
            vec![self.currency.clone()],
            vec![self.pass.clone()]
        ]).await;

        if val.is_err() {
            println!("{}", val.unwrap_err());
            return Err("Failed");
        }

        for (index, entry) in self.history.iter().enumerate() {
            let mut args = SerializeArgs::new();

            args.sheet = Some(self.name.clone() + "_ACCOUNT");
            args.start_idx = Some((index + 4) as u32);
            let val2 = entry.write_to_gsheet(client, args).await;

            if val2.is_err() {
                println!("{}", val2.unwrap_err());
            }
        }

        Ok(())
    }

    async fn read_from_gsheet(client: &crate::sheet_helper::GlobalSheetClient, args: SerializeArgs) -> Result<Box<Self>, &'static str> {
        let sheet_name: String = args.sheet.ok_or("Sheet name not provided!")?;

        let sheet_result = read_range(client, SHEET_ID, (sheet_name.clone() + "!A:D").as_str()).await;

        if sheet_result.is_err() {
            println!("{}", sheet_result.unwrap_err());
            return Err("failure");
        }

        let sheet_value = read_range(client, SHEET_ID, (sheet_name.clone() + "!A:D").as_str()).await.unwrap();

        let mut result = Account {
            name: sheet_name[..(sheet_name.len() - 8)].to_string(),
            points: sheet_value[0][0].parse::<i32>().unwrap().clone(),
            currency: sheet_value[1][0].to_string(),
            pass: sheet_value[2][0].to_string(),
            history: vec![]
        };

        for (idx, row) in sheet_value.iter().enumerate() {
            // skip account variables
            if idx < 3 {
                continue;
            }

            if row[0].is_empty() {
                break;
            }

            let mut args = SerializeArgs::new();

            args.start_idx = Some(idx as u32 + 1);
            args.sheet = Some(sheet_name.clone());

            let entry = HistoryEntry::read_from_gsheet(client, args).await;
            
            match entry {
                Ok(value) => result.history.push(value.as_ref().clone()),
                Err(e) => println!("{}", e),
            }
        }

        Ok(Box::new(result))
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct HistoryEntry {
    pub event_name: String,
    pub difference: i32,
    pub prev_bal: i32,
    pub post_bal: i32,
}

#[async_trait]
impl SheetSerializable for HistoryEntry {
    async fn write_to_gsheet(&self, client: &crate::sheet_helper::GlobalSheetClient, args: SerializeArgs) -> Result<(), &'static str> {
        let sheet_name: String = args.sheet.ok_or("Sheet name not provided!")?;
        let idx: u32 = args.start_idx.ok_or("Index not provided!")?;



        let val = write_range(client, SHEET_ID, &(sheet_name + "!A" + idx.to_string().as_str() + ":D" + idx.to_string().as_str()), vec![
            vec![self.event_name.clone(), self.difference.to_string(), self.prev_bal.to_string(), self.post_bal.to_string()]
        ]).await;

        if val.is_err() {
            println!("{}", val.unwrap_err());
            return Err("Failed");
        }

        Ok(())
    }

    async fn read_from_gsheet(client: &crate::sheet_helper::GlobalSheetClient, args: SerializeArgs) -> Result<Box<Self>, &'static str> {
        let sheet_name: String = args.sheet.ok_or("Sheet name not provided!")?;
        let idx: u32 = args.start_idx.ok_or("Index not provided!")?;



        let value = read_range(client, SHEET_ID, &(sheet_name + "!A" + idx.to_string().as_str() + ":D" + idx.to_string().as_str())).await.unwrap();
        
        

        Ok(Box::new(HistoryEntry {
            event_name: value[0][0].clone(),
            difference: value[0][1].parse::<i32>().unwrap(),
            prev_bal: value[0][2].parse::<i32>().unwrap(),
            post_bal: value[0][3].parse::<i32>().unwrap(),
        }))
    }
}