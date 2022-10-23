use nix::{sched, unistd};
use std::time;

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
    estimate_loops_per_msec();
    Ok(())
}

fn estimate_loops_per_msec() {
    let start = time::Instant::now();
    for _ in 0..NLOOP_FOR_ESTIMATION {}
    let end = time::Instant::now();
    println!(
        "{:?}",
        NLOOP_FOR_ESTIMATION as u128 / end.duration_since(start).as_millis() / 1000
    );
}
