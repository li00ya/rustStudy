/*
Vec<T> ，也被称为 vector。vector 允许我们在一个单独的数据结构
中储存多于一个的值，它在内存中彼此相邻地排列所有的值。vector 只能储存相同类型的值。
为了创建一个新的空 vector，可以调用 Vec::new 函数,更常见的做法
是使用初始值来创建一个Vec，而且为了方便 Rust 提供了vec! 宏。这个
宏会根据我们提供的值来创建一个新的Vec

*/
fn main() {
	let mut v: Vec<i32> = Vec::new();

	v.push(5);
	v.push(6);
	v.push(7);

    println!("vector {:?}", v);
	let v = vec![1, 2, 3, 4];
	let two: &i32 = &v[1];
	println!{"v[1] is {}", two};

	let third = v.get(2).unwrap();		//v.get(2) return Option<&T>
	println!{"v[2] is {}", third};

	vec_show(&v);

	let mut v = vec![1, 2, 3, 4];

	for i in &mut v {
		*i += 50;
	}

	vec_show(&v);

	#[derive(Debug)]
	enum Types {
		Int(i32),
		Float(f32),
		Txt(String),
	}

	let v = vec![Types::Int(3), Types::Float(3.5), Types::Txt("this is vec".to_string())];

	println!{"v is {:?}", v};
}

fn vec_show(vec: &Vec<i32>)
{
	print!("Vec: ");
	for i in vec {
		print!("{}, ", i);
	}
	println!{""};
}
