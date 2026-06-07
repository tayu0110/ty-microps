use crate::{
    net::{
        NET_DEVICE_FLAG_LOOPBACK, NET_DEVICE_TYPE_LOOPBACK, NET_DEVICES, NetDevice, NetDeviceSpec,
        net_device_register,
    },
    util::{debug, debugdump, error, info},
};

const LOOPBACK_MTU: u16 = u16::MAX;

pub fn loopback_init() -> i32 {
    let mut dev = NetDevice::default();
    dev.r#type = NET_DEVICE_TYPE_LOOPBACK;
    dev.mtu = LOOPBACK_MTU;
    dev.flags = NET_DEVICE_FLAG_LOOPBACK;
    dev.hlen = 0;
    dev.alen = 0;
    dev.spec = NetDeviceSpec::Loopback;
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

pub(crate) fn loopback_output(
    dev: &mut NetDevice,
    r#type: u16,
    data: &[u8],
    _dst: Option<()>,
) -> i32 {
    debug!(
        "dev={}, type=0x{:04x}, len={}",
        dev.name,
        r#type,
        data.len()
    );
    debugdump(data);
    dev.input(r#type, data)
}
