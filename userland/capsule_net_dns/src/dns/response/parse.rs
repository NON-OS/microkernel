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
use super::record::{answer_from, read_record};
use crate::dns::{read_name as read, Name};
use crate::dns::{skip, Header, HDR_LEN, TYPE_A, TYPE_AAAA};

const TYPE_CNAME: u16 = 5;
const CLASS_IN: u16 = 1;

/// CNAME links followed from the question name before giving up.
const MAX_CHAIN: usize = 8;

/*
 * The answer is a record of the type the question asked for, owned by the
 * question's name or by the name a chain of CNAMEs from it leads to (RFC
 * 1034 3.6.2, 4.3.2). The first A or AAAA of any owner was taken: a record
 * for some other name, anywhere in the response, was the answer, and asked
 * for A a response that listed the name's AAAA first had its A missed. A
 * chain may be listed in any order, and a loop in it ends with no answer.
 * The answer's TTL is the least along the chain.
 */
pub fn first_address(message: &[u8]) -> Result<(Header, Option<Answer>), ParseError> {
    let header = Header::parse(message).ok_or(ParseError::Truncated)?;
    if !header.is_response() {
        return Err(ParseError::NotAResponse);
    }
    if header.qdcount == 0 {
        return Ok((header, None));
    }
    let (qname, at) = read(message, HDR_LEN)?;
    let qtype = message
        .get(at..at + 2)
        .map(|b| u16::from_be_bytes([b[0], b[1]]))
        .ok_or(ParseError::Truncated)?;
    let answers = skip_questions(message, at, header.qdcount)?;
    let mut current = qname;
    let mut ttl = u32::MAX;
    for _ in 0..=MAX_CHAIN {
        match find(message, answers, header.ancount, &current, qtype)? {
            Found::Answer(mut answer) => {
                answer.ttl = answer.ttl.min(ttl);
                return Ok((header, Some(answer)));
            }
            Found::Alias(target, link_ttl) => {
                current = target;
                ttl = ttl.min(link_ttl);
            }
            Found::Nothing => break,
        }
    }
    Ok((header, None))
}

// One per chain step, on the stack: boxing the inline Name would allocate per CNAME hop.
#[allow(clippy::large_enum_variant)]
enum Found {
    Answer(Answer),
    Alias(Name, u32),
    Nothing,
}

/// One pass over the answers for records owned by `owner`.
fn find(message: &[u8], mut pos: usize, count: u16, owner: &Name, qtype: u16) -> Result<Found, ParseError> {
    let mut alias = None;
    for _ in 0..count {
        let (name, at) = read(message, pos)?;
        let rr = read_record(message, at)?;
        pos = rr.next;
        if rr.class != CLASS_IN || name != *owner {
            continue;
        }
        if rr.rtype == qtype && (qtype == TYPE_A || qtype == TYPE_AAAA) {
            if let Some(answer) = answer_from(rr.rtype, rr.ttl, &message[rr.rdata..rr.next]) {
                return Ok(Found::Answer(answer));
            }
        }
        if rr.rtype == TYPE_CNAME && alias.is_none() && rr.rdlen > 0 {
            // The target may be compressed, pointing anywhere before it, but
            // what it holds inline lies within the record's data.
            if let Ok((target, end)) = read(message, rr.rdata) {
                if end <= rr.next {
                    alias = Some((target, rr.ttl));
                }
            }
        }
    }
    Ok(alias.map_or(Found::Nothing, |(target, ttl)| Found::Alias(target, ttl)))
}

/// Past the first question's type and class, then past any further questions.
fn skip_questions(message: &[u8], after_name: usize, qdcount: u16) -> Result<usize, ParseError> {
    let mut pos = after_name;
    for i in 0..qdcount {
        if i > 0 {
            pos = skip(message, pos)?;
        }
        if pos + 4 > message.len() {
            return Err(ParseError::Truncated);
        }
        pos += 4;
    }
    Ok(pos)
}
