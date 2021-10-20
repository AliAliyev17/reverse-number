extern crate rand;

fn main() {
    let mut m: u32;
    let mut n: u64;
    for i in 0..100000 {
        m = rand::random();
        n = m as u64;
        let rev_n = reverse(n);
        //println!("{}-{}", n, rev_n);
    }
}

fn reverse(n: u64) -> u64 {
    n.to_string().chars().rev().collect::<String>().parse::<u64>().unwrap()
}

fn reverse_longer(n: u64) -> u64 {
    #![feature(int_log)]
    let iters = (n as f64).log10() as u32;
    let mut num = n;
    let mut rev_n = String::new();
    let mut divident: u64;
    for i in (0..iters).rev() {
        rev_n.push_str(&(num/10u64.pow(i)).to_string());
        num = num%(10u64.pow(i));
    }
    rev_n.parse::<u64>().unwrap()
}
