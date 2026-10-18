// NONOS Operating System (AGPL-3.0-or-later)
use crate::descriptors::parse_config;
use crate::scsi::{
    parse_capacity16, read10, read16, read_capacity16, write16, CAPACITY16_DATA_LEN,
};
use crate::span::{needs_cdb16, sectors, sectors_per_block, span, SECTOR_BYTES};

extern crate alloc;
use alloc::vec::Vec;

// A USB device with 1, 2 or 4 KiB logical blocks is served to the kernel in
// 512-byte sectors. These proofs hold the span a sector request becomes to
// the device blocks that cover it exactly, and run the same read and
// read-modify-write the driver does against a device kept in memory.

#[test]
fn only_the_block_lengths_a_sector_divides_are_served() {
    assert_eq!(sectors_per_block(512), Some(1));
    assert_eq!(sectors_per_block(1024), Some(2));
    assert_eq!(sectors_per_block(2048), Some(4));
    assert_eq!(sectors_per_block(4096), Some(8));
    for odd in [0, 256, 520, 528, 3072, 8192, 65536, u32::MAX] {
        assert_eq!(sectors_per_block(odd), None, "{odd}");
    }
}

#[test]
fn a_span_covers_the_request_exactly_and_no_more() {
    for per in [1u32, 2, 4, 8] {
        for lba in 0u64..80 {
            for count in 1u32..=64 {
                let sp = span(lba, count, per).expect("span");
                let (p, end) = (per as u64, lba + count as u64);
                let from = sp.first * p;
                let to = (sp.first + sp.blocks as u64) * p;
                assert!(from <= lba && end <= to, "covers {lba}+{count} per {per}");
                assert!(lba - from < p && to - end < p, "no whole spare block");
                assert_eq!(sp.head, ((lba - from) * SECTOR_BYTES as u64) as usize);
                assert_eq!(sp.aligned, from == lba && to == end);
            }
        }
    }
}

#[test]
fn an_empty_or_overflowing_request_has_no_span() {
    assert_eq!(span(0, 0, 8), None);
    assert_eq!(span(u64::MAX - 3, 8, 8), None);
    assert_eq!(span(0, 1, 0), None);
}

#[test]
fn a_device_holds_its_blocks_in_sectors() {
    assert_eq!(sectors(7_812_500, 8), 62_500_000);
    assert_eq!(sectors(u64::MAX, 8), u64::MAX, "saturates");
}

/// A device of `blocks` blocks of `len` bytes, in memory.
struct Device {
    len: usize,
    bytes: Vec<u8>,
    reads: usize,
    writes: usize,
}

impl Device {
    fn new(len: usize, blocks: usize) -> Self {
        let bytes = (0..len * blocks).map(|i| (i * 7 + i / 512) as u8).collect();
        Self { len, bytes, reads: 0, writes: 0 }
    }
    fn read(&mut self, first: u64, out: &mut [u8]) {
        assert_eq!(out.len() % self.len, 0, "whole blocks only");
        let at = first as usize * self.len;
        out.copy_from_slice(&self.bytes[at..at + out.len()]);
        self.reads += 1;
    }
    fn write(&mut self, first: u64, data: &[u8]) {
        assert_eq!(data.len() % self.len, 0, "whole blocks only");
        let at = first as usize * self.len;
        self.bytes[at..at + data.len()].copy_from_slice(data);
        self.writes += 1;
    }
}

// The driver's read_sectors and write_sectors (server/handlers/block_rw.rs),
// step for step, against the in-memory device.
fn read_sectors(dev: &mut Device, lba: u64, count: u32) -> Vec<u8> {
    let per = sectors_per_block(dev.len as u32).unwrap();
    let sp = span(lba, count, per).unwrap();
    let mut out = vec![0u8; count as usize * 512];
    if sp.aligned {
        dev.read(sp.first, &mut out);
    } else {
        let mut blocks = vec![0u8; sp.blocks as usize * dev.len];
        dev.read(sp.first, &mut blocks);
        let n = out.len();
        out.copy_from_slice(&blocks[sp.head..sp.head + n]);
    }
    out
}

fn write_sectors(dev: &mut Device, lba: u64, data: &[u8]) {
    let per = sectors_per_block(dev.len as u32).unwrap();
    let sp = span(lba, (data.len() / 512) as u32, per).unwrap();
    if sp.aligned {
        return dev.write(sp.first, data);
    }
    let mut blocks = vec![0u8; sp.blocks as usize * dev.len];
    dev.read(sp.first, &mut blocks);
    blocks[sp.head..sp.head + data.len()].copy_from_slice(data);
    dev.write(sp.first, &blocks);
}

#[test]
fn sectors_read_and_written_on_a_4k_device_are_the_bytes_a_512_device_would_hold() {
    for len in [512usize, 1024, 2048, 4096] {
        let blocks = 40 * 4096 / len;
        let mut dev = Device::new(len, blocks);
        let mut model = dev.bytes.clone();
        let mut seed = 0x9E37_79B9_7F4A_7C15u64;
        for _ in 0..400 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let count = (seed % 64) as u32 + 1;
            let lba = (seed >> 8) % (model.len() as u64 / 512 - count as u64);
            let at = lba as usize * 512;
            let n = count as usize * 512;
            assert_eq!(read_sectors(&mut dev, lba, count), model[at..at + n], "read {lba}+{count}");
            let data: Vec<u8> = (0..n).map(|i| (seed as usize + i * 13) as u8).collect();
            write_sectors(&mut dev, lba, &data);
            model[at..at + n].copy_from_slice(&data);
            assert_eq!(dev.bytes, model, "write {lba}+{count} on {len}-byte blocks");
        }
    }
}

#[test]
fn an_aligned_write_is_not_read_first() {
    let mut dev = Device::new(4096, 16);
    write_sectors(&mut dev, 8, &[0xAB; 8 * 512]);
    assert_eq!((dev.reads, dev.writes), (0, 1));
    write_sectors(&mut dev, 9, &[0xCD; 512]);
    assert_eq!((dev.reads, dev.writes), (1, 2), "a part block is read, changed, written");
}

#[test]
fn the_ten_byte_forms_are_kept_while_they_reach() {
    assert!(!needs_cdb16(0, 64));
    assert!(!needs_cdb16((1 << 32) - 64, 64), "ends at the last 32-bit LBA");
    assert!(needs_cdb16((1 << 32) - 63, 64));
    assert!(needs_cdb16(1 << 40, 1));
    assert!(needs_cdb16(0, 0x1_0000), "count past 16 bits");
}

#[test]
fn the_sixteen_byte_commands_are_encoded_as_sbc_says() {
    let (cdb, len) = read16(0x0102_0304_0506_0708, 0x0A0B_0C0D);
    assert_eq!(len, 16);
    assert_eq!(cdb[0], 0x88);
    assert_eq!(cdb[2..10], [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(cdb[10..14], [0x0A, 0x0B, 0x0C, 0x0D]);
    assert_eq!(write16(5, 9).0[0], 0x8A);
    let (cdb, len) = read_capacity16();
    assert_eq!((cdb[0], cdb[1], len), (0x9E, 0x10, 16));
    assert_eq!(u32::from_be_bytes(cdb[10..14].try_into().unwrap()) as usize, CAPACITY16_DATA_LEN);
    assert_eq!(read10(7, 3).0[0], 0x28, "the ten-byte read is unchanged");
}

#[test]
fn read_capacity16_gives_the_count_and_the_length() {
    let mut raw = [0u8; 32];
    raw[0..8].copy_from_slice(&0x1_D1C0_BEAFu64.to_be_bytes());
    raw[8..12].copy_from_slice(&4096u32.to_be_bytes());
    assert_eq!(parse_capacity16(&raw), Some((0x1_D1C0_BEB0, 4096)));
    assert_eq!(parse_capacity16(&raw[..12]), Some((0x1_D1C0_BEB0, 4096)), "12 bytes suffice");
    assert_eq!(parse_capacity16(&raw[..11]), None);
    raw[0..8].copy_from_slice(&[0xFF; 8]);
    assert_eq!(parse_capacity16(&raw), None, "a count that does not fit");
}

fn ss_config(bursts: (u8, u8), companion_first: bool) -> Vec<u8> {
    let mut raw = Vec::new();
    raw.extend_from_slice(&[9, 0x02, 0, 0, 1, 1, 0, 0x80, 50]);
    raw.extend_from_slice(&[9, 0x04, 0, 0, 2, 0x08, 0x06, 0x50, 0]);
    if companion_first {
        raw.extend_from_slice(&[6, 0x30, 9, 0, 0, 0]);
    }
    raw.extend_from_slice(&[7, 0x05, 0x81, 0x02, 0x00, 0x04, 0]);
    raw.extend_from_slice(&[6, 0x30, bursts.0, 0, 0, 0]);
    raw.extend_from_slice(&[7, 0x05, 0x02, 0x02, 0x00, 0x04, 0]);
    raw.extend_from_slice(&[6, 0x30, bursts.1, 0, 0, 0]);
    let total = raw.len() as u16;
    raw[2..4].copy_from_slice(&total.to_le_bytes());
    raw
}

#[test]
fn each_pipe_takes_the_burst_of_the_companion_after_it() {
    let b = parse_config(&ss_config((15, 3), false)).expect("parses").bindings[0];
    assert_eq!((b.max_burst_in, b.max_burst_out), (15, 3));
    assert_eq!((b.max_packet_in, b.max_packet_out), (1024, 1024));
}

#[test]
fn a_burst_past_fifteen_or_a_companion_with_no_endpoint_is_ignored() {
    let b = parse_config(&ss_config((16, 0xFF), true)).expect("parses").bindings[0];
    assert_eq!((b.max_burst_in, b.max_burst_out), (0, 0));
}

#[test]
fn a_high_speed_device_has_no_burst() {
    let mut raw = Vec::new();
    raw.extend_from_slice(&[9, 0x02, 0, 0, 1, 1, 0, 0x80, 50]);
    raw.extend_from_slice(&[9, 0x04, 0, 0, 2, 0x08, 0x06, 0x50, 0]);
    raw.extend_from_slice(&[7, 0x05, 0x81, 0x02, 0x00, 0x02, 0]);
    raw.extend_from_slice(&[7, 0x05, 0x02, 0x02, 0x00, 0x02, 0]);
    let total = raw.len() as u16;
    raw[2..4].copy_from_slice(&total.to_le_bytes());
    let b = parse_config(&raw).expect("parses").bindings[0];
    assert_eq!((b.max_burst_in, b.max_burst_out), (0, 0));
}
