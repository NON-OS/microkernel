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

//! What the shell knows of the verifier the keyring holds, and the blob it
//! keeps on disk for it. The shell is the one writer of that file: each
//! blob the keyring hands back is written once, and a blob read from disk
//! is already kept.

use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Known {
    /// Not asked yet, or the keyring has not answered.
    #[default]
    Unknown,
    /// No passphrase is set: locking is not offered.
    Absent,
    Set,
}

#[derive(Default)]
pub struct Verifier {
    known: Known,
    blob: Vec<u8>,
    kept: bool,
    /// This boot asked the person to choose one already.
    asked: bool,
}

impl Verifier {
    pub fn known(&self) -> Known {
        self.known
    }

    /// The keyring's standing: whether it holds a record, and its blob.
    pub fn standing(&mut self, set: bool, blob: &[u8]) {
        self.known = if set { Known::Set } else { Known::Absent };
        if set {
            self.took(blob);
        }
    }

    /// A blob the keyring handed back for the record it holds now.
    pub fn took(&mut self, blob: &[u8]) {
        self.known = Known::Set;
        if blob != self.blob.as_slice() {
            self.blob = blob.to_vec();
            self.kept = false;
        }
    }

    /// The blob read back from disk, which the keyring took.
    pub fn loaded(&mut self, blob: &[u8]) {
        self.known = Known::Set;
        self.blob = blob.to_vec();
        self.kept = true;
    }

    /// The blob the shell holds, to hand a keyring that lost its record.
    pub fn blob(&self) -> &[u8] {
        &self.blob
    }

    /// The blob to write, when one is not on disk yet. An empty blob (the
    /// record could not be sealed) is never written.
    pub fn due_write(&self) -> Option<&[u8]> {
        (!self.kept && !self.blob.is_empty()).then_some(self.blob.as_slice())
    }

    pub fn written(&mut self) {
        self.kept = true;
    }

    /// Whether to ask the person to choose a passphrase now: none is set
    /// and this boot has not asked.
    pub fn ask_due(&self) -> bool {
        self.known == Known::Absent && !self.asked
    }

    pub fn asked(&mut self) {
        self.asked = true;
    }
}
