pub fn slicing_examples() {
    {
        let mut s = String::from("hello world");

        let word = first_word(&s);
        // we now have the index of the space

        s.clear();
        // now the index of the space is useless and would produce an error if we tried to use it
        println!("The first space is {word}, but s is empty :( -> {s}");
    }
    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        println!("{} {}", hello, world);
    }
    {
        let my_string = String::from("hello world");

        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word_slice(&my_string[0..6]);
        println!("The first word is: {word}");
        let word = first_word_slice(&my_string[..]);
        println!("The first word is: {word}");
        // `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let word = first_word_slice(&my_string);
        println!("The first word is: {word}");

        let my_string_literal = "hello world";

        // `first_word` works on slices of string literals, whether partial or whole
        let word = first_word_slice(&my_string_literal[0..6]);
        println!("The first word is: {word}");
        let word = first_word_slice(&my_string_literal[..]);
        println!("The first word is: {word}");

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word_slice(my_string_literal);
        println!("The first word is: {word}");
    }
    {
        // slicing arrays
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// taking str as argument includes String and str
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}