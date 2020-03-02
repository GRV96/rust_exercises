mod functions;
use functions::is_leap_year;

fn main() {
    println!("Leap year test");
	println!("-800: {}", is_leap_year(-800));
	println!("-200: {}", is_leap_year(-200));
	println!("-16: {}", is_leap_year(-16));
	println!("0: {}", is_leap_year(0));
	println!("16: {}", is_leap_year(16));
	println!("200: {}", is_leap_year(200));
	println!("800: {}", is_leap_year(800));
}
