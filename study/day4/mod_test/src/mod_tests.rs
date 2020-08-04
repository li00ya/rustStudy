/*
绝对路径（absolute path）从 crate 根开始，以crate名或者字面值 crate 开头。
相对路径（relative path）从当前模块开始，以self、super或当前模块的标识符开头.
crate::foo 等价于 /foo - 如果我们认为 “根文件系统” 为包含 main.rs 或 lib.rs 的目录
super::foo 等价于 ../foo
self::foo 等价于 ./foo
Rust默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。父模块中的项不能
使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。这是因为子模块封装并
隐藏了他们的实现详情，但是子模块可以看到他们定义的上下文。
使用关键字pub可以转化为公有.

*/

pub fn is_run() {
	println!("this is mod mod_tests file.");
}

