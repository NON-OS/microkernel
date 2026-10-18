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

//! Every way a bring-up attempt ends short of playing, and what it leaves.
//!
//! The attempt used to drop its broker handles with the CORB and RIRB
//! engines on: the handles unmapped the rings first, the broker scrubbed and
//! freed their frames, and only the release that came after switched bus
//! mastering off. A codec answering late was written by the RIRB engine into
//! a frame that already belonged to someone else. Each test here ends an
//! attempt a different way and reads the order off the broker log.

use crate::constants::{CORBCTL, CORBCTL_RUN, DPLBASE, RIRBCTL, RIRBCTL_DMAEN, SDCTL_RUN, VMAJ};
use crate::controller::verdict::Verdict;
use crate::error::HdaError;
use crate::proofs::fixtures::{alc236_hp, intel_hdmi, sim_codec};
use crate::proofs::lifecycle::attempt::{attempt, attempt_until};
use crate::proofs::lifecycle::board::{board, LOW};
use crate::proofs::lifecycle::given_back::quiesced_then_released;
use crate::proofs::lifecycle::teardown::sd_ctl;
use crate::setup::Started;

const HDMI_ONLY: u32 = Verdict::HdmiOnly as u32;

#[test]
fn an_hdmi_only_controller_is_stopped_and_reset_before_its_claim_goes_back() {
    let hdmi = || board(vec![(0, sim_codec(&intel_hdmi()))], &LOW);
    let hdmi_only = |r: &_, _: &_| matches!(r, Ok(Started::Silent(s)) if s.verdict == HDMI_ONLY);
    let (_b, _, log) = attempt_until(hdmi, hdmi_only);
    quiesced_then_released(&log);
}

#[test]
fn rings_refused_after_the_claim_still_stop_what_firmware_left_running() {
    let b = board(vec![(0, sim_codec(&alc236_hp()))], &[]);
    // A boot chime's leavings: both rings, the position buffer, an engine.
    b.sim.bar.present8(CORBCTL as usize, CORBCTL_RUN);
    b.sim.bar.present8(RIRBCTL as usize, RIRBCTL_DMAEN);
    b.sim.bar.present32(DPLBASE as usize, 0x0010_0001);
    b.sim.bar.present8(sd_ctl(4), SDCTL_RUN);
    let (r, log) = attempt(&b);
    assert_eq!(r.err(), Some(HdaError::BrokerCallFailed(-12)));
    quiesced_then_released(&log);
}

#[test]
fn a_controller_refused_after_its_reset_is_let_go_in_order() {
    let b = board(vec![(0, sim_codec(&alc236_hp()))], &LOW);
    b.sim.bar.present8(VMAJ as usize, 0);
    let (r, log) = attempt(&b);
    assert_eq!(r.err(), Some(HdaError::UnsupportedController));
    quiesced_then_released(&log);
}

#[test]
fn a_stream_refused_with_the_rings_running_stops_them_before_the_release() {
    let rings_only = || board(vec![(0, sim_codec(&alc236_hp()))], &LOW[..2]);
    let (b, _, log) =
        attempt_until(rings_only, |r, _| matches!(r, Err(HdaError::BrokerCallFailed(-12))));
    assert!(!b.sim.sent().is_empty(), "the codec was never asked anything over the rings");
    quiesced_then_released(&log);
}
