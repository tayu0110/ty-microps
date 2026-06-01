use std::{
    process::exit,
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering::Relaxed},
    },
    thread::sleep,
    time::Duration,
};

use log::{debug, error, info};
use ty_microps::net::{net_init, net_run, net_shutdown};

static TERMINATE: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| Arc::new(AtomicBool::new(false)));

fn setup() -> i32 {
    if let Err(err) = signal_hook::flag::register(signal_hook::consts::SIGINT, (*TERMINATE).clone())
    {
        error!("signal handler registration failure: {err}");
    }
    info!("setup protocol stack...");
    if net_init() == -1 {
        error!("net_init() failure");
        return -1;
    }
    if net_run() == -1 {
        error!("net_run() failure");
        return -1;
    }
    0
}

fn app_main() -> i32 {
    debug!("press Ctrl+C to terminate");
    while !TERMINATE.load(Relaxed) {
        sleep(Duration::from_secs(1));
    }
    debug!("terminate");
    0
}

fn cleanup() -> i32 {
    info!("cleanup protocol stack...");
    if net_shutdown() == -1 {
        error!("net_shutdown() failure");
        return -1;
    }
    0
}

fn main() {
    env_logger::init();
    if setup() == -1 {
        error!("setup() failure");
        exit(-1);
    }
    let ret = app_main();
    if cleanup() == -1 {
        error!("cleanup() failure");
        exit(-1);
    }
    exit(ret);
}
