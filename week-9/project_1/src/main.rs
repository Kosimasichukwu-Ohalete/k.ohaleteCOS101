use std::io::Write;

fn main() {
    let header = "lager,stout,non-alcoholic";
    let data_1= "\n33 Export,Legend,Maltina";
    let data_2 = "\nDesperados,Turbo King,Amstel Malta";
    let data_3 = "\nGoldberg,Williams,Malta Gold";
    let data_4 = "\nGulder, ,Fayrouz";
    let data_5 = "\nHeineken, , ";
    let data_6 = "\nStar,_ , ";
    let mut file = std::fs::File::create("Nigerian_breweries.csv").expect("create failed");
    file.write_all(header.as_bytes()).expect("error");
    file.write_all(data_1.as_bytes()).unwrap();
    file.write_all(data_2.as_bytes()).unwrap();
    file.write_all(data_3.as_bytes()).unwrap();
    file.write_all(data_4.as_bytes()).unwrap();
    file.write_all(data_5.as_bytes()).unwrap();
    file.write_all(data_6.as_bytes()).unwrap();
    println!("Data input worked");
}