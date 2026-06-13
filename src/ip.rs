use std::net::Ipv4Addr;

use bincode::{Decode, decode_from_slice};

use crate::{
    net::{NET_PROTOCOL_TYPE_IP, NetDevice, net_protocol_register},
    util::{cksum16, debug, debugdump, error},
};

const IP_VERSION_IPV4: u8 = 4;

const IP_HDR_SIZE_MIN: usize = 20;
const IP_HDR_SIZE_MAX: usize = 60;

const IP_HDR_FLAG_MF: u16 = 0x2000;
const IP_HDR_FLAG_DF: u16 = 0x4000;
const IP_HDR_FLAG_RF: u16 = 0x8000;

#[repr(C)]
#[derive(Decode)]
pub struct IPHeader {
    vhl: u8,
    tos: u8,
    total: u16,
    id: u16,
    offset: u16,
    ttl: u8,
    protocol: u8,
    sum: u16,
    src: u32,
    dst: u32,
}

impl IPHeader {
    pub fn version(&self) -> u8 {
        self.vhl >> 4
    }

    pub fn header_length(&self) -> u8 {
        (self.vhl & 0xF) << 2
    }

    pub fn type_of_service(&self) -> u8 {
        self.tos
    }

    pub fn total_length(&self) -> u16 {
        self.total
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn flags(&self) -> u8 {
        (self.offset >> 13) as u8
    }

    pub fn fragment_offset(&self) -> u16 {
        self.offset & 0x1FFF
    }

    pub fn ttl(&self) -> u8 {
        self.ttl
    }

    pub fn protocol(&self) -> u8 {
        self.protocol
    }

    pub fn checksum(&self) -> u16 {
        self.sum
    }

    pub fn source_address(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.src)
    }

    pub fn destination_address(&self) -> Ipv4Addr {
        Ipv4Addr::from_bits(self.dst)
    }
}

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

    if data.len() < IP_HDR_SIZE_MIN {
        error!("too short");
        return;
    }

    let header: IPHeader = match decode_from_slice(
        data,
        bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding(),
    ) {
        Ok((header, _len)) => {
            // data = &data[_len..];
            header
        }
        Err(err) => {
            error!("failed to decode header: {}", err);
            return;
        }
    };
    if header.version() != IP_VERSION_IPV4 {
        error!("ip version error: v={}", header.version());
        return;
    }
    if data.len() < header.header_length() as usize {
        error!(
            "header length error: len={} < hlen={}",
            data.len(),
            header.header_length()
        );
        return;
    }
    if cksum16(&data[..header.header_length() as usize], 0) != 0 {
        error!("checksum error");
        return;
    }
    if data.len() < header.total_length() as usize {
        error!(
            "total length error: len={} < total={}",
            data.len(),
            header.total_length()
        );
        return;
    }
    if header.offset & IP_HDR_FLAG_MF != 0 || header.fragment_offset() != 0 {
        error!("fragments does not support");
        return;
    }
    ip_print(data);
}

fn ip_print(data: &[u8]) {
    let header: IPHeader = match decode_from_slice(
        data,
        bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding(),
    ) {
        Ok((header, _len)) => {
            // data = &data[_len..];
            header
        }
        Err(err) => {
            error!("failed to decode header: {}", err);
            return;
        }
    };

    debug!(
        "{:>16}: 0x{:02x} [v: {}, hl: {}]",
        "vhl",
        header.vhl,
        header.version(),
        header.header_length()
    );
    debug!("{:>16}: 0x{:02x}", "tos", header.type_of_service());
    debug!(
        "{:>16}: {} (payload: {})",
        "total",
        header.total_length(),
        header.total_length() - header.header_length() as u16
    );
    debug!("{:>16}: {}", "id", header.id());
    debug!(
        "{:>16}: 0x{:04x} [flags={:x}, offset={}]",
        "offset",
        header.offset,
        header.flags(),
        header.fragment_offset()
    );
    debug!("{:>16}: {}", "ttl", header.ttl());
    debug!("{:>16}: {}", "protocol", header.protocol());
    debug!("{:>16}: 0x{:04x}", "sum", header.checksum());
    debug!("{:>16}: {}", "src", header.source_address());
    debug!("{:>16}: {}", "dst", header.destination_address());
    debugdump(data);
}
