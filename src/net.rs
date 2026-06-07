//! 原作コード: [net.c](https://github.com/pandax381/microps/blob/master/net.c)

use std::sync::Mutex;

use crate::{
    driver::loopback::loopback_output,
    platform::{platform_init, platform_run, platform_shutdown},
    util::{debug, debugdump, error, info, warn},
};

pub const NET_DEVICE_TYPE_DUMMY: u16 = 0x0000;
pub const NET_DEVICE_TYPE_LOOPBACK: u16 = 0x0001;
const NET_DEVICE_TYPE_ETHERNET: u16 = 0x0002;

const NET_DEVICE_FLAG_UP: u16 = 0x0001;
pub const NET_DEVICE_FLAG_LOOPBACK: u16 = 0x0010;
const NET_DEVICE_FLAG_BROADCAST: u16 = 0x0020;
const NET_DEVICE_FLAG_P2P: u16 = 0x0040;
const NET_DEVICE_FLAG_NEED_ARP: u16 = 0x0100;

const NET_DEVICE_ADDR_LEN: usize = 16;

type NetDeviceOpsOpen = fn(dev: &mut NetDevice) -> i32;
type NetDeviceOpsClose = fn(dev: &mut NetDevice) -> i32;
type NetDeviceOpsOutput = fn(dev: &mut NetDevice, r#type: u16, data: &[u8], dst: Option<()>) -> i32;

#[derive(Default)]
pub enum NetDeviceSpec {
    #[default]
    Loopback,
}

impl NetDeviceSpec {
    fn open(&self) -> Option<NetDeviceOpsOpen> {
        None
    }

    fn close(&self) -> Option<NetDeviceOpsClose> {
        None
    }

    fn output(&self) -> NetDeviceOpsOutput {
        match self {
            Self::Loopback => loopback_output,
        }
    }
}

#[derive(Default)]
pub struct NetDevice {
    // next, indexは省略
    //  next    : 連結リストにはしないので不要
    //  index   : Vecのインデックスで代えられるので不要
    pub name: String,
    pub r#type: u16,
    pub mtu: u16,
    pub flags: u16,
    pub hlen: u16,
    pub alen: u16,
    addr: [u8; NET_DEVICE_ADDR_LEN],
    broadcast: [u8; NET_DEVICE_ADDR_LEN],
    pub spec: NetDeviceSpec,
}

impl NetDevice {
    pub fn is_up(&self) -> bool {
        self.flags & NET_DEVICE_FLAG_UP != 0
    }

    pub fn state(&self) -> &'static str {
        if self.is_up() { "UP" } else { "DOWN" }
    }

    fn open(&mut self) -> i32 {
        info!("dev={}", self.name);
        if self.is_up() {
            error!("already opened, dev={}", self.name);
            return -1;
        }
        if let Some(open) = self.spec.open()
            && open(self) == -1
        {
            error!("failure, dev={}", self.name);
            return -1;
        }
        self.flags |= NET_DEVICE_FLAG_UP;
        0
    }

    fn close(&mut self) -> i32 {
        info!("dev={}", self.name);
        if !self.is_up() {
            error!("not opened, dev={}", self.name);
            return -1;
        }
        if let Some(close) = self.spec.close()
            && close(self) == -1
        {
            error!("failure, dev={}", self.name);
            return -1;
        }
        self.flags &= !NET_DEVICE_FLAG_UP;
        0
    }

    pub fn input(&mut self, r#type: u16, data: &[u8]) -> i32 {
        debug!(
            "dev={}, type=0x{:04x}, len={}",
            self.name,
            r#type,
            data.len()
        );
        debugdump(data);
        0
    }

    pub fn output(&mut self, r#type: u16, data: &[u8], _dst: Option<()>) -> i32 {
        debug!(
            "dev={}, type=0x{:04x}, len={}",
            self.name,
            r#type,
            data.len()
        );
        debugdump(data);
        if !self.is_up() {
            error!("not opened, dev={}", self.name);
            return -1;
        }
        if let output = self.spec.output()
            && output(self, r#type, data, _dst) == -1
        {
            error!("failure, dev={}, len={}", self.name, data.len());
            return -1;
        }
        if (self.mtu as usize) < data.len() {
            error!(
                "too long, dev={}, mtu={}, len={}",
                self.name,
                self.mtu,
                data.len()
            );
            return -1;
        }
        0
    }
}

pub static NET_DEVICES: Mutex<Vec<NetDevice>> = Mutex::new(vec![]);

pub fn net_device_register(mut dev: NetDevice) -> i32 {
    let mut devices = NET_DEVICES.lock().unwrap();
    let ret = devices.len();
    dev.name = format!("net{}", devices.len());
    info!("success, dev={}, type=0x{:04x}", dev.name, dev.r#type);
    devices.push(dev);
    ret as i32
}

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
    let mut devices = NET_DEVICES.lock().unwrap();
    for dev in &mut *devices {
        dev.open();
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
    let mut devices = NET_DEVICES.lock().unwrap();
    for dev in &mut *devices {
        dev.close();
    }
    info!("success");
    0
}
