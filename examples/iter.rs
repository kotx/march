use march::Chain;

fn main() {
    let mut chain = Chain::new();
    let sentence = "The quick brown fox jumped over the lazy dog".to_lowercase();
    let mut words = sentence.split_whitespace().into_iter();
    chain.feed(&mut words);

    let mut sentence = chain.generate_iter().take(10); // generate at most 10 items
    while let Some(&word) = sentence.next() {
        print!("{} ", word);
    }

    println!();
}