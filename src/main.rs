use proconio::input;
use std::{collections::{BinaryHeap, VecDeque, HashSet}, fmt::format, result, string, thread::AccessError, usize};

fn main() {
    input! {
        n: usize,
        boon_shape: [(usize, usize); n],
        m: usize,
        string_list: [String; m],
        // vec_a: [usize; n],
    }

    let mut sekitui_word_kouho: Vec<HashSet<char>> = vec![HashSet::new(); n]; 
    for i in 0..n {
        let (len, word_num) = boon_shape[i];
        for j in 0..m {
            let word = &string_list[j];
            if word.len() == len {
                sekitui_word_kouho[i].insert(word.chars().nth(word_num - 1).unwrap());
            }
        }
    }

    for i in 0..m {
        let word = &string_list[i];
        if word.len() != n {
            println!("No");
            continue;
        }
        if check_string(n, &sekitui_word_kouho, word) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn check_string(n: usize, sekitui_word_kouho: &Vec<HashSet<char>>, word: &String) -> bool {
    for i in 0..n {
        let kouho_list = &sekitui_word_kouho[i];
        let target_char = word.chars().nth(i).unwrap();
        if !kouho_list.contains(&target_char) {
            return false;
        }
    }
    return true;
}
