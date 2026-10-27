/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The download, as Linux v6.12 `mt76_connac2_load_patch` and
//! `mt76_connac_mcu_send_ram_firmware` run it: the patch under the patch
//! semaphore (skipped when the chip says it already has it), each piece
//! announced (address, length, mode) and its bytes sent, the patch
//! finished, the semaphore given back whatever happened; then each RAM
//! region the same way and the firmware started at the override address.

use super::mode::{patch_mode, ram_mode};
use super::patch::sections;
use super::piece::{piece, words};
use super::ram::{regions, FEATURE_NON_DL, FEATURE_OVERRIDE_ADDR};
use super::Step;
use crate::dma::Dma;
use crate::hw::Regs;
use crate::mcu::cmd::{FW_START_REQ, PATCH_FINISH_REQ, PATCH_SEM_CONTROL};
use crate::mcu::Mcu;

const SEM_RELEASE: u32 = 0;
const SEM_GET: u32 = 1;
const PATCH_IS_DL: u8 = 1;
const PATCH_NOT_DL_SEM_SUCCESS: u8 = 2;
const PATCH_REL_SEM_SUCCESS: u8 = 3;

pub fn patch(m: &mut Mcu, r: &Regs, d: &mut Dma, img: &[u8]) -> Result<(), Step> {
    let secs = sections(img).ok_or(Step::BadPatch)?;
    let sem = m.send(r, d, PATCH_SEM_CONTROL, &words(&[SEM_GET])[..4], true);
    match sem.map_err(|_| Step::Semaphore)?.status_byte() {
        PATCH_IS_DL => return Ok(()),
        PATCH_NOT_DL_SEM_SUCCESS => {}
        _ => return Err(Step::Semaphore),
    }
    let sent = secs.iter().try_for_each(|s| {
        piece(m, r, d, (s.addr, patch_mode(s.info)), &img[s.offs..s.offs + s.len])
    });
    let finished = sent.and_then(|()| {
        let ok = m.send(r, d, PATCH_FINISH_REQ, &[0; 4], true).map(|rep| rep.status_byte() == 0);
        ok.ok().filter(|&o| o).map(|_| ()).ok_or(Step::Finish)
    });
    let rel = m.send(r, d, PATCH_SEM_CONTROL, &words(&[SEM_RELEASE])[..4], true);
    let released = rel.is_ok_and(|rep| rep.status_byte() == PATCH_REL_SEM_SUCCESS);
    finished.and_then(|()| released.then_some(()).ok_or(Step::Semaphore))
}

pub fn ram(m: &mut Mcu, r: &Regs, d: &mut Dma, img: &[u8]) -> Result<(), Step> {
    let mut start = None;
    for g in regions(img).ok_or(Step::BadRam)? {
        if g.feature & FEATURE_NON_DL != 0 {
            continue;
        }
        if g.feature & FEATURE_OVERRIDE_ADDR != 0 {
            start = Some(g.addr);
        }
        piece(m, r, d, (g.addr, ram_mode(g.feature)), &img[g.offs..g.offs + g.len])?;
    }
    let (option, addr) = start.map_or((0, 0), |a| (1, a)); // FW_START_OVERRIDE
    let req = words(&[option, addr]);
    m.send(r, d, FW_START_REQ, &req[..8], true).map(|_| ()).map_err(|_| Step::Start)
}
