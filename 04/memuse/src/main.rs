use std::process;
use std::io::{self, Write};

fn main() {
    let size = 10000000;
	println!("メモリ獲得前");
	let output = process::Command::new("free").output().expect("free failed");
	io::stdout().write_all(&output.stdout).unwrap();
	let _array = vec![1000 as i64; size];
	println!("メモリ獲得後");
	let output = process::Command::new("free").output().expect("free failed");
	io::stdout().write_all(&output.stdout).unwrap();
}
