// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Any bytes as a UFS host and the device behind it: every register read
//! answers with the next bytes, and every doorbell lands the next bytes in
//! the response UPIU and the data region, as a hostile device would. The
//! bring-up never panics and never hangs (a hang is a libFuzzer timeout),
//! and a unit it serves is a real disk: 512 or 4096-byte blocks, a size in
//! bytes that fits 64 bits, a single-level LUN named once, a serial no
//! longer than its field, and a transfer that fits the data region. The
//! REPORT LUNS, READ CAPACITY, sense and response readers hold the same
//! bounds on any bytes alone.

#![no_main]

use std::cell::Cell;

use libfuzzer_sys::fuzz_target;
use ufs_proofs::ufs::bring_up::bring_up;
use ufs_proofs::ufs::disk::rw::{read, write};
use ufs_proofs::ufs::env::{Clock, DmaBuf, Log, Mmio};
use ufs_proofs::ufs::facts::hci::UTRLDBR;
use ufs_proofs::ufs::facts::upiu::AT_SENSE;
use ufs_proofs::ufs::scsi::codes::WLUN_BIT;
use ufs_proofs::ufs::scsi::luns::{self, MAX_UNITS};
use ufs_proofs::ufs::scsi::{capacity, sense};
use ufs_proofs::ufs::slot::layout::{Lists, RSP_AT, RSP_READ};
use ufs_proofs::ufs::upiu::response::Rsp;

/// One region a bring-up is handed, owned here so the driver's writes land
/// in live memory. The bus address is only a number the device is told.
struct Region(Box<[u8]>);

impl Region {
    fn buf(&mut self, bus: u64) -> DmaBuf {
        DmaBuf { va: self.0.as_mut_ptr() as u64, bus, len: self.0.len() }
    }
}

/// The host and device: the input, read front to back, and a clock that
/// moves on every look so every wait ends.
struct Hostile<'a> {
    src: &'a [u8],
    at: Cell<usize>,
    now: Cell<u64>,
    lists: Lists,
}

impl Hostile<'_> {
    fn take(&self, n: usize) -> &[u8] {
        let at = self.at.get().min(self.src.len());
        let end = at.saturating_add(n).min(self.src.len());
        self.at.set(end);
        &self.src[at..end]
    }

    fn byte(&self) -> u8 {
        self.take(1).first().copied().unwrap_or(0)
    }

    /// A doorbell: the device writes a response and some data.
    fn complete(&self) {
        let rsp = self.take(RSP_READ);
        self.lists.ucd.put(RSP_AT, rsp);
        let n = (self.byte() as usize * 16).min(self.lists.data.len);
        let data = self.take(n);
        self.lists.data.put(0, data);
    }
}

impl Mmio for Hostile<'_> {
    fn r32(&self, _off: u32) -> u32 {
        let mut w = [0u8; 4];
        let b = self.take(4);
        w[..b.len()].copy_from_slice(b);
        u32::from_le_bytes(w)
    }

    fn w32(&self, off: u32, v: u32) {
        if off == UTRLDBR && v & 1 != 0 {
            self.complete();
        }
    }
}

impl Clock for Hostile<'_> {
    fn now_ms(&self) -> u64 {
        let t = self.now.get().saturating_add(1);
        self.now.set(t);
        t
    }
}

impl Log for Hostile<'_> {
    fn line(&self, _text: &[u8]) {}
}

fn readers(data: &[u8]) {
    let (found, n) = luns::parse(data);
    assert!(n <= MAX_UNITS);
    assert!(found[..n].windows(2).all(|w| w[0] < w[1]), "LUNs not ascending and once each");
    assert!(found[..n].iter().all(|&l| l & WLUN_BIT == 0), "a well-known LU served");
    let mut b = [0u8; 32];
    let k = data.len().min(b.len());
    b[..k].copy_from_slice(&data[..k]);
    if let Some((blocks, _)) = capacity::parse_10(&b) {
        assert!(blocks >= 1 && blocks <= u32::MAX as u64);
    }
    let (blocks, _) = capacity::parse_16(&b);
    assert!(blocks >= 1);
    let _ = sense::parse(data);
    let mut r = [0u8; RSP_READ];
    let k = data.len().min(r.len());
    r[..k].copy_from_slice(&data[..k]);
    assert!(Rsp(&r).sense().len() <= RSP_READ - AT_SENSE);
}

fuzz_target!(|data: &[u8]| {
    readers(data);
    let Some((&flags, src)) = data.split_first() else { return };
    let mut regions = [4096, 4096, 4096, 64 * 1024].map(|n| Region(vec![0u8; n].into_boxed_slice()));
    let [utrl, utmrl, ucd, region] = &mut regions;
    let lists = Lists {
        utrl: utrl.buf(0x10_0000),
        utmrl: utmrl.buf(0x20_0000),
        ucd: ucd.buf(0x30_0000),
        data: region.buf(0x40_0000),
    };
    let hw = Hostile { src, at: Cell::new(0), now: Cell::new(0), lists };
    let Ok(ufs) = bring_up(&hw, lists, flags & 1 != 0) else { return };
    let units: Vec<_> = ufs.units.iter().flatten().copied().collect();
    assert!(!units.is_empty(), "a bring-up that serves no unit succeeded");
    assert!(ufs.units[units.len()..].iter().all(Option::is_none), "units not packed");
    for (i, u) in units.iter().enumerate() {
        assert!(u.block_len == 512 || u.block_len == 4096, "block of {}", u.block_len);
        assert!(u.blocks.checked_mul(u.block_len as u64).is_some(), "size overflows");
        assert!(u.lun & WLUN_BIT == 0 && !units[..i].iter().any(|o| o.lun == u.lun));
        assert!(u.serial_len <= u.serial.len());
        assert!(ufs.max_blocks(u) as usize * u.block_len as usize <= ufs.max_bytes());
    }
    let u = units[0];
    let mut buf = vec![0u8; ufs.max_bytes()];
    let _ = read(&hw, &ufs, &u, 0, 1, &mut buf);
    let _ = write(&hw, &ufs, &u, 0, 1, &buf);
});
