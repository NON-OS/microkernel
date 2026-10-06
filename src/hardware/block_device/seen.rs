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


//! What the block layer saw on a disk while choosing the NONOS disk, said
//! on the log when it changes. A stick the layer passed over left nothing:
//! a boot from it showed only that the store was not there, and no line
//! said which disk was asked or why it was not taken.

use alloc::format;
use alloc::string::String;

use super::backend::Backend;
use super::BlockDeviceError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Seen {
    /// The driver gave no size: no driver, no device, or still looking.
    NoSize(BlockDeviceError),
    /// The disk ends before the store would start.
    TooSmall(u64),
    /// Its blocks are not 512 bytes.
    NotFiveTwelve(u64),
    /// What its store sector held, and its disk plan sector when asked.
    At {
        sectors: u64,
        lba: u64,
        got: Result<[u8; 8], BlockDeviceError>,
        plan: Option<Result<[u8; 8], BlockDeviceError>>,
    },
}

pub(super) fn name(backend: Backend) -> &'static str {
    match backend {
        Backend::UsbMsc => "USB stick (driver.usb_msc0)",
        Backend::Nvme => "NVMe (driver.nvme0)",
        Backend::Ahci => "SATA (driver.ahci0)",
        Backend::VirtioBlk => "virtio-blk (driver.virtio_blk0)",
    }
}

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
        Seen::NoSize(BlockDeviceError::NotReady) => String::from("its driver is still looking for its device"),
        Seen::NoSize(e) => format!("no size given ({e:?})"),
        Seen::TooSmall(n) => format!("{n} sectors, too small to hold the store"),
        Seen::NotFiveTwelve(n) => format!("{n} sectors whose blocks are not 512 bytes; not used"),
        Seen::At { sectors, lba, got, plan } => {
            let plan = match plan {
                Some(p) => format!(", disk plan {}", sector(p)),
                None => String::new(),
            };
            format!("{sectors} sectors; LBA {lba} holds {}{plan}", sector(got))
        }
    };
    format!("[BLOCK] asked {}: {what}", name(backend))
}
