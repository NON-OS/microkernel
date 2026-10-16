// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! The classifier mounted by path, with the theme types stubbed.

pub mod term {
    pub mod theme {
        pub mod types {
            /// The colours a part is drawn in; the shipping profile carries
            /// more, which the classification never reads.
            #[derive(Clone, Copy)]
            pub struct Theme {
                pub fg: u32,
                pub accent: u32,
                pub path: u32,
                pub dim: u32,
            }
        }
    }
}

#[path = "../../../capsule_terminal/src/paint/syntax/mod.rs"]
pub mod syntax;
