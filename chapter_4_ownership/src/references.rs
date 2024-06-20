pub fn references_examples() {
    {
        let s1 = String::from("hello");
        let len = calcualte_length_by_ref(&s1);
        println!("The length of '{}' is {}.", s1, len);
        // s1 is still valid here because we only gave the reference to the function
        // so the function don't take ownership of the variable
        // this also means we cannot mutate the value of s1 inside the function
    }
    {
        // to mutate a refrence we can do this
        let mut s = String::from("hello");
        change(&mut s);
        println!("{}", s);
        // there can only be one mutable reference to a variable at a time
        // we also can't have a mutable reference while we have a immutable reference vice versa
    }
    {
        let mut s = String::from("hello");

        let r1 = &s;    // no problem
        let r2 = &s;    // no problem
        println!("{} and {}", r1, r2);
        // now r1 and r2 won't be used anymore

        let r3 = &mut s;    // no problem
        println!("{}", r3);
        // this works because after r1 and r2 are used, they get out of scope, 
        // so we can use r3 in the new scope
    }
}

fn calcualte_length_by_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}