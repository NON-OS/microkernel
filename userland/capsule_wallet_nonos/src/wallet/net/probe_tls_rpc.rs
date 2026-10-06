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

/// How far the TLS handshake with the RPC host got, flag by flag, as
/// `step::checks` takes them for the diagnostic.
#[derive(Clone, Copy)]
pub struct TlsProbe {
    pub server_hello: bool,
    pub encrypted_record: bool,
    pub certificate: bool,
    pub chain: bool,
    pub anchor: bool,
    pub signature: bool,
    pub hostname: bool,
    pub validity: bool,
    pub finished: bool,
    pub client_finished: bool,
    pub chain_id: bool,
}

impl TlsProbe {
    /// Every check held: the chain, its root, its signatures, the host
    /// name, the dates, the handshake signature and Finished.
    pub fn mark_trusted(&mut self) {
        self.chain = true;
        self.anchor = true;
        self.signature = true;
        self.hostname = true;
        self.validity = true;
        self.finished = true;
        self.client_finished = true;
    }

    /// The flight was refused for `problem`. The chain is read in the order
    /// the walk checks it (the leaf's name, then every date, then the root),
    /// so a later finding says the earlier checks held; a signature that
    /// failed is found by none of them.
    pub fn mark_refused(&mut self, problem: Option<nonos_tls::CertProblem>) {
        use nonos_tls::CertProblem::{
            Expired, NameMismatch, NotYetValid, UnknownIssuer, Unreadable,
        };
        let (hostname, validity, anchor) = match problem {
            Some(NameMismatch | Unreadable) => (false, false, false),
            Some(Expired | NotYetValid) => (true, false, false),
            Some(UnknownIssuer) => (true, true, false),
            None => (true, true, true),
        };
        self.chain = problem != Some(Unreadable);
        self.hostname = hostname;
        self.validity = validity;
        self.anchor = anchor;
    }

    pub fn blocked() -> Self {
        Self {
            server_hello: false,
            encrypted_record: false,
            certificate: false,
            chain: false,
            anchor: false,
            signature: false,
            hostname: false,
            validity: false,
            finished: false,
            client_finished: false,
            chain_id: false,
        }
    }
}
