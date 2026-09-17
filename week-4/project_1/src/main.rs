//Quadratic Roots

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    /*y=ax^2 + bx +c
    x could be any other named algebra in the alphabet*/

    println!("Enter the coefficient of x^2"); 
    io::stdin().read_line(&mut input1).expect("Not a valid string for input1");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the coefficient of x");
    io::stdin().read_line(&mut input2).expect("Not a valid string for input2");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the last value without a coefficient");
    io::stdin().read_line(&mut input3).expect("Not a valid string for input3");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let  d = (b * b) - (4.0 * a * c);

    println!("The discriminant of the quadratic equation {}", d);

    if d>0.0 {
        println!("The quadratic equation is of two distinct roots");
    }
        else if d==0.0 {
            println!("The quadratic equation is exactly one real root");
        }
            else {
                println!("THe quadratic equation has no real roots");
            }
}
