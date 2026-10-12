// NONOS Operating System (AGPL-3.0-or-later)
//! Hardening proofs: no input, however malformed, may make a parser panic,
//! read out of bounds or hang; each runs over hundreds of thousands of
//! pseudo-random inputs. The AP controls every beacon byte, the firmware every
//! DMA byte and the RX write pointer. The CCMP and EAPOL ones now run against
//! the core, in `nonos_wifi_core_proofs`.

use crate::constants::{RB_SIZE, RX_QUEUE_SIZE, RX_RB_OFFSET};
use crate::dot11::parse::{find_ie, parse_beacon};
use crate::rx::packet::parse as rx_parse;
use crate::rx::recv::receive;

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn fuzz_buf(s: &mut u64, max_len: usize) -> Vec<u8> {
    let len = (xorshift(s) as usize) % (max_len + 1);
    let mut b = vec![0u8; len];
    for x in b.iter_mut() {
        *x = xorshift(s) as u8;
    }
    b
}

#[test]
fn beacon_parser_never_panics_on_arbitrary_frames() {
    let mut s = 0x1234_5678_9abc_def0;
    for _ in 0..400_000 {
        let buf = fuzz_buf(&mut s, 80);
        let _ = parse_beacon(&buf);
    }
}

#[test]
fn find_ie_terminates_and_never_panics() {
    let mut s = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..400_000 {
        let tags = fuzz_buf(&mut s, 64);
        let id = xorshift(&mut s) as u8;
        let _ = find_ie(&tags, id);
    }
}

#[test]
fn rx_packet_parser_never_panics_on_hostile_dma() {
    let mut s = 0xDEAD_BEEF_CAFE_BABE;
    for _ in 0..400_000 {
        let buf = fuzz_buf(&mut s, 64);
        let _ = rx_parse(&buf);
    }
}

#[test]
fn receive_never_panics_on_arbitrary_ring_state_and_reads_in_bounds() {
    // A garbage DMA buffer plus arbitrary read and firmware-write indices, as a
    // hostile or buggy firmware could present, must never fault the receive
    // path. The buffer is exactly the layout size, so an in-bounds read is also
    // a check that the masked index never runs past it.
    let mut dma = vec![0u8; RX_RB_OFFSET + RX_QUEUE_SIZE * RB_SIZE];
    let mut s = 0x0BAD_F00D_1357_2468;
    for x in dma.iter_mut() {
        *x = xorshift(&mut s) as u8;
    }
    let dma_va = dma.as_ptr() as u64;
    let mut out = [0u8; 256];
    for _ in 0..300_000 {
        let read = xorshift(&mut s) as usize;
        let write = xorshift(&mut s) as usize;
        let _ = unsafe { receive(dma_va, RX_RB_OFFSET, read, write, &mut out) };
    }
}
