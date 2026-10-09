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

/*
 * `qwen get TIER...` and `qwen tiers`: the words that ask for tiers to be
 * downloaded or listed rather than a question. Only as the first word after
 * `qwen`; `qwen small get ...` still asks a question that starts with "get".
 */

use alloc::vec::Vec;

use super::ask::Ask;
use crate::term::util::is_space;

/* The words as typed, and as the model fetcher takes them first in its argv. */
pub const GET: &[u8] = b"get";
pub const LIST: &[u8] = b"tiers";
/*
 * `qwen get --direct TIER...`: the person asks for a direct download for
 * this run only, passed to the fetcher as typed; it says the mirror then
 * sees this machine's address. Nothing else ever leaves directly.
 */
pub const DIRECT: &[u8] = b"--direct";

#[derive(Debug, PartialEq, Eq)]
pub enum Fetch<'a> {
    /* `qwen get` and the words after it, each to be a tier. */
    Get(Vec<&'a [u8]>),
    /* `qwen tiers`. */
    List,
}

/* `ask` as a fetch request, or `None` when it is not one. */
pub fn parse<'a>(ask: &Ask<'a>) -> Option<Fetch<'a>> {
    if ask.tier.is_some() {
        return None;
    }
    let mut words = ask.question.split(|&b| is_space(b)).filter(|w| !w.is_empty());
    match words.next()? {
        w if w == GET => Some(Fetch::Get(words.collect())),
        w if w == LIST && words.next().is_none() => Some(Fetch::List),
        _ => None,
    }
}

/* The argv the fetcher takes: the verb, then each tier, NUL-separated. */
pub fn request(fetch: &Fetch<'_>) -> Vec<u8> {
    match fetch {
        Fetch::List => LIST.to_vec(),
        Fetch::Get(tiers) => {
            let mut argv = GET.to_vec();
            for tier in tiers {
                argv.push(0);
                argv.extend_from_slice(tier);
            }
            argv
        }
    }
}
