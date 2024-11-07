fn main() {
  println!("cargo:rerung-if-changed=../swagger.v3.json");

  let status = std::process::Command::new("cmd")
        .args(&["/C", "npx"])
        .arg("openapi-generator-cli")
        .arg("generate")
        .arg("-i")
        .arg("../swagger.v3.json")
        .arg("-g")
        .arg("rust")
        .arg("--additional-properties=library=reqwest,packageName=jira_api_client,useSingleRequestParameter=true")
        .status()
        .expect("Failed to run openapi-generator-cli");

    if !status.success() {
        panic!("OpenAPI generator failed with status: {}", status);
    }
}
