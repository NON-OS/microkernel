// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "embed-trailer",
    about = "Embed the kernel self-attestation path trailer into a signed NONOS kernel"
)]
pub struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,

    /// The trailer `nonos-stark-enroll kernel` wrote for this kernel. It is
    /// carried verbatim as the image's proof region.
    #[arg(long, value_name = "FILE")]
    pub proof_file: PathBuf,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Accept a path-only trailer, for a development image whose loader is
    /// built with dev-attest. A release never passes it.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub path_only: bool,
}
