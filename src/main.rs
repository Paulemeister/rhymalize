use serde_json::{from_str, Value};

fn main() {
    // what to search
    let word = "example";

    // search for word and get first hit
    let search_url = format!(
        "https://en.wiktionary.org/w/api.php?format=json&action=query&list=search&srprop=redirecttitle&srlimit=1&srsearch={}",
        word
    );
    let search_response_str = reqwest::blocking::get(&search_url).unwrap().text().unwrap();
    let search_response = from_str::<Value>(search_response_str.as_str()).unwrap();
    // id of first hit
    let first_hit_id: &i64 = &search_response["query"]["search"][0]["pageid"]
        .as_i64()
        .unwrap();

    // get the content of the "pronunciation" section (id=3 ?) of the current revision
    let search_url = format!("https://en.wiktionary.org/w/api.php?format=json&action=query&pageids={}&prop=revisions&rvslots=main&rvprop=content&rvsection=3",first_hit_id);

    let pron_sec_res = from_str::<Value>(
        reqwest::blocking::get(&search_url)
            .unwrap()
            .text()
            .unwrap()
            .as_str(),
    )
    .unwrap();

    // store the text of the section
    let pron_sec = pron_sec_res["query"]["pages"][first_hit_id.to_string()]["revisions"][0]
        ["slots"]["main"]["*"]
        .as_str()
        .unwrap();

    // filter out only the ipa pronunciations
    let ipa_s = pron_sec
        .split('\n')
        .filter(|&x| x.starts_with("* {{a|")) // filter wanted lines
        .flat_map(|x| {
            x.split("{{")
                .last()
                .unwrap() // get second parethesis
                .split(['|', '}'])
                .skip(2)
                .filter(|&x| !x.is_empty()) // filter out the last two '{'
        })
        .collect::<Vec<&str>>();

    println!("{:?}", ipa_s);
}
