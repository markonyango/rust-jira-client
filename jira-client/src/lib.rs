use jira_api_client::apis::{configuration::Configuration, filters_api::{GetFavouriteFiltersParams, GetFilterParams}, issues_api::GetIssueParams};
use reqwest::Method;
use serde_json::json;
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
      basic_auth: None,
      ..Configuration::new()
    } }
  }

  pub fn authenticate(&mut self, username: String, password: String) {
    self.configuration.basic_auth = Some((username, Some(password)));
  }

  pub fn logout(&mut self) {
    self.configuration.basic_auth = None;
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
    .invoke_handler(tauri::generate_handler![search, get_issue, get_filters, get_filter, authenticate, logout])
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

#[tauri::command]
async fn get_filters(state: tauri::State<'_, Mutex<AppState>>) -> Result<String> {
  let configuration = state.lock().await.configuration.clone();
  let params = GetFavouriteFiltersParams { expand: None };

  let response = match jira_api_client::apis::filters_api::get_favourite_filters(&configuration, params).await {
    Ok(result) => format!("{}", serde_json::to_string(&result).unwrap()),
    Err(e) => e.to_string()
  };

  Ok(response)
}

#[tauri::command]
async fn get_filter(state: tauri::State<'_, Mutex<AppState>>, id: i64) -> Result<String> {
  let configuration = state.lock().await.configuration.clone();
  let params = GetFilterParams { id, expand: None, override_share_permissions: None };

  let response = match jira_api_client::apis::filters_api::get_filter(&configuration, params).await {
    Ok(result) => format!("{}", serde_json::to_string(&result).unwrap()),
    Err(e) => e.to_string()
  };

  Ok(response)
}

#[tauri::command]
async fn authenticate(state: tauri::State<'_, Mutex<AppState>>, username: String, password: String) -> Result<String> {
  state.lock().await.authenticate(username, password); 

  Ok(json!({ "message": "authenticated" }).to_string())
}

#[tauri::command]
async fn logout(state: tauri::State<'_, Mutex<AppState>>) -> Result<String> {
  state.lock().await.logout(); 

  Ok(json!({ "message": "logged out" }).to_string())
}
