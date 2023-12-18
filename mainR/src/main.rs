use reqwest;
use scraper::{Html, Selector};
use std::env;
use fantoccini::{Client, ClientBuilder};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let username = env::var("USERNAME")?;
    let password = env::var("PASSWORD")?;

    schema_mau_se_login(&username, &password).await?;
    testing(&username, &password).await?;

    Ok(())
}

async fn schema_mau_se_login(username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {

    /*
    let client = reqwest::Client::new();
    let params = [("username", username), ("password", password)];
    let response = client.post("https://schema.mau.se/login_do.jsp")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    // Check and print the status code
    println!("Response Status: {}", response.status());

    // Optionally, inspect and print headers
    for (key, value) in response.headers() {
        println!("{}: {:?}", key, value);
    }

    let body = response.text().await?;
    
    // Temporarily print the entire response body for inspection
    println!("Response Body: {}", body);

    let document = Html::parse_document(&body);
    let selector = Selector::parse(".title").unwrap();
    println!("Selector: {:?}", selector);


    let mut found_message = false;
    for element in document.select(&selector) {
        let login_failed_message = element.text().collect::<Vec<_>>().join("");
        println!("Login failed message: {}", login_failed_message);
        found_message = true;
        break;
    }

    if !found_message {
        // Optionally, look for specific content in the body
        // if body.contains("specific failure indication text") {
        //     println!("Login failed due to specific reason.");
        // } else {
            println!("Login successful!");
        // }
    }

    */
    Ok(())
}


async fn testing(username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg("https://schema.mau.se/login_do.jsp")
        .arg("-H")
        .arg("Content-Type: application/x-www-form-urlencoded")
        .arg("-d")
        .arg("username=YOUR_USERNAME&password=YOUR_PASSWORD")
        .output()?;

    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout);
        println!("Response: {}", response);
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", error_message);
    }

    Ok(())
}
