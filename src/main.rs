mod functions;
use functions::greatest_common_divisor;
use functions::is_leap_year;
use functions::swap_values;
use functions::fibonacci;

mod fraction;
use fraction::Fraction;

fn main() {
    println!("Leap year test");
	println!("-800: {}", is_leap_year(-800));
	println!("-200: {}", is_leap_year(-200));
	println!("-16: {}", is_leap_year(-16));
	println!("0: {}", is_leap_year(0));
	println!("16: {}", is_leap_year(16));
	println!("200: {}", is_leap_year(200));
	println!("800: {}", is_leap_year(800));

	println!("\nValue swapping");
	let mut a:i32 = 72;
	let mut b:i32 = 48;
	println!("a = {}, b = {}", a, b);
	swap_values(&mut a, &mut b);
	println!("a = {}, b = {}", a, b);

	println!("\nGreatest common divisor");
	println!("gcd({}, {}) = {}", a, b, greatest_common_divisor(a as u32, b as u32));

	println!("\nFibonacci sequence");
	println!("0: {}", fibonacci(0));
	println!("1: {}", fibonacci(1));
	println!("2: {}", fibonacci(2));
	println!("10: {}", fibonacci(10));
	// Above 93, the result cannot be stored in u64.
	println!("92: {}", fibonacci(92));
	println!("93: {}", fibonacci(93));

	let mut num:i32 = 3;
	let mut den:i32 = 7;
	/* Does not compile because _fraction1 and _fraction2 contian a Result enum.
	println!("\nnum: {}, den: {}", num, den);
	let _fraction1 = Fraction::new(&num, &den);
	println!("Fraction 1: {}", _fraction1);
	println!("Numerator: {}, denominator: {}",
		_fraction1.get_numerator(), _fraction1.get_denominator());
	println!("Fraction 1 = {}", _fraction1.get_value());

	num = 5;
	den = 15;
	println!("\nnum: {}, den: {}", num, den);
	let _fraction2 = Fraction::new(&num, &den);
	println!("Fraction 2: {}", _fraction2);
	println!("Numerator: {}, denominator: {}",
		_fraction2.get_numerator(), _fraction2.get_denominator());
	println!("Fraction 2 = {}", _fraction2.get_value());
	*/

	num = 11;
	den = 0;
	println!("\nnum: {}, den: {}", num, den);
	let _fraction3 = Fraction::new(&num, &den);
	match _fraction3 {
		Ok(frac) => {
			println!("Fraction 3: {}", frac);
			println!("Numerator: {}, denominator: {}",
			frac.get_numerator(), frac.get_denominator());
			println!("Fraction 3 = {}", frac.get_value());
		},
		Err(msg) => {
			println!("{}", msg);
		}
	}
}
