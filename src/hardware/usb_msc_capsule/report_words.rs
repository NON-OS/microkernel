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


//! driver.usb_msc0's search report (userland/capsule_driver_usb_msc/src/
//! scan/report.rs) in words. Pure, so kernel_proofs holds every stage.

use alloc::format;
use alloc::string::String;

/// The report's length on the wire.
pub(crate) const REPORT_LEN: usize = 20;

/// The report's bytes in words.
pub(crate) fn words(r: &[u8]) -> String {
    let (stage, port, closed, connected) = (r[0], r[1], r[2], r[3]);
    let detail = i32::from_le_bytes([r[4], r[5], r[6], r[7]]);
    let aux = u32::from_le_bytes([r[8], r[9], r[10], r[11]]);
    let blocks = u64::from_le_bytes([r[12], r[13], r[14], r[15], r[16], r[17], r[18], r[19]]);
    let what = match stage {
        1 if detail == 0 => String::from("driver.xhci0 has not registered; no USB port can be asked yet"),
        1 => format!("driver.xhci0 would not list its ports (errno {detail})"),
        2 => String::from("no device is connected to any USB port"),
        3 => format!("port {port}: another USB class driver is addressing it"),
        4 => format!("port {port}: the device took no address (errno {detail})"),
        5 => format!("port {port}: its configuration descriptor did not read (errno {detail})"),
        6 => format!(
            "port {port}: not a USB stick: interface class {:02x}, subclass {:02x}, protocol {:02x} \
             (a stick is 08 06 50)",
            (aux >> 16) & 0xff,
            (aux >> 8) & 0xff,
            aux & 0xff
        ),
        7 => format!("port {port}: SET_CONFIGURATION or its bulk pipes were refused (errno {detail})"),
        8 => format!("port {port}: did not say it was ready in thirty seconds (errno {detail})"),
        9 => format!("port {port}: READ CAPACITY failed (errno {detail})"),
        10 => format!("port {port}: bound, {blocks} blocks of {aux} bytes"),
        s => format!("an unknown stage {s}"),
    };
    format!("[USB-MSC] {what}; {connected} port(s) connected, {closed} given up")
}
