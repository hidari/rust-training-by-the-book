extern crate add_one;
extern crate add_two;
extern crate rand;

fn main() {
    let num = 10;
    println!("{} plus one is {}", num, add_one::add_one(num));
    println!("{} plus one is {}", num, add_two::add_two(num));
}
