/*
HashMap(K, V)
像 vector 一样，哈希 map 将它们的数据储存在堆上，这个  HashMap 的键类型是  String 而值
类型是  i32 。类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同
类型
*/
use std::collections::HashMap;

fn main() {
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	println!("scores: {:#?}", scores);

	let name = String::from("Blue");
	let score = scores.get(&name).unwrap();

	println!("name \"{}\", score {}", name, score);

	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

	//更新map,使用insert可以直接覆盖旧值
	scores.insert(String::from("Yellow"), 150);
	let score = scores.get(&"Yellow".to_string()).unwrap();
	println!("name \"{}\", score {}", "Yellow", score);
	
	

	/* entry 只有键没有对应的值时才会生效 */
	scores.entry(String::from("Yellow")).or_insert(50); //不变
	scores.entry(String::from("Green")).or_insert(20);	//20
	println!("scores: {:#?}", scores);


	let text = "hello world wonderful world";
	let mut map = HashMap::new();
	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;	// or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）
	}
	println!("{:?}", map);

}
