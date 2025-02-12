use futures::StreamExt;
use chromiumoxide::Browser;
use tokio;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the browser using the WebSocket URL
    let ws_url = "ws://localhost:9222/devtools/page/DD50EC9528F817695A5FCD5AD1B5C90B";
    println!("Connecting to WebSocket URL: {}", ws_url);

    let (mut browser, mut handler) = Browser::connect(ws_url).await?;
    println!("Successfully connected to the browser.");

    // Spawn a task to handle browser events
    let handle = tokio::spawn(async move {
        while let Some(event) = handler.next().await {
            if event.is_err() {
                eprintln!("Handler encountered an error: {:?}", event.unwrap_err());
                break;
            }
        }
    });

    // List all available targets (tabs)
    let targets = browser.fetch_targets().await?;
    println!("Available targets (tabs):");
    for target in &targets {
        println!(
            "ID: {:?}, URL: {:?}, Title: {:?}",
            target.target_id, target.url, target.title
        );
    }

    // Find the target with the desired URL
    let desired_url = "https://docs.rs/chromiumoxide/0.7.0/chromiumoxide/";
    let target = targets
        .into_iter()
        .find(|t| t.url == desired_url)
        .ok_or("Target not found")?;

    println!("Found target with URL: {}", target.url);

    // Add a small delay to ensure the tab is ready for attachment
    println!("Waiting for the tab to be ready...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Attach to the target
    let page = match browser.get_page(target.target_id).await {
        Ok(page) => {
            println!("Successfully attached to the tab.");
            page
        }
        Err(e) => {
            eprintln!("Failed to attach to the tab: {}", e);
            eprintln!("Ensure the tab is still open and accessible.");
            return Err(e.into());
        }
    };

    // Ensure the tab is pointing to the desired URL
    let current_url = page.url().await?;
    if let Some(url) = current_url {
        if url != desired_url {
            println!("Tab is not pointing to the desired URL. Navigating...");
            page.goto(desired_url).await?;
        }
    } else {
        eprintln!("Failed to retrieve the current URL.");
        return Ok(());
    }

    // Wait for the page to load completely
    println!("Waiting for navigation to complete...");
    page.wait_for_navigation().await?;

    // Locate the <a> element by its title attribute and click it
    println!("Locating the <a> element with title 'Example'...");
    let element = page
        .find_element("a[title='Example']")
        .await
        .map_err(|e| format!("Failed to find the element: {}", e))?;

    println!("Clicking the element...");
    element.click().await?;

    // Wait for any potential navigation or action triggered by the click
    println!("Waiting for any post-click navigation or action...");
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Close the browser
    println!("Closing the browser...");
    browser.close().await?;

    // Wait for the handler task to finish
    println!("Waiting for handler task to finish...");
    handle.await?;

    println!("Done!");
    Ok(())
}