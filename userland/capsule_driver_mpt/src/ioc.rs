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

//! The IOC brought to operational: ready (a message unit reset if it is
//! not), its facts, IOC INIT, the reply frames handed over, and its port
//! enabled. Run at bring-up, and again to bring the IOC back after a
//! request it never completed.

use crate::constants::layout::{REPLY_BYTES, REPLY_FRAMES, REPLY_OFF, REQUEST_BYTES, REQUEST_OFF};
use crate::constants::msg::{
    FACTS_DWORDS, FACTS_MAX_BUSES, FACTS_MAX_DEVICES, FACTS_REQUEST_FRAME_SIZE,
    HANDSHAKE_REPLY_MAX, INIT_DWORDS, INIT_HEADER_VERSION, INIT_MAX_BUSES, INIT_MAX_DEVICES,
    INIT_MSG_VERSION, INIT_REPLY_FRAME_SIZE, INIT_WHO, MPI_HEADER_VERSION, MPI_VERSION,
    MSG_CONTEXT, MSG_FUNCTION, PORT_ENABLE_DWORDS, REPLY_IOC_STATUS, WHOINIT_HOST_DRIVER,
};
use crate::constants::regs::{
    ADDRESS_REPLY, DOORBELL_FUNCTION_SHIFT, FIFO_EMPTY, FUNCTION_IOC_FACTS, FUNCTION_IOC_INIT,
    FUNCTION_IOC_MESSAGE_UNIT_RESET, FUNCTION_PORT_ENABLE, HIM_ALL, IOCSTATUS_MASK,
    IOCSTATUS_SUCCESS, IOC_STATE_MASK, IOC_STATE_OPERATIONAL, IOC_STATE_READY, REG_DOORBELL,
    REG_HOST_INTERRUPT_MASK, REG_REPLY_FIFO, REG_REQUEST_FIFO,
};
use crate::constants::timing::{PORT_ENABLE_MS, RESET_MS};
use crate::handshake::handshake;
use crate::regs::Regs;
use crate::wait;

/// The contexts the driver's own messages carry; a SCSI request's never
/// reach them.
pub const CONTEXT_PORT_ENABLE: u32 = 0x1F00_0001;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Facts {
    pub max_devices: u8,
    pub max_buses: u8,
}

fn state<R: Regs>(r: &R) -> u32 {
    r.r32(REG_DOORBELL) & IOC_STATE_MASK
}

/// Ready: as it is, or after a message unit reset.
pub fn ready<R: Regs>(r: &R) -> Result<(), &'static str> {
    if state(r) == IOC_STATE_READY {
        return Ok(());
    }
    r.w32(REG_DOORBELL, (FUNCTION_IOC_MESSAGE_UNIT_RESET as u32) << DOORBELL_FUNCTION_SHIFT);
    if !wait::until(RESET_MS, || state(r) == IOC_STATE_READY) {
        return Err("IOC not ready after reset");
    }
    Ok(())
}

fn status_of(reply: &[u8]) -> u16 {
    u16::from_le_bytes([reply[REPLY_IOC_STATUS], reply[REPLY_IOC_STATUS + 1]]) & IOCSTATUS_MASK
}

pub fn facts<R: Regs>(r: &R) -> Result<Facts, &'static str> {
    let mut req = [0u32; FACTS_DWORDS];
    req[0] = (FUNCTION_IOC_FACTS as u32) << 24;
    let mut reply = [0u8; HANDSHAKE_REPLY_MAX];
    let n = handshake(r, &req, &mut reply)?;
    // An older IOC's facts are shorter than MPI 1.5's; every field read here
    // is in the part they share.
    if n <= FACTS_MAX_BUSES
        || reply[MSG_FUNCTION] != FUNCTION_IOC_FACTS
        || status_of(&reply) != IOCSTATUS_SUCCESS
    {
        return Err("IOC facts refused");
    }
    let frame = [reply[FACTS_REQUEST_FRAME_SIZE], reply[FACTS_REQUEST_FRAME_SIZE + 1]];
    if !crate::frame::served(u16::from_le_bytes(frame)) {
        return Err("IOC request frames not served");
    }
    let f = Facts { max_devices: reply[FACTS_MAX_DEVICES], max_buses: reply[FACTS_MAX_BUSES] };
    if f.max_buses == 0 {
        return Err("IOC has no bus");
    }
    Ok(f)
}

pub fn init<R: Regs>(r: &R, f: Facts) -> Result<(), &'static str> {
    let mut m = [0u8; INIT_DWORDS * 4];
    m[INIT_WHO] = WHOINIT_HOST_DRIVER;
    m[MSG_FUNCTION] = FUNCTION_IOC_INIT;
    m[INIT_MAX_DEVICES] = f.max_devices;
    m[INIT_MAX_BUSES] = 1;
    m[INIT_REPLY_FRAME_SIZE..INIT_REPLY_FRAME_SIZE + 2]
        .copy_from_slice(&(REPLY_BYTES as u16).to_le_bytes());
    m[INIT_MSG_VERSION..INIT_MSG_VERSION + 2].copy_from_slice(&MPI_VERSION.to_le_bytes());
    m[INIT_HEADER_VERSION..INIT_HEADER_VERSION + 2]
        .copy_from_slice(&MPI_HEADER_VERSION.to_le_bytes());
    let req: [u32; INIT_DWORDS] =
        core::array::from_fn(|i| u32::from_le_bytes(m[4 * i..4 * i + 4].try_into().unwrap()));
    let mut reply = [0u8; HANDSHAKE_REPLY_MAX];
    handshake(r, &req, &mut reply)?;
    if reply[MSG_FUNCTION] != FUNCTION_IOC_INIT || status_of(&reply) != IOCSTATUS_SUCCESS {
        return Err("IOC init refused");
    }
    if state(r) != IOC_STATE_OPERATIONAL {
        return Err("IOC not operational after init");
    }
    Ok(())
}

/// PORT ENABLE through the request FIFO, its completion read back.
fn port_enable<R: Regs>(r: &R, va: u64, dev: u64) -> Result<(), &'static str> {
    let frame = (va as usize + REQUEST_OFF) as *mut u8;
    // SAFETY: the request frame, the grant's first bytes, mapped for the
    // driver's life; nothing is posted from it.
    unsafe { core::ptr::write_bytes(frame, 0, REQUEST_BYTES) };
    let mut m = [0u8; PORT_ENABLE_DWORDS * 4];
    m[MSG_FUNCTION] = FUNCTION_PORT_ENABLE;
    m[MSG_CONTEXT..MSG_CONTEXT + 4].copy_from_slice(&CONTEXT_PORT_ENABLE.to_le_bytes());
    // SAFETY: as above, PORT_ENABLE_DWORDS * 4 bytes of the frame.
    unsafe { core::ptr::copy_nonoverlapping(m.as_ptr(), frame, m.len()) };
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    r.w32(REG_REQUEST_FIFO, (dev + REQUEST_OFF as u64) as u32);
    let mut v = FIFO_EMPTY;
    if !wait::until(PORT_ENABLE_MS, || {
        v = r.r32(REG_REPLY_FIFO);
        v != FIFO_EMPTY
    }) {
        return Err("port enable not completed");
    }
    if v & ADDRESS_REPLY == 0 {
        return if v == CONTEXT_PORT_ENABLE {
            Ok(())
        } else {
            Err("port enable answered for another")
        };
    }
    // An address reply carries a status; its frame goes back to the IOC.
    let at = (v << 1) as u64;
    let off = at.wrapping_sub(dev + REPLY_OFF as u64) as usize;
    if off >= REPLY_FRAMES * REPLY_BYTES || !off.is_multiple_of(REPLY_BYTES) {
        return Err("port enable reply outside the frames");
    }
    // SAFETY: a reply frame of the grant, as checked.
    let reply = unsafe {
        core::slice::from_raw_parts((va as usize + REPLY_OFF + off) as *const u8, REPLY_BYTES)
    };
    let ok = status_of(reply) == IOCSTATUS_SUCCESS;
    r.w32(REG_REPLY_FIFO, at as u32);
    ok.then_some(()).ok_or("port enable refused")
}

/// Every interrupt masked (the driver polls), the IOC ready and its facts
/// read, IOC INIT with this driver's reply frame size and the IOC's own
/// device count on one bus, every reply frame handed to the reply free
/// FIFO, the port enabled.
pub fn start<R: Regs>(r: &R, va: u64, dev: u64) -> Result<Facts, &'static str> {
    r.w32(REG_HOST_INTERRUPT_MASK, HIM_ALL);
    ready(r)?;
    let f = facts(r)?;
    init(r, f)?;
    for i in 0..REPLY_FRAMES {
        r.w32(REG_REPLY_FIFO, (dev + (REPLY_OFF + i * REPLY_BYTES) as u64) as u32);
    }
    port_enable(r, va, dev)?;
    Ok(f)
}
