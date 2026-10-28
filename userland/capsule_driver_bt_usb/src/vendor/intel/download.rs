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

//! One firmware file into the bootloader: its three header sections, its
//! commands in aligned runs, then the bootloader's Secure Send Result
//! (0xff, 0x06), zero if it took the firmware, within five seconds
//! (`btintel_download_wait`). Once a fragment has gone, any failure sends
//! the controller back to its bootloader, as Linux does.

use super::ask::SECURE_SEND;
use super::boot::to_bootloader;
use super::cmds::{boot_params, runs};
use super::send::send;
use super::sfi::{Layout, DATA};
use super::wire::Wire;
use nonos_bt_hci::usb::UsbPipes;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode::Opcode;

const RESULT_LOOKS: u32 = 5_000;

/// The address to boot, once the bootloader has taken the firmware.
pub fn download<P: UsbPipes>(w: &mut Wire<P>, fw: &[u8], l: &Layout) -> Result<u32, HciError> {
    let cmds = &fw[l.payload..];
    let (Some(runs), Some((addr, _))) = (runs(cmds), boot_params(cmds)) else {
        return Err(w.refuse(&[b"firmware commands malformed"]));
    };
    w.result = None;
    let headers = l.headers.iter().map(|&(kind, at, n)| (kind, &fw[at..at + n]));
    let mut all = headers.chain(runs.iter().map(|run| (DATA, *run)));
    let sent = all.try_for_each(|(kind, data)| send(w, kind, data));
    let taken = sent.and_then(|()| result(w));
    taken.inspect_err(|_| to_bootloader(w)).map(|()| addr)
}

fn result<P: UsbPipes>(w: &mut Wire<P>) -> Result<(), HciError> {
    let told = match w.result {
        Some(r) => Some(r),
        None => w.wait(RESULT_LOOKS, |e| match e {
            [0xff, 5, 0x06, r, ..] => Some(*r),
            _ => None,
        })?,
    };
    match told {
        Some(0) => Ok(()),
        Some(r) => Err(HciError::Status(Opcode(SECURE_SEND), r)),
        None => Err(HciError::Timeout(Opcode(SECURE_SEND))),
    }
}
