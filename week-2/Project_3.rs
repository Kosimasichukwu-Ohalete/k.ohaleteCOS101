fn main() {
	let p:f64=210_000.0;
	let r:f64=5.0;
	let t:f64=3.0;
/* amount after three years is given as a
   the formula is given as A=P*(1-(r/100))^t*/

   let a= p *(1.0-(r / 100.0).powf(t));
    println!("The amount after three years is N{}",a);

}