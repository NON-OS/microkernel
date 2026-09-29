// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/boot/handoff/types/constants.rs"]
pub mod constants;

#[path = "../../../../../../../../src/boot/handoff/types/framebuffer.rs"]
pub mod framebuffer;

pub use constants::{flags, pixel_format, HANDOFF_MAGIC, HANDOFF_VERSION};
pub use constants::{truncate_cmdline, validate_cmdline_len};
pub use framebuffer::FramebufferInfo;
