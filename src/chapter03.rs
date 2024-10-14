
// Fix the error below with least amount of modification to the code
pub fn main1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    //let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

// Fill the blanks in the code to make it compile
pub fn main2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}


// Fix the error below with least amount of modification
pub fn main3() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    let y: i32 = 20;
    println!("The value of x is {} and value of y is {}", x, y);
}


// Fix the error with the use of define_x
pub fn main4() {
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}

pub fn main5(){

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}


// Remove a line in the code to make it compile
pub fn main6() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}

pub fn main7() {
    let x = 1;
    println!("{x}");
    // Or
    // let mut x = 1;
    // x = x;
}

// Fix the error below with least amount of modification
pub fn main8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
    // Or
    // let (x, y) = (1, 2);
    // let x = x + 2;
    //
    // assert_eq!(x, 3);
    // assert_eq!(y, 2);
    // println!("Success!");
}

pub fn main9() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
    println!("{0}, {1}", x,y);
}