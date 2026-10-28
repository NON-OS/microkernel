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

//! An Intel controller behind the USB pipes, written from btintel.c's and
//! btusb.c's description of the device, apart from the loader under test.
//! Events go out on the interrupt endpoint in 16-octet transfers, or on
//! bulk IN while `bulk_events` is set; a transfer longer than the host
//! armed fails the proof, as babble would.

use std::collections::VecDeque;

#[derive(Default)]
pub struct Intel {
    /// Read Version's return parameters after the status: with parameter
    /// 0xff, then without, each before and after boot (index: booted).
    pub tlv: [Vec<u8>; 2],
    pub old: [Vec<u8>; 2],
    /// Read Boot Params' return parameters after the status.
    pub boot_params: Vec<u8>,
    /// Data octets the firmware holds; once all have come, the result.
    pub expect: usize,
    pub result: u8,
    /// The result is reported before the last fragment's Complete.
    pub result_first: bool,
    /// The fragment (counted from 1) answered with status 0x12.
    pub refuse_fragment: usize,
    pub boots: bool,
    /// An intermediate loader: what Read Version answers while it runs and
    /// the data octets of the firmware it then takes; set while it runs.
    pub iml: Option<(Vec<u8>, usize)>,
    pub in_iml: bool,
    pub bulk_events: bool,
    pub booted: bool,
    /// What the host did: every opcode in order, every Secure Send's
    /// parameters, every DDC entry, the boot address, hard resets.
    pub ops: Vec<u16>,
    pub fragments: Vec<Vec<u8>>,
    pub ddc: Vec<Vec<u8>>,
    pub boot_addr: Option<u32>,
    pub boot_addrs: Vec<u32>,
    pub to_bootloader: u32,
    pub data: usize,
    pub pauses: u32,
    pub(super) interrupt: VecDeque<Vec<u8>>,
    pub(super) bulk_in: VecDeque<Vec<u8>>,
}
