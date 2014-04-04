fn max(i: int, n: int) -> int {
    return if i % n == 0 {
			i/n - 1
		} else {
			i/n
		};
}

fn sum(i: int, n: int) -> f64 {
    let max = max(i, n);
    let sum: f64 = (n + max * n) as f64 * (max as f64 / 2.0);
    return sum;
}

fn main() {
	let i = 1000;
	let n1 = 3;
	let n2 = 5;
	let n3 = 15;
	println!("{}", sum(i, n1) + sum(i, n2) - sum(i, n3));
}
