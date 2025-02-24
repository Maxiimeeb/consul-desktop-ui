use serde_json::Value;
use tauri::AppHandle;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod consul;

#[tauri::command]
async fn get_consul_values(
    app: AppHandle,
    consul_client: consul::client::ConsulClient,
) -> Value {
    let values = consul::ConsulValues::new_from_client(&consul_client).await;
    Value::Object(values.to_json())
}

#[tauri::command]
async fn save_consul_values(
    app: AppHandle,
    consul_client: consul::client::ConsulClient,
    initial_values: Value,
    new_values: Value,
) -> Result<Value, String> {
    let new_values = consul::ConsulValues::new_from_json(new_values.as_object().unwrap().clone())?;
    let initial_values = consul::ConsulValues::new_from_json(initial_values.as_object().unwrap().clone())?;
    let result = consul::update_consul(&consul_client, &initial_values, &new_values).await;

    match result {
        Ok(_) => {},
        Err(e) => return Err(e.to_string())
    }

    Ok(Value::Object(new_values.to_json()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_consul_values, save_consul_values])
        // .invoke_handler(tauri::generate_handler![save_consul_values])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
