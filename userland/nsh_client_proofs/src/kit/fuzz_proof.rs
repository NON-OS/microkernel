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

//! A `HOST_PROOF` built from fuzzed fields and properly signed, so the
//! fuzzer reaches every host check, with its `Oracle` beside it.

use super::attest::doc_with;
use super::enrol::{EnrolSpec, SigMode};
use super::fuzz_oracle::{mode, Oracle};
use super::proof::ProofSpec;
use super::server::Server;
use super::{keys, AK, HOST, NOT_AFTER, NOW, PCRS};

/// The proof `b` describes for the server's transcript as it stands.
pub fn fuzzed(b: &[u8], s: &Server) -> (ProofSpec, Oracle) {
    let at = |i: usize| b.get(i).copied().unwrap_or(0);
    let (flags, stranger) = (at(0), keys::stranger());
    let host = if flags & 0x80 != 0 { stranger } else { keys::host() };
    let signer = if flags & 0x10 != 0 { stranger } else { keys::host() };
    let (e_sig, h_sig) = (mode(flags >> 2), mode(flags >> 5));
    let mut e = EnrolSpec::good();
    (e.sig, e.fleet[0], e.ak[0], e.host_keys[0]) =
        (e_sig, 0xF1 ^ at(1), AK[0] ^ at(2), e.host_keys[0] ^ at(3));
    e.authority_id = if flags & 1 != 0 { stranger.id() } else { e.authority_id };
    e.signer = if flags & 2 != 0 { stranger } else { e.signer };
    let (ep, ex, dp, dx) = (usize::from(at(4) & 3), at(5), usize::from(at(11) & 3), at(12));
    e.pcrs[ep][0] ^= ex;
    e.not_after = if at(6) == 0 { NOT_AFTER } else { NOW + u64::from(at(6)) - 128 };
    e.host = [HOST, b"db-2.eu.example", b"HOST"][usize::from(at(7) % 3)].to_vec();
    let mut challenge = ProofSpec { host, ..ProofSpec::good() }.challenge(s);
    challenge[usize::from(at(9) & 31)] ^= at(8);
    let (mut ak, mut pcrs) = (AK, PCRS);
    ak[0] ^= at(10);
    pcrs[dp][0] ^= dx;
    let mut doc = doc_with(&challenge, &ak, &pcrs);
    match at(13) & 3 {
        0 => {}
        1 => {
            doc.pop();
        }
        2 => doc.push(0),
        _ => doc[0] = b'X',
    }
    let entries = vec![7; usize::from(at(14).wrapping_add(2) & 3) * 45 + usize::from(at(15) & 1)];
    let o = Oracle {
        parses: at(7) % 3 != 2 && !entries.is_empty() && entries.len().is_multiple_of(45),
        host_sig: (flags & 0x10 != 0) == (flags & 0x80 != 0) && h_sig == SigMode::Good,
        attester: at(13) & 3 == 0 && at(8) == 0,
        enrol: flags & 3 == 0
            && e_sig == SigMode::Good
            && at(1) == 0
            && at(2) == at(10)
            && e.host_keys == host.digest()
            && at(7) % 3 == 0
            && e.not_after > NOW,
        pcrs: (ex == 0 && dx == 0) || (ep == dp && ex == dx),
    };
    let spec = ProofSpec { host, signer, sig: h_sig, enrol: e, doc: Some(doc), entries };
    (spec, o)
}
