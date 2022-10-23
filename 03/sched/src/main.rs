use nix::{
    sched,
    sys::wait,
    unistd::{self, fork, ForkResult},
};
use std::time::{self, Duration};
use std::{fs::File, io::Write};

const NLOOP_FOR_ESTIMATION: usize = 100000000;

#[derive(Debug)]
enum SchedError {
    FewArguments(usize),
    CannotParseParrallel,
}

fn main() -> Result<(), SchedError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("引数が不足。");
        return Err(SchedError::FewArguments(args.len()));
    }
    let arg_1 = args.get(1).expect("");
    let concurrency = match arg_1.parse::<usize>() {
        Ok(x) => x,
        Err(_) => {
            println!("並列度は1以上の整数であること。");
            return Err(SchedError::CannotParseParrallel);
        }
    };
    let mut cpu_set = nix::sched::CpuSet::new();
    cpu_set.set(concurrency).expect("failed set cuncurrency");
    sched::sched_setaffinity(unistd::Pid::from_raw(0), &cpu_set).expect("failed shced setaffinity");
    let nloop_per_msec = estimate_loops_per_msec();

    let start = time::Instant::now();
    for i in 0..concurrency {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { .. }) => (),
            Ok(ForkResult::Child) => {
                child_fn(i, nloop_per_msec, start);
            }
            Err(_) => unsafe { libc::_exit(1) },
        }
    }
    for _ in 0..concurrency {
        wait::wait().expect("wait failed");
    }
    Ok(())
}

fn estimate_loops_per_msec() -> u128 {
    let start = time::Instant::now();
    for _ in 0..NLOOP_FOR_ESTIMATION {}
    let end = time::Instant::now();
    NLOOP_FOR_ESTIMATION as u128 / end.duration_since(start).as_millis()
}

fn child_fn(n: usize, nloop: u128, start: time::Instant) {
    let mut progress: [Duration; 100] = [Duration::ZERO; 100];
    for i in 0..100 {
        for _ in 0..nloop {}
        progress[i] = time::Instant::now().duration_since(start);
    }
    let mut f = File::create(format!("{}.data", n)).expect("create file failed");
    for i in 0..100 {
        write!(f, "{}\t{}\n", progress[i].as_secs_f64() * 1000.0, i).expect("write file failed");
    }
    unsafe { libc::_exit(0) };
}
