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

use super::super::constants::{Color, MAX_CONSOLES};
use super::super::cursor::update_cursor;
use super::super::error::VgaError;
use super::super::state::{ACTIVE_CONSOLE, CHARS_WRITTEN, CONSOLES};
use super::lock::{acquire_lock, release_lock};
use core::ptr::addr_of_mut;
use core::sync::atomic::Ordering;

pub fn write_byte(byte: u8) {
    if !acquire_lock() {
        return;
    }

    let index = ACTIVE_CONSOLE.load(Ordering::Relaxed);
    // SAFETY: Index from atomic, lock held
    unsafe {
        let consoles = addr_of_mut!(CONSOLES);
        (*consoles)[index].write_byte(byte);
        (*consoles)[index].flush_to_vga();
        update_cursor((*consoles)[index].row, (*consoles)[index].col);
    }
    CHARS_WRITTEN.fetch_add(1, Ordering::Relaxed);

    release_lock();
}

pub fn write_str(s: &str) {
    if !acquire_lock() {
        return;
    }

    let index = ACTIVE_CONSOLE.load(Ordering::Relaxed);
    // SAFETY: Index from atomic, lock held
    unsafe {
        let consoles = addr_of_mut!(CONSOLES);
        for byte in s.bytes() {
            (*consoles)[index].write_byte(byte);
        }
        (*consoles)[index].flush_to_vga();
        update_cursor((*consoles)[index].row, (*consoles)[index].col);
    }
    CHARS_WRITTEN.fetch_add(s.len() as u64, Ordering::Relaxed);

    release_lock();
}

pub fn write_str_to_console(index: usize, s: &str) -> Result<(), VgaError> {
    if index >= MAX_CONSOLES {
        return Err(VgaError::InvalidConsole);
    }

    if !acquire_lock() {
        return Err(VgaError::LockContention);
    }

    // SAFETY: Index bounds checked, lock held
    unsafe {
        let consoles = addr_of_mut!(CONSOLES);
        for byte in s.bytes() {
            (*consoles)[index].write_byte(byte);
        }

        if index == ACTIVE_CONSOLE.load(Ordering::Relaxed) {
            (*consoles)[index].flush_to_vga();
            update_cursor((*consoles)[index].row, (*consoles)[index].col);
        }
    }

    release_lock();
    Ok(())
}

pub fn clear() {
    if !acquire_lock() {
        return;
    }

    let index = ACTIVE_CONSOLE.load(Ordering::Relaxed);
    // SAFETY: Index from atomic, lock held
    unsafe {
        let consoles = addr_of_mut!(CONSOLES);
        (*consoles)[index].clear();
        (*consoles)[index].flush_to_vga();
        update_cursor(0, 0);
    }

    release_lock();
}

pub fn set_color(fg: Color, bg: Color) {
    if !acquire_lock() {
        return;
    }

    let index = ACTIVE_CONSOLE.load(Ordering::Relaxed);
    // SAFETY: Index from atomic, lock held
    unsafe {
        let consoles = addr_of_mut!(CONSOLES);
        (*consoles)[index].set_color(fg, bg);
    }

    release_lock();
}
