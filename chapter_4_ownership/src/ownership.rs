pub fn ownership_examples() {
    // variables are valid as long as they don't go out of scope
    
    {                       // s is not valid here, it’s not yet declared
        let s = "hello";    // s is valid from this point forward

        println!("{}", s);   // s is valid from this point forward
    }                        // this scope is now over, and s is no longer valid

    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() append a literal to a String
        println!("{}", s);
        // we can mutate a String because it is stored on the heap
        // string literals are directly stored on the stack and have a fixed size - immutable
    }

    {
        let x = 5;
        let y = x;
        println!("x = {x}, y = {y}");
        // x and y can directly be pushed on stack, because they are ints with a fixed size
        // so copying a int that's stored on the stack is very cheap

        let s1 = String::from("hello");
        let s2 = s1;
        println!("s2 = {s2}, s1 is not valid anymore!");
        // rust only makes a copy of the pointer, length and capacity, that are on the stack
        // rust does not copy the data on the heap
        // this also means that s1 is not valid anymore - s1 was moved to s2
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {s1}, s2 = {s2}");
        // s1 and s2 are valid, because clone() makes a deep copy of the data on the heap
    }

    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here

        let x = 5;

        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it’s okay to still
                                        // use x afterward
    }   // x goes out of scope, then s - s was already moved so ntohing special happens

    {
        let s1 = gives_ownership();         // gives_ownership moves its return 
                                            // value into s1
        println!("s1 = {s1}");
        let s2 = String::from("hello");     // s2 comes into scope
        println!("s2 = {s2}");
        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also 
                                            // moves its return value into s3
        println!("s3 = {s3}");
    }   // s3 goes out of scope, gets dropped. s2 was moved, s1 goes out of scope and is dropped

    {
        let s1 = String::from("hello");
        let (s2, len) = calcualte_length(s1);   // calculate_length() takes ownership of s1, 
                                                // but returns it along with the length
        println!("s2 = {s2}, len = {len}");
    }
}

fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("some_string = {}", some_string);
}   // some_string goes out of scope here, and ´drop´ gets called

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("some_integer = {}", some_integer);
}   // some_integer goes out of scope here, but nothing special happens

fn gives_ownership() -> String {                // gives_ownership will move its 
                                                // return value into the function 
                                                // that calls it
    let some_string = String::from("yours");    // some_string comes into scope

    some_string                                 // some_string is returned and 
                                                // moves out to the calling function
}

// function takes and gives back ownership
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into
                                                        // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calcualte_length(s: String) -> (String, usize) {
    let length = s.len();   // len() returns the length of a String

    (s, length)
}