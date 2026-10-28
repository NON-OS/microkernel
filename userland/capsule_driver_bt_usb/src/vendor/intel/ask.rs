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

//! Commands, one at a time. Most go on the default endpoint and are
//! answered by Command Complete; Secure Send (0xfc09) goes on bulk OUT in
//! the bootloader (`btusb_send_frame_intel`); and Intel Reset into the
//! firmware is answered by nothing, so it is only posted.

use alloc::vec::Vec;

use super::wire::Wire;
use nonos_bt_hci::host::{answer, Answer};
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::{HciError, Transport};
use nonos_bt_wire::hci::command::command;
use nonos_bt_wire::hci::opcode::Opcode;
use nonos_bt_wire::WireError;

pub const SECURE_SEND: u16 = 0xfc09;
/// About two seconds for an answer, HCI_CMD_TIMEOUT's.
const ANSWER_LOOKS: u32 = 2_000;
const E_IO: i32 = -5;

impl<P: UsbPipes> Wire<'_, P> {
    /// The return parameters after a zero status.
    pub fn command(&mut self, op: u16, params: &[u8]) -> Result<Vec<u8>, HciError> {
        self.post(op, params)?;
        self.complete(op)
    }

    pub fn post(&mut self, op: u16, params: &[u8]) -> Result<(), HciError> {
        self.hci.send_command(&command(Opcode(op), params)?)
    }

    pub fn secure(&mut self, params: &[u8]) -> Result<(), HciError> {
        let packet = command(Opcode(SECURE_SEND), params)?;
        match self.hci.pipes().bulk_out(&packet).map_err(HciError::Transport)? {
            n if n == packet.len() => self.complete(SECURE_SEND).map(drop),
            _ => Err(HciError::Transport(E_IO)),
        }
    }

    fn complete(&mut self, op: u16) -> Result<Vec<u8>, HciError> {
        let mine = |e: &[u8]| answer(e, Some(Opcode(op))).ok().flatten().and_then(|a| a.1);
        match self.wait(ANSWER_LOOKS, mine)? {
            Some(Answer::Complete(ret)) => match ret.split_first() {
                Some((0, rest)) => Ok(rest.to_vec()),
                Some((&s, _)) => Err(HciError::Status(Opcode(op), s)),
                None => Err(HciError::Wire(WireError::Short)),
            },
            Some(Answer::Status(s)) => Err(HciError::Status(Opcode(op), s)),
            None => Err(HciError::Timeout(Opcode(op))),
        }
    }

    /// Said, and the controller left to the standard bring-up to refuse.
    pub fn refuse(&self, words: &[&[u8]]) -> HciError {
        (self.say)(&[b"intel: ", &words.concat()]);
        HciError::NotBluetooth
    }
}
