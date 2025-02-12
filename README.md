Step 1: Open chrome in debug mode. On Mac, you can do it as follow in terminal
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222 --user-data-dir=~/Library/Application\ Support/Google/Chrome 

Step 2: on newly opened browser, go this website: https://docs.rs/chromiumoxide/0.7.0/chromiumoxide/
On second tab: http://localhost:9222/json

Step 3: Find the websocket url of the rust website. e.g. ws://localhost:9222/devtools/page/289FADBCA79B9DC520DD0CC261733068

Step 4: In Rust main.rs, line number 9, update it "ws://localhost:9222/devtools/page/289FADBCA79B9DC520DD0CC261733068"

Step 5: cargo run

then check the output


clicking, typing, dropdown

1. Verify Selector: Use browser dev tools (e.g., Chrome's "Inspect" tool) to test the selector in the console:
document.querySelector("input.search-input[name='search'][aria-label='Run search in the documentation']")

2. Wait for Element: If the element is dynamically loaded, use page.find_element before interacting with it:
page.find_element("input.search-input[name='search'][aria-label='Run search in the documentation']").await?;

3. Click a button
// Locate and click a button
let button = page.find_element("button#your-button-id").await?;
button.click().await?;

4. Click & Dropdown
// Locate and click the dropdown to open it
let dropdown = page.find_element("select#your-dropdown-id").await?;
dropdown.click().await?;

// Select an option from the dropdown
let option = page.find_element("select#your-dropdown-id option[value='your-option-value']").await?;
option.click().await?;

