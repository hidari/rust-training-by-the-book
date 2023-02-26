extern crate rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_works() {
        assert_eq!(3, add_one(2));
    }
}
