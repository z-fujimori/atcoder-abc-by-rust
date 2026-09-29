use proconio::input;
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque}, fmt::format, future, i64, print, println, result, string, thread::AccessError, vec,
};

fn main() {
    input! {
        q: usize,
        s: String,
        t: String,
        query_vec: [(usize, usize); q]
    }

    let mut head = 0;
    let s_len = s.len();
    let t_len = t.len();
    let mut yes_indent_vec = vec![];

    if s_len >= t_len {
        while head <= s_len - t_len {
            let bytes = s.as_bytes();
            let s2: &[u8] = &bytes[head..head + t_len];
    
            if s2 == t.as_bytes() {
                yes_indent_vec.push(head);
            }
    
            head += 1;
        }
    }

    // println!("{:?}", yes_indent_vec);

    for (l, r) in query_vec {
        if r - l + 1 < t_len {
            println!("No");
            continue;
        }
        let pos = yes_indent_vec.partition_point(|&x| x < l - 1);

        let exists = pos < yes_indent_vec.len() && yes_indent_vec[pos] <= r - t_len;

        if exists {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
