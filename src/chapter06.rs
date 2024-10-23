// Fix error without adding new line
pub fn main1() {
    let s: &str = "hello, world";

    println!("Success! main1");
}

// Fix the error with at least two solutions
pub fn main2() {
    let s: Box<&str> = "hello, world".into();
    greetings(s);

    fn greetings(s: Box<&str>) {
        println!("{}",s)
    }
    //Or:
/*    let s: Box<&str> = "hello, world".into();
    greetings(*s);

    fn greetings(s: &str) {
        println!("{}",s)
    }*/
}