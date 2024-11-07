use jira_api_client::apis::{configuration::Configuration, issues_api::GetIssueParams};
use reqwest::Method;
use tauri::Manager;
use tokio::sync::Mutex;
use crate::prelude::*;

mod error;
mod prelude;

#[derive(Default, Debug)]
struct AppState {
  pub configuration: Configuration
}

impl AppState {
  pub fn new() -> Self {
    AppState { configuration: Configuration {
      base_path: "http://localhost:8080".to_string(),
      basic_auth: Some(("admin".to_string(), Some("admin".to_string()))),
      ..Configuration::new()
    } }
  }
}

pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      app.manage(Mutex::new(AppState::new()));
      Ok(())
    })
    .plugin(tauri_plugin_log::Builder::new()
      .level(log::LevelFilter::Trace)
      .level_for("reqwest", log::LevelFilter::Trace)
      .build()
    )
    .invoke_handler(tauri::generate_handler![search, get_issue])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn search(query: Option<String>) -> Result<String> {
  log::info!("Searching with JQL query: {:?}", query);
  let response = reqwest::Client::new()
    .request(Method::GET, "http://localhost:8080/rest/api/2/search")
    .basic_auth("admin", Some("admin"))
    .query(&[("jql", &query.unwrap_or("".to_string()))])
    .header("Accept", "application/json")
    .send()
    .await?
    .text()
    .await?;

  Ok(response)
}

#[tauri::command]
async fn get_issue(state: tauri::State<'_, Mutex<AppState>>, id: String) -> Result<String> {
  log::info!("Requesting data for issue {:?}", id);

  let configuration = state.lock().await.configuration.clone();
  let params = GetIssueParams { issue_id_or_key: id, fields: None, fields_by_keys: None, expand: None, properties: None, update_history: None, fail_fast: None };

  let response = match jira_api_client::apis::issues_api::get_issue(&configuration, params).await {
    Ok(res) => format!("{}", serde_json::to_string(&res).unwrap()),
    Err(e) => e.to_string()
  };

  Ok(response)
}
