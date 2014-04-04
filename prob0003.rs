fn main() {
	// The prime factors of 13195 are 5, 7, 13 and 29.
	//let limit: uint = 13195;
	let mut limit: uint = 600851475143;
	let mut i: uint = 2;
	loop {
		if i > limit/2{
			break;
		} else if limit % i == 0 {
			limit = limit / i;
			i = 2;
		} else {
			i += 1;
		}
	}
	println!("{}", limit);
}
