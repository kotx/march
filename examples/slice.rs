use march::Chain;

fn main() {
    let mut chain: Chain<u8> = Chain::new();
    chain.feed([1u8, 2, 3, 5]).feed([3u8, 9, 2]);

    let sentence = chain.generate();
    dbg!(sentence);
}