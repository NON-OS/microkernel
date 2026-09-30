/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* Why a record was refused, so a log can say which part was wrong. */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refused {
    /* No version's length. */
    Length,
    /* A length that fits, without that version's magic: zeros included. */
    Magic,
    /* A time zone setup does not offer. */
    Timezone,
    /* A name setup's name step would not take, or bytes past its end. */
    Name,
    /* A tier that is no tier's name, or bytes past its end. */
    Tier,
}

impl Refused {
    pub fn name(self) -> &'static str {
        match self {
            Refused::Length => "length",
            Refused::Magic => "magic",
            Refused::Timezone => "time zone",
            Refused::Name => "name",
            Refused::Tier => "qwen tier",
        }
    }
}
