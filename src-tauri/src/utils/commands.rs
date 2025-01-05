use crate::data::{Dex, Party};

#[tauri::command]
pub async fn get_dex() -> Dex {
    Dex::load().expect("Error while getting dex.")
}

#[tauri::command]
pub async fn get_collection() -> Party {
    Party::load().expect("Error while getting collection.")
}
