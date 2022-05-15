use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::iter::Iterator;

// See if &str fidd by one character
fn diff_one_shot(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        false
    } else {
        let mut counter = 0;
        for i in 0..a.len() {
            let diff = a.chars().nth(i).unwrap() == b.chars().nth(i).unwrap();
            if diff {
                counter += 1;
            }
        }
        counter == 2
    }
}

// Iters words we are matching on
fn find_next_words(node: &str, words: Vec<&str>) -> BTreeSet<String> {
    let mut v = BTreeSet::new();
    for word in words.iter() {
        let m = diff_one_shot(node, word);
        if m {
            v.insert(word.to_string());
        }
    }
    v
}

// Find our word ladder of begin word to end word of one char diff.  Return Vec Solution.
fn find_word_ladder(begin_word: &str, end_word: &str, word_list: Vec<&str>) -> Vec<String> {
    // Vec to BTreeSet
    let mut word_list_set = BTreeSet::new();
    for word in word_list.clone() {
        word_list_set.insert(word.to_string());
    }

    // Build a Dequeue to store our nodes for BFS
    let mut node_list: VecDeque<(String, BTreeSet<String>)> =
        VecDeque::from([(begin_word.to_string(), word_list_set)]);

    // Go huntin'
    while !node_list.is_empty() {
        // Grab left most node
        if let Some((node, path)) = node_list.pop_front() {
            // Find words minus paths we already found
            let node_next_words = find_next_words(&node, word_list.to_vec());
            let next_words: BTreeSet<String> = (&path - &node_next_words).iter().cloned().collect();
            // Iter those words
            for next in next_words.iter() {
                // Solution??
                if next == end_word {
                    let mut solution = vec![begin_word.to_string()];
                    for p in path {
                        solution.push(p);
                    }
                    return solution;
                // No?  Add the next nodes for huntin'
                } else {
                    let mut new_path = path.clone();
                    new_path.insert(next.clone());
                    let mut new_node_list: VecDeque<_> = [(next.clone(), new_path)].into();
                    node_list.append(&mut new_node_list);
                }
            }
        }
    }
    vec![]
}

fn main() {
    let begin_word = "hit";
    let end_word = "cog";
    let word_list = ["hot", "dot", "dog", "lot", "log", "cog"];
    let word_ladder = find_word_ladder(begin_word, end_word, word_list.to_vec());
    println!("Begin Word: {} End Word: {}", begin_word, end_word);
    println!("Solution: {:?}", word_ladder);
}
