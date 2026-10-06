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

//! The tier words `qwen` takes, as the kernel's allowlist has them.

use alloc::format;
use alloc::string::String;

use super::default_tier::Source;

/// The tiers the kernel runs, by family and smallest first within each.
/// With none named, the default runs (`default_tier`).
pub const TIERS: &[&[u8]] = &[
    b"small",
    b"medium",
    b"large",
    b"xlarge",
    b"xxl",
    b"max",
    b"qwen3-0.6b",
    b"qwen3-1.7b",
    b"qwen3-4b",
    b"qwen3-8b",
    b"qwen3-14b",
    b"qwen3-30b-a3b",
    b"qwen3-32b",
    b"coder-1.5b",
    b"coder-7b",
    b"coder-14b",
    b"coder-32b",
];

/* Said when the policy service could not be asked which tier was chosen. */
pub const UNANSWERED: &[u8] = b"qwen: the policy service did not answer, so the tier chosen at \
    setup or in Settings is not known here";
/* Said when the tier policy names is not one this terminal runs. */
pub const UNKNOWN: &[u8] =
    b"qwen: the tier chosen at setup or in Settings is not one this Terminal runs";

/*
 * The tier the person chose, from a policy reply (None: the service did not
 * answer), or None when there is none to run, with a line to say when the
 * reply could not be read as a choice. Then the default runs
 * (`default_tier`), and says so; nothing chosen needs no line of its own.
 */
pub fn pick_said(reply: Option<&[u8]>) -> (Option<&'static [u8]>, Option<&'static [u8]>) {
    let Some(reply) = reply else { return (None, Some(UNANSWERED)) };
    let word = |c: &u8| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'.' || *c == b'-';
    let want = &reply[..reply.iter().position(|c| !word(c)).unwrap_or(reply.len())];
    if let Some(t) = TIERS.iter().copied().find(|t| *t == want) {
        return (Some(t), None);
    }
    let unread = reply.first().is_some_and(|&c| c != 0);
    (None, unread.then_some(UNKNOWN))
}

/* The line that says a default runs, and why; None for the person's choice. */
pub fn default_line(tier: &str, source: Source) -> Option<String> {
    let why = match source {
        Source::Chosen => return None,
        Source::Stick => "the tier on the release stick, which installs offline",
        Source::Largest => "the largest tier this machine's memory runs",
        Source::Smallest => "the smallest tier, though none fits this machine's memory",
    };
    Some(format!("qwen: no tier chosen at setup or in Settings; running the default, {tier}: {why}"))
}
