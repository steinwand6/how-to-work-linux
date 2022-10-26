use std::path::Path;

use nix::fcntl::{flock, open, FlockArg, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{read, write};

fn main() {
    let path = Path::new("count");
    let fd = open(path, OFlag::O_RDWR, Mode::empty()).expect("open failed");
    flock(fd, FlockArg::LockExclusive).expect("flock failed");

    let mut buf = [0u8; 8];
    read(fd, &mut buf).expect("read failed");
    let content = buf
        .iter()
        .filter(|b| **b != 0)
        .map(|b| *b as char)
        .collect::<String>();
    let mut count: i64 = content
        .parse()
        .expect(format!("parse error: {}", content).as_str());
    count += 1;
    let content = count.to_string().into_bytes();

    let fd = open(path, OFlag::O_WRONLY | OFlag::O_TRUNC, Mode::empty()).expect("open failed");
    write(fd, &content).expect("write failed");
    flock(fd, FlockArg::Unlock).expect("unlock failed");
}
