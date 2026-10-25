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

//! The tool's arguments: the record first, then named options, each given
//! once except `--authority`, `--ckpt` and `--revoked`, which may repeat.

use alloc::string::String;
use alloc::vec::Vec;

pub(super) struct Args {
    pub record: String,
    pub doc: String,
    pub fleet: [u8; 16],
    pub authorities: Vec<String>,
    pub ckpts: Vec<String>,
    pub revoked: Vec<String>,
    pub session: Option<[u8; 16]>,
    pub subject: Option<[u8; 64]>,
}

pub(super) fn parse(args: &[String]) -> Option<Args> {
    let (record, mut rest) = args.split_first()?;
    let (mut doc, mut fleet) = (None, None);
    let (mut authorities, mut ckpts, mut revoked) = (Vec::new(), Vec::new(), Vec::new());
    let (mut session, mut subject) = (None, None);
    while let [name, value, tail @ ..] = rest {
        match name.as_str() {
            "--doc" if doc.is_none() => doc = Some(value.clone()),
            "--fleet" if fleet.is_none() => fleet = Some(hex(value)?),
            "--session" if session.is_none() => session = Some(hex(value)?),
            "--subject" if subject.is_none() => subject = Some(hex(value)?),
            "--revoked" => revoked.push(value.clone()),
            "--authority" => authorities.push(value.clone()),
            "--ckpt" => ckpts.push(value.clone()),
            _ => return None,
        }
        rest = tail;
    }
    if !rest.is_empty() || record.starts_with("--") || authorities.is_empty() {
        return None;
    }
    let (record, doc, fleet) = (record.clone(), doc?, fleet?);
    Some(Args { record, doc, fleet, authorities, ckpts, revoked, session, subject })
}

/// `N` bytes as `2N` hex digits.
fn hex<const N: usize>(s: &str) -> Option<[u8; N]> {
    let b = s.as_bytes();
    if b.len() != 2 * N {
        return None;
    }
    let mut out = [0u8; N];
    for (o, pair) in out.iter_mut().zip(b.chunks_exact(2)) {
        let digit = |c: u8| char::from(c).to_digit(16);
        *o = u8::try_from(digit(pair[0])? * 16 + digit(pair[1])?).ok()?;
    }
    Some(out)
}
