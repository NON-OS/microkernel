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

//! One line of a report judged by its place and its kind.

use crate::parse::{Fault, Summary};
use crate::parse_fields::{build_ok, end_of, log_ok};
use crate::parse_rows::{acpi_ok, pci_ok};
use crate::HEADER;

/// Line `i` (from 0) checked and counted into `s`; true for the `end` line.
pub(crate) fn judge(i: usize, line: &[u8], s: &mut Summary) -> Result<bool, Fault> {
    let n = i + 1;
    let ok = match i {
        0 if line == HEADER => true,
        0 => return Err(Fault::NoHeader),
        1 => build_ok(line),
        2 => {
            s.unconfined = line == b"posture dma-unconfined";
            s.unconfined || line == b"posture confined"
        }
        _ => {
            if line.starts_with(b"end ") {
                let (count, cut) = end_of(line).ok_or(Fault::Field(n))?;
                if count != i as u64 {
                    return Err(Fault::Count(n));
                }
                s.cut = cut;
                return Ok(true);
            }
            body(line, n, s)?
        }
    };
    if ok {
        Ok(false)
    } else {
        Err(Fault::Field(n))
    }
}

fn body(line: &[u8], n: usize, s: &mut Summary) -> Result<bool, Fault> {
    if let Some(w) = line.strip_prefix(b"pci ") {
        s.pci += 1;
        return Ok(pci_ok(w));
    }
    if let Some(w) = line.strip_prefix(b"acpi ") {
        s.acpi += 1;
        return Ok(acpi_ok(w));
    }
    if line == b"log -" {
        s.dropped += 1;
        return Ok(true);
    }
    let Some(w) = line.strip_prefix(b"log ") else {
        return Ok(false);
    };
    if !log_ok(w) {
        return Err(Fault::Leak(n));
    }
    s.log += 1;
    Ok(true)
}
