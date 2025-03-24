use std::collections::HashMap;

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
            node.count += 1;
        }
    }

    // todo 修bug
    pub fn generalized_longest_common_prefix(&self, min_count: usize) -> String {
        let mut common_prefix = String::new();
        let mut node = &self.root;

        loop {
            let candidates: Vec<_> = node
                .children
                .iter()
                .filter(|(_, child)| child.count >= min_count)
                .collect();

            if candidates.is_empty() {
                break;
            }

            let (c, next_node) = candidates
                .iter()
                .max_by_key(|(_, child)| child.count)
                .unwrap();

            common_prefix.push(**c);
            node = next_node;
        }

        common_prefix
    }
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    count: usize,
}

impl TrieNode {
    fn new() -> Self {
        Default::default()
    }

    fn is_end(&self) -> bool {
        self.children.is_empty()
    }
}
