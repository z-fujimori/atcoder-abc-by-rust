use proconio::input;
use std::{collections::{BinaryHeap, VecDeque}, fmt::format, result, string, thread::AccessError, usize};

fn main() {
    input! {
        h: usize,
        w: usize,
        // n: usize,
        // vec_a: [usize; n],
    }

    let jyouge = "#".repeat(w);
    let aida = "#".to_string() + &".".repeat(w - 2) + "#\n";
    let ans = jyouge.clone() + "\n" + &aida.repeat(h - 2) + &jyouge;
    println!("{}", ans);
}
