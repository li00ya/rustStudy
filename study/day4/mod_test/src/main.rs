// 保证 folder 在 `./folder.rs` 或 `./folder/mod.rs` 中定义
mod mod_tests;	//相当于 mod mod_tests { include!("mod_tests.rs") };
mod folder;
mod math;	//使用pub可以消除math模块中函数未使用警告

use mod_tests::is_run;
use folder::is_run as run;	//这里as功能相当于typedef

fn main() 
{
	is_run();
	run();

	let ret = math::add(3, 5);
	println!("add ret is {}", ret);

	let ret = math::sub(5, 2);
	println!("sub ret is {}", ret);
}
