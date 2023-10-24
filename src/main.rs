use std::cmp;
use std::cmp::Ord;
use std::collections::HashMap;
use std::iter::zip;
use std::{
    collections::BinaryHeap,
    io::{self, BufRead},
};

struct MinMaxHeap<A> {
    min_heap: BinaryHeap<cmp::Reverse<A>>,
    max_heap: BinaryHeap<A>,
}

impl<A: Ord + Clone> MinMaxHeap<A> {
    fn push(&mut self, x: A) {
        self.min_heap.push(cmp::Reverse(x.clone()));
        self.max_heap.push(x);
    }

    fn peek_min(&self) -> Option<&A> {
        self.min_heap.peek().map(|rev| &rev.0)
    }

    fn peek_max(&self) -> Option<&A> {
        self.max_heap.peek()
    }
}

struct GeneData {
    health: i32,
    ix: i32,
}

struct DNA {
    genes: String,
    first_ix: i32,
    last_ix: i32,
}

impl DNA {
    fn health(&self, health_table: HealthTable) -> i64 {}
}

// store the DNA strings (first, last, d) with the goal
// of finding the minimum and maximum by health
// can we iterate over the set of DNA and insert
// each one into a heap based on health?
// maybe we keep both a min-heap and a max-heap?

// Calculating the health of a DNA string
// for each i in DNA
// find the longest substring i..j that is a gene
// add the of gene i..j to the total for that DNA

type HealthTable = HashMap<String, Vec<GeneData>>;

fn longest_gene(s: String, health_table: HealthTable) -> Option<String> {
    let mut longest = Vec::with_capacity(s.len());

    for c in s.chars() {
        let k: String = longest.iter().chain([c].iter()).collect();

        if health_table.contains_key(&k) {
            longest.push(c);
        } else {
            break;
        }
    }

    if longest.is_empty() {
        None
    } else {
        Some(longest.iter().collect())
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let genes: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let health: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let s = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut health_table: HealthTable = HashMap::with_capacity(genes.len());

    let gene_data = health.iter().enumerate().map(|(ix, h)| GeneData {
        ix: ix as i32,
        health: *h,
    });

    zip(genes.into_iter(), gene_data).for_each(|(gene, data)| {
        if let Some(ds) = health_table.get_mut(&gene) {
            ds.push(data);
        } else {
            health_table.insert(gene, vec![data]);
        }
    });

    let mut dnas = Vec::with_capacity(s as usize);

    for _ in 0..s {
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let first = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let last = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let d = &first_multiple_input[2];

        dnas.push(DNA {
            genes: *d,
            first_ix: first,
            last_ix: last,
        })
    }
}
