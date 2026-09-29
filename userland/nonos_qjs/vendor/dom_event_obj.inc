/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * The event object a dispatch builds, and whether the last one was
 * cancelled.
 */

/* The event object a dispatch hands every listener. */
static JSValue make_event(JSContext *ctx, int node, const char *type) {
    JSValue ev = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, ev, "type", JS_NewString(ctx, type));
    JS_SetPropertyStr(ctx, ev, "target", make_element(ctx, node));
    JS_SetPropertyStr(ctx, ev, "bubbles", JS_TRUE);
    JS_SetPropertyStr(ctx, ev, "cancelable", JS_TRUE);
    JS_SetPropertyStr(ctx, ev, "defaultPrevented", JS_FALSE);
    JS_SetPropertyStr(ctx, ev, "isTrusted", JS_TRUE);
    JS_SetPropertyStr(ctx, ev, "timeStamp", JS_NewInt32(ctx, 0));
    JS_SetPropertyStr(ctx, ev, "preventDefault", JS_NewCFunction(ctx, ev_prevent, "preventDefault", 0));
    JS_SetPropertyStr(ctx, ev, "stopPropagation", JS_NewCFunction(ctx, ev_stop, "stopPropagation", 0));
    JS_SetPropertyStr(ctx, ev, "stopImmediatePropagation", JS_NewCFunction(ctx, ev_stop_now, "stopImmediatePropagation", 0));
    return ev;
}

/* Whether the last dispatch was cancelled, so the host can skip the action
 * the event would otherwise have triggered. */
static int g_last_prevented = 0;
int njs_event_default_prevented(void) { return g_last_prevented; }
