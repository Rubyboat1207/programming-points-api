use crate::sheet_helper::GlobalSheetClient;

#[async_trait]
pub trait SheetSerializable {
    async fn write_to_gsheet(&self, client: &GlobalSheetClient, args: SerializeArgs) -> Result<(), &'static str>;
    async fn read_from_gsheet(client: &GlobalSheetClient, args: SerializeArgs) -> Result<Box<Self>, &'static str>;
}

pub struct SerializeArgs {
    pub sheet: Option<String>,
    pub start_idx: Option<u32>
}

impl SerializeArgs {
    pub fn new() -> SerializeArgs{
        SerializeArgs {
            sheet: None,
            start_idx: None,
        }
    }
}