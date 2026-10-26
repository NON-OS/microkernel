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

//! Every answer the job sends goes by its request id, and what the kernel
//! returns for it reaches the job (job_told_tests.rs: what it then means).

/// Both places the job answers from pass the kernel's return on.
#[test]
fn every_answer_passes_its_return_on() {
    let frame = include_str!("../../../capsule_prove/src/job/serve_frame.rs");
    let drain = include_str!("../../../capsule_prove/src/job/drain.rs");
    assert!(
        frame.contains("let rc = mk_ipc_reply_to(m.request,") && frame.contains("j.replied(rc);")
    );
    assert!(drain.contains("job.replied(mk_ipc_reply_to(m.request,"));
    assert!(![frame, drain].iter().any(|t| t.contains("mk_ipc_reply(")), "by pid");
}
