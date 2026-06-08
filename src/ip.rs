use crate::{
    net::{NET_PROTOCOL_TYPE_IP, NetDevice, net_protocol_register},
    util::{debug, debugdump, error},
};

pub fn ip_init() -> i32 {
    let ret = net_protocol_register(NET_PROTOCOL_TYPE_IP, ip_input);
    if ret == -1 {
        error!("net_protocol_register() failure");
        return -1;
    }
    ret
}

fn ip_input(dev: &mut NetDevice, data: &[u8]) {
    debug!("dev={}, len={}", dev.name, data.len());
    debugdump(data);
}
