use rhymalize::*;
fn main() {
    // what to search
    let word = "example";

    let ipa_s = get_single(word);

    match ipa_s {
        Ok(a) => println!("{:?}", a),
        Err(e) => println!("{}", e),
    }
}
