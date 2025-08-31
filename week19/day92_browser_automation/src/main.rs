use fantoccini::{Client, Locator};
use tokio;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    println!("🕸️ Launching browser automation...");

    // Connect to WebDriver (ChromeDriver or GeckoDriver)
    let mut client = Client::new("http://localhost:9515").await?;

    // Navigate to a page
    client.goto("https://www.rust-lang.org").await?;

    // Get the page title
    let title = client.title().await?;
    println!("📄 Page Title: {}", title);

    // Click the "Learn" link if it exists
    if let Ok(link) = client.find(Locator::LinkText("Learn")).await {
        println!("🔗 Clicking 'Learn' link...");
        link.click().await?;
    }

    // Take a screenshot (optional)
    let screenshot = client.screenshot().await?;
    std::fs::write("screenshot.png", &screenshot).expect("❌ Failed to save screenshot");
    println!("📸 Screenshot saved to screenshot.png");

    // Close the session
    client.close().await?;

    Ok(())
}