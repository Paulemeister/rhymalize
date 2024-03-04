use rhymalize::ipa_utils::ipa::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;

    let word = Word::try_from(buffer.trim_end()).unwrap();
    println!("{:#?}", word);
    Ok(())
}
