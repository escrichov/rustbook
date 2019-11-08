use std::f32;
use std::f64;

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn return_early() -> i32 {
    return 10;
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    1.8 * celsius + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn nthfib(nth: u128) -> u128 {
    let x = if nth == 0 {
        0
    } else if nth == 1 {
        1
    } else {
        nthfib(nth - 1) + nthfib(nth - 2)
    };
    x
}

fn twelve_days_christmas_song() {
    for i in 0..12 {
        let days = [
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
            "tenth", "eleventh", "twelfth",
        ];
        let phrases = [
            "A partridge in a pear tree",
            "Two turtle doves,",
            "Three French hens,",
            "Four calling birds,",
            "Five gold rings,",
            "Six geese a-laying,",
            "Seven swans a-swimming,",
            "Eight maids a-milking,",
            "Nine ladies dancing,",
            "Ten lords a-leaping,",
            "Eleven pipers piping,",
            "Twelve drummers drumming,",
        ];

        println!(
            "On the {} day of Christmas my true love sent to me",
            days[i]
        );
        for j in (0..=i).rev() {
            println!("{}", phrases[j]);
        }
        println!("");
    }
}

fn main() {
    // Const
    const CONSTANT: u32 = 1;
    println!("Constant: {}", CONSTANT);

    // Integer types
    let i8bit: i8 = 127;
    let i16bit: i16 = 32767;
    let i32bit: i32 = 2147483647;
    let i64bit: i64 = 9223372036854775807;
    let i128bit: i128 = 170141183460469231731687303715884105727;
    let u8bit: u8 = 255;
    let u16bit: u16 = 65535;
    let u32bit: u32 = 4294967295;
    let u64bit: u64 = 18446744073709551615;
    let u128bit: u128 = 340282366920938463463374607431768211455;
    println!(
        "Signed Numbers: {}, {}, {}, {}, {}",
        i8bit, i16bit, i32bit, i64bit, i128bit
    );
    println!(
        "Unsigned Numbers: {}, {}, {}, {}, {}",
        u8bit, u16bit, u32bit, u64bit, u128bit
    );

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
    println!(
        "Operations: {}, {}, {}, {}, {}",
        sum, difference, product, quotient, remainder
    );

    // Floating point
    let float32: f32 = f32::MAX;
    let float64: f64 = f64::MAX;
    println!("Float numbers: {}, {}", float32, float64);

    // Shadowing variables
    let shadowing = 5;
    let shadowing = shadowing * 2;
    println!("Shadowing: {}", shadowing);

    // Boolean
    let boolean: bool = false;
    println!("Boolean: {}", boolean);

    // Character type
    let character: char = '😻';
    println!("Character: {}", character);

    // Array
    let array = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
    println!("Array position 3: {}", array[2]);

    // Tuples
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tuple);
    println!("Tuple position 2: {}", tuple.1);

    // Expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("Expression: {}", y);

    // Functions
    println!("Function five: {}", five());
    println!("Function plus_one: {}", plus_one(5));
    println!("Function return_early: {}", return_early());

    // If
    let number = 3;
    if number != 0 {
        println!("If");
    }

    // If let statement
    let condition = true;
    let _number = if condition { 5 } else { 6 };
    println!("If let statement");

    // Loop
    loop {
        println!("Loop");
        break;
    }

    // Loop let statement
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop let statement: {}", result);

    // While
    let mut number = 3;
    while number != 0 {
        number -= 1;
    }
    println!("While: {}", number);

    // For
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("For: {}", element);
    }

    // For range
    for number in 1..=5 {
        println!("For range: {}!", number);
    }

    // Celsius to fahrenheit
    println!("º to F: {}", celsius_to_fahrenheit(24.0));
    println!("º to F: {}", fahrenheit_to_celsius(75.2));

    // Fibonacci
    println!("nth fibonacci 20: {}", nthfib(20));

    // The twelve days christmas song
    twelve_days_christmas_song();
}
