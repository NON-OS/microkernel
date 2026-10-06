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

//! Which statements leave the plain path: anything piped or redirected, and
//! of those, the pipelines with a long-running stage that become a job.

use alloc::vec::Vec;

use crate::command::dispatch::split_stages;

pub(super) fn is_plain(args: &[&[u8]]) -> bool {
    !args.iter().any(|a| matches!(*a, b"|" | b">" | b">>" | b"<"))
}

// A pure pipeline (no redirects) whose stages include a long command
// becomes owned, parsed stages for a `PipelineStages` job. Anything with a
// redirect, or a pipeline with no long stage, returns `None` so the caller
// falls back to `Verdict::Instant`.
pub(super) fn pipeline_stages(args: &[&[u8]]) -> Option<Vec<Vec<Vec<u8>>>> {
    if args.iter().any(|a| matches!(*a, b">" | b">>" | b"<")) {
        return None;
    }
    if !args.iter().any(|a| *a == b"|") {
        return None;
    }
    let segments = split_stages(args);
    if !segments.iter().any(|seg| is_long_stage(seg)) {
        return None;
    }
    let mut stages: Vec<Vec<Vec<u8>>> = Vec::new();
    for seg in &segments {
        let tokens: Vec<Vec<u8>> = seg.iter().map(|tok| tok.to_vec()).collect();
        stages.push(tokens);
    }
    Some(stages)
}

fn is_long_stage(seg: &[&[u8]]) -> bool {
    matches!(seg.first().copied(), Some(b"ping") | Some(b"install"))
}
