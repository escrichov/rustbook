fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1; // Move: move pointer from s1 to s2. S1 pointer is invalid
                 // S1 is no longer valid
                 // This line of code throws an error:
                 // println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Copy data
    println!("{}, world!", s1);
    println!("{}, world!", s2);

    // OWNERSHIP
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1

    let _s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    // BORROWING
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    // You can not change a borrowed object, by default is immutable
    change_mutable(&mut s);

    let mut s = String::from("hello");

    // But mutable references have one big restriction: you can have only one mutable reference
    // to a particular piece of data in a particular scope.
    let _r1 = &mut s;
    let r2 = &mut s;
    // This code will fail:
    // println!("{}, {}", r1, r2);
    println!("{}", r2);

    // Different scopes
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s;

    // Mutable and immutable combined is not permitted if the variable it is used after the
    // mutable variable is created
    let mut s = String::from("hello");
    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
    let _r3 = &mut s; // BIG PROBLEM
                      // This code does not work:
                      //println!("{}, {}, and {}", r1, r2, r3);

    // This code will compile because the last usage of the immutable references
    // occurs before the mutable reference is introduced
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // DANGLING REFERENCES
    // This code does not work:
    //let reference_to_nothing = dangle();
    let s = no_dangle();
    println!("{}", s);

    // SLICES
    let s = String::from("hello world");
    let _word = first_word(&s);
    //s.clear(); // error!

    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("the first word is: {}", word);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world");
}

//fn dangle() -> &String { // dangle returns a reference to a String
//
//    let s = String::from("hello"); // s is a new String
//
//    &s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
//  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
