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

//! What the check found, kept for the rest of the boot to read.

use nonos_boot_measure::gate::Admitted;
use spin::Once;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The TPM's log replays to PCR 4 and the loader it names is enrolled
    /// under a signed root at or above the rollback floor.
    Measured(Admitted),
    /// No TPM or no log: the loader file the loader handed over is enrolled,
    /// but nothing measured it, so this is the loader's word only.
    SelfReported(Admitted),
    /// The evidence was there and failed; the code is `BootError::code`.
    Refused(u32),
    /// No boot-root record or no loader trailer reached the kernel.
    NoEvidence,
}

static VERDICT: Once<Verdict> = Once::new();

/// The verdict, once the check has run.
pub fn verdict() -> Option<Verdict> {
    VERDICT.get().copied()
}

pub(super) fn keep(v: Verdict) -> Verdict {
    *VERDICT.call_once(|| v)
}
