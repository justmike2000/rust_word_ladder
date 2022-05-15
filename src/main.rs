use linked_hash_set::LinkedHashSet;
use std::collections::VecDeque;
use std::iter::Iterator;

// See if &str fidd by one character
fn diff_one_shot(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false
    }
    let mut counter = 0;
    for i in 0..a.len() {
        if a.chars().nth(i).unwrap() == b.chars().nth(i).unwrap() {
            counter += 1;
        }
    }
    counter == 2
}

// Iters words we are matching on
fn find_next_words(node: &str, words: LinkedHashSet<String>) -> LinkedHashSet<String> {
    let mut found_words = LinkedHashSet::new();
    for word in words.iter() {
        if diff_one_shot(node, word) {
            found_words.insert(word.to_string());
        }
    }
    found_words
}

// Find our word ladder of begin word to end word of one char diff.  Return Vec Solution.
fn find_word_ladder(
    begin_word: &str,
    end_word: &str,
    word_list: &mut [&str],
) -> LinkedHashSet<String> {
    let mut word_list_set = LinkedHashSet::new();
    // Vec to LinkedHashSet
    for word in word_list.iter() {
        word_list_set.insert(word.to_string());
    }

    // Build a Dequeue to store our nodes for BFS
    let first_path: LinkedHashSet<_> = vec![begin_word.to_string()].into_iter().collect();
    let mut node_list: VecDeque<(String, LinkedHashSet<String>)> =
        VecDeque::from([(begin_word.to_string(), first_path)]);

    // Go huntin'
    while !node_list.is_empty() {
        // Grab left most node
        if let Some((node, path)) = node_list.pop_front() {
            // Find words minus paths we already found
            let node_next_words = find_next_words(&node, word_list_set.clone());
            let next_words: LinkedHashSet<String> =
                (&node_next_words - &path).iter().cloned().collect();
            // Iter those words
            for next in next_words.iter() {
                // Solution??
                if next == end_word {
                    let mut solution = LinkedHashSet::new();
                    solution.insert(begin_word.to_string());
                    for p in path {
                        solution.insert(p);
                    }
                    solution.insert(end_word.to_string());
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
    LinkedHashSet::new()
}

fn main() {
    let begin_word = "hit";
    let end_word = "cog";
    let mut word_list = ["hot", "dot", "dog", "lot", "log", "cog"];
    let word_ladder = find_word_ladder(begin_word, end_word, &mut word_list);
    println!("Begin Word: {} End Word: {}", begin_word, end_word);
    println!("Solution: {:?}", word_ladder);
}
