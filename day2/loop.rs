fn main()
{
	test_loop();

	test_while();

	test_for1();
	test_for2();
	test_for3();
}

/* equal to while(1)*/
fn test_loop()
{
	let mut cnt = 0u32;

	let ret = loop {
		cnt += 1;
		if cnt == 10 {
			break cnt * 2;
		}
	};

	println!("the loop ret is {}, cnt {}", ret, cnt);
}

fn test_while()
{
	let mut cnt = 15;

	while cnt / 2 != 0 {
		cnt -= 1;
	}

	println!("the while cnt is {}", cnt);
}

fn test_for1()
{
	let arr = [1, 3, 5, 7, 9];

	for ele in arr.iter() {
		println!("the arr is {}", ele);
	}
}

fn test_for2()
{
	for num in (1..=4).rev() {
		println!("the for2 num is {}", num);
	}

}

fn test_for3()
{
	for num in (1..4).rev() {
		println!("the for3 num is {}", num);
	}

}
