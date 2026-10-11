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

//! The high-priority mailboxes in the register window, each 64 bits wide
//! with the driver writing the low word (TG3_64BIT_REG_LOW), and where
//! tg3_init_one puts each vector's three.

pub const TG3_64BIT_REG_HIGH: usize = 0x0;
pub const TG3_64BIT_REG_LOW: usize = 0x4;
pub const MAILBOX_INTERRUPT_0: usize = 0x0200;
pub const MAILBOX_RCV_STD_PROD_IDX: usize = 0x0268;
pub const MAILBOX_RCV_JUMBO_PROD_IDX: usize = 0x0270;
pub const MAILBOX_RCVRET_CON_IDX_0: usize = 0x0280;
pub const MAILBOX_SNDHOST_PROD_IDX_0: usize = 0x0300;

pub const INT_MBOX_0: usize = MAILBOX_INTERRUPT_0 + TG3_64BIT_REG_LOW;
pub const RX_STD_PROD_IDX_REG: usize = MAILBOX_RCV_STD_PROD_IDX + TG3_64BIT_REG_LOW;
pub const RX_JMB_PROD_IDX_REG: usize = MAILBOX_RCV_JUMBO_PROD_IDX + TG3_64BIT_REG_LOW;
pub const RCVRET_CON_MBOX_0: usize = MAILBOX_RCVRET_CON_IDX_0 + TG3_64BIT_REG_LOW;
pub const SNDHOST_PROD_MBOX_0: usize = MAILBOX_SNDHOST_PROD_IDX_0 + TG3_64BIT_REG_LOW;

/// Vector `n`'s interrupt mailbox: each is 8 past the one before.
pub fn int_mbox(n: usize) -> usize {
    INT_MBOX_0 + 8 * n
}

/// Vector `n`'s return-ring consumer mailbox, n at least 1. tg3_init_one
/// gives vector 1 vector 0's and moves 8 on for each after it.
pub fn consumer_mbox(n: usize) -> usize {
    RCVRET_CON_MBOX_0 + 8 * (n - 1)
}
