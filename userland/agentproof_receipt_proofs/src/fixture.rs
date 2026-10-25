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
//! One boxed run's receipt, whole: a finished record by a trusted
//! supervisor whose slot 0 binds this app to this box, and a v5 document
//! signed by the fixture's AK for this receipt's challenge, in the form a
//! receipt carries it (its unsigned count and log cleared).

use agentproof_policy::boxed::hash::box_hash;
use agentproof_policy::boxed::preset::parse;
use agentproof_receipt::napr::doc_form::receipt_doc;
use agentproof_receipt::napr::{challenge, write};
use agentproof_receipt::seal_slot::slot0;
use nonos_linux_policy::{app_id, policy_hash, unlisted};
use nonos_proof_abi::{Record, FLAG_FINAL};

use crate::build::Spec;

pub const CAPS: u64 = 0x1_0400_1979;

pub struct Fixture {
    pub record: Record,
    pub box_word: Vec<u8>,
    pub policy: [u8; 256],
    pub launcher: [u8; 32],
    pub spec: Spec,
}

impl Fixture {
    pub fn new(box_word: &[u8]) -> Self {
        let policy = unlisted([7; 32]).encode();
        let app = app_id(&[7; 32], &policy_hash(&policy));
        let hash = box_hash(&parse(box_word).unwrap());
        let record = Record {
            flags: FLAG_FINAL,
            sup_meas: [5; 32],
            sup_caps: CAPS,
            authority: 0,
            seal_no: 9,
            trace: [1; 32],
            rows: 3,
            hc: [1, 2, 3, 4],
            meas: [6; 32],
            n_meas: 1,
            app_id: slot0(&app, Some(&hash)).unwrap(),
        };
        Self {
            record,
            box_word: box_word.to_vec(),
            policy,
            launcher: [0; 32],
            spec: Spec::default(),
        }
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        let rec = self.record.encode();
        let bound = write::bound(&rec, &self.box_word, &self.policy, &self.launcher);
        self.spec.challenge = Some(challenge(&bound));
        write::finish(bound, &receipt_doc(&self.spec.doc_bytes()).unwrap())
    }
}
