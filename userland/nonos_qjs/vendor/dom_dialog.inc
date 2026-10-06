/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * alert, confirm and prompt: what a page asked the reader, parked for the
 * browser to say. Included by dom_bindings.c before dom_events.inc, which
 * installs it.
 */

/* A script runs to its end in one go, on a 2 s budget, so a dialog that
 * waits for the reader cannot be built: the page would be stopped long
 * before anyone read it. The prelude answers at once (confirm false,
 * prompt null, never yes) and this keeps what was asked, so the browser
 * can show it and say what it answered. They were not defined, so the
 * call threw and took the rest of the script with it.
 *
 * The last asked since the browser looked is kept, with how many were
 * asked in all; its text is held to a whole UTF-8 character. */
static int g_dialog_kind = -1;
static int g_dialog_count;
static char g_dialog_text[512];

static JSValue js_dialog(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    int kind = 0;
    if (argc < 2 || JS_ToInt32(ctx, &kind, v[0]) != 0 || kind < 0 || kind > 2)
        return JS_UNDEFINED;
    const char *s = JS_ToCString(ctx, v[1]);
    if (!s) return JS_EXCEPTION;
    int i = 0;
    for (; s[i] && i < (int)sizeof(g_dialog_text) - 1; i++) g_dialog_text[i] = s[i];
    /* Cut inside a character: back to where it began. */
    while (i > 0 && ((unsigned char)s[i] & 0xC0) == 0x80) i--;
    g_dialog_text[i] = 0;
    JS_FreeCString(ctx, s);
    g_dialog_kind = kind;
    g_dialog_count++;
    return JS_UNDEFINED;
}

/* The dialog asked since the last read: its kind (0 alert, 1 confirm,
 * 2 prompt) or -1 for none, its text, and how many were asked in all.
 * Reading clears it. */
int njs_take_dialog(const char **text, int *count) {
    int kind = g_dialog_kind;
    *text = g_dialog_text;
    *count = g_dialog_count;
    g_dialog_kind = -1;
    g_dialog_count = 0;
    return kind;
}
