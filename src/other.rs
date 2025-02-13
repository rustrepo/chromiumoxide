use futures::StreamExt;
use chromiumoxide::Browser;
use tokio;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the browser using the WebSocket URL
    let ws_url = "ws://localhost:9222/devtools/page/5136F7DEE34F893A85BD72A6ED2391F3";
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
    let desired_url = "https://www.rust-example.com/";
    let target = targets
        .into_iter()
        .find(|t| t.url == desired_url)
        .ok_or("Target not found")?;

    println!("Found target with URL: {}", target.url);

    // Clone the target_id to avoid moving it in the loop
    let target_id = target.target_id.clone();

    // Add a small delay to ensure the target is ready for attachment
    println!("Waiting for the target to be ready...");
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Retry mechanism for attaching to the target
    let mut retries = 3;
    let page = loop {
        match browser.get_page(target_id.clone()).await {
            Ok(page) => {
                println!("Successfully attached to the tab.");
                break page;
            }
            Err(e) if retries > 0 => {
                eprintln!("Failed to attach to the tab (retries left: {}): {}", retries, e);
                retries -= 1;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                eprintln!("Failed to attach to the tab after retries: {}", e);
                return Err(e.into());
            }
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

    // Locate the search input element and interact with it
    let search_input = page
        .find_element("input.search-input[name='search'][aria-label='Run search in the documentation']")
        .await?;

    println!("Clicking the search input...");
    search_input.click().await?;

    println!("Typing 'Type' into the search input...");
    search_input.type_str("Type").await?;

    println!("Pressing 'Enter'...");
    search_input.press_key("Enter").await?;

    // Wait for the search results to load
    println!("Waiting for search results to load...");
    println!("Waiting for the search tabs element to appear...");
    let search_tabs = page
    .find_element("#search-tabs")
    .await
    .map_err(|e| format!("Failed to find the search tabs element: {}", e))?;

    println!("Search tabs element found: {:?}", search_tabs);

    // Wait for the handler task to finish
    

    println!("Done!");
    Ok(())
}
