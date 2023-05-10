# March

A [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) crate for Rust.
This implementation should work for any item that implements [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) + [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) + [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html).

## Usage

```rust
use march::Chain;

fn main() {
    let mut chain = Chain::new();
    let sentence = "The quick brown fox jumped over the lazy dog".to_lowercase();
    let mut words = sentence.split_whitespace().into_iter();
    chain.feed(&mut words);

    let sentence = chain.generate();
    dbg!(sentence);
}
```

See [examples](examples/) for more usages.
