/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * The event object a dispatch builds, and whether the last one was
 * cancelled.
 */

/* performance.now(), the clock an event's timeStamp is read on. It was 0
 * on every event, so code that tells a double click or a repeat from the
 * time between two events saw no time pass and took every click after
 * the first for a repeat. */
static double event_time(JSContext *ctx) {
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue perf = JS_GetPropertyStr(ctx, global, "performance");
    JSValue now = JS_GetPropertyStr(ctx, perf, "now");
    double t = 0;
    if (JS_IsFunction(ctx, now)) {
        JSValue r = JS_Call(ctx, now, perf, 0, 0);
        if (JS_IsException(r)) JS_FreeValue(ctx, JS_GetException(ctx));
        else JS_ToFloat64(ctx, &t, r);
        JS_FreeValue(ctx, r);
    }
    JS_FreeValue(ctx, now);
    JS_FreeValue(ctx, perf);
    JS_FreeValue(ctx, global);
    return t;
}

/* The event object a dispatch hands every listener. */
static JSValue make_event(JSContext *ctx, int node, const char *type) {
    JSValue ev = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, ev, "type", JS_NewString(ctx, type));
    /* Node -1 is the document itself, whose own events (scroll) start there. */
    JSValue target = make_element(ctx, node);
    if (node < 0) {
        JSValue global = JS_GetGlobalObject(ctx);
        target = JS_GetPropertyStr(ctx, global, "document");
        JS_FreeValue(ctx, global);
    }
    JS_SetPropertyStr(ctx, ev, "target", target);
    JS_SetPropertyStr(ctx, ev, "bubbles", JS_TRUE);
    JS_SetPropertyStr(ctx, ev, "cancelable", JS_TRUE);
    JS_SetPropertyStr(ctx, ev, "defaultPrevented", JS_FALSE);
    JS_SetPropertyStr(ctx, ev, "isTrusted", JS_TRUE);
    JS_SetPropertyStr(ctx, ev, "timeStamp", JS_NewFloat64(ctx, event_time(ctx)));
    JS_SetPropertyStr(ctx, ev, "preventDefault", JS_NewCFunction(ctx, ev_prevent, "preventDefault", 0));
    JS_SetPropertyStr(ctx, ev, "stopPropagation", JS_NewCFunction(ctx, ev_stop, "stopPropagation", 0));
    JS_SetPropertyStr(ctx, ev, "stopImmediatePropagation", JS_NewCFunction(ctx, ev_stop_now, "stopImmediatePropagation", 0));
    return ev;
}

/* Whether the last dispatch was cancelled, so the host can skip the action
 * the event would otherwise have triggered. */
static int g_last_prevented = 0;
int njs_event_default_prevented(void) { return g_last_prevented; }
