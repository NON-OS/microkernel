/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The Apps step's model: which optional apps this image carries, and how
 * the choice reads on the review screen.
 */

mod listed;
mod said;

pub use listed::{listed, present};
pub use said::said;
