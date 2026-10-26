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
//! One bring-up on the modelled PCH, read for the switch-over alone: the
//! routing writes tried, whether a switch was kept, and what was said.

use nonos_libc::{take_log, take_said, Call};

use super::bringup_device::Once;
use super::intel_ports_model::{bring_up, routing};
use crate::discover::Found;

/// `dev` brought up; whether it kept a switch to hand back, and the
/// routing writes it tried.
pub fn up(dev: Found) -> (bool, Vec<Call>) {
    let (_hc, got) = bring_up(dev, Once::AnswersNoop(1));
    let driver = got.unwrap_or_else(|e| panic!("bring-up: {e:?}"));
    let calls = routing(&take_log()).iter().map(|m| m.0).collect();
    (driver.ports.is_some(), calls)
}

/// The one switch line said, and that it said the switch did not take.
pub fn said_not_taken() {
    let said: Vec<String> = take_said().into_iter().filter(|l| l.contains("port switch")).collect();
    assert_eq!(said.len(), 1, "said once: {said:?}");
    assert!(said[0].contains("refused or not taken; carrying on"), "{said:?}");
}
