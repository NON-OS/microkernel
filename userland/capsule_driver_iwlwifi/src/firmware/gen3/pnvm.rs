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

//! The platform NVM (PNVM): per-SKU radio configuration an AX210-family GF
//! radio's firmware reads after ALIVE, when the ALIVE notification names a
//! non-zero SKU. Linux v6.12 `fw/pnvm.c`: the file is a TLV stream of SKU
//! sections; the section whose `IWL_UCODE_TLV_PNVM_SKU` matches the SKU and
//! whose `IWL_UCODE_TLV_HW_TYPE` matches this MAC and RF carries the payload
//! chunks (`IWL_UCODE_TLV_SEC_RT`). The firmware runs with fragmented PNVM
//! (`IWL_UCODE_TLV_CAPA_FRAGMENTED_PNVM_IMG`, set in every bundled AX210 image),
//! so `iwl_pcie_load_payloads_segments` places each chunk in its own DMA block
//! and a descriptor array of their addresses, and the peripheral scratch's
//! `pnvm_cfg` takes the array's address and the chunks' total size before the
//! doorbell. Parsing is pure and every offset is checked; the layout writes
//! only into the region it is given.

use alloc::vec::Vec;

use super::plan::PAGE;
use super::prph_scratch::layout::{OFF_PNVM_BASE, OFF_PNVM_SIZE};
use super::region::{put64, zero, Region};

/// `IWL_UCODE_TLV_SEC_RT`: one payload chunk, a 4-byte offset then the data.
const TLV_SEC_RT: u32 = 19;
/// `IWL_UCODE_TLV_HW_TYPE`: the MAC type and RF id a section is for.
const TLV_HW_TYPE: u32 = 58;
/// `IWL_UCODE_TLV_PNVM_VERSION`: the section's version (a SHA1 prefix).
const TLV_PNVM_VERSION: u32 = 62;
/// `IWL_UCODE_TLV_PNVM_SKU`: starts a section for one SKU id.
const TLV_PNVM_SKU: u32 = 64;
/// A deprecated separator some files put in a SEC_RT TLV.
const SEC_RT_SEPARATOR: u32 = 0xDDDD_EEEE;
/// `IPC_DRAM_MAP_ENTRY_NUM_MAX`: the descriptor array's entries.
pub const MAX_CHUNKS: usize = 64;
/// The descriptor array: one little-endian u64 address per entry.
pub const DESC_LEN: usize = MAX_CHUNKS * 8;

/// The section of a PNVM file this adapter runs.
#[derive(Debug, PartialEq, Eq)]
pub struct Pnvm<'a> {
    pub chunks: Vec<&'a [u8]>,
    pub version: u32,
}

/// One TLV of `data` at `at`: its type and value, and where the next one
/// starts (values are padded to four bytes). `Ok(None)` at the end (fewer than
/// a header's eight bytes left, which Linux's loops also stop at); `Err` when
/// the value runs past `data`, which `iwl_pnvm_parse` refuses as invalid.
type Tlv<'a> = (u32, &'a [u8], usize);
fn tlv(data: &[u8], at: usize) -> Result<Option<Tlv<'_>>, ()> {
    let Some(head) = at.checked_add(8).and_then(|end| data.get(at..end)) else {
        return Ok(None);
    };
    let kind = u32::from_le_bytes([head[0], head[1], head[2], head[3]]);
    let len = u32::from_le_bytes([head[4], head[5], head[6], head[7]]) as usize;
    let start = at + 8;
    let value = start.checked_add(len).and_then(|end| data.get(start..end)).ok_or(())?;
    let next = len.checked_add(3).and_then(|l| start.checked_add(l & !3)).ok_or(())?;
    Ok(Some((kind, value, next)))
}

fn le32(b: &[u8], at: usize) -> Option<u32> {
    let s = b.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

fn le16(b: &[u8], at: usize) -> Option<u16> {
    let s = b.get(at..at.checked_add(2)?)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}

/// The section of `file` for `sku` on a `mac`/`rf` adapter (the MAC type and
/// RF type `select` reads), as `iwl_pnvm_parse` finds it, or `None` when no
/// section matches, a match carries no chunk, or the file is malformed.
pub fn parse(file: &[u8], sku: [u32; 3], mac: u16, rf: u16) -> Option<Pnvm<'_>> {
    let mut at = 0;
    while let Some((kind, value, next)) = tlv(file, at).ok()? {
        at = next;
        if kind != TLV_PNVM_SKU {
            continue;
        }
        let ids = [le32(value, 0)?, le32(value, 4)?, le32(value, 8)?];
        if ids == sku {
            if let Some(p) = section(file, at, mac, rf) {
                return Some(p);
            }
        }
    }
    None
}

// `iwl_pnvm_handle_section`: read from `at` to the next SKU TLV (or the end).
fn section(file: &[u8], mut at: usize, mac: u16, rf: u16) -> Option<Pnvm<'_>> {
    let mut chunks = Vec::new();
    let mut version = 0;
    let mut hw_match = false;
    while let Some((kind, value, next)) = tlv(file, at).ok()? {
        match kind {
            TLV_PNVM_SKU => break,
            TLV_PNVM_VERSION => version = le32(value, 0).unwrap_or(0),
            TLV_HW_TYPE if !hw_match => {
                if let (Some(m), Some(r)) = (le16(value, 0), le16(value, 2)) {
                    hw_match = m == mac && r == rf;
                }
            }
            TLV_SEC_RT => {
                if le32(value, 0)? != SEC_RT_SEPARATOR {
                    if chunks.len() == MAX_CHUNKS {
                        return None;
                    }
                    chunks.push(value.get(4..)?);
                }
            }
            _ => {}
        }
        at = next;
    }
    (hw_match && !chunks.is_empty()).then_some(Pnvm { chunks, version })
}

/// The bytes a section needs in DMA memory: the descriptor array's page, then
/// each chunk in its own page-aligned block.
pub fn area_len(p: &Pnvm<'_>) -> usize {
    PAGE + p.chunks.iter().map(|c| round_page(c.len())).sum::<usize>()
}

/// The most any section of `file` for a `mac`/`rf` adapter needs, whatever
/// its SKU: what the control region reserves before ALIVE names the SKU.
pub fn reserve(file: &[u8], mac: u16, rf: u16) -> usize {
    let mut most = 0;
    let mut at = 0;
    while let Ok(Some((kind, _, next))) = tlv(file, at) {
        at = next;
        if kind == TLV_PNVM_SKU {
            if let Some(p) = section(file, at, mac, rf) {
                most = most.max(area_len(&p));
            }
        }
    }
    most
}

fn round_page(n: usize) -> usize {
    (n + PAGE - 1) & !(PAGE - 1)
}

/// Where a laid-out PNVM is: the descriptor array's device address and the
/// chunks' total size, the two values `pnvm_cfg` takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placed {
    pub desc: u64,
    pub size: u32,
}

/// Copy `p` into `region` from `off` (page-aligned, `area_len(p)` bytes free):
/// the descriptor array first, each chunk after it in its own page, and the
/// array filled with the chunks' device addresses. `None` if the area does not
/// fit or a write is refused.
pub fn lay_out<R: Region + ?Sized>(region: &R, off: usize, p: &Pnvm<'_>) -> Option<Placed> {
    if !off.is_multiple_of(PAGE) || off.checked_add(area_len(p))? > region.len() {
        return None;
    }
    if !zero(region, off, DESC_LEN) {
        return None;
    }
    let mut at = off + PAGE;
    let mut size: u32 = 0;
    for (i, chunk) in p.chunks.iter().enumerate() {
        if !region.write(at, chunk) || !put64(region, off + i * 8, region.dev() + at as u64) {
            return None;
        }
        size = size.checked_add(u32::try_from(chunk.len()).ok()?)?;
        at += round_page(chunk.len());
    }
    Some(Placed { desc: region.dev() + off as u64, size })
}

/// Point the peripheral scratch at `placed` (`iwl_pcie_set_pnvm_segments`).
pub fn point_scratch<R: Region + ?Sized>(region: &R, scratch_off: usize, placed: Placed) -> bool {
    put64(region, scratch_off + OFF_PNVM_BASE, placed.desc)
        && region.write(scratch_off + OFF_PNVM_SIZE, &placed.size.to_le_bytes())
}
