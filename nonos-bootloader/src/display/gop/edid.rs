// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::pick;
use uefi::prelude::*;
use uefi::proto::unsafe_protocol;
use uefi::table::boot::{BootServices, OpenProtocolAttributes, OpenProtocolParams};

// EFI_EDID_ACTIVE_PROTOCOL and EFI_EDID_DISCOVERED_PROTOCOL (UEFI 2.10
// section 12.9.3), installed by the GOP driver on the same child handle as
// the output. Both are { UINT32 SizeOfEdid; UINT8 *Edid; }. Active is what
// the GOP is driving (an override when platform firmware set one), so it
// is asked first.
#[repr(C)]
#[unsafe_protocol("bd8c1056-9f36-44ec-92a8-a6337f817986")]
struct EdidActive {
    size: u32,
    edid: *const u8,
}

#[repr(C)]
#[unsafe_protocol("1c0c34f6-d380-41fa-a049-8ad06c1a66aa")]
struct EdidDiscovered {
    size: u32,
    edid: *const u8,
}

pub(crate) fn read_edid(bs: &BootServices, h: Handle) -> Option<pick::Edid> {
    let params = OpenProtocolParams { handle: h, agent: bs.image_handle(), controller: None };
    // SAFETY: GetProtocol opens without taking ownership, and the EDID
    // buffer belongs to the GOP driver for as long as the handle exists.
    if let Ok(p) =
        unsafe { bs.open_protocol::<EdidActive>(params, OpenProtocolAttributes::GetProtocol) }
    {
        if let Some(e) = parse_blob(p.size, p.edid) {
            return Some(e);
        }
    }
    let params = OpenProtocolParams { handle: h, agent: bs.image_handle(), controller: None };
    // SAFETY: as above.
    let p =
        unsafe { bs.open_protocol::<EdidDiscovered>(params, OpenProtocolAttributes::GetProtocol) }
            .ok()?;
    parse_blob(p.size, p.edid)
}

fn parse_blob(size: u32, edid: *const u8) -> Option<pick::Edid> {
    if edid.is_null() || size < 128 {
        return None;
    }
    // SAFETY: the protocol says SizeOfEdid bytes live at Edid; only the
    // 128 byte base block is read.
    let bytes = unsafe { core::slice::from_raw_parts(edid, 128) };
    pick::parse_edid(bytes)
}
