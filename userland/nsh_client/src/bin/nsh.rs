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

//! `nsh`: connect to an attested NØNOS host and log in (`connect`).

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let keys = nsh_client::connect::keys(std::io::stdin());
    let status = nsh_client::connect::run(&args, keys, &mut std::io::stdout());
    std::process::exit(status);
}
