fn euclidian_algorithm(a:u32, b:u32) -> u32 {
	let r:u32 = a%b;
	if r == 0 {
		return b;
	}
	return euclidian_algorithm(b, r);
}

pub fn fibonacci(n:u32) -> u64 {
	let mut fib0:u64 = 0;
	let mut fib1:u64 = 1;

	if n == 0 {
		return fib0;
	}
	if n == 1 {
		return fib1;
	}

	for _ in 2..n+1 {
		let tmp:u64 = fib1;
		fib1 += fib0;
		fib0 = tmp;
	}
	return fib1;
}

pub fn greatest_common_divisor(a:u32, b:u32) -> u32 {
	if a < b {
		euclidian_algorithm(b, a);
	}
	return euclidian_algorithm(a, b);
}

pub fn is_leap_year(year:i32) -> bool {
	return year%4==0 && year%100!=0 || year%400==0;
}

pub fn swap_values(val1:&mut i32, val2:&mut i32) {
	let tmp:i32 = *val1;
	*val1 = *val2;
	*val2 = tmp;
}
