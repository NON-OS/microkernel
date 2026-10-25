// NONOS Operating System (AGPL-3.0-or-later)
//! ITS commands against IHI 0069 5.3: opcode, then each field in place.

use crate::gic::its::encode::cmd::{discard, inv, mapd, mapti, unmapd};
use crate::gic::its::encode::route::{mapc, sync, target};
use crate::gic::its::encode::typer::{decode, ItsTyper};

const V: u64 = 1 << 63;

#[test]
fn device_commands_put_the_device_id_in_the_top_of_the_first_word() {
    let itt = 0x0004_4000_1234_5600;
    assert_eq!(mapd(0x12, itt, 5), [0x08 | 0x12 << 32, 4, V | itt, 0]);
    assert_eq!(mapd(0x12, itt | 0xff, 5)[2], V | itt, "an ITT is 256-byte aligned");
    assert_eq!(mapd(0, itt | 1 << 52, 1)[2], V | itt, "past 52 bits is not an address");
    assert_eq!(mapd(0xffff_ffff, itt, 0)[..2], [0x08 | 0xffff_ffff << 32, 0]);
    assert_eq!(unmapd(7), [0x08 | 7 << 32, 0, 0, 0], "valid clear");
    assert_eq!(mapti(7, 3, 8195, 0), [0x0a | 7 << 32, 3 | 8195 << 32, 0, 0]);
    assert_eq!(mapti(1, 31, 9215, 0xffff)[2], 0xffff);
    assert_eq!(inv(0x10, 2), [0x0c | 0x10 << 32, 2, 0, 0]);
    assert_eq!(discard(0x10, 2), [0x0f | 0x10 << 32, 2, 0, 0]);
}

#[test]
fn a_redistributor_is_named_by_address_or_by_processor_number() {
    assert_eq!(target(true, 0x080a_1234, 9), 0x080a_0000);
    assert_eq!(target(true, 0x0010_0000_0000_0000, 0), 0, "past 52 bits");
    assert_eq!(target(false, 0x080a_0000, 3), 3 << 16);
    assert_eq!(target(false, 0, 0xffff), 0xffff << 16);
    let t = target(true, 0x080a_0000, 0);
    assert_eq!(mapc(0, t), [0x09, 0, V | 0x080a_0000, 0]);
    assert_eq!(mapc(5, 2 << 16)[2], V | 2 << 16 | 5);
    assert_eq!(sync(t), [0x05, 0, 0x080a_0000, 0]);
}

#[test]
fn typer_fields_decode_one_more_than_they_read() {
    let typer = 1 | 7 << 4 | 15 << 8 | 15 << 13 | 1 << 19;
    let want = ItsTyper {
        physical: true,
        ite_bytes: 8,
        event_bits: 16,
        device_bits: 16,
        pta: true,
        hcc: 0,
    };
    assert_eq!(decode(typer), want);
    let other = decode(0xf << 4 | 31 << 8 | 31 << 13 | 0xff << 24);
    assert_eq!((other.physical, other.ite_bytes, other.event_bits), (false, 16, 32));
    assert_eq!((other.device_bits, other.pta, other.hcc), (32, false, 255));
}
