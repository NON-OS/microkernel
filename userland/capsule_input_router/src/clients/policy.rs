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

use core::ptr;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

const SERVICE: &[u8] = b"policy";
const HDR_LEN: usize = 12;
const OP_GET: u16 = 0x0001;
const FIELD_MOUSE_SENSITIVITY: u32 = 0x0102;
const FIELD_SCREEN_TIMEOUT: u32 = 0x010A;
const FIELD_AUTO_LOCK_TIMEOUT: u32 = 0x0113;
const FIELD_LOCK_INHIBIT_HOLDS: u32 = 0x0131;
const KIND_BOOL: u8 = 1;
const KIND_U8: u8 = 2;
const E_OK: u16 = 0;
const REPLY_TIMEOUT_MS: u64 = 150;

pub fn mouse_sensitivity(port_slot: &mut u32) -> Option<u8> {
    get_u8(port_slot, FIELD_MOUSE_SENSITIVITY)
}

/// Minutes without input before the screen blanks; 0 never.
pub fn screen_timeout(port_slot: &mut u32) -> Option<u8> {
    get_u8(port_slot, FIELD_SCREEN_TIMEOUT)
}

/// Minutes without input before the shell is asked to lock; 0 never.
pub fn auto_lock_timeout(port_slot: &mut u32) -> Option<u8> {
    get_u8(port_slot, FIELD_AUTO_LOCK_TIMEOUT)
}

/// Whether an idle inhibit holds back auto-lock too.
pub fn lock_inhibit_holds(port_slot: &mut u32) -> Option<bool> {
    get(port_slot, FIELD_LOCK_INHIBIT_HOLDS, KIND_BOOL).map(|v| v != 0)
}

fn get_u8(port_slot: &mut u32, field: u32) -> Option<u8> {
    get(port_slot, field, KIND_U8)
}

fn get(port_slot: &mut u32, field: u32, kind: u8) -> Option<u8> {
    if *port_slot == 0 {
        let mut port = 0u32;
        let rc = mk_service_lookup(
            SERVICE.as_ptr(),
            SERVICE.len(),
            &mut port as *mut u32,
            ptr::null_mut(),
        );
        if rc < 0 || port == 0 {
            return None;
        }
        *port_slot = port;
    }
    let mut tx = [0u8; HDR_LEN];
    tx[0..2].copy_from_slice(&OP_GET.to_le_bytes());
    tx[2..6].copy_from_slice(&field.to_le_bytes());
    tx[6] = kind;
    let mut rx = [0u8; HDR_LEN + 4];
    let n = mk_ipc_call_timeout(
        *port_slot as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        REPLY_TIMEOUT_MS,
    );
    if n < HDR_LEN as i64 {
        *port_slot = 0;
        return None;
    }
    let status = u16::from_le_bytes([rx[8], rx[9]]);
    if status != E_OK || rx[6] != kind || n < (HDR_LEN + 1) as i64 {
        return None;
    }
    Some(rx[HDR_LEN])
}
