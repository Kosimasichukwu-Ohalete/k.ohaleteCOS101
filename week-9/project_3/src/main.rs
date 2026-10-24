use std::io::Write;

fn main() {
    let commissioners = [
    "Aigbogun Alamba Daudu",
    "Muritala Afeez Bendu",
    "Okorocha Callistus Ogbona",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etiye",
    ];

    let ministries = [
    "Internal Affairs",
    "Justice",
    "Defense",
    "Power & Steel",
    "Petroleum",
    ];

    let zones = [
    "South West",
    "North East",
    "South South",
    "South West",
    "South EAst",
    ];

    let mut file = std::fs::File::create("EFCC_Merge.txt").expect("create failed");

    let header = "Commissioner, Ministry, Zone\n";

    file.write_all(header.as_bytes()).expect("error");

    println!("{}", header);

    for i in 0..commissioners.len() {
        let data = format!(
            "{}, {}, {}\n",
            commissioners[i],
            ministries[i],
            zones[i]
            );
        print!("{:?}",data );
        file.write_all(data.as_bytes()).expect("error");


    }
    println!("Data merged successfully");
}
