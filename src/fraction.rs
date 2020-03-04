use std::fmt;
use std::mem;

pub struct Fraction {
	numerator:i32,
	denominator:i32
}

impl fmt::Display for Fraction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}/{}", self.numerator, self.denominator);
    }
}

impl Fraction {
	pub fn new(numerator:&i32, denominator:&i32) -> Fraction {
		if *denominator == 0 {
			panic!("Fraction denominator cannot be 0.");
		}
		let mut fraction = Fraction {numerator: *numerator, denominator: *denominator};
		fraction.simplify();
		return fraction;
	}

	pub fn get_denominator(&self) -> i32 {
		return self.denominator;
	}

	pub fn get_numerator(&self) -> i32 {
		return self.numerator;
	}

	pub fn get_value(&self) -> f64 {
		return self.numerator as f64 / self.denominator as f64;
	}

	fn greatest_common_divisor(num1:&i32, num2:&i32) -> u32 {
		let mut a:u32 = (*num1).abs() as u32;
		let mut b:u32 = (*num2).abs() as u32;

		if a < b {
			mem::swap(&mut a, &mut b);
		}

		loop {
			let r:u32 = a%b;
			if r == 0 {
				break;
			}
			a = b;
			b = r;
		}

		return b;
	}

	pub fn set(&mut self, numerator:&i32, denominator:&i32) {
		if *denominator == 0 {
			panic!("Fraction denominator cannot be 0.");
		}
		self.numerator = *numerator;
		self.denominator = *denominator;
		self.simplify();
	}

	pub fn set_denominator(&mut self, denominator:&i32) {
		if *denominator == 0 {
			panic!("Fraction denominator cannot be 0.");
		}
		self.denominator = *denominator;
		self.simplify();
	}

	pub fn set_numerator(&mut self, numerator:&i32) {
		self.numerator = *numerator;
		self.simplify();
	}

	fn simplify(&mut self) {
		let gcd:u32 = Fraction::greatest_common_divisor(&self.numerator, &self.denominator);
		if gcd > 1 {
			self.numerator /= gcd as i32;
			self.denominator /= gcd as i32;
		}
	}
}
