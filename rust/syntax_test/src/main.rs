// https://fasterthanli.me/articles/a-half-hour-to-learn-rust

// use std::cmp::min;
// use std::cmp::{min, max};
use std::cmp::*;

fn greet() {
    println!("Hi there!");
}

fn fair_dice_roll4() -> i32 {
    return 4;
}

fn fair_dice_roll2() -> i32 {
    2
}

fn luck_roll(feeling_lucky: bool) -> bool {
    feeling_lucky
}

struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}

fn print_number(n: Number) {
    if let Number { odd: true, value } = n {
        println!("Odd number: {}", value);
    } else if let Number { odd: false, value } = n {
        println!("Even number: {}", value);
    }
}

fn print_number_match(n: Number) {
    match n {
        Number { odd: true, value } => println!("Odd number: {}", value),
        Number { odd: false, value } => println!("Even number: {}", value),
    }
}

fn print_number_dumb(n: Number) {
    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("{}", n.value),
    }
}

trait Signed {
    fn is_strictly_negative(self) -> bool;
}

impl Signed for Number {
    fn is_strictly_negative(self) -> bool {
        self.value < 0
    }
}


fn main() {
    greet();

    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x, y| x + y);

    {
        let x = 420;
        println!("Inner: {}", x);
        let x = { 420 };
        println!("Inner 2: {}", x);
    }

    println!("Outer: {}", x);

    let y = {
        let a = 60;
        let b = 9;
        // return a + b; // this doesn't work
        a + b
    };

    println!("Block sum: {}", y);

    println!("Roll 4: {}", fair_dice_roll4());
    println!("Roll 2: {}", fair_dice_roll2());

    println!("Am I lucky: {}", luck_roll(true));

    let x = (10, 20);
    println!("First num: {}", x.0);

    let nick = "khuedoan";
    println!("Length: {}", nick.len());
    println!("Same length: {}", str::len(nick));

    let one_to_five = (1,2,3,4,5);
    // let least = std::cmp::min(one_to_five.0, one_to_five.4);
    let least = min(one_to_five.0, one_to_five.4);
    let most = max(one_to_five.0, one_to_five.4);
    println!("Least: {}", least);
    println!("Most: {}", most);

    // let _v = Vec::new();
    // struct Vec2 { 
    //     x: f64,
    //     y: f64,
    // }

    // let _v1 = Vec2 { x: 1.0, y: 2.0 };
    // let _v2 = Vec2 { x: 3.0, y: 4.0 };

    // let _v3 = Vec2 {
    //     x: 14.0,
    //     .._v2
    // };
    
    // let _v4 = Vec2 {
    //     .._v3
    // };
    
    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        odd: false,
        value: 2,
    };
    let three = Number {
        odd: true,
        value: 3,
    };
    let mut minus_two = Number {
        odd: false,
        value: -2,
    };

    minus_two.value = 4;

    let minus_three = Number {
        odd: false,
        value: -3,
    };

    // print_number(one);
    // print_number(two);
    print_number_match(one);
    print_number_match(two);
    print_number_dumb(three);
    println!("{}", minus_two.is_strictly_positive());
    println!("{}", minus_three.is_strictly_negative());
}

// match next
