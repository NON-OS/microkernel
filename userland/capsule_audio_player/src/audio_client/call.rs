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

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};
use super::proto::{close_request, feed_request, open_request, pause_request, read_status, read_stream_id, resume_request, E_AGAIN, E_NODEV};
use nonos_audio_proto::{output_message, output_status_request, read_output_status, OUTPUT_NOT_ANSWERING, OUTPUT_REPLY_LEN};

const SERVICE_NAME: &[u8] = b"audio.server";
const REQUEST_ID: u32 = 1;
const CALL_TIMEOUT_MS: u64 = 1000;

pub enum FeedResult { Fed, WouldBlock }

pub struct AudioClient { port: u64, stream_id: u32 }

impl AudioClient {
    pub fn connect() -> Result<AudioClient, &'static str> {
        let mut port = 0u32;
        let mut pid = 0u32;
        let rc = mk_service_lookup(SERVICE_NAME.as_ptr(), SERVICE_NAME.len(), &mut port, &mut pid);
        if rc < 0 || port == 0 {
            return Err("audio.server lookup failed");
        }
        Ok(AudioClient { port: port as u64, stream_id: 0 })
    }

    fn round_trip(&self, req: &[u8]) -> ([u8; 32], i64) {
        let mut resp = [0u8; 32];
        let rc = mk_ipc_call_timeout(self.port, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len(), CALL_TIMEOUT_MS);
        (resp, rc)
    }
    pub fn open(&mut self, format: u16) -> Result<u32, &'static str> {
        // One stream per client: a second open gives the first back rather
        // than leaving it on the server, which holds two per process.
        self.close();
        let (resp, rc) = self.round_trip(&open_request(REQUEST_ID, format));
        if rc < 28 {
            return Err("audio.server open: no reply");
        }
        match read_status(&resp[..rc as usize]) {
            // This machine has no output the system can play on: ask why, and
            // say it in the words the user reads instead of a bare refusal.
            E_NODEV => return Err(output_message(self.output_status().0)),
            s if s < 0 => return Err("audio.server open rejected"),
            _ => {}
        }
        self.stream_id = read_stream_id(&resp[..rc as usize]);
        Ok(self.stream_id)
    }
    /// What audio.server says this machine's audio is: an output code and
    /// its flags (`nonos_audio_proto::output`).
    pub fn output_status(&self) -> (u32, u32) {
        let mut req = [0u8; nonos_audio_proto::HDR_LEN];
        let n = output_status_request(&mut req, REQUEST_ID);
        let mut resp = [0u8; OUTPUT_REPLY_LEN];
        let rc = mk_ipc_call_timeout(self.port, req.as_ptr(), n, resp.as_mut_ptr(), resp.len(), CALL_TIMEOUT_MS);
        if rc <= 0 {
            return (OUTPUT_NOT_ANSWERING, 0);
        }
        read_output_status(&resp[..rc as usize]).unwrap_or((OUTPUT_NOT_ANSWERING, 0))
    }
    pub fn feed(&mut self, pcm: &[i16]) -> Result<FeedResult, &'static str> {
        if pcm.is_empty() || pcm.len() % 2 != 0 {
            return Err("audio.server feed: invalid pcm length");
        }
        let (resp, rc) = self.round_trip(&feed_request(REQUEST_ID, self.stream_id, pcm));
        if rc < 24 {
            return Err("audio.server feed: no reply");
        }
        match read_status(&resp[..rc as usize]) {
            s if s == E_AGAIN => Ok(FeedResult::WouldBlock),
            s if s < 0 => Err("audio.server feed rejected"),
            _ => Ok(FeedResult::Fed),
        }
    }
    pub fn pause(&mut self) {
        if self.stream_id == 0 {
            return;
        }
        self.round_trip(&pause_request(REQUEST_ID, self.stream_id));
        crate::mark::mark("[PLAYER] pause-sent\n");
    }
    pub fn resume(&mut self) {
        if self.stream_id == 0 {
            return;
        }
        self.round_trip(&resume_request(REQUEST_ID, self.stream_id));
        crate::mark::mark("[PLAYER] resume-sent\n");
    }
    pub fn close(&mut self) {
        if self.stream_id == 0 {
            return;
        }
        self.round_trip(&close_request(REQUEST_ID, self.stream_id));
        self.stream_id = 0;
    }
}

// The server frees a stream only on close or when its owner process ends, so a
// client dropped with one open would hold that slot until the process exits.
impl Drop for AudioClient {
    fn drop(&mut self) {
        self.close();
    }
}
