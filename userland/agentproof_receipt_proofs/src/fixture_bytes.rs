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

//! The fixture as receipt bytes, written as the job writes them: the doc with
//! its unsigned count, flag and log cleared, the registry's entries, and the
//! capsules its log's PCR 14 events record.

use agentproof_receipt::napr::capsules::capsules_of;
use agentproof_receipt::napr::doc_clear::log_of;
use agentproof_receipt::napr::doc_form::receipt_doc;
use agentproof_receipt::napr::{challenge, write};

use crate::fixture::Fixture;

impl Fixture {
    pub fn bytes(&mut self) -> Vec<u8> {
        let rec = self.record.encode();
        let bound = write::bound(&rec, &self.box_word, &self.policy, &self.launcher);
        self.spec.challenge = Some(challenge(&bound));
        let kernel = self.spec.doc_bytes();
        let capsules = capsules_of(log_of(&kernel).unwrap()).unwrap();
        let doc = receipt_doc(&kernel).unwrap();
        write::finish(bound, &doc, &self.spec.entries.concat(), &capsules)
    }
}
