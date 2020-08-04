#[derive(Debug)]
struct User {
	username:String,
	email:String,
	acount:u64,
	active:bool,
}

struct Color(i32, i32, u32);
struct Point(i32, i32, u32);	//名字不同，两结构体就不相同

use rand::Rng;
fn build_user(username: String, email: String)-> User
{
	let count = rand::thread_rng().gen_range(100, 1073741823);
	User {
		username:username,
		email:email,
		acount:count as u64,
		active:true,
	}
}

fn build_user1(username: String, email: String)-> User
{
	let count = rand::thread_rng().gen_range(100, 1073741823);
	User {
		username,	//字段名和参数名相同时，可以省略字段名
		email,
		acount:count as u64,
		active:true,
	}
}

//struct method,方法的第一个参数必须是self
/*impl 块的另一个有用的功能是：允许在  impl 块中定义不以self作为参数的函数。这被称为
关联函数（associated functions），因为它们与结构体相关联。它们仍是函数而不是方法，因为
它们并不作用于一个结构体的实例。你已经使用过  String::from 关联函数了。
*/
impl User{	//指定结构体类型
	fn name_len(&self) ->usize	//&self --> rt: &User
	{
		self.username.len()
	}

	fn is_same(&self, other: &User) ->bool
	{
		self.acount == other.acount
	}

	fn copy(other: &User) -> User
	{
		User {
			email:String::from("111222@163.com"),
			username:String::from("sdfggg"),
			..*other
		}
	}
}

fn main()
{
	let use1 = build_user("lihua".to_string(), "12345678@qq.com".to_string());
	let use2 = build_user1("xinhua".to_string(), "66666612@qq.com".to_string());

	let use3 = User {
		username:String::from("wangli"),
		email:String::from("1111111@163.com"),
		..use1		//表示剩下字段值与use1相同
	};

	let _p1 = Color(0, 1, 2);
	let _p2 = Point(0, 1, 2);

	println!("user1: {:?}", use1);
	println!("user1: {:#?}", use1);
	println!("user2: {:?}", use2);
	println!("user3: {:?}", use3);

	let l3 = use3.name_len();
	println!("use3 username len is {}", l3);
	let l3 = use3.is_same(&use2);
	println!("use3 same as use2? ->{}", l3);

	let use4 = User::copy(&use1);
	println!("use4: {:#?}", use4);
}

/*
println! 宏能处理很多类型的格式，不过， {} 默认告诉  println! 使用被称为  Display 的格
式：意在提供给直接终端用户查看的输出。目前为止见过的基本类型都默认实现了  Display ，因
为它就是向用户展示  1 或其他任何基本类型的唯一方式。不过对于结构体， println! 应该用来
输出的格式是不明确的，因为这有更多显示的可能性：是否需要逗号？需要打印出大括号吗？所
有字段都应该显示吗？由于这种不确定性，Rust 不会尝试猜测我们的意图，所以结构体并没有提
供一个  Display 实现。
在{}中加入  :? 指示符告诉  println! 我们想要使用叫做  Debug 的输出格式。 Debug 是一个
trait，它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的
值。
Rust 确实 包含了打印出调试信息的功能，不过我们必须为结构体显式选择这个功能。为此，在结
构体定义之前加上  #[derive(Debug)] 注解,
当我们有一个更大的结构体时，能有更易读一点的输出就好了，为此可以使用  {:#?} 替换
println! 字符串中的  {:?} 。
*/
