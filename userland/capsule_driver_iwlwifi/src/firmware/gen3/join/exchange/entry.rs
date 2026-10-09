// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The exchange's entry: the counts carried in, the run, and the counts and
//! the MLME's state carried back out.

use alloc::vec::Vec;

use nonos_wifi_core::mlme::Mlme;

use super::super::super::region::{Clock, Region};
use super::super::bss::Bss;
use super::super::fw::Fw;
use super::air::Air;
use super::drive::run;
use super::end::ExchangeEnd;
use super::progress::{state_code, Progress};
use crate::regs::Mmio;

/// Run the exchange from the beacon's authentication frame `first` to a
/// connected MLME, or to its end.
pub fn exchange<M: Mmio, R: Region + ?Sized, C: Clock>(
    fw: &mut Fw<'_, '_, M, R, C>,
    bss: &mut Bss,
    mlme: &mut Mlme,
    first: Vec<u8>,
    progress: &mut Progress,
) -> Result<(), ExchangeEnd> {
    let mut air = Air { pending: Vec::new(), seq: 0, p: *progress };
    let out = run(fw, bss, mlme, first, &mut air);
    *progress = Progress { state: state_code(mlme.state()), ..air.p };
    out
}
