use std::io;

pub fn get_input() -> io::Result<String> {
    let mut value = String::new();
    io::stdin().read_line(&mut value)?;
    Ok(value.trim().to_owned())
}

pub fn test1() {
    let mut all_inp_values: Vec<String> = Vec::new();

    let mut iterator: isize = 2;

    while iterator > 0 {
        println!("Please enter a value");
        match get_input() {
            Ok(value) => {
                all_inp_values.push(value);
                iterator -= 1;
            }
            Err(e) => println!("An Error occured please enter the values {}", e),
        }
    }
    println!("Your values are");
    println!("");
    all_inp_values.iter().for_each(|a| {
        println!("1 . {a}");
    });
}

