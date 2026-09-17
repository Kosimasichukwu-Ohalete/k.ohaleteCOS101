//The Incentive Calculator
 use std::io;

 fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter employee's age");
    io::stdin().read_line(&mut input1).expect("Not a valid string for employee's age");
    let a:u8 = input1.trim().parse().expect("Not a valid number for employee's age");

    println!("Enter employee's experience");
    io::stdin().read_line(&mut input2).expect("Not a valid string for employee's experience");
    let e = input2.trim();

    if a>=40 && e=="experienced" {
        println!("The annual incentive is N1,560,000");
    }
    else if a>=29 && a<=39 && e=="experienced" {
        println!("The annual incentive is N1,480,000");
    }
    else if a<=28 && e=="experienced" {
        println!("The annual incentive is N1,300,000");
    } 
    else {
        println!("The annual incentive is N100,000");
    }
 }