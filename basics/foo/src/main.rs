use std::io;
use std::collections::HashMap;


fn main() {
    println!("Enter the desired length(will display fizzbuzz functionality: ");

    //Get User Input To run fizzbuzz functionality
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    //Convert input string into a unsigned 8 bit integer u8 
    let choice: u8 = user_input.trim().parse().expect("Not an Integer");

    let mut custom_map = HashMap::new();
        custom_map.insert(3, "Fizz");
        custom_map.insert(5, "Buzz ");
        //Can add all the keys we want and should work
        custom_map.insert(7, "Test ");
        custom_map.insert(9, "Run ");
        custom_map.insert(11, "fkdsjaf");

    //This solution is a bit more scalable than 40 thousand if else statements
    for i in 1..=choice {
        let mut output = String::new();

        for (key, value) in &custom_map {
            if i % key == 0 {
                output.push_str(value);
            }
        }
        if output.is_empty(){
            output = i.to_string();
        }
        println!("{}", output)   
    }
}
