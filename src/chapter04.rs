
// Remove something to make it work
pub fn main1() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;

    let z = 10; // Type of z ? i:32

    println!("Success!");
}

// Fill the blank
pub fn main2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

// Modify `assert_eq!` to make it work
pub fn main3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// Fill the blanks to make it work
pub fn main4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

// Fix errors and panics to make it work
pub fn main5() {
   let v1 = 251_u16 + 8;
   let v2 = u16::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
    //Or use u32
}

// Modify `assert!` to make it work
pub fn main6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

//Fill the blank to make it work
pub fn main7() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");

    fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
    }
}


//Make it work in two distinct ways
pub fn main8() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
    //Or  assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);

    println!("Success!");
}


pub fn main9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 97..123 {
        println!("{}",c);
    }
    //Or
    /*
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
*/
}
// Fill the blanks
use std::ops::{Range, RangeInclusive};
pub fn main10() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}


// Fill the blanks and fix the errors

// Fill the blanks and fix the errors
pub fn main11() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2_f32 == 3.0_f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}


// Make it work
use std::mem::size_of_val;
use std::{thread, time};

pub fn main12() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}


pub fn main13() {
// Make it work
    let c1 = '中';
    print_char(c1);

    fn print_char(c : char) {
        println!("{}", c);
    }

}


// Make println! work
pub fn main14(){
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}


// Make it work
pub fn main15() {
    let f = true;
    let t = true && false;
    assert_eq!(t, !f); //Or || in t let

    println!("Success!");
}


// Make it work, don't modify `implicitly_ret_unit` !
pub fn main16() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}


// Modify `4` in assert to make it work
//use std::mem::size_of_val; already used
pub fn main17() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

// Make it work with two ways
pub fn main18() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

   assert_eq!(v, 3);

    //Or:
    /*let v = {
        let mut x = 1;
        x += 2
    };

   assert_eq!(v, ());

   println!("Success!");*/

   println!("Success!");
}

pub fn main19() {
   let v = {
       let x = 3;
       x
   };

   assert!(v == 3);

   println!("Success!");
}

pub fn main20() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!(chapter 04.20)");

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
}

pub fn main21() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, ());

    println!("Success!");

    fn sum(x: i32, y: i32) {
        x + y;
    }

    //Or:
/*    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }*/
}

pub fn main22() {
   print();

    // Replace i32 with another type
    fn print() -> () {
        println!("Success!");
    }
}

// Solve it in two ways
// DON'T let `println!` work
pub fn main23() {
    never_return();

    println!("Failed!");

    fn never_return() -> ! {
        // Implement this function, don't modify the fn signatures
        loop {
            println!("Infinity loop");
            thread::sleep(time::Duration::from_secs(3))
        }

        //Or:
        //panic!("PANIC")
    }
}

pub fn main24() {
    println!("Success! main24");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    todo!();

    //Or:
    //panic!("PANIC")

    //Or:
    // loop {
    //     println!("Infinity loop");
    //     thread::sleep(time::Duration::from_secs(3))
    // }

    //Or:
    //unimplemented!()
}


pub fn main25() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
