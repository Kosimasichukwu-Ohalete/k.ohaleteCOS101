use std::io::Write;

fn main() {
    let header = "Student Name, Matric.Number, Department, Level";
    let data_1= "\nOluchi Mordi, ACC10211111, Accounting, 300 ";
    let data_2 = "\nAdams Aliyu,ECO10110101,Economics,100";
    let data_3 = "\nSharia Bolade,CSC10328828,Computer, 200";
    let data_4 = "\nAdekunle Gold,EEE11020202 ,Electrical,200";
    let data_5 = "\nBianca Edemoh,MEE10202001, Mechanical,100";

    let students = [
    data_1,
    data_2,
    data_3,
    data_4,
    data_5,];
    // Displaying students details on screen
    println!("{}",header);

    for student in students {
        print!("{}",student);
    }
    //Write student details to file
    let mut file = std::fs::File::create("PAU_SMIS.csv").expect("create failed");

    file.write_all(header.as_bytes()).expect("error");

    for student in students {
    file.write_all(student.as_bytes()).unwrap();
    }
    println!("Data input worked\n");

}
