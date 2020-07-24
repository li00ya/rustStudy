fn main() {
	let mut s = String::new();	//空字符串

	let a = String::from("this is a, ");
	let b = "this is b,".to_string();

	let c = a + &b;	//拼接a，b	, a 被释放
//	let d = format!{"{}{}", a, b};
	
	s.push('t');
	s.push_str("his is s");

	let a = &s[..4];

    println!("{}, {}, {}", b, c, s);
    println!("{}", a);

    println!("chars:");
	for i in s.chars() {	
		print!("{}, ", i);
	}
    println!("");
 
	println!("bytes:");
	for i in s.bytes() {	
		print!("{}, ", i);
	}
    println!("");
}
