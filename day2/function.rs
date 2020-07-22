
fn main() {
	let x: i32 = 3;

	let y = func1(x);
	func2(x);
	func2(y);

	func3(x);
}

fn func1(x: i32) -> i32 {
	return x + 1;
}

fn func2(x: i32) {
	println!("the num is {}", x);
}

fn func3(x: i32) {

	let x: bool = if x != 0 {true} else {false};

	if x {
		println!("if is ture!");
	} else {
		println!("if is false!");
	}
}
