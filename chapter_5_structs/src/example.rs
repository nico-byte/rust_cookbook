#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn example_program() {
    let scale = 2;
    let rect1 = Rectangle { 
        width: dbg!(30 * scale), 
        height: 50, 
    };
    
    dbg!(&rect1);

    let area_rect1 = area(&rect1);
    dbg!(&area_rect1);
    // dbg! works similar to println! but it prints to the stderror console rather than the 
    // standard output console. We can use this to debug our code.
    // dgb! takes ownership of avlue but also returns it
    // dbg! also tells us where it is called from, so we can see where the "error" is coming from.
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}