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

//! Any bytes as an SDHCI host and the card behind it: every register read
//! answers with the next bytes, and every command lands the next bytes in
//! the data region, as a hostile controller or card would. The bring-up
//! never panics and never hangs (a hang is a libFuzzer timeout), and a disk
//! it serves has sectors, a bus width the host can drive, and a transfer
//! that fits the data region. The CSD, CID, OCR and EXT_CSD readers, the
//! READ and WRITE head check and the kernel's host lists hold their bounds
//! on any bytes alone.

#![no_main]

use std::cell::Cell;

use emmc_proofs::emmc::disk::{EmmcDisk, DATA_BUF_BYTES, SECTOR_SIZE};
use emmc_proofs::emmc::env::{Clock, DmaBuf, Log, Mmio};
use emmc_proofs::emmc::mmc::ext_csd::{ExtCsd, CMD6_MAX_MS, CMD6_MIN_MS, EXT_CSD_LEN};
use emmc_proofs::emmc::mmc::regs::{ocr_ready, ocr_sector_mode, r2, Cid, Csd};
use emmc_proofs::emmc::sd::csd::{csd_agrees, sd_sectors};
use emmc_proofs::emmc::sd::regs::{ccs, sd_cid};
use emmc_proofs::emmc::sdhci::regs::COMMAND;
use emmc_proofs::emmc::sdhci::Host;
use emmc_proofs::kernel_inventory::{is_intel_emmc, INTEL_EMMC_DEVICE_IDS};
use emmc_proofs::kernel_sd_host::is_sd_host;
use emmc_proofs::protocol::RW_HEADER_LEN;
use emmc_proofs::rw_parse::parse;
use libfuzzer_sys::fuzz_target;

/// The input, read front to back, shared by the host's registers and the
/// card's data.
struct Feed<'a> {
    src: &'a [u8],
    at: Cell<usize>,
    now: Cell<u64>,
}

impl Feed<'_> {
    fn take(&self, n: usize) -> &[u8] {
        let at = self.at.get().min(self.src.len());
        let end = at.saturating_add(n).min(self.src.len());
        self.at.set(end);
        &self.src[at..end]
    }

    fn word(&self) -> u32 {
        let mut w = [0u8; 4];
        let b = self.take(4);
        w[..b.len()].copy_from_slice(b);
        u32::from_le_bytes(w)
    }
}

/// The register window: reads from the feed; a command lands data.
struct Bus<'a> {
    feed: &'a Feed<'a>,
    data: DmaBuf,
}

impl Mmio for Bus<'_> {
    fn r8(&self, _off: u32) -> u8 {
        self.feed.word() as u8
    }

    fn r16(&self, _off: u32) -> u16 {
        self.feed.word() as u16
    }

    fn r32(&self, _off: u32) -> u32 {
        self.feed.word()
    }

    fn w8(&self, _off: u32, _v: u8) {}

    fn w16(&self, off: u32, _v: u16) {
        if off == COMMAND {
            let n = (self.feed.word() as usize % (EXT_CSD_LEN + 1)).min(self.data.len);
            self.data.put(0, self.feed.take(n));
        }
    }

    fn w32(&self, _off: u32, _v: u32) {}
}

/// A clock that moves on every look, so every wait ends.
struct Ticks<'a>(&'a Feed<'a>);

impl Clock for Ticks<'_> {
    fn now_ms(&self) -> u64 {
        let t = self.0.now.get().saturating_add(1);
        self.0.now.set(t);
        t
    }
}

struct Quiet;

impl Log for Quiet {
    fn line(&self, _text: &[u8]) {}
}

fn u128_at(data: &[u8], at: usize) -> u128 {
    let mut b = [0u8; 16];
    let src = data.get(at..).unwrap_or(&[]);
    let k = src.len().min(16);
    b[..k].copy_from_slice(&src[..k]);
    u128::from_le_bytes(b)
}

fn registers(data: &[u8]) {
    let reg = u128_at(data, 0);
    let _ = Cid::parse(reg);
    let _ = sd_cid(reg);
    let _ = Csd::parse(reg).sectors();
    let _ = (sd_sectors(reg), csd_agrees(reg, true), csd_agrees(reg, false));
    let words = [0, 4, 8, 12].map(|i| u128_at(data, i) as u32);
    let _ = r2(words);
    let ocr = words[0];
    let _ = (ocr_ready(ocr), ocr_sector_mode(ocr), ccs(ocr));
    let mut raw = [0u8; EXT_CSD_LEN];
    let k = data.len().min(EXT_CSD_LEN);
    raw[..k].copy_from_slice(&data[..k]);
    let e = ExtCsd::new(raw);
    for index in 0..=u8::MAX {
        let ms = e.switch_ms(index);
        assert!((CMD6_MIN_MS..=CMD6_MAX_MS).contains(&ms), "a SWITCH waits {ms} ms");
    }
    let _ = (e.boot_sectors(), e.rpmb_sectors(), e.cache_kib(), e.cache_on(), e.hs_hz());
    let _ = (e.rev(), e.device_type(), e.sec_count(), e.partition_access());
    assert!(e.bus_test_same(&e));
}

fn head(data: &[u8]) {
    let capacity = u128_at(data, RW_HEADER_LEN) as u64;
    let max = (u128_at(data, RW_HEADER_LEN + 8) as u32) % 1024;
    if let Ok((lba, n)) = parse(data, capacity, max) {
        assert!(n >= 1 && n <= max, "a run of {n} past the most of {max}");
        assert!(lba.checked_add(n as u64).is_some_and(|end| end <= capacity));
    }
}

fn lists(data: &[u8]) {
    let w = u128_at(data, 0);
    let (vendor, device) = (w as u16, (w >> 16) as u16);
    let (class, subclass, progif) = ((w >> 32) as u8, (w >> 40) as u8, (w >> 48) as u8);
    if is_intel_emmc(vendor, device, class, subclass) {
        assert!(vendor == 0x8086 && class == 0x08 && subclass == 0x05);
        assert!(INTEL_EMMC_DEVICE_IDS.contains(&device));
    }
    let _ = is_sd_host(vendor, device, class, subclass, progif);
}

fuzz_target!(|data: &[u8]| {
    registers(data);
    head(data);
    lists(data);
    let Some((&flags, src)) = data.split_first() else { return };
    let mut table_mem = vec![0u8; 4096].into_boxed_slice();
    let mut data_mem = vec![0u8; DATA_BUF_BYTES].into_boxed_slice();
    let table = DmaBuf { va: table_mem.as_mut_ptr() as u64, bus: 0x10_0000, len: table_mem.len() };
    let region = DmaBuf { va: data_mem.as_mut_ptr() as u64, bus: 0x20_0000, len: data_mem.len() };
    let feed = Feed { src, at: Cell::new(0), now: Cell::new(0) };
    let bus = Bus { feed: &feed, data: region };
    let host = Host::new(bus, Ticks(&feed), Quiet, table, flags & 1 != 0);
    let Ok(host) = EmmcDisk::reset(host) else { return };
    let Ok(mut disk) = EmmcDisk::bring_up(host, region) else { return };
    assert!(disk.capacity_sectors() > 0, "a disk of no sectors is served");
    assert!(matches!(disk.card.width, 1 | 4 | 8), "a bus {} bits wide", disk.card.width);
    assert!(disk.max_sectors() as usize * SECTOR_SIZE <= disk.data.len);
    let mut buf = vec![0u8; SECTOR_SIZE];
    let _ = disk.read(0, 1, &mut buf);
    let _ = disk.write(0, 1, &buf);
    let _ = disk.flush();
    disk.quiesce();
    drop((table_mem, data_mem));
});
