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
//! The core against a chip that lies at its receive boundary: a join goes
//! through the junk between honest frames, a session keeps its link and
//! still delivers the AP's data, and a chip that floods cannot hold one
//! serve pass past its bound, nor a join past its time.

use super::rig::{connect, station};

const SEEDS: [u64; 4] = [0x9e37_79b9_7f4a_7c15, 0x2545_f491_4f6c_dd1d, 7, 0xdead_beef];
/// A serve pass drains a session's frames and a scan's or a look's.
const PASS_MAX: u64 = 32 + 16 + 16;

#[test]
fn a_join_and_its_session_hold_through_a_lying_chip() {
    for seed in SEEDS {
        let mut st = station(seed, false);
        assert_eq!(connect(&mut st), 0, "seed {seed:#x}");
        let before = st.stats().rx_eth;
        let link = st.session().map(|s| (s.mac(), s.bssid));
        (0..400).for_each(|_| st.tick());
        assert_eq!(st.session().map(|s| (s.mac(), s.bssid)), link, "the link held, as it was");
        assert!(st.stats().rx_eth > before, "and the AP's data still came");
    }
}

#[test]
fn a_flooding_chip_holds_no_pass_past_its_bound_and_no_join_past_its_time() {
    for seed in SEEDS {
        let mut st = station(seed, false);
        assert_eq!(connect(&mut st), 0);
        st.radio_mut().unwrap().flood = true;
        for _ in 0..50 {
            let before = st.radio().unwrap().calls;
            st.tick();
            assert!(st.radio().unwrap().calls - before <= PASS_MAX, "one pass");
        }
        let mut st = station(seed, true);
        assert_ne!(connect(&mut st), 0, "nothing but junk joins nothing");
        for _ in 0..50 {
            let before = st.radio().unwrap().calls;
            st.tick();
            assert!(st.radio().unwrap().calls - before <= PASS_MAX, "a scanning pass");
        }
    }
}
