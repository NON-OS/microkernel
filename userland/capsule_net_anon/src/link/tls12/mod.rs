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


//! A TLS 1.2 client for relays that do not speak TLS 1.3, and nothing else.
//!
//! The shared nonos_tls stays TLS 1.3 only, with no downgrade path in it.
//! This module lives in net.anon, which alone links it, and is reached only
//! after a relay has refused a TLS 1.3 hello outright.
//!
//! What it accepts is the narrowest TLS 1.2 a Tor-lineage relay answers:
//! ECDHE over P-256, signed with the relay's RSA link key, and an AEAD
//! record layer, ChaCha20-Poly1305 or AES-256-GCM, both sealed by the
//! kernel's AEAD call. There is no RSA key exchange, no CBC, no compression,
//! no session tickets or resumption and no renegotiation: none of them is
//! offered, a server that picks one anyway is refused, and a HelloRequest
//! after the handshake ends the link.
//!
//! The relay's certificate is not checked against any authority. As with
//! TLS 1.3, the link layer binds it through the CERTS cell: the relay's
//! identity key signs, through its signing key, the SHA-256 of exactly the
//! certificate this handshake received, and the ServerKeyExchange signature
//! proves the relay holds that certificate's key for this handshake's
//! randoms.
//!
//! A TLS 1.3 server that ends up negotiating TLS 1.2 marks the last eight
//! bytes of its random as RFC 8446 4.1.3 requires. This client only gets
//! here after asking for TLS 1.3 and being refused, so that mark means
//! something in between forged the refusal, and the handshake is refused.

mod alert;
mod constants;
mod error;
mod flight;
mod gather;
mod handshake;
pub mod hello;
pub mod keys;
mod messages;
pub mod prf;
pub mod record;
pub mod server;
pub mod spki;
mod stream;

pub use error::Tls12Error;
pub use handshake::connect;
pub use stream::Tls12Stream;
