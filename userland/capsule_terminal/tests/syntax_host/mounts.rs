// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! The classifier mounted by path, with the theme types stubbed.

pub mod term {
    pub mod theme {
        pub mod types {
            #[derive(Clone, Copy)]
            pub struct Theme {
                pub bg: u32,
                pub fg: u32,
                pub accent: u32,
                pub path: u32,
                pub dim: u32,
                pub ok: u32,
                pub err: u32,
                pub run: u32,
                pub chrome_edge: u32,
            }
        }
    }
}

use term as crate_term;

#[path = "../../src/paint/syntax/mod.rs"]
pub mod syntax;
