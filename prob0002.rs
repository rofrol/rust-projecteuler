fn main() {
	let mut a = 1;
	let mut b = 1;
	let mut fib = 1;
	let mut sum = 0;
	while fib < 4000000 {
		if fib % 2 == 0 {
			sum += fib;
		}
		fib = a + b;
		a = b;
		b = fib;
	}
	println!("{}", sum);
}
