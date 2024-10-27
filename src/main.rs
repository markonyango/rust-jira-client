use openapi::{apis::{configuration::Configuration, issues_api::{bulk_fetch_issues, get_issue}}, models::BulkFetchIssueRequestBean};

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.basic_auth = Some(("admin".to_string(), Some("admin".to_string())));
    configuration.base_path = "http://localhost:8080".to_string();
    let request_bean = BulkFetchIssueRequestBean::new(vec!["OPD-123".to_string()]);
    match bulk_fetch_issues(&configuration, request_bean).await {
    Ok(issues) => println!("{:?}", issues),
    Err(e) => eprintln!("{:?}", e),
    }

    match get_issue(&configuration, "OPD-1",None, None, None, None, None, None).await {
    Ok(issue) => println!("{:#?}", issue),
    Err(e) => eprintln!("{:?}", e),
    }
}
