// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The words of the package install-consent prompt: the kernel-verified
//! slug, its publisher namespace, the trust tier, and the capability
//! ceiling. The list is shown whole wherever the screen has room; when the
//! panel still cuts it short, a warning note says so instead of quietly
//! dropping a granted capability.

use alloc::format;
use alloc::vec::Vec;

use super::prompt_buttons::APPROVE;
use super::prompts::Words;
use crate::render::ui_font::valid_str;
use crate::state::Context;

const TITLE: &str = "Install package?";
const TRUNCATED: &str = "! caps hidden - do not approve";

pub(super) fn words(ctx: &Context) -> Option<Words> {
    let s = &ctx.pending_pkg_install.as_ref()?.summary;
    let tier = if s.tier == 2 { "Publisher-signed" } else { "NONOS-enrolled" };
    let mut caps = Vec::with_capacity(320);
    super::cap_names::append(s.caps, &mut caps);
    let (slug, ns, caps) = (valid_str(&s.slug), valid_str(&s.namespace), valid_str(&caps));
    Some(Words {
        title: TITLE.into(),
        body: format!("{slug} from {ns}, {tier}. Caps: {caps}"),
        buttons: APPROVE,
        warn_cut: Some(TRUNCATED),
    })
}
