pub fn add(x: i32, y: i32) -> i32
{
	if super::DEBUG {
		println!("{} add {}", x, y);
	}
	x + y
}
