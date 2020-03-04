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
	println!("\nnum: {}, den: {}", num, den);
	let fraction1 = Fraction::new(&num, &den);
	displayfraction(&fraction1, 1);

	num = -5;
	den = 15;
	println!("\nnum: {}, den: {}", num, den);
	let fraction2 = Fraction::new(&num, &den);
	displayfraction(&fraction2, 2);

	num = 48;
	den = 72;
	println!("\nnum: {}, den: {}", num, den);
	let mut fraction3 = Fraction::new(&num, &den);
	displayfraction(&fraction3, 3);

	num = 18;
	den = 24;
	println!("\nnum: {}", num);
	fraction3.set_numerator(&num);
	displayfraction(&fraction3, 3);

	println!("\nden: {}", den);
	fraction3.set_denominator(&den);
	displayfraction(&fraction3, 3);

	num = -16;
	den = -20;
	println!("\nnum: {}, den: {}", num, den);
	fraction3.set(&num, &den);
	displayfraction(&fraction3, 3);
}

fn displayfraction(fraction:&Fraction, frac_number: u32) {
	println!("Fraction {}: {}", frac_number, fraction);
	println!("Numerator: {}, denominator: {}",
		fraction.get_numerator(), fraction.get_denominator());
	println!("Fraction {} = {}", frac_number, fraction.get_value());
}
