use ch10_generics_trait_lifetime::aggregator::{NewsArticle, Tweet};
use ch10_generics_trait_lifetime::aggregator::Summary;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&number_list);
    println!("The largest char is {}", result);

    let tweet = Tweet{
        username: String::from("Hidari0415"),
        content: String::from("銀髪は至高。"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle{
        headline: String::from("滑らかな銀髪ロングの美少女"),
        location: String::from("大阪"),
        author: String::from("Hidari0415"),
        content: String::from("銀髪の美少女こそ至高。この世の宝。"),
    };
    println!("New article available {}", article.summarize())
}

fn largest_i32(list: &[i32]) -> i32{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> T{
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
