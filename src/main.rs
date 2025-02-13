use std::cmp::Ordering;
use std::process::exit;

fn main() {
    let mut args = std::env::args().skip(1);
    if args.len() != 2{
        println!("there must be two arguments");
        exit(64);
    }
    let version1 = args.next().unwrap();
    let version2 = args.next().unwrap();
    let operation = version1.cmp(&version2);
    match operation {
        Ordering::Equal => {println!("{version1} is equal to {version2}")}
        Ordering::Less => {println!("{version1} is less than {version2}")}
        Ordering::Greater => {println!("{version1} is greater than {version2}")}
    }
}