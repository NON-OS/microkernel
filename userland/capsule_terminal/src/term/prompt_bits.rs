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

//! What the prompt says after the path: the git branch of the work tree and
//! how many jobs run in the background, each only when it says something.

use alloc::vec::Vec;

/// How many directories up from the working one a `.git` is looked for.
pub const UP: usize = 4;

/// The branch `.git/HEAD` names: `ref: refs/heads/<name>` gives the name; a
/// detached HEAD, a bare commit id, gives its first seven characters.
pub fn branch(head: &[u8]) -> Option<&[u8]> {
    let head = head.strip_suffix(b"\n").unwrap_or(head);
    if let Some(name) = head.strip_prefix(b"ref: refs/heads/") {
        return (!name.is_empty() && name.iter().all(|b| b.is_ascii_graphic())).then_some(name);
    }
    let hex = head.len() == 40 && head.iter().all(u8::is_ascii_hexdigit);
    hex.then(|| &head[..7])
}

/// The `.git/HEAD` paths to try, from the working directory up `UP` levels.
pub fn heads(cwd: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut dir = cwd.strip_suffix(b"/").unwrap_or(cwd);
    for _ in 0..=UP {
        let mut p = dir.to_vec();
        p.extend_from_slice(b"/.git/HEAD");
        out.push(p);
        match dir.iter().rposition(|&b| b == b'/') {
            Some(0) | None => break,
            Some(i) => dir = &dir[..i],
        }
    }
    out
}

/// `1 job`, `3 jobs`; empty for none.
pub fn jobs_label(n: usize, buf: &mut [u8; 12]) -> &[u8] {
    if n == 0 {
        return &[];
    }
    let d = crate::term::util::format_u64(n.min(9999) as u64, &mut buf[..4]);
    let word: &[u8] = if n == 1 { b" job" } else { b" jobs" };
    buf[d..d + word.len()].copy_from_slice(word);
    &buf[..d + word.len()]
}

/// Which of the branch and the job count fit in `room` cells beside a path
/// of `path` cells. The job count goes first, then the branch: the path is
/// where the reader is, and gives way to neither.
pub fn fit(room: usize, path: usize, branch: usize, jobs: usize) -> (bool, bool) {
    let seg = |n: usize| if n == 0 { 0 } else { n + 1 };
    let left = room.saturating_sub(path);
    let both = seg(branch) + seg(jobs) <= left;
    let b = branch > 0 && seg(branch) <= left;
    (b, jobs > 0 && (both || (!b && seg(jobs) <= left)))
}
