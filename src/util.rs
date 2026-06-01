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
