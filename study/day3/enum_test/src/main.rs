/*
Quit 没有关联任何数据。
Move 包含一个匿名结构体。
Write 包含单独一个  String 。
ChangeColor 包含三个  i32 
*/
#[derive(Debug)]
#[allow(dead_code)]	//消除未使用枚举值的警告
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message{
	fn show(&self)
	{
		println!("context: {:?}", self);
	}
}

fn main()
{
	let m = Message::Write(String::from("hello"));
	m.show();

// 枚举Option<T>,define in std标准库中原型
// T表示任意类型
/*
enum  Option<T> {
	Some(T),
	None,
}
*/

	let a = Some(5);
	let b = Some("some value");
	let c: Option<i32> = None;	//使用None, 需要指定类型

	if a.unwrap() == 5 {	//method unwrap() mean turn Option<T> to T
		println!("a is 5");
	} else {
		println!("a {:?}", a);
	}

	if b == Some("some value") {
		println!("b is some value");
	} else {
		println!("b {:?}", b);
	}
	println!("c {:?}", c);
}

