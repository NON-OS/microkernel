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

//! The player's output, found when it is there to find.
//!
//! The audio service was looked up once, when the window opened, and a
//! player opened before it registered (early in a boot, or while the sound
//! card was still being brought up) said "No audio output: the audio service
//! is not running" for as long as it stayed open, after the service had
//! started. Until it is found the service is looked up again each time play
//! is pressed: a name lookup, no wait. A track still loads and shows its
//! waveform without it.

use super::defs::{Fed, FeedSink};
use crate::audio_client::{AudioClient, FeedResult};
use crate::trouble::NO_SERVICE;

pub struct LateSink {
    client: Option<AudioClient>,
    /// The format the transport last opened with, for a client found later.
    format: Option<u16>,
}

impl LateSink {
    pub fn new() -> Self {
        LateSink { client: AudioClient::connect().ok(), format: None }
    }

    /// The client, looking the service up again if it was not found before,
    /// with the stream opened in the format play asked for.
    fn client(&mut self) -> Result<&mut AudioClient, &'static str> {
        if self.client.is_none() {
            let mut found = AudioClient::connect().map_err(|_| NO_SERVICE)?;
            if let Some(format) = self.format {
                found.open(format)?;
            }
            self.client = Some(found);
        }
        self.client.as_mut().ok_or(NO_SERVICE)
    }
}

impl FeedSink for LateSink {
    fn open(&mut self, format: u16) -> Result<(), &'static str> {
        self.format = Some(format);
        match self.client.as_mut() {
            Some(c) => c.open(format).map(|_| ()),
            // Nothing to open on yet: play says why, and looks again.
            None => Ok(()),
        }
    }

    fn feed(&mut self, pcm: &[i16]) -> Fed {
        let client = match self.client() {
            Ok(c) => c,
            Err(why) => return Fed::Failed(why),
        };
        match client.feed(pcm) {
            Ok(FeedResult::Fed) => Fed::Accepted,
            Ok(FeedResult::WouldBlock) => Fed::WouldBlock,
            Err(why) => Fed::Failed(why),
        }
    }

    fn pause(&mut self) {
        if let Some(c) = self.client.as_mut() {
            c.pause();
        }
    }

    fn resume(&mut self) {
        if let Some(c) = self.client.as_mut() {
            c.resume();
        }
    }

    fn close(&mut self) {
        if let Some(c) = self.client.as_mut() {
            c.close();
        }
    }
}
