// mod Option_Study;
use std::num::ParseIntError;

fn main() {
    parse_file2();
    parse_file3();
    // let _s = parse_file4a();
    let _s = parse_file4b();
    
}

// fn parse_file() {
//     let s = std::fs::read_to_string("./src/input.txt");
//     let s = s.unwrap_or("".to_string());
//     let s = s.split("\n");
// }

// 2
fn parse_file2() {
    let s = include_str!("input.txt");
    let str_to_int = str::parse::<i64>;
    let mut s = s.split('\n').map(str_to_int);

    let first = get_int2(s.next());
    print_num(first);
    let first = get_int2(s.next());
    print_num(first);
    let first = get_int2(s.next());
    print_num(first);
}

fn get_int2(i: Option<Result<i64, ParseIntError>>) -> i64 {
    i.unwrap().unwrap()
}

// 3
fn parse_file3() {
    let s = include_str!("input.txt");
    let str_to_int = str::parse::<i64>;
    let mut s = s.split('\n').map(str_to_int).map(Result::unwrap);

    let first = get_int3(s.next());
    print_num(first);
    let first = get_int3(s.next());
    print_num(first);
    let first = get_int3(s.next());
    print_num(first);
}

fn get_int3(i: Option<i64>) -> i64 {
    i.unwrap()
}

// 4
fn parse_file4a() -> Vec<i64> {
    let s = include_str!("input.txt");
    let str_to_int = str::parse::<i64>;
    let s = s.split('\n').map(str_to_int).map(Result::unwrap);
    let s: Vec<i64> = s.collect(); // this might error if encounter empty string
    s
}

// 5
fn parse_file4b() {
    // let s = include_str!("input.txt");
    // let str_to_int = str::parse::<i64>;
    // let s = s.split('\n').map(str_to_int);
    // let s: Vec<_> = s.collect::<Result<Vec<_>, _>>();
    // Some(s)
    let s = include_str!("input.txt")
    .split('\n')
    .map(str::parse::<i64>)
    .collect::<Result<Vec<_>, _>>()?;
}

fn print_num(num: i64) {
    println!("{num}");
}

// 5
fn find_pair_whose_sum_is_2020(s: Vec<i64>) -> Option<(i64, i64)> {
    for i in 0..s.len() {
        for j in 0..s.len() {
            if i == j {
                continue;
            }
            if s[i] + s[j] == 2020 {
                return Some((s[i], s[j]));
            }
        }
    }
    None
}