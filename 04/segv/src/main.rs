fn main() {
	println!("不正メモリアクセス前");
	unsafe { *(1 as *mut u32) = 42; }
	println!("不正メモリアクセス後");
}
