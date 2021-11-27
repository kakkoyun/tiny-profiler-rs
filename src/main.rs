use anyhow::{bail, Result};
use profiler::*;
use structopt::StructOpt;

#[path = "bpf/profiler.skel.rs"]
mod profiler;

#[derive(Debug, StructOpt)]
struct Command {
    /// verbose output
    #[structopt(long, short)]
    verbose: bool,
    /// glibc path
    #[structopt(long, short, default_value = "/lib/x86_64-linux-gnu/libc.so.6")]
    glibc: String,
    #[structopt(long, short)]
    /// pid to observe
    pid: Option<i32>,
}

fn bump_memlock_rlimit() -> Result<()> {
    let rlimit = libc::rlimit {
        rlim_cur: 128 << 20,
        rlim_max: 128 << 20,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
        bail!("Failed to increase rlimit");
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Command::from_args();

    let mut skel_builder = ProfilerSkelBuilder::default();
    if opts.verbose {
        skel_builder.obj_builder.debug(true);
    }

    bump_memlock_rlimit()?;
    // let mut open_skel = skel_builder.open()?;
    // if let Some(pid) = opts.pid {
    //     open_skel.rodata().target_pid = pid;
    // }
    // let mut skel = open_skel.load()?;

    // TODO(kakkoyun):
    // - Every 10s read the maps
    // - Print aggregated stats
    // - Cleanup maps

    // let running = Arc::new(AtomicBool::new(true));
    // let r = running.clone();
    // ctrlc::set_handler(move || {
    //     r.store(false, Ordering::SeqCst);
    // })?;
    //
    // while running.load(Ordering::SeqCst) {
    //     perf.poll(Duration::from_millis(100))?;
    // }

    Ok(())
}
