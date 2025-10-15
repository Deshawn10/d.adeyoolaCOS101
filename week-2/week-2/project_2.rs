fn main() {
	//Sales record and given values
	let toshiba:f64 = 450_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let dell:f64 = 2_850_000.0;
	let acer:f64 = 250_000.0;

	//sum of values
	let s = toshiba + mac + hp + dell + acer;
	println!("The Sum(S) is {}", s);

	//average of values
	let a = s / 5.0;
	println!("The Average(A) is {}", a);

}