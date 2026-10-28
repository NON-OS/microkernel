/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The firmware package's two files for this chip, read and checked before
//! the card's DMA is touched: a medium without the package, or with other
//! files, stops here with the card untouched beyond its identity.

use alloc::vec::Vec;

use crate::fw::package::File;
use crate::fw::{fetch, Fetched};
use crate::say::Line;
use crate::stage::Stage;

pub struct Files {
    pub patch: Vec<u8>,
    pub ram: Vec<u8>,
}

fn one(f: &File) -> Result<Vec<u8>, Stage> {
    match fetch(f) {
        Fetched::Bytes(b) => {
            Line::new("package: ").text(f.name).text(" verified").say();
            Ok(b)
        }
        Fetched::Missing => {
            Line::new("package: ").text(f.name).text(" missing").say();
            Err(Stage::PackageMissing)
        }
        Fetched::Mismatch => {
            Line::new("package: ").text(f.name).text(" does not match its sha256").say();
            Err(Stage::PackageMismatch)
        }
    }
}

/// The chip's ROM patch and RAM code, each checked.
pub fn read((patch, ram): (&File, &File)) -> Result<Files, Stage> {
    Ok(Files { patch: one(patch)?, ram: one(ram)? })
}
