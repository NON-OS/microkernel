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

//! The record as the host keeps it: handed over as it grows, checkpointed,
//! held to its cap with room kept for its end, and signed by the host keys
//! or, if they fail, left to its checkpoints.

use super::good::{check, received};
use crate::kit::host::{config, Kernel, Keys};
use crate::kit::roles::Table;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::run::Run;
use nsh_client::Verdict;
use nsh_server::{Config, EndReason, Event as S, MIN_RECORD};
use nsh_wire::record::FOOTER_LEN;

pub(super) fn run_with(cfg: Config, keys: Keys) -> Run {
    let mut run = Run::with(&GrantB::default(), cfg, Kernel::honest(), keys, Table::shipped());
    run.pump();
    run
}

/// Fill the record with one-byte commands until the shell is stopped for it.
pub(super) fn filled() -> Run {
    let mut run = run_with(Config { max_record: MIN_RECORD, ..config() }, Keys::default());
    while !run.s_events.contains(&S::Stop(EndReason::Host)) {
        run.host(|s, now, r| s.command(b"x", now, r)).expect("command");
    }
    run
}

#[test]
fn checkpoints_cover_the_record_as_it_grows() {
    let mut run = run_with(Config { ckpt_every: 2, ..config() }, Keys::default());
    for n in 0..5u8 {
        run.host(|s, now, r| s.command(&[b'a' + n], now, r)).expect("command");
    }
    run.host(|s, now, r| s.exit(0, now, r)).expect("exit");
    let ckpts = run.checkpoints();
    assert_eq!(ckpts.len(), 3, "after entries 2, 4 and 6");
    let refs: Vec<&[u8]> = ckpts.iter().map(Vec::as_slice).collect();
    let rec = received(&run);
    assert!(matches!(
        check(&run, &rec, &refs).expect("checks").verdict,
        Verdict::Complete { entries: 6, .. }
    ));
    let cut = &rec[..rec.len() - FOOTER_LEN - 4];
    let got = check(&run, cut, &refs[..2]).expect("a record cut short checks to its checkpoint");
    assert_eq!(got.verdict, Verdict::CutShort { seq: 3, boxed: false });
}
