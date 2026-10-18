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

#[path = "build/model_weights.rs"]
mod model_weights;
#[path = "build/sha.rs"]
mod sha;

use model_weights::model_weights;
use sha::resolve_sha;

fn main() {
    let sha = resolve_sha();
    println!("cargo:rustc-env=ABOUT_GIT_SHA={sha}");
    model_weights();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../../LICENSE");
    println!("cargo:rerun-if-env-changed=NONOS_BUILD_SHA");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
}
