// use std::{char::CharTryFromError, fmt::format, sync::mpsc::SendError};
// use std::sync::mpsc::SendError;

use std::fs;



fn main() {
    let a : i32 = 1234;
    let b :i32 = 345;
    let name : String = String::from("karthikeya");
    print!("{}" , a+b);
    println!("{}",name);
    for i in 0..20{ 
        if i % 2 == 0{ 
            println!("{}" , i)
        }
    }
    let senetence : String = String::from("nandam venkata karthikeya");
    let word : String = first_word_print(senetence);
    println!("{}" , word);
    stack_vs_heap();

    let mut  value : String = String::from("Karhtikeya");
    borrow_function(&value); // if we do not pass this rust compiler automatically set's the value to the & address refference
    println!("{}" ,value); // -> this works
    
    mutable_from_the_parent(&mut value);
    println!("{}" ,value); // -> this works

    borrowing_mutating_main();

    struct_test();
    struct_implements();
    enum_section();
    implement_shapes();
    error_handling();
    main_option();

}


fn first_word_print(sentence : String) -> String { 
    let mut word = String::new();
    for char in sentence.chars() { 
        if char == ' ' { 
            break;
        } 
            
        word.push_str(char.to_string().as_str());
    };
    return word;
}

/*

    & Stack is the same as berfore and new thig it is added is heap
    & Heap -> continuously growing element which is not that oragnized and that fast as stack
    ? Any heap value which they need to store in the heap stack value posses these type :
        ^ length , capacity and pointer -> length is normal length of the string or any [] ,
        ^ Capacity is the container like structure if we exceeds the container it need to send the data pointer to somewhere else and get the data from there
        ^ pointer is the address of the heap where it points to the allocated data basically for a number 0->H this reffers to the pointer address then we can increase the last number to find the full data

*/

fn stack_vs_heap(){ 
    let mut  first : String = String::from("Hello");
    let second : String = String::from("World");
    println!("len {} , capacity {} , pointer {:p}" , first.len() , first.capacity() , first.as_ptr());
    first.push_str("MAMMA MAMMA");
    let third: String = format!("{} {}",first , second);
    println!("len {} , capacity {} , pointer {:p}" , first.len() , first.capacity() , first.as_ptr());
    println!("len {} , capacity {} , pointer {:p}" , second.len() , second.capacity() , second.as_ptr());
    println!("len {} , capacity {} , pointer {:p}" , third.len() , third.capacity() , third.as_ptr());
    println!("{}" , third);
}

/*
    & This is the main aspect of giving the owner ship to the heap ;
    ^ 1. s1 -> String::from("lksdjflkjs");
    ^ 2. s2 -> s1 ->>> here the s1 is dead in the heap we can no longer acces the after this unless we can specify the value .clone() in teh allocation 
 */

fn owner_ship() { 
    let new_str : String = String::from("Hello");
    let second_str : String = new_str;
    // println!("{}" , new_str); //^ You cannot do this owner ship error but you can do this    let second_str : String = new_str.clone();
    // & and also you cannot this also ;
    new_owner(second_str);
   // println!("{}" , second_str); //^ this also not possibble -> passing the reffence and allocating the value in the heap with the same pointer
}   

fn new_owner (new_one : String){
    println!("{}" , new_one)
}

/*
 & Borrowing by the refference; -> using the same concept as the C++
 */

fn borrow_function(some_value : &String)  { // this thing is the passing the reff value of the String
    println!("{}" , some_value);
}   


/*
    & Mutating the value same as the c++ * poiter and reffernce
 */

fn mutable_from_the_parent(some_value : &mut String){ //^ -> we need to specifie the &mut compulsary
    some_value.push_str("Hey mowa");
}


/*
    & Borrowing only happens in the one way and if we specify the value in the other time means only 
    & only one mutable element can be exec for one reffence &mut value 
    ^ if we specify the value for second time it will throw error

*/

fn borrowing_mutating_main(){ 
    let mut  s1 : String = String::from("Karthik");
    let s2 = &mut s1;
    // let s3 = &s1; //^ -> unless you use the value of the first borrow all refference will work if you use println!("{}" , s2) -> thorws error;
    borrowing_mutating_once(s2);
    println!("{}" , s1);
}

fn borrowing_mutating_once(some_value : &mut String){ 
    some_value.push_str(" Hey mama");
}

/*
    & Structs
 */

struct User { 
    name : String,
    age : u16,
    completed : bool,
    username : String,
    password : String
}

fn struct_test() { 
    let user = User {
        name : String::from("kathikeya"),
        age : 23,
        completed : false,
        username : String::from("kart@gmail.com"),
        password : String::from("1234567")
    };
    println!("{}" , user.username);
}


/* 
    & Implementation and struct simply the class type in the any program 
 */

struct Rec { 
    height : u8,
    width : u8
}

impl Rec {
    fn area(&self) -> u8{ 
        return self.width * self.height
    }
    fn perimeter(&self)-> u8 { 
        2 * (self.height + self.width) // -> is same as doing the thing return 
    }
}
// similar to the classes
fn struct_implements(){ 
    let rect_value : Rec = Rec { 
        height : 18,
        width : 12
    };
    println!("Area of the rectangle is {}" , rect_value.area());
    println!("Perimeter of the rectangle is {}" , rect_value.perimeter());
}

/*
     & Enums 
 */
#[derive(Debug)]
 enum Direction { 
    North,
    East,
    West,
    South
}

fn enum_section(){ 
    let mut direction : Direction = Direction::North;
    direction = Direction::South;
    print!("{:?}" , direction)
}

// enums with specific types 

#[derive(Debug)]
enum Shapes {
    Circle(f64),
    Rectangle(f64 , f64),
}

/*
    ? Pattern matching
 */

fn pattern_match_event(shape : Shapes) -> f64 { 
    let ans : f64= match shape {
        Shapes::Circle(radius) => 3.16 *radius * radius,
        Shapes::Rectangle(width , height) => width * height
    };
    return ans;
}
fn implement_shapes(){ 
    let circle : Shapes = Shapes::Circle(1234.00);
    let rectangle : Shapes = Shapes::Rectangle(12.23, 15.5);


    let circle_ans : f64 = pattern_match_event(circle);
    let rectangle_ans : f64 = pattern_match_event(rectangle);

    println!("Cicle radius with match {} and rectangle radius with match {} " , circle_ans , rectangle_ans);

}


/* 
    & Error handling Result<T , U> is a rust built in error handling struct which takes 2 OK Error
*/

fn error_handling(){ 
    let res = fs::read_to_string("example.txt");
    match res { 
        Ok(content) => println!("Content looks good {} " , content),
        Err(error) => println!("Error while catching the block {} " , error )
    }
}
// if we want to return only the value of the Result to the string;

/*
    & Options to eradicate the null value
 */

fn find_first_a(s : String) -> Option<u8>{ 
    for(index , val) in s.chars().enumerate(){ 
        if val == 'a'{ 
            return Some(index as u8);
        }
    }
    return None;
}

fn main_option(){ 
    let value : String = String::from("123456");
    let res = find_first_a(value);
    match res { 
        Some(value) => println!("The a is found at the index {} " , value),
        None => println!("No value found in the provided value")
    }
}