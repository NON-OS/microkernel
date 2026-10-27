/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Sending: a command framed onto ring 17 and, when asked, its reply
//! waited for (three seconds, `mt7921_mcu_send_message`); firmware bytes
//! in chunks of at most 4096 onto ring 16, bare
//! (`__mt76_mcu_send_firmware`). Linux fails a chunk at once when the ring
//! is full and cleans the ring after each; this waits up to a second for
//! the chip to free a slot before calling the ring stuck.

use alloc::vec::Vec;

use super::cmd::Cmd;
use super::frame::frame;
use super::reply::Reply;
use super::{Mcu, McuError};
use crate::clock::until_ms;
use crate::dma::layout::{CMD_MAX, FW_CHUNK};
use crate::dma::Dma;
use crate::hw::Regs;

const REPLY_MS: u64 = 3000;
const ROOM_MS: u64 = 1000;

impl Mcu {
    /// Send `cmd` with `body`; with `wait`, its reply, else an empty one
    /// once the command is on the ring.
    pub fn send(
        &mut self,
        regs: &Regs,
        dma: &mut Dma,
        cmd: Cmd,
        body: &[u8],
        wait: bool,
    ) -> Result<Reply, McuError> {
        let seq = self.next_seq();
        let mut buf = [0u8; CMD_MAX];
        let n = frame(cmd, seq, body, &mut buf).ok_or(McuError::TooLong)?;
        if !dma.tx_wm.wait_room(regs, ROOM_MS) {
            return Err(McuError::NoRoom);
        }
        dma.tx_wm.push(&buf[..n]).ok_or(McuError::TooLong)?;
        dma.tx_wm.kick(regs);
        if !wait {
            return Ok(Reply { event: Vec::new() });
        }
        let mut got = None;
        until_ms(REPLY_MS, || {
            got = self.pump_for(regs, dma, Some((seq, cmd)));
            got.is_some()
        });
        got.unwrap_or(Err(McuError::Timeout))
    }

    /// `data` onto the download ring, chunk by chunk.
    pub fn scatter(&mut self, regs: &Regs, dma: &mut Dma, data: &[u8]) -> Result<(), McuError> {
        for chunk in data.chunks(FW_CHUNK) {
            if !dma.tx_fwdl.wait_room(regs, ROOM_MS) {
                return Err(McuError::NoRoom);
            }
            dma.tx_fwdl.push(chunk).ok_or(McuError::TooLong)?;
            dma.tx_fwdl.kick(regs);
        }
        Ok(())
    }
}
