use crate::config::AppConfig;

async fn check_token(token: &str, remote_path: &str) -> bool {
    // Placeholder for token validation logic
    // In a real implementation, this would involve making a request to the remote server
    // to verify the token's validity.
    println!(
        "Validating token '{}' for remote path '{}'",
        token, remote_path
    );
    true
}

pub async fn run_login(token: String, remote_path: String) {
    let result = check_token(&token, &remote_path).await;

    if !result {
        panic!("Invalid token or remote path.");
    }

    let config = AppConfig::new(remote_path, token);
    match config.save_config("config.toml") {
        Ok(_) => println!("Configuration saved successfully."),
        Err(e) => panic!("Failed to save configuration: {}", e),
    }
}

pub async fn run_logout() {
    println!("Logging out...");

    match AppConfig::delete_config("config.toml") {
        Ok(_) => println!("Configuration deleted successfully."),
        Err(e) => panic!("Failed to delete configuration: {}", e),
    }
}
