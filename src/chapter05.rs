
pub fn main1() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world"); //Or x = "Hello world";
    // Or &String::from("Hello world");
    let y = &x;
    //Or x.clone(); or x.as_str();
    println!("{}, {}",x, y);
}

// Don't modify code in main!
pub fn main2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);

    // Only modify the code below!
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }
}


pub fn main3() {
    let s = give_ownership();
    println!("{}", s);

    // Only modify the code below!
    fn give_ownership() -> String {
        let s = String::from("Hello world main3");
        // Convert String to Vec
        let _s = s.clone().into_bytes(); //Or let _s = s.as_bytes();
        s
    }
}



// Fix the error without removing any code
pub fn main4() {
    let s = &String::from("Hello World main4");
    print_str(s); //Or & here
    //Or print_str(s.clone());

    println!("{}", s);

    fn print_str(s: &String)  {
        println!("{}",s)
    }
}

// Don't use clone ,use copy instead
pub fn main5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}


// make the necessary variable mutable
pub fn main6() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

pub fn main7() {
    let x = Box::new(5);

    let mut y = x.clone();    // update this line, don't change other lines!
    //Or let mut y = Box::new(2);

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

pub fn main_theory() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}


pub fn main8() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t.1);
}

pub fn main9() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = &t; //Or t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}


pub fn main10() {
   let x = 5;
   // Fill the blank
   let p = &x;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

pub fn main11() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y); //Or y.clone()

    println!("Success!");
}

// Fix error
pub fn main12() {
    let mut s = String::from("hello, "); //Or &String::from("hello, ");

    borrow_object(&s);

    println!("Success!");

    fn borrow_object(s: &String) {}
}

// Fix error
pub fn main13() {
    let mut s = String::from("hello, "); //Or let s = &mut String::from("hello, ");

    push_str(&mut s);

    println!("Success! main13");

    fn push_str(s: &mut String) {
    s.push_str("world")
}
}

pub fn main14() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");

    println!("Success!");
}

pub fn main15() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!main15");

    // Get memory address string
    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}

// Remove something to make it work
// Don't remove a whole line !
pub fn main16() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

pub fn main17() {
    // Fix error by modifying this line
    let s = String::from("hello, ");

    borrow_object(&s);

    println!("Success! main17");

    fn borrow_object(s: &String) {}
}

// This code has no errors!
pub fn main18() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");

    fn borrow_object(s: &String) {}
}

// Comment one line to make it work
pub fn main19() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    //println!("{}",r1);
}

pub fn main20() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time

    //println!("{}{}", r1, r2);
    //Or: r1.push_str("world");
}


