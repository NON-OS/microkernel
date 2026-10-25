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

//! `verify-signature`'s flags, all three required.

use super::flags::Given;
use super::VerifyArgs;

const FLAGS: &[&str] = &["--verdict", "--signature", "--public-key"];

pub fn verify(args: &[String]) -> Result<VerifyArgs, String> {
    let given = Given::read(args, FLAGS)?;
    let need = |name: &str| given.get(name).ok_or(format!("`{name}` is required"));
    let (verdict, signature) = (need("--verdict")?.into(), need("--signature")?.into());
    Ok(VerifyArgs { verdict, signature, public_key: need("--public-key")?.to_string() })
}
