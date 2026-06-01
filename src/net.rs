//! 原作コード: [net.c](https://github.com/pandax381/microps/blob/master/net.c)

use log::{error, info, warn};

use crate::platform::{platform_init, platform_run, platform_shutdown};

pub fn net_init() -> i32 {
    info!("initialize...");
    if platform_init() == -1 {
        error!("platform_init() failure");
        return -1;
    }
    info!("success");
    0
}

pub fn net_run() -> i32 {
    info!("startup...");
    if platform_run() == -1 {
        error!("platform_run() failure");
        return -1;
    }
    info!("success");
    0
}

pub fn net_shutdown() -> i32 {
    info!("shutting down...");
    if platform_shutdown() == -1 {
        warn!("platform_shutdown() failure");
        return -1;
    }
    info!("success");
    0
}
