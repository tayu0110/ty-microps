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
use ty_microps::net::{
    NET_DEVICE_TYPE_DUMMY, NET_DEVICES, NetDevice, net_device_register, net_init, net_run,
    net_shutdown,
};

fn dummy_init() -> i32 {
    let mut dev = NetDevice::default();
    dev.r#type = NET_DEVICE_TYPE_DUMMY;
    dev.mtu = 128;
    dev.hlen = 0;
    dev.alen = 0;
    let ret = net_device_register(dev);
    if ret == -1 {
        error!("net_device_register() failure");
        return -1;
    }
    info!(
        "success, dev={}",
        NET_DEVICES.lock().unwrap()[ret as usize].name
    );
    ret
}

static TERMINATE: LazyLock<Arc<AtomicBool>> = LazyLock::new(|| Arc::new(AtomicBool::new(false)));
static TEST_DATA: &[u8] = &[
    0x45, 0x00, 0x00, 0x30, 0x00, 0x80, 0x00, 0x00, 0xff, 0x01, 0xbd, 0x4a, 0x7f, 0x00, 0x00, 0x01,
    0x7f, 0x00, 0x00, 0x01, 0x08, 0x00, 0x35, 0x64, 0x00, 0x80, 0x00, 0x01, 0x31, 0x32, 0x33, 0x34,
    0x35, 0x36, 0x37, 0x38, 0x39, 0x30, 0x21, 0x40, 0x23, 0x24, 0x25, 0x5e, 0x26, 0x2a, 0x28, 0x29,
];

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
    let dev = dummy_init();
    if dev < 0 {
        error!("dummy_init() failure");
        return -1;
    }
    if net_run() == -1 {
        error!("net_run() failure");
        return -1;
    }
    dev
}

fn app_main(dev: usize) -> i32 {
    debug!("press Ctrl+C to terminate");
    while !TERMINATE.load(Relaxed) {
        let devices = NET_DEVICES.lock().unwrap();
        if devices[dev].output(0x0800, TEST_DATA, None) == -1 {
            error!("net_device_output() failure");
            break;
        }
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
    let dev = setup();
    if dev == -1 {
        error!("setup() failure");
        exit(-1);
    }
    let ret = app_main(dev as usize);
    if cleanup() == -1 {
        error!("cleanup() failure");
        exit(-1);
    }
    exit(ret);
}
