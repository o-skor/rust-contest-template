#![allow(unused_imports)]
#![allow(dead_code)]

use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::mem::swap;

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().unwrap();
            }
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn solve_case<T: Write>(scan: &mut Scanner, out: &mut BufWriter<T>) {
    let a: i32 = scan.next();
    let b: i32 = scan.next();
    writeln!(out, "{}", a + b).unwrap();
}

fn solve<T: Write>(scan: &mut Scanner, out: &mut BufWriter<T>) {
    let t: usize = scan.next();
    for _ in 0..t {
        solve_case(scan, out);
    }
}

fn main() {
    std::thread::Builder::new()
        .name("solver_thread".into())
        .stack_size(1024 * 1024 * 1024)
        .spawn(|| {
            let mut scan = Scanner::default();
            let out = stdout();
            let mut out = BufWriter::new(out.lock());
            solve(&mut scan, &mut out);
        })
        .unwrap()
        .join()
        .unwrap();
}
