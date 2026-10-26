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

//! `agentproof-receipt pin <receipt>`: the trust file a receipt verifies
//! under, pinned from that receipt on first use (verify/pin.rs), written as
//! trust_file.rs reads it and saying in words what it does not prove. Exit
//! 0 with the file on stdout; 1 when the receipt is refused, by name.

use alloc::format;
use alloc::string::String;
use alloc::vec;

use crate::cli::{hex, Answer};
use crate::trust::Trust;
use crate::verify::pin;

pub fn pinned(receipt: &[u8]) -> Answer {
    match pin(receipt) {
        Ok(t) => Answer { code: 0, out: vec![text(&t)], err: vec![] },
        Err(r) => Answer {
            code: 1,
            out: vec![],
            err: vec![format!("agentproof-receipt: refused: {} ({r:?})", r.name())],
        },
    }
}

pub fn text(t: &Trust) -> String {
    let mut s = String::from("nonos-receipt-trust 1\n");
    s += "# Pinned on first use from one receipt, which verified under it. Nothing\n";
    s += "# here says the TPM's maker, or anyone, vouched for this AK, release or\n";
    s += "# supervisor: check them before you rely on this file. No EK chain.\n";
    for r in &t.releases {
        s +=
            &format!("release {} {} {} {}\n", r.version, hex(&r.pcr8), hex(&r.pcr9), hex(&r.pcr14));
    }
    for v in &t.supervisors {
        s += &format!("supervisor {} {:x}\n", hex(&v.meas), v.caps);
    }
    s + &format!("ak {}", hex(&t.ak))
}
