/*
*Rust内存管理: Ownership的规则
*	在Rust中，每一个值都有对应的变量，这个变量称为值的owner,
*	一个值在某一时刻只能有一个owner,
*	当owner超出作用域后，值会被销毁.
*/

fn main()
{
	let s1 = "this is s1".to_string();

	let s2 = s1;//此时s1传递给s2之后，s1被销毁 

	let s3 = s2.clone(); //s2复制了一份数据传给s3

	let _len = takesowner(s2);	//把s2传递到函数中，并在函数内部被释放
	let len = two(&s3);	//把s3的值传递到函数中
	println!("len {}, s3: {}", len, s3);

	let s = first_word(&s3);

	let c = strchr(&s3, 2);
	println!("first word: {}, strstr is {}", s, c);
}

fn takesowner(s: String) -> usize
{
	s.len()
}

fn two(s: &String) -> usize
{
    s.len()
}

fn first_word(s: &String) -> &str
{
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b's' {
			return &s[..=i];
		}
	}

	return &s[..];
}

fn strchr(s: &String, i: usize)->char
{
	let bytes = s.as_bytes();

	return bytes[i] as char;
}
