fn longest<'a>(short: &'a str, long: &'a str) -> &'a str {
    return if short.len() > long.len() {
        short
    } else {
        long
    };
}

pub fn main_a33() {
    let short = "hello";
    let long = "this is a long message";
    println!("{}", longest(short, long))
}
