//! 原作コード: [util.c](https://github.com/pandax381/microps/blob/master/util.c)

use std::io::Write;

pub fn hexdump<W: Write>(fp: &mut W, data: &[u8]) {
    writeln!(
        fp,
        "+------+-------------------------------------------------+------------------+"
    )
    .ok();
    for (offset, ch) in data.chunks(16).enumerate().map(|(o, c)| (o << 4, c)) {
        write!(fp, "| {:04x} | ", offset).ok();
        for index in 0..16 {
            if index < ch.len() {
                write!(fp, "{:02x} ", ch[index]).ok();
            } else {
                write!(fp, "   ").ok();
            }
        }
        write!(fp, "| ").ok();
        for index in 0..16 {
            if index < ch.len() {
                if ch[index].is_ascii_graphic() {
                    write!(fp, "{}", ch[index] as char).ok();
                } else {
                    write!(fp, ".").ok();
                }
            } else {
                write!(fp, " ").ok();
            }
        }
        writeln!(fp, " |").ok();
    }
    writeln!(
        fp,
        "+------+-------------------------------------------------+------------------+"
    )
    .ok();
}

pub fn debugdump(_data: &[u8]) {
    #[cfg(feature = "hexdump")]
    {
        hexdump(&mut std::io::stderr(), _data);
    }
}

pub(crate) mod log_macros {
    macro_rules! wrap_log_macros {
    ( $name:ident, $actual:ident, $d:tt ) => {
        #[macro_export]
        macro_rules! $name {
            ( $d lit:literal, $d ( $d args:expr ),* ) => {
                ::log::$actual!("{} ({}:{})", format!($d lit, $d( $d args ),*), file!(), line!())
            };
            ( $d lit:literal ) => {
                ::log::$actual!("{} ({}:{})", $d lit, file!(), line!())
            };
        }
    };
    ( $name:ident, $actual:ident ) => {
        wrap_log_macros!($name, $actual, $);
    };
}
    wrap_log_macros!(debug, debug);
    wrap_log_macros!(error, error);
    wrap_log_macros!(info, info);
    wrap_log_macros!(_warn, warn);
    pub use {_warn as warn, debug, error, info};
}
pub use log_macros::*;
