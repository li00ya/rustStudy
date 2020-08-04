pub fn sub(x: i32, y: i32) -> i32
{
	super::add::add(x, -y)	//或者使用mod里面的重导函数 super::add(x, -y)
//	x - y
}
