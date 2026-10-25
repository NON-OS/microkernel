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

//! The patience of each network, one number for a route and for a proxy,
//! so a TLS handshake over either waits the same.

use crate::pick::Route;
use crate::refusal::Proxy;

#[test]
fn each_network_waits_its_own_patience() {
    assert_eq!((Proxy::Nym.patience_ms(), Route::Nym(4100).patience_ms()), (60_000, 60_000));
    for anyone in [Proxy::Anyone, Proxy::AnyoneService, Proxy::AnyoneName] {
        assert_eq!(anyone.patience_ms(), 30_000);
    }
    assert_eq!(Route::Anon(4200).patience_ms(), 30_000);
    assert_eq!(Route::Direct.patience_ms(), 0, "a socket waits on the wire");
}
