fn main() {
    let mut m = 5;

	match m {
	5 => {
			m += 1;
			println!("match sucess. m is {}", m);
		},
	_ => (),
	}

	/* if let 分支 = 条件 等价于 match 条件 分支：*/
	if let 6 = m {
		m += 1;
		println!("if match sucess. m is {}", m);
	}

	let five = Some(5);
	let six = add_one(five);
	println!("six is {:?}", six);
}

fn add_one(x: Option<i32>) -> Option<i32>
{
	match x {
	Some(i) => Some(i + 1),
	None => None,
	}
}
