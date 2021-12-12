use crate::utils::Answer;
use regex::Regex;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}

fn to_node<'a>(s: &'a str) -> Node {
    match (s, s.to_uppercase().eq(s)) {
        ("start", _) => Node::Start,
        ("end", _) => Node::End,
        (x, false) => Node::Small(x.to_owned()),
        (x, true) => Node::Big(x.to_owned()),
    }
}

type Adjs = HashMap<Node, HashSet<Node>>;

//recursively find paths to end from given node
//if mulligan is available, ignore one instance of a repeated visit in a small cave
fn find_paths(adjs: &Adjs, mut mulligan: bool, mut visited: HashSet<Node>, from: Node) -> u32 {
    match &from {
        Node::End => return 1,
        Node::Big(_) => (),
        n @ _ => {
            if visited.contains(&n) {
                if !mulligan {
                    return 0;
                } else {
                    mulligan = false;
                }
            }
            visited.insert(n.clone());
        }
    }
    //recur for valid neighbours, returning sum of each recursion
    adjs.get(&from)
        .unwrap()
        .iter()
        .filter(|a| **a != Node::Start)
        .map(|a| find_paths(adjs, mulligan, visited.clone(), a.clone()))
        .sum()
}

pub fn day12(input: String) -> Answer {
    let mut answer = Answer::default();

    //parse into a hashmap of adjacencies
    let re = Regex::new(r"(\w+)-(\w+)").unwrap();
    let mut adjs: Adjs = HashMap::new();

    for c in re.captures_iter(&input) {
        let x = to_node(&c[1]);
        let y = to_node(&c[2]);
        adjs.entry(x.clone())
            .or_insert(HashSet::new())
            .insert(y.clone());
        adjs.entry(y).or_insert(HashSet::new()).insert(x.clone());
    }

    answer.record_parsed();

    //part 1: enumerate paths from start to end
    let p1 = find_paths(&adjs, false, HashSet::new(), Node::Start);
    answer.record(&p1);

    //part 1: enumerate paths from start to end
    let p2 = find_paths(&adjs, true, HashSet::new(), Node::Start);
    answer.record(&p2);

    answer
}
