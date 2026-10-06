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

//! An EK certificate the way a manufacturer provisions one. swtpm_setup
//! derives the EKs from the profile's templates and has swtpm_localca certify
//! them into their NV indices, with a CA of its own made in the test's
//! directory rather than the system's.

use super::dir::Dir;
use super::tools::tool;

/// TPM state carrying an RSA 2048 EK certificate at `0x01C00002`, and the
/// directory swtpm_setup wrote its own copy of the certificate to. `None`,
/// said aloud, when swtpm_setup is not installed.
pub fn provisioned(test: &str) -> Option<(Dir, Dir)> {
    let (Some(_), Some(localca)) = (on_path("swtpm_setup"), on_path("swtpm_localca")) else {
        eprintln!("{test}: swtpm_setup not installed, certificate test skipped");
        return None;
    };
    let (state, work) = (Dir::new(), Dir::new());
    let ca = work.file("ca");
    std::fs::create_dir_all(&ca).expect("ca dir");
    let conf = format!(
        "statedir = {ca}\nsigningkey = {ca}/signkey.pem\nissuercert = {ca}/issuercert.pem\n\
         certserial = {ca}/certserial\n"
    );
    std::fs::write(work.file("localca.conf"), conf).expect("localca conf");
    let opts = "--platform-manufacturer NONOS\n--platform-version 2.1\n--platform-model test\n";
    std::fs::write(work.file("localca.options"), opts).expect("localca options");
    let setup = format!(
        "create_certs_tool = {localca}\ncreate_certs_tool_config = {}\n\
         create_certs_tool_options = {}\nactive_pcr_banks = sha256\n",
        work.file("localca.conf"),
        work.file("localca.options"),
    );
    std::fs::write(work.file("setup.conf"), setup).expect("setup conf");
    let s = state.path().to_str().expect("utf-8 path");
    let (conf, out) = (work.file("setup.conf"), work.file(""));
    let args = ["--tpm2", "--tpmstate", s, "--create-ek-cert", "--config", &conf, "--overwrite"];
    tool(None, "swtpm_setup", &[&args[..], &["--write-ek-cert-files", &out]].concat());
    Some((state, work))
}

fn on_path(name: &str) -> Option<String> {
    let path = std::env::var_os("PATH")?;
    let found = std::env::split_paths(&path).map(|d| d.join(name)).find(|f| f.is_file())?;
    Some(found.display().to_string())
}
