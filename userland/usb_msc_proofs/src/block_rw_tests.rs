// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's own sector reads and writes (`server/handlers/block_rw.rs`,
//! mounted), over BOT, against a disk of 512, 1024, 2048 and 4096-byte
//! blocks. These used to run against a copy of the two functions.

use crate::block_rw::{read_write, Rw};
use crate::descriptors::MscBinding;
use crate::disk::Disk;
use crate::protocol::{E_INVAL, E_MSGSIZE, E_NOTSUP, E_NXIO};
use crate::state::State;
use crate::xhci::{attach, device};

fn disk(len: u32, blocks: u64) -> Disk {
    let b = MscBinding { interface: 0, bulk_in: 0x81, bulk_out: 0x02, ..MscBinding::default() };
    let mut d = Disk::bound(1, 1, &b);
    (d.blocks, d.block_len) = (blocks, len);
    d
}

fn header(lba: u64, count: u32) -> Vec<u8> {
    let mut h = lba.to_le_bytes().to_vec();
    h.extend_from_slice(&count.to_le_bytes());
    h
}

fn read(d: &Disk, s: &mut State, lba: u64, count: u32) -> Result<Vec<u8>, i32> {
    let mut out = vec![0u8; 64 * 512];
    let n = read_write(s, d, Rw::Read(&mut out), &header(lba, count))?;
    Ok(out[..n].to_vec())
}

fn write(d: &Disk, s: &mut State, lba: u64, data: &[u8]) -> Result<usize, i32> {
    let mut body = header(lba, (data.len() / 512) as u32);
    body.extend_from_slice(data);
    read_write(s, d, Rw::Write, &body)
}

fn stored<T>(f: impl FnOnce(&crate::store::Store) -> T) -> T {
    device(|dev| f(dev.store.as_ref().expect("attached")))
}

#[test]
fn sectors_read_and_written_on_any_block_size_are_the_bytes_a_512_device_holds() {
    for len in [512usize, 1024, 2048, 4096] {
        let blocks = 40 * 4096 / len;
        attach(len, blocks);
        let (d, mut s) = (disk(len as u32, blocks as u64), State::new());
        let mut model = stored(|st| st.bytes.clone());
        let mut seed = 0x9E37_79B9_7F4A_7C15u64;
        for _ in 0..200 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let count = (seed % 64) as u32 + 1;
            let lba = (seed >> 8) % (model.len() as u64 / 512 - count as u64);
            let (at, n) = (lba as usize * 512, count as usize * 512);
            assert_eq!(
                read(&d, &mut s, lba, count).unwrap(),
                model[at..at + n],
                "read {lba}+{count}"
            );
            let data: Vec<u8> = (0..n).map(|i| (seed as usize + i * 13) as u8).collect();
            assert_eq!(write(&d, &mut s, lba, &data), Ok(n));
            model[at..at + n].copy_from_slice(&data);
            assert!(stored(|st| st.bytes == model), "write {lba}+{count} on {len}-byte blocks");
        }
    }
}

#[test]
fn an_aligned_write_is_not_read_first_and_a_part_block_is() {
    attach(4096, 16);
    let (d, mut s) = (disk(4096, 16), State::new());
    assert_eq!(write(&d, &mut s, 8, &[0xAB; 8 * 512]), Ok(8 * 512));
    assert_eq!(stored(|st| (st.reads, st.writes)), (0, 1));
    assert_eq!(write(&d, &mut s, 9, &[0xCD; 512]), Ok(512));
    assert_eq!(stored(|st| (st.reads, st.writes)), (1, 2), "read, changed, written");
}

#[test]
fn a_request_the_disk_cannot_hold_is_refused_before_the_bus() {
    attach(512, 100);
    let (d, mut s) = (disk(512, 100), State::new());
    assert_eq!(read(&d, &mut s, 99, 2), Err(E_NXIO), "past the end");
    assert_eq!(read(&d, &mut s, u64::MAX, 1), Err(E_NXIO), "an end that overflows");
    assert_eq!(read(&d, &mut s, 0, 0), Err(E_INVAL));
    assert_eq!(read(&d, &mut s, 0, 65), Err(E_MSGSIZE));
    let mut body = header(0, 2);
    body.extend_from_slice(&[0; 512]);
    assert_eq!(read_write(&mut s, &d, Rw::Write, &body), Err(E_INVAL), "data short of the count");
    assert_eq!(read(&disk(3072, 100), &mut s, 0, 1), Err(E_NOTSUP));
    assert_eq!(read_write(&mut s, &d, Rw::Write, &[0; 11]), Err(E_INVAL), "a short header");
    assert_eq!(stored(|st| (st.reads, st.writes)), (0, 0));
}
