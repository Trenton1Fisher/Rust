use std::io;

fn main() {
    let choice = String::new();
    while choice != "5"{
        let mut choice = String::new();

        println!("Simple Math Equations");
        println!("(1) Addition \n(2) Subtraction \n(3) Multiplication \n(4) Division \n(5) Close Program");
        println!("Enter Your Math Equation Choice: ");

        io::stdin().read_line(&mut choice).expect("Error Reading Input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not an integer");
                return; // Exit the program if input is not an integer
            }
        };
    
        if choice == 1 {
            call_addition();
        }

        else if choice == 2 {
            call_subtraction();
        }
        else if choice == 3 {
            call_multiplication();
        }
        else if choice == 4{
            call_division();
        }
        else if choice == 5 {
            break;
        }
    }
   
}

fn call_addition(){
    let mut op1 = String::new();
    let mut op2 = String::new();

    println!("Enter Operand 1: ");
    io::stdin().read_line(&mut op1).expect("Error Reading Input");
    println!("Enter Operand 2: ");
    io::stdin().read_line(&mut op2).expect("Error Reading Input");

    let op1: u32 = match op1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not an integer");
            return; // Exit the program if input is not an integer
        }
    };

    let op2: u32 = match op2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not an integer");
            return; // Exit the program if input is not an integer
        }
    };

    println!("The Sum of both operands is: {} \n", op1 + op2);

}

fn call_subtraction(){
    let mut op1 = String::new();
    let mut op2 = String::new();

    println!("Enter Operand 1: ");
    io::stdin().read_line(&mut op1).expect("Error Reading Input");
    println!("Enter Operand 2: ");
    io::stdin().read_line(&mut op2).expect("Error Reading Input");

    let op1: u32 = match op1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not an integer");
            return; // Exit the program if input is not an integer
        }
    };

    let op2: u32 = match op2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not an integer");
            return; // Exit the program if input is not an integer
        }
    };

    println!("The Sum of both operands is: {} \n", op1 - op2);
}

fn call_multiplication() {
    let mut op1 = String::new();
    let mut op2 = String::new();

    println!("Enter Operand 1: ");
    io::stdin().read_line(&mut op1).expect("Error Reading Input");
    println!("Enter Operand 2: ");
    io::stdin().read_line(&mut op2).expect("Error Reading Input");

    let op1: u32 = match op1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not an integer");
            return; // Exit the program if input is not an integer
        }
    };

    let op2: u32 = match op2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not an integer");
            return; // Exit the program if input is not an integer
        }
    };

    println!("The Sum of both operands is: {} \n", op1 * op2);
}

fn call_division() {
    let mut op1 = String::new();
    let mut op2 = String::new();

    println!("Enter Operand 1: ");
    io::stdin().read_line(&mut op1).expect("Error Reading Input");
    println!("Enter Operand 2: ");
    io::stdin().read_line(&mut op2).expect("Error Reading Input");

    let op1: u32 = match op1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not an integer");
            return; // Exit the program if input is not an integer
        }
    };

    let op2: u32 = match op2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not an integer");
            return; // Exit the program if input is not an integer
        }
    };

    println!("The Sum of both operands is: {} \n", op1 / op2);
}
