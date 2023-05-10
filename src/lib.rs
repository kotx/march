//! A [Markov chain](https://en.wikipedia.org/wiki/Markov_chain) crate for Rust.
//! This implementation should work for any item that implements [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) + [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) + [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html).
//! ## Usage
//!
//! ```rust
//! use march::Chain;
//!
//! fn main() {
//!     let mut chain = Chain::new();
//!     let sentence = "The quick brown fox jumped over the lazy dog".to_lowercase();
//!     let mut words = sentence.split_whitespace().into_iter();
//!     chain.feed(&mut words);
//!
//!     let sentence = chain.generate();
//!     dbg!(sentence);
//! }
//! ```

use std::collections::HashMap;
use std::hash::Hash;

use petgraph::visit::{EdgeRef, GraphBase, IntoEdges};
use petgraph::{graph::NodeIndex, visit::Walker, Graph};

use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Item<T> {
    Start,
    End,
    Data(T),
}

impl<T> From<T> for Item<T> {
    fn from(value: T) -> Self {
        Item::Data(value)
    }
}

type ChainGraph<T> = Graph<Item<T>, u32>;
type NodeId<T> = <ChainGraph<T> as GraphBase>::NodeId;

pub struct Chain<T> {
    pub graph: ChainGraph<T>,
    pub start: NodeId<T>,
    pub end: NodeId<T>,
    words: HashMap<Item<T>, NodeId<T>>, // TODO: Is this inefficient? This stores an item twice (in the map and the graph)
}

impl<T> Chain<T>
where
    T: Hash + Eq + Clone,
{
    /// Creates a new Markov chain with start and end nodes.
    pub fn new() -> Chain<T> {
        let mut graph = Graph::new();
        let start = graph.add_node(Item::Start);
        let end = graph.add_node(Item::End);

        Chain {
            graph,
            start,
            end,
            words: HashMap::new(),
        }
    }

    /// Increments the weight of an edge between `a` and `b` by 1.
    pub fn bump_edge(&mut self, a: NodeId<T>, b: NodeId<T>) {
        let mut weight = 1;
        if let Some(edge) = self.graph.edges_connecting(a, b).next() {
            weight += edge.weight();
        }

        self.graph.update_edge(a, b, weight);
    }

    /// If necessary, creates a node and returns it.
    pub fn ensure_node(&mut self, item: T) -> NodeIndex {
        if let Some(&node) = self.words.get(&Item::Data(item.clone())) {
            node
        } else {
            let node = self.graph.add_node(item.clone().into());
            self.words.insert(item.into(), node);
            node
        }
    }

    /// Feeds a sequence of items into the chain.
    pub fn feed(&mut self, items: impl IntoIterator<Item = T>) -> &mut Self {
        let mut items = items.into_iter();

        let mut prev = self.start;
        while let Some(item) = items.next() {
            let node = self.ensure_node(item);

            self.bump_edge(prev, node);

            prev = node;
        }

        if prev != self.start {
            self.bump_edge(prev, self.end);
        }

        self
    }

    /// Sample words from the chain.
    pub fn generate(&self) -> Vec<&T> {
        let mut items = Vec::new();
        let mut walker = self.walker().iter(&self.graph);
        while let Some(idx) = walker.next() {
            let item = &self.graph[idx];
            if let Item::Data(data) = item {
                items.push(data);
            } else {
                break;
            }
        }

        items
    }

    /// Returns an iterator that samples random words from the chain.
    pub fn generate_iter(&self) -> impl Iterator<Item = &T> {
        self.walker()
            .iter(&self.graph)
            .filter_map(|idx| match &self.graph[idx] {
                Item::Data(data) => Some(data),
                Item::End => None,
                Item::Start => unreachable!(),
            })
            .into_iter()
    }

    fn walker(&self) -> RandomWalk<NodeId<T>, ThreadRng> {
        RandomWalk::new(self.start)
    }
}

pub struct RandomWalk<N, R: Rng> {
    current: N,
    rng: R,
}

impl<N> RandomWalk<N, ThreadRng>
where
    N: Copy,
{
    pub fn new(start: N) -> RandomWalk<N, ThreadRng> {
        RandomWalk::with_rng(start, thread_rng())
    }
}

impl<N, R: Rng> RandomWalk<N, R>
where
    N: Copy,
{
    pub fn with_rng(start: N, rng: R) -> RandomWalk<N, R> {
        RandomWalk {
            current: start,
            rng,
        }
    }

    fn next<G>(&mut self, graph: G) -> Option<N>
    where
        G: IntoEdges<NodeId = N, EdgeWeight = u32>,
    {
        let edges: Vec<_> = graph.edges(self.current).collect();
        if edges.len() == 0 {
            None
        } else {
            let weights = edges.iter().map(|e| e.weight());
            let dist = WeightedIndex::new(weights).expect("couldn't create weighted index");
            let idx = dist.sample(&mut self.rng);
            // let idx = self.rng.gen_range(0..edges.len());
            let node = edges[idx].target();

            self.current = node;
            Some(self.current)
        }
    }
}

impl<G, R: Rng> Walker<G> for RandomWalk<G::NodeId, R>
where
    G: IntoEdges<EdgeWeight = u32>,
{
    type Item = G::NodeId;

    fn walk_next(&mut self, context: G) -> Option<Self::Item> {
        self.next(&context)
    }
}
