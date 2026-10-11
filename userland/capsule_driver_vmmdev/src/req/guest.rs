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

//! The guest's half of the introduction: the host's version asked, the
//! guest info told (without claiming any Guest Additions version), the
//! events narrowed to the mouse's, and no capability offered, so the host
//! sends no display, seamless, balloon or shared-service traffic.

use super::page::Page;
use crate::constants::mouse::EVENT_MOUSE_POSITION;
use crate::constants::req::{
    CTL_GUEST_FILTER_MASK, GET_HOST_VERSION, INTERFACE_VERSION, OS_TYPE_UNKNOWN_X64,
    REPORT_GUEST_INFO, SET_GUEST_CAPABILITIES,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u32,
}

pub fn host_version(p: &mut Page) -> Result<HostVersion, i32> {
    let b = p.perform(GET_HOST_VERSION, &[0, 0, 0, 0])?;
    Ok(HostVersion { major: b[0] as u16, minor: (b[0] >> 16) as u16, build: b[1] })
}

pub fn report_guest_info(p: &mut Page) -> Result<(), i32> {
    p.perform(REPORT_GUEST_INFO, &[INTERFACE_VERSION, OS_TYPE_UNKNOWN_X64]).map(drop)
}

pub fn set_event_filter(p: &mut Page) -> Result<(), i32> {
    p.perform(CTL_GUEST_FILTER_MASK, &[EVENT_MOUSE_POSITION, !EVENT_MOUSE_POSITION]).map(drop)
}

pub fn set_no_capabilities(p: &mut Page) -> Result<(), i32> {
    p.perform(SET_GUEST_CAPABILITIES, &[0, u32::MAX]).map(drop)
}
