// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! How long the exchange waits, and how often it resends.

/// How long a sent frame waits for an answer before it is sent again.
pub const RETX_MS: u32 = 300;
/// Resends in a row with no answer before the join is given up.
pub const IDLE_TRIES: u32 = 6;
/// The whole exchange, answered or not, on the clock: a hunt over every
/// channel (under 10 s) and the exchange stay inside the 20 s the client
/// gives a connect.
pub const EXCHANGE_MS: u64 = 7500;
/// Waits that end with nothing received, at most. A working clock reaches
/// `EXCHANGE_MS` first (two such waits per `RETX_MS` at most); this ends the
/// exchange if the uptime clock stops.
pub const SILENT_WAITS_MAX: u32 = 64;
