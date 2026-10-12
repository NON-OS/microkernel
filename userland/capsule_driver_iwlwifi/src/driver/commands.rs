// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Host commands to the alive firmware, and the packets it posts back.

use super::Driver;

impl Driver {
    /// Issue a host command to the alive firmware over the command queue,
    /// returning the new ring write pointer or `None` if it does not fit a
    /// slot. Composes the command in the DMA buffer that staged the firmware,
    /// fills the slot's TFD, advances the ring and rings the doorbell.
    pub fn issue_command(&mut self, cmd: u8, group: u8, payload: &[u8]) -> Option<usize> {
        if (self.dma_len as usize) < crate::constants::DMA_LAYOUT_END {
            return None;
        }
        let q = crate::hcmd::send::CmdQueue {
            dma_user_va: self.dma_user_va,
            dma_device_addr: self.dma_device_addr,
            ring_off: crate::constants::CMD_RING_OFFSET,
            cmd_off: crate::constants::CMD_AREA_OFFSET,
            queue: crate::constants::CMD_QUEUE_ID,
        };
        let next =
            unsafe { crate::hcmd::send::send_host_command(&self.regs, &q, self.cmd_write_ptr, cmd, group, payload)? };
        self.cmd_write_ptr = next;
        Some(next)
    }

    /// Receive the next packet the firmware posted, copying its payload into
    /// `out` and returning `(command, group, sequence, payload length)`, or
    /// `None` if the receive ring is empty. Reads the firmware's write pointer
    /// from the device and advances the driver's read pointer.
    pub fn poll_response(&mut self, out: &mut [u8]) -> Option<(u8, u8, u16, usize)> {
        if (self.dma_len as usize) < crate::constants::DMA_LAYOUT_END {
            return None;
        }
        let write_idx = self.regs.read32(crate::constants::RX_WPTR_REG) as usize;
        let (cmd, group, seq, n, next) = unsafe {
            crate::rx::recv::receive(
                self.dma_user_va,
                crate::constants::RX_RB_OFFSET,
                self.rx_read_ptr,
                write_idx,
                out,
            )?
        };
        self.rx_read_ptr = next;
        Some((cmd, group, seq, n))
    }
}
