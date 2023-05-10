use march::Chain;

fn main() {
    let mut chain = Chain::new();
    let sentence = "The quick brown fox jumped over the lazy dog".to_lowercase();
    let mut words = sentence.split_whitespace().into_iter();
    chain.feed(&mut words);

    let sentence: Vec<&str> = chain.generate_iter().copied().collect();
    println!("{}", sentence.join(" "));
}