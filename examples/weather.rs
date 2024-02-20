use also_sync::also_sync_tokio;

#[also_sync_tokio]
async fn get_weather() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get("https://wttr.in/")
        // Trick wttr.in into thinking it has been curled
        // so the output is terminal formatted, not HTML.
        .header("user-agent", "curl/7.88.1")
        .send()
        .await?
        .text()
        .await
}

fn main() {
    match get_weather_sync() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error fetching the weather from wttr.in: {}", e),
    }
}
