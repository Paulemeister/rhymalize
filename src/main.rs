extern crate regex;
extern crate reqwest;

use regex::Regex;

fn main() {
    // Define the word for which you want to retrieve the IPA pronunciation
    let word = "example";

    // Make the API request to search for the word
    let search_url = format!(
        "https://en.wiktionary.org/w/api.php?action=query&format=json&list=search&srsearch={}",
        word
    );
    let search_response = reqwest::blocking::get(&search_url).unwrap().text().unwrap();

    // Parse the search response to get the page title
    let page_title = parse_search_response(&search_response);

    if let Some(title) = page_title {
        // Make the API request to fetch the page content
        let page_url = format!(
            "https://en.wiktionary.org/w/api.php?action=query&format=json&prop=revisions&rvprop=content&titles={}",
            title
        );
        let page_response = reqwest::blocking::get(&page_url).unwrap().text().unwrap();

        // Parse the page response to extract the IPA pronunciation
        let ipa = parse_page_response(&page_response);

        println!("IPA pronunciation for {}: {}", word, ipa);
    } else {
        println!("No results found for the word {}", word);
    }
}

// Function to parse the search response and extract the page title
fn parse_search_response(response: &str) -> Option<String> {
    let json: serde_json::Value = serde_json::from_str(response).unwrap();
    if let Some(page) = json["query"]["search"][0].as_object() {
        if let Some(title) = page.get("title") {
            return Some(title.to_string());
        }
    }
    None
}

// Function to parse the page response and extract the IPA pronunciation
fn parse_page_response(response: &str) -> String {
    let re = Regex::new(r"{{IPA\|/([^/]+)/(?:\|lang=\w+)?}}").unwrap();
    if let Some(captures) = re.captures(response) {
        return captures[1].to_string();
    }
    String::new()
}
