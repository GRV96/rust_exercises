mod functions;
use functions::greatest_common_divisor;
use functions::is_leap_year;
use functions::swap_values;
use functions::fibonacci;

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
}
