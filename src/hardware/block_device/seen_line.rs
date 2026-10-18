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

//! The line the log gets for what a disk showed.

use alloc::{format, string::String};

use super::backend::Backend;
use super::gpt::{Held, Shown};
use super::seen::Seen;
use super::BlockDeviceError;

/// The eight bytes as text where printable, `.` elsewhere.
fn shown(head: &[u8; 8]) -> String {
    head.iter().map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' }).collect()
}

fn sector(got: &Result<[u8; 8], BlockDeviceError>) -> String {
    match got {
        Ok(head) => format!("\"{}\"", shown(head)),
        Err(e) => format!("unread ({e:?})"),
    }
}

/// One line saying what `backend` showed.
pub(super) fn line(backend: Backend, seen: &Seen) -> String {
    let what = match seen {
        Seen::NoSize(BlockDeviceError::Dead) => String::from("no disk behind this driver"),
        Seen::NoSize(BlockDeviceError::NotReady) => {
            String::from("its driver is still looking for its device")
        }
        Seen::NoSize(e) => format!("no size given ({e:?})"),
        Seen::TooSmall(n) => format!("{n} sectors, too small to hold the store"),
        Seen::NotFiveTwelve(n) => format!("{n} sectors whose blocks are not 512 bytes; not used"),
        Seen::NoTable(n, e) => format!("{n} sectors; its partition table unread ({e:?})"),
        Seen::Crossed(n) => {
            format!("{n} sectors; its GPT names another system's partition over the fixed store")
        }
        Seen::At { at: (sectors, lba), got, plan, gpt, boot, shown } => {
            let plan = match plan {
                Some(p) => format!(", disk plan {}", sector(p)),
                None => String::new(),
            };
            let by = if *gpt { "its table's store" } else { "the fixed store" };
            let boot = if *boot { "; the loader's disk" } else { "" };
            let held = match shown {
                Shown::Named(Held::Backup) => "; its primary GPT failed, its backup read",
                Shown::Named(Held::Disagreeing) => "; its backup GPT disagrees, its primary read",
                Shown::Failed => {
                    "; both GPT headers failed; the fixed layout used where its magic is"
                }
                _ => "",
            };
            format!("{sectors} sectors; {by} at LBA {lba} holds {}{plan}{boot}{held}", sector(got))
        }
    };
    format!("[BLOCK] asked {}: {what}", backend.shown())
}
