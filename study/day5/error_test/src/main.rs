
use std::fs::File;
use std::io::ErrorKind;

fn main() {
/*
	let v = vec![1, 2, 3];
    println!("{}", v[99]);
*/

	let f = File::open("hello.txt");
	let f = match f {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file: {:?}", e),
			},
			other_error => panic!("Problem opening the file: {:?}", other_error),
		},
	};

	/*等价于*/

	let f = File::open("hello.txt").unwrap_or_else(|error| {
		if error.kind() == ErrorKind::NotFound {
			File::create("hello.txt").unwrap_or_else(|error| {
				panic!("Problem creating the file: {:?}", error);
			})
		} else {
			panic!("Problem opening the file: {:?}", error);
		}
	});

	read_file_string1();
	read_file_string2();
	read_file_string3();
	let s = read_file_string4();

	println!("return {:#?}", s);

}

use std::io;
use std::io::Read;

fn read_file_string1() ->Result<String, io::Error>
{
	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};

	let mut s = String::new(); 

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

fn read_file_string2() ->Result<String, io::Error>
{
	let mut f = File::open("hello.txt")?;	//? 相当于return Result<T, E>

	let mut s = String::new(); 

	f.read_to_string(&mut s)?;

	Ok(s)
}

fn read_file_string3() ->Result<String, io::Error>
{
	let mut s = String::new(); 
	File::open("hello.txt")?.read_to_string(&mut s)?;

	Ok(s)
}

use std::fs;
fn read_file_string4() ->Result<String, io::Error>
{
	fs::read_to_string("hello.txt")
}
