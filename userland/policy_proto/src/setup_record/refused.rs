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
    /* A computer name the kernel would not take, or bytes past its end. */
    Host,
    /* A network this build does not know. */
    Route,
    /* No wallpaper kept, one past the collection, or a desktop wallpaper
     * that is not among those kept. */
    Wallpapers,
}

impl Refused {
    pub fn name(self) -> &'static str {
        match self {
            Refused::Length => "length",
            Refused::Magic => "magic",
            Refused::Timezone => "time zone",
            Refused::Name => "name",
            Refused::Tier => "qwen tier",
            Refused::Host => "computer name",
            Refused::Route => "network route",
            Refused::Wallpapers => "wallpapers kept",
        }
    }
}
