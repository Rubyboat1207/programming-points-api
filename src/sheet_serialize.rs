use crate::sheet_helper::GlobalSheetClient;

pub trait SheetSerializable {
    fn serialize(client: GlobalSheetClient);
    fn deserialize(client: GlobalSheetClient);
}