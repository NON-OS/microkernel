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

use super::answer::Answer;
use super::error::ParseError;
use crate::dns::{TYPE_A, TYPE_AAAA};

/// The fixed part of a resource record after its owner name, and where its
/// data lies in the message.
pub(super) struct Record {
    pub rtype: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdata: usize,
    pub rdlen: usize,
    /// Offset of the next record.
    pub next: usize,
}

/// The record whose fixed part starts at `pos`, its data bounded by the
/// message.
pub(super) fn read_record(message: &[u8], pos: usize) -> Result<Record, ParseError> {
    let fixed = message.get(pos..pos.checked_add(10).ok_or(ParseError::Truncated)?).ok_or(ParseError::Truncated)?;
    let rdlen = usize::from(u16::from_be_bytes([fixed[8], fixed[9]]));
    let rdata = pos + 10;
    let next = rdata + rdlen;
    if next > message.len() {
        return Err(ParseError::Truncated);
    }
    Ok(Record {
        rtype: u16::from_be_bytes([fixed[0], fixed[1]]),
        class: u16::from_be_bytes([fixed[2], fixed[3]]),
        ttl: sane_ttl(u32::from_be_bytes([fixed[4], fixed[5], fixed[6], fixed[7]])),
        rdata,
        rdlen,
        next,
    })
}

/// RFC 2181 8: a TTL with the top bit set is treated as zero.
fn sane_ttl(raw: u32) -> u32 {
    if raw > 0x7FFF_FFFF {
        0
    } else {
        raw
    }
}

/// The address an A or AAAA record carries, if its data has that type's size.
pub(super) fn answer_from(rtype: u16, ttl: u32, rdata: &[u8]) -> Option<Answer> {
    let answer = Answer { rtype, ttl, ipv4: ipv4_from(rtype, rdata), ipv6: ipv6_from(rtype, rdata) };
    let whole = match answer.rtype {
        TYPE_A => answer.ipv4.is_some(),
        TYPE_AAAA => answer.ipv6.is_some(),
        _ => false,
    };
    whole.then_some(answer)
}

fn ipv4_from(rtype: u16, rdata: &[u8]) -> Option<[u8; 4]> {
    if rtype != TYPE_A || rdata.len() != 4 {
        return None;
    }
    let mut out = [0u8; 4];
    out.copy_from_slice(rdata);
    Some(out)
}

fn ipv6_from(rtype: u16, rdata: &[u8]) -> Option<[u8; 16]> {
    if rtype != TYPE_AAAA || rdata.len() != 16 {
        return None;
    }
    let mut out = [0u8; 16];
    out.copy_from_slice(rdata);
    Some(out)
}
