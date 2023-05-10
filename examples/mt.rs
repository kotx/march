/// Load an exported chain from MT (https://mt.ziad87.net)
/// Usage: mt [chain.json]

use march::Chain;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Instant};

#[derive(Serialize, Deserialize)]
pub struct MtChains {
    chains: HashMap<String, Vec<MtChain>>,
    starter: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MtChain {
    Bool(bool),
    String(String),
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: mt [chain.json]");
        std::process::exit(1);
    }
    
    let file = std::fs::read_to_string(&args[1]).unwrap();
    let mt_chains: MtChains = serde_json::from_str(&file).unwrap();

    let mut chain = Chain::new();

    let now = Instant::now();

    for key in mt_chains.starter {
        let node = chain.ensure_node(key.clone());
        chain.bump_edge(chain.start, node);
    }

    for (key, targets) in mt_chains.chains {
        let key_node = chain.ensure_node(key.clone().into());

        for target in targets {
            let target = match target {
                MtChain::Bool(true) => chain.end,
                MtChain::String(word) => chain.ensure_node(word.into()),
                _ => panic!(),
            };

            chain.bump_edge(key_node, target);
        }
    }

    let elapsed = now.elapsed();
    println!("Chain loaded in {elapsed:.2?}");

    let now = Instant::now();
    let sentence = chain.generate();
    let elapsed = now.elapsed();
    dbg!(sentence);
    println!("Generated in {elapsed:.2?}");
}
