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

//! `GetEventLog`, asked for the crypto-agile log.

use uefi::table::boot::{BootServices, OpenProtocolAttributes, OpenProtocolParams, SearchType};
use uefi::Identify;

use crate::security::tpm_types::Tcg2Protocol;

/// `EFI_TCG2_EVENT_LOG_FORMAT_TCG_2`.
const FORMAT_TCG_2: u32 = 2;

/// The log's start and its last entry's start, from `GetEventLog`.
pub(super) fn locate(bs: &BootServices) -> Option<(u64, u64)> {
    let handles = bs.locate_handle_buffer(SearchType::ByProtocol(&Tcg2Protocol::GUID)).ok()?;
    let handle = *handles.first()?;
    let params = OpenProtocolParams { handle, agent: bs.image_handle(), controller: None };
    /* SAFETY: eK@nonos.systems - GetProtocol opens without taking ownership. */
    let proto =
        unsafe { bs.open_protocol::<Tcg2Protocol>(params, OpenProtocolAttributes::GetProtocol) }
            .ok()?;
    let tcg2 = &*proto as *const Tcg2Protocol as *mut Tcg2Protocol;
    let (mut start, mut last, mut truncated) = (0u64, 0u64, false);
    /* SAFETY: eK@nonos.systems - an open protocol and three live out-pointers. */
    let status = unsafe {
        ((*tcg2).get_event_log)(tcg2, FORMAT_TCG_2, &mut start, &mut last, &mut truncated)
    };
    if status.is_error() || start == 0 || truncated {
        return None;
    }
    Some((start, if last == 0 { start } else { last }))
}
