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

#[derive(Clone, Copy)]
pub struct NetStatus {
    pub rpc_connect_ok: bool,
    pub tls_chain_ok: bool,
    pub rpc_chain_ok: bool,
    /// The RPC host whose chain id `rpc_chain_ok` holds for.
    pub rpc_host: &'static str,
    /// The network the last check went over, None before the first.
    pub route: Option<super::WalletRoute>,
    pub status: &'static [u8],
}
