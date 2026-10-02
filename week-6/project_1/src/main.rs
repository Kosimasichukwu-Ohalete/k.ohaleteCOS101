use std::io;

fn main() {
    println!("Welcome to your one stop resturant!");
    println!("I present to you the resturant menu");
    println!("Please take good note of the letter representatives");
    println!("Have a great time");
    println!("P - Pounded Yam: #3000");
    println!("F - Fried Rice: #2500");
    println!("A - Amala: #2000");
    println!("E - Egusi Soup: #1500");
    println!("W - Water: #500");

    //Input of food requested
    println!("Enter food type:");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Not a valid string");
    
    //The quantity of food needed
    println!("Enter quantity");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid input");
    let quantity: u32 = input.trim().parse().expect("Not valid");

    let  price:u32;

    if food_type.trim() == "P" {
        price = 3200;
    }else if food_type.trim() == "F" {
        price = 3000;
    }else if food_type.trim() == "A" {
        price = 2500
    }else if food_type.trim() == "E" {
        price = 2000
    }else if food_type.trim() == "W" {
        price = 2500
    }else {
        println!("Invalid food type");
        return;
    }

    //to calculate total items bought

    let total = price * quantity;
    if total > 10000 {
        let discount = total * 5/100;
        let amount = total - discount;
        println!("Total = {}",total);
        println!("Discount = {}",discount);
        println!("Amount to pay = {}", amount);
    }else {
        println!("Total = {}",total);
    }
}
