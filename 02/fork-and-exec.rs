#!/usr/bin/env rust-script
// cargo-deps: nix, libc

use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, getppid, getpid, execve}};
use std::ffi::CString;

match unsafe{fork()} {
	Ok(ForkResult::Parent { child, .. }) => {
		println!("親プロセス: pid={}, 子プロセスのpid={}", getpid(), child);
		waitpid(child, None).unwrap();
	}
	Ok(ForkResult::Child) => {
		let cmd = CString::new("/bin/echo").expect("CString::new failed");
		let args = [CString::new("echo")?,
					CString::new(format!("pid={} からこんにちは", getpid())).expect("CString::new failed")];
		let env = CString::new("").expect("CString::new failed");
		println!("子プロセス: pid={}, 親プロセスのpid={}", getpid(), getppid());
		execve(&cmd, &args, &[env]).expect("execve failed");
	}
	Err(_) => unsafe { libc::_exit(1) }
}
