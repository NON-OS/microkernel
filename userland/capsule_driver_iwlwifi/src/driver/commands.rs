// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The packets the alive firmware posts back.

use super::Driver;

impl Driver {
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
