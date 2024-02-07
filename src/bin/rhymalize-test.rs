use rhymalize::ipa_utils::fetching::*;
fn main() {
    // what to search
    let word = "example";

    let ipa_s = get_single(word);

    match ipa_s {
        Ok(a) => println!("{:?}", a),
        Err(e) => println!("{}", e),
    }

    let word_list = vec!["can't", "example"];

    let ipa_s_list = get_multiple(word_list);

    match ipa_s_list {
        Ok(a) => a.iter().for_each(|x| println!("{:?}\n", x)),
        Err(e) => println!("{}", e),
    }
}
