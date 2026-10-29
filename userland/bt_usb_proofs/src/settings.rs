// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The one item of Settings' row schema the Bluetooth panel names: the
//! tone a row's words are drawn in, as capsule_settings'
//! settings/schema/rows.rs has it. A variant the panel uses and this lacks
//! fails to compile here.

pub mod schema {
    pub mod rows {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Tone {
            Ok,
            Warn,
            Idle,
        }
    }
}
