#!/usr/bin/env rust-script
// cargo-deps: nix, libc

use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, getppid, getpid}};

match unsafe{fork()} {
   Ok(ForkResult::Parent { child, .. }) => {
	   println!("親プロセス: pid={}, 子プロセスのpid={}", getpid(), child);
       waitpid(child, None).unwrap();
   }
   Ok(ForkResult::Child) => {
       // Unsafe to use `println!` (or `unwrap`) here. See Safety.
       // write(libc::STDOUT_FILENO, "I'm a new child process\n".as_bytes()).ok();
	   println!("子プロセス: pid={}, 親プロセスのpid={}", getpid(), getppid());
       unsafe { libc::_exit(0) };
   }
   Err(_) => unsafe { libc::_exit(1) }
}
