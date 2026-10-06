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

//! The request at the window's start, and the registry once the person
//! agrees. The transcript and the registry it rebuilds are dropped when this
//! step returns, before the prover needs the memory.

use alloc::string::String;
use alloc::vec::Vec;

use super::state::{Outcome, Step, Work};
use super::text::ascii;
use super::{errno, files, tpm};
use crate::assemble::{enrolled, parse_request, Request, REQUEST_MAX, TRANSCRIPT_MAX};

/// The request, or why there is none to show.
pub fn request() -> Result<Request, String> {
    let bytes = files::read(files::REQUEST, REQUEST_MAX)
        .map_err(|e| alloc::format!("The request {}", errno::file(e)))?;
    parse_request(&bytes).map_err(|r| alloc::format!("Request refused: {}", r.why()))
}

pub fn summary(r: &Request) -> String {
    alloc::format!("Prove to {}, window {}", ascii(&r.verifier), r.window)
}

pub fn registry(w: &mut Work) -> Outcome {
    let req = w.request.as_ref().ok_or_else(|| String::from("No request was read"))?;
    let transcript = files::read(files::TRANSCRIPT, TRANSCRIPT_MAX)
        .map_err(|e| alloc::format!("The registry transcript {}", errno::file(e)))?;
    let (answers, last) = tpm::ek_answers();
    if answers.is_empty() {
        return Err(alloc::format!("Endorsement key: {}", errno::ek(last)));
    }
    let eks: Vec<&[u8]> = answers.iter().map(Vec::as_slice).collect();
    let device = enrolled(&transcript, req, &eks)
        .map_err(|r| alloc::format!("Registry refused: {}", r.why()))?;
    let line = alloc::format!(
        "Enrolled: the registry is the one the verifier accepts, depth {}",
        device.depth
    );
    w.enrolled = Some(device);
    Ok((line, Some(Step::Slots)))
}
