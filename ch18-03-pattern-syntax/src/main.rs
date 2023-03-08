fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!("========================");

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    println!("========================");

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    println!("========================");

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    println!("========================");

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point{x: 0, y: 7};
    let Point{x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point{x, y: 0} => println!("On the x axis at {}", x),
        Point{x: 0, y} => println!("On the y axis at {}", y),
        Point{x, y} => println!("On neither axis: ({}, {})", x, y),
    }

    println!("========================");

    enum Message{
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move{x, y} => println!("Move in the x direction {} and in the y direction {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change the color to red {}, green {}, blue {}", r, g, b),
    }

    println!("========================");

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let _sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("========================");

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }
    println!("setting is {:?}", setting_value);

    println!("========================");

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
    }

    println!("========================");

    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some( ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is : {:?}", robot_name);
}

