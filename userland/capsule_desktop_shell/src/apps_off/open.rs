/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

use nonos_libc::mk_time_millis;

use super::is_off;
use crate::server::handlers::launcher_request::{self, LaunchOutcome};
use crate::state::apps::LauncherApp;
use crate::state::toast::TOAST_TEXT_MAX;
use crate::state::{Context, NotifyLevel, LAUNCHER_APPS};

const OFF_TAIL: &[u8] = b" turned off at setup";

/* A dock launch, never asked of the kernel for an app it will not spawn. */
pub fn request(app: &LauncherApp) -> LaunchOutcome {
    if is_off(app.service) {
        return LaunchOutcome::Failed;
    }
    launcher_request::request(app)
}

/* Launch `service` from a menu or the Launchpad, or say why it is off. */
pub fn open(ctx: &mut Context, service: &[u8]) {
    if is_off(service) {
        say_off(ctx, service, mk_time_millis());
        return;
    }
    let _ = launcher_request::request_service(service);
}

/* Say why a dock click opened nothing. */
pub fn toast_failed(ctx: &mut Context, service: &[u8], now: i64) {
    if is_off(service) {
        say_off(ctx, service, now);
        return;
    }
    ctx.toasts.push(b"could not open window", NotifyLevel::Error, now);
}

/* "<App> turned off at setup", the app named as its tile names it. */
fn say_off(ctx: &mut Context, service: &[u8], now: i64) {
    let app = LAUNCHER_APPS.iter().find(|a| a.service == service);
    let name = app.map_or(&b"This app"[..], |a| a.label);
    let mut line = [0u8; TOAST_TEXT_MAX];
    let n = name.len().min(TOAST_TEXT_MAX - OFF_TAIL.len());
    line[..n].copy_from_slice(&name[..n]);
    line[n..n + OFF_TAIL.len()].copy_from_slice(OFF_TAIL);
    ctx.toasts.push(&line[..n + OFF_TAIL.len()], NotifyLevel::Warn, now);
}
