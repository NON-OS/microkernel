// NONOS Operating System (AGPL-3.0-or-later)
//! Each interface and its bulk pipes. A whole interface descriptor opens
//! an interface; one cut short closes the last one, so what follows it is
//! no interface's. An endpoint is taken when it is whole, bulk, sized 1 to
//! 1024 with bits 15:11 clear (USB 2.0 9.6.6, as recalled), and the first
//! of its direction; a whole companion directly after it gives its burst.

use super::least::whole;
use crate::bus::Pipes;
use crate::desc::Interface;

pub fn interfaces(descs: &[Vec<u8>]) -> Vec<Interface> {
    let (mut out, mut open) = (Vec::<Interface>::new(), false);
    let mut before: Option<bool> = None;
    for d in descs {
        let last = before.take();
        match d[1] {
            0x04 => {
                open = whole(d);
                if open {
                    let (number, alt, class, subclass, protocol) = (d[2], d[3], d[5], d[6], d[7]);
                    let pipes = Pipes::default();
                    out.push(Interface { number, alt, class, subclass, protocol, pipes });
                }
            }
            0x05 if open && whole(d) => before = take(out.last_mut().unwrap(), d),
            0x30 if whole(d) => match (last, out.last_mut()) {
                (Some(true), Some(i)) => i.pipes.burst_in = d[2] & 0x0F,
                (Some(false), Some(i)) => i.pipes.burst_out = d[2] & 0x0F,
                _ => {}
            },
            _ => {}
        }
    }
    out
}

fn take(i: &mut Interface, d: &[u8]) -> Option<bool> {
    let raw = u16::from_le_bytes([d[4], d[5]]);
    let dir_in = d[2] & 0x80 != 0;
    let free = if dir_in { i.pipes.bulk_in } else { i.pipes.bulk_out } == 0;
    let sized = raw >> 11 == 0 && (1..=1024).contains(&raw);
    if d[3] & 0x3 != 0x2 || !free || !sized {
        return None;
    }
    let p = &mut i.pipes;
    match dir_in {
        true => (p.bulk_in, p.max_packet_in) = (d[2], raw),
        false => (p.bulk_out, p.max_packet_out) = (d[2], raw),
    }
    Some(dir_in)
}
