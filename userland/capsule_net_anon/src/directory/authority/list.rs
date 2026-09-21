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

//! The seven authority addresses, copied from the fork's own configuration.

use super::hex::hex;
use super::types::Authority;

/*
 * The seven authorities, read out of the fork's src/app/config/auth_dirs.inc.
 *
 * There is no fallback mirror list to go with them. The fork's
 * fallback_dirs.inc holds a comment and nothing else, so these addresses are
 * the only way into the network and every one of them has to be tried before
 * concluding the network is unreachable.
 */
pub const AUTHORITIES: &[Authority] = &[
    Authority {
        address: [49, 13, 145, 234],
        dir_port: 9230,
        v3ident: hex(b"88F29CB5FE86A688E31990A3B20BD562D0C089E1"),
    },
    Authority {
        address: [5, 161, 108, 187],
        dir_port: 9230,
        v3ident: hex(b"9572A9D4141A2AFC43C416E978D805E7C98B3B25"),
    },
    Authority {
        address: [5, 78, 90, 106],
        dir_port: 9230,
        v3ident: hex(b"FFBFD3D2E92EEAA9162335FF8DF259CC62713784"),
    },
    Authority {
        address: [5, 161, 228, 187],
        dir_port: 9230,
        v3ident: hex(b"6CE85CF74AB78E4D350E0418234B97F47AB32A20"),
    },
    Authority {
        address: [5, 78, 94, 15],
        dir_port: 9230,
        v3ident: hex(b"39C78145CFDF464E624626D4F78A315387132082"),
    },
    Authority {
        address: [95, 216, 32, 105],
        dir_port: 9230,
        v3ident: hex(b"5F18C895685A4207E0778FEB2A9CE4C90DABE7A6"),
    },
    Authority {
        address: [176, 9, 29, 53],
        dir_port: 9230,
        v3ident: hex(b"271F7D1592BF37AEB67BF48164928720EF9D0648"),
    },
];
