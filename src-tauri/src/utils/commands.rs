use crate::shared::{dex, party, DexMon};

#[tauri::command]
pub async fn get_dex() -> Vec<DexMon> {
    dex::load_dex()
}

#[tauri::command]
pub async fn get_party() -> Result<String, String> {
    party::load_party()
        .map_err(|e| e.to_string())
        .map(|party| serde_json::to_string(&party).unwrap())
}
