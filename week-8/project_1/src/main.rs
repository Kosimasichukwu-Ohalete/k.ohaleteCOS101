use std::io;

fn main() {
    let public_servant=vec!["APS 1-2","APS 3-5","APS 5-8","KL1 8-10","KL2 10-13","SES"];
    let office_admin=vec!["Intern","Adminstrator","Senior Adminstrator","Office Manager","Director","CEO"];
    let academic=vec!["-","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let lawyer=vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let teacher=vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];

    let mut choice5 = String::new();
    println!("Select your department:");
    println!("1. Academic");
    println!("2. Office Admin");
    println!("3. Lawyer");
    println!("4. Teacher");

    io::stdin().read_line(&mut choice5).unwrap();
    let department: usize = choice5.trim().parse().unwrap();

    match department {
        1 => {
            let a = academic1();
    println!("Your level in the civil service is {}", public_servant[a-1]);
        }
       2 => {
            let b = office_admin1();
    println!("Your level in the civil service is {}", public_servant[b-1]);
       } 
       3 => { 
             let c = lawyer1();
    println!("Your level in the civil service is {}",public_servant[c-1]);

       }
       4 => {
            let d = teacher1();
    println!("Your level in the civil service is {}",public_servant[d-1]);

       }
       _ => println!("Invalid department selection"),
    }
  

}

fn academic1() -> usize   {
    let academic=vec!["-","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let mut choice = String::new();
    
    println!("Please select your role under the academic department:
        1. {}
        2. {}
        3. {}
        4. {}
        5. {}
        6. {}", academic[0], academic[1], academic[2], academic[3], academic[4], academic[5]);
    io::stdin()
        .read_line(&mut choice)
        .expect("Your input could not be stored.");
    let choice: usize = choice.trim().parse().expect("Ensure you inputed a valid string");
    let role = academic[choice-1].to_string();
    println!("Your role is {}", role);
    choice

}

fn office_admin1() -> usize  {
    let office_admin=vec!["Intern","Adminstrator","Senior Adminstrator","Office Manager","Director","CEO"];
    let mut choice1 = String::new();

    println!("Please select your role under the office admin department:
        1.{}
        2.{}
        3.{}
        4.{}
        5.{}
        6.{}", office_admin[0], office_admin[1], office_admin[2], office_admin[3], office_admin[4], office_admin[5]);
    io::stdin()
        .read_line(&mut choice1)
        .expect("Your input could not be stored.");
    let choice1: usize = choice1.trim().parse().expect("Ensure you inputed a valid string");
    choice1
}

fn lawyer1() -> usize {
    let lawyer=vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
    let mut choice2 = String::new();

    println!("Please select your role under the office:
        1.{}
        2.{}
        3.{}
        4.{}
        5.{}
        6.{}", lawyer[0], lawyer[1], lawyer[2], lawyer[3], lawyer[4], lawyer[5]);
    io::stdin()
        .read_line(&mut choice2)
        .expect("Your input could not be stored.");
    let choice2: usize = choice2.trim().parse().expect("Ensure you inputed a valid string");
    choice2
}

fn teacher1() -> usize {
    let teacher=vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];
    let mut choice3 = String::new();

    println!("Please select your role under the office:
        1.{}
        2.{}
        3.{}
        4.{}
        5.{}
        6.{}", teacher[0],teacher[1], teacher[2], teacher[3], teacher[4], teacher[5]);
    io::stdin()
        .read_line(&mut choice3)
        .expect("Your input could not be stored.");
    let choice3: usize = choice3.trim().parse().expect("Ensure you inputed a valid string");
    choice3
}