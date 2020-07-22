/*
*基本数据类型
*	布尔值(bool)
*	字符(char)
*	有符号整型(i8, i16, i32, i64, i128)
*	无符号整型(u8, u16, u32, u64, u128)
*	指针大小的有符号/无符号整型(isize/usize，取决于计算机架构，32bit 的系统上，isize 等价于i32)
*	浮点数(f32, f64)
*	数组(arrays)，由相同类型元素构成，长度固定。
*/

fn main() {
	let x = "32";
	let z = 'z';
	let m = {
		let n = 3;
		n + 1		//后无符号，意为返回此表达式的值，若加分号，则变成语句，无返回值
	};
	println!("x is {}, z is {}, m is {}", x, z, m);

	/*变量名前添加_，可以消除编译警告：变量未使用*/
	// 不可变
	let _a = true;
	let _b: bool = true;
	let (_x, _y) = (1, 2);

	// 可变
	let mut _z = 5;
	_z = 6;

	/* 定义必须初始化 */
	// 静态变量(不可变)
	static _N: i32 = 5;

	// 静态变量(可变)
	static mut _M: i32 = 5;

	//tuple and array
	let x: (i32, f64, u8) = (500, 12.5, 2);
	let y = (500, 12.5, 2);
	let m = y.0;
	let n = x.2;
	let (a, _, c) = x;		//_ 表示占位符
	println!("x is {:?}, y is {:?}", x, y);
	println!("m(y0) is {}, a {}, n(y2) is {}, c {}, c 0x{:x}", m, a, n, c, c);

	let x = [1, 2, 3, 4];
	let y: [i32; 4] = [1, 2, 3, 4];
	let z = [1; 4];
	println!("x is {:?}, y is {:?}, z is {:?}", x, y, z);

/*
切片(slice)，指向一段内存的指针。
	切片并没有拷贝原有的数组，只是指向原有数组的一个连续部分，行为同数组。访问切片指向的数组/数据结构
，可以使用&操作符。
*/
	let a: [i32; 4] = [1, 2, 3, 4];

	let _e: &[i32] = &a; // 全部
	let _e = &a[0..4]; // [0, 4)
	let _e = &a[..]; // 全部

	let _e = &a[1..3]; // [2, 3]
	let _e = &a[1..]; // [2, 3, 4]
	let _e = &a[..3]; // [1, 2, 3]
/*
字符串(str)
	在 Rust 中，str 是不可变的静态分配的一个未知长度的UTF-8字节序列。&str 是指向该字符串的切片。
	字符串切片&str指向的字符串是静态分配的，在 Rust 中，有另一个堆分配的，可变长的字符串类型String
(非基本数据类型)。通常由字符串切片&str通过 to_string() 或 String::from() 方法转换得到。
*/

	let a = "Hello, world!"; //a: &'static str
	let b: &str = "你好, 世界!";

	let s1 = "Hello, world!".to_string();
	let s2 = String::from("Hello, world!");
	let s3 = String::from("Hello, ") + "world!";

	println!("a->{}, b->{}, s1->{}, s2->{}, s3->{}", a, b, s1, s2, s3);

	let a = &s1[1..=2];
	println!("a is {},", a);
	
	slice();
}

fn slice()
{
	 let arr = [100, 200, 300, 400, 500, 600];
	 let mut i = 0;
	 let a = &arr[1..=3];
	 let len = a.len();

	 println!("arr {:?}, Elements of 'a' array:", arr);
	 while i<len
	 {
	  println!("i{} {}", i, a[i]);
	  i=i+1;
	 }

 }
