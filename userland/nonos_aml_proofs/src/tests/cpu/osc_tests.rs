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

//! Processor _OSC: asked before the tables are read, with the capability
//! word the caller gives, and firmware's answer kept.

use super::osc_rig::machine;
use crate::consts::at;
use crate::cpu::{cpu_tables, processor_osc, ProcessorOsc, PROCESSOR_UUID};
use crate::platform::OscError;
use crate::probe::Probe;
use crate::run::{boot, call, num};
use crate::tests::platform::run::evaluator;

#[test]
fn asked_with_the_word_given_then_the_tables_appear() {
    let (mut aml, _) = boot(&machine(PROCESSOR_UUID[0]), 2);
    let mut probe = Probe::default();
    let cpu0 = at(&aml, "\\_PR.CPU0").unwrap();
    assert!(cpu_tables(&mut aml, &mut evaluator(&mut probe), cpu0).pstates.is_empty());
    let got = processor_osc(&mut aml, &mut evaluator(&mut probe), cpu0, 0);
    assert_eq!(got, ProcessorOsc::Granted(0));
    assert_eq!(num(&call(&mut aml, "\\CAP", vec![]).unwrap()), 0, "nothing claimed");
    assert_eq!(cpu_tables(&mut aml, &mut evaluator(&mut probe), cpu0).pstates.len(), 1);
    let cpu1 = at(&aml, "\\_PR.CPU1").unwrap();
    assert_eq!(processor_osc(&mut aml, &mut evaluator(&mut probe), cpu1, 0), ProcessorOsc::Absent);
}

#[test]
fn another_uuid_is_refused() {
    let (mut aml, _) = boot(&machine(0x00), 2);
    let mut probe = Probe::default();
    let cpu0 = at(&aml, "\\_PR.CPU0").unwrap();
    let got = processor_osc(&mut aml, &mut evaluator(&mut probe), cpu0, 0);
    assert_eq!(got, ProcessorOsc::Refused(OscError::UnrecognizedUuid));
}
