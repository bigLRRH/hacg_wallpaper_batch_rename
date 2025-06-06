// * 可以更抽象一点，不局限于字符串，可以是任意类型的序列
// * 但是这里为了方便，直接使用字符串
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
            node.weight += 1;
        }
    }

    pub fn generalized_longest_common_prefix(&self, min_weight: usize) -> String {
        let mut common_prefix = String::new();
        let mut node = &self.root;

        while let Some((c, next_node)) = node.get_max_weight_child(min_weight) {
            common_prefix.push(*c);
            node = next_node;
        }

        common_prefix
    }
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    weight: usize, // 记录当前节点的个数
}

impl TrieNode {
    fn new() -> Self {
        Default::default()
    }

    /// 获取权重最大的子节点，如果有多个，继续比较子节点的子节点
    fn get_max_weight_child(&self, min_weight: usize) -> Option<(&char, &TrieNode)> {
        self.children
            .iter()
            .filter(|(_, node)| node.weight >= min_weight)
            .max_by(|a, b| {
                a.1.weight.cmp(&b.1.weight).then_with(|| {
                    match (
                        // * 可能会重复比较同一个节点
                        // * 我想到一个绝妙的算法，但是我懒得写了
                        a.1.get_max_weight_child(min_weight),
                        b.1.get_max_weight_child(min_weight),
                    ) {
                        (Some((_, a_node)), Some((_, b_node))) => a_node.weight.cmp(&b_node.weight),
                        (Some(_), None) => std::cmp::Ordering::Greater,
                        (None, Some(_)) => std::cmp::Ordering::Less,
                        // * 实际上几乎不会出现这种情况，暂时不优化
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                })
            })
    }
}
