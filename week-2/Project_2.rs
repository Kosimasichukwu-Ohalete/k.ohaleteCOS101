fn main() {
//P.M Okeke and Sons Limited Account for sales
	
	let toshiba:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let dell:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;

/*Let t be the total amount gotten from items sold,

  the quantity of each type of item included;

  Toshiba(2), Mac(1), HP(3), Dell(3), Acer(1)

  Let q be the total number of items sold */

    let q:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
    let t:f64 =(2.0*toshiba) + (1.0*mac) + (3.0*hp) + (3.0*dell) + (1.0*acer);
    println!("The sum of items sold is N{}",t);

  let average:f64 =t/q;
    println!("The average is {}",average);


}