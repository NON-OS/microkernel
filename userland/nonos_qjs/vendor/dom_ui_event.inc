/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * A key or a pointer press sent to the page, with what it carries: the key
 * and its code, the modifiers, where the pointer was. Included by
 * dom_bindings.c after dom_dispatch.inc, whose walk it uses.
 *
 * No keydown or keyup reached a page at all, so a page could not answer
 * Esc in its own dialog, Enter in a search box it runs itself, or the
 * arrows in a carousel; and a click came with no position, so a map, a
 * colour picker or a menu placed where it was opened had nothing to go on.
 */

extern int njs_dom_box(void *host, int node, int which);
extern int njs_dom_scroll_y(void *host);

/* The modifier bits the host passes: 1 Shift, 2 Ctrl, 4 Alt, 8 Meta. */
static void set_mods(JSContext *ctx, JSValueConst ev, int mods) {
    JS_SetPropertyStr(ctx, ev, "shiftKey", JS_NewBool(ctx, mods & 1));
    JS_SetPropertyStr(ctx, ev, "ctrlKey", JS_NewBool(ctx, mods & 2));
    JS_SetPropertyStr(ctx, ev, "altKey", JS_NewBool(ctx, mods & 4));
    JS_SetPropertyStr(ctx, ev, "metaKey", JS_NewBool(ctx, mods & 8));
}

/* keydown, keypress or keyup on `node` (the focused field, or the body):
 * `key` is what the key means ("a", "Enter", "ArrowUp"), `code` the key
 * it is on ("KeyA"), and keyCode the number older pages compare against.
 * A listener's preventDefault is reported as for any dispatch, and the
 * browser then leaves the key alone. */
int njs_dispatch_key(JSContext *ctx, int node, const char *type, const char *key,
                     const char *code, int key_code, int mods) {
    njs_enter(JS_GetRuntime(ctx));
    JSValue ev = make_event(ctx, node, type);
    JS_SetPropertyStr(ctx, ev, "key", JS_NewString(ctx, key));
    JS_SetPropertyStr(ctx, ev, "code", JS_NewString(ctx, code));
    JS_SetPropertyStr(ctx, ev, "keyCode", JS_NewInt32(ctx, key_code));
    JS_SetPropertyStr(ctx, ev, "which", JS_NewInt32(ctx, key_code));
    /* charCode is the character's for keypress, and 0 for the others. */
    int press = __builtin_strcmp(type, "keypress") == 0;
    JS_SetPropertyStr(ctx, ev, "charCode", JS_NewInt32(ctx, press ? key_code : 0));
    JS_SetPropertyStr(ctx, ev, "repeat", JS_FALSE);
    JS_SetPropertyStr(ctx, ev, "isComposing", JS_FALSE);
    JS_SetPropertyStr(ctx, ev, "location", JS_NewInt32(ctx, 0));
    JS_SetPropertyStr(ctx, ev, "detail", JS_NewInt32(ctx, 0));
    set_mods(ctx, ev, mods);
    int fired = run_dispatch(ctx, node, type, ev);
    njs_leave();
    return fired;
}

/* A pointer event on `node` at window point (x, y), the page area's own
 * coordinates: mousedown, mouseup, click, pointerdown or pointerup.
 * `button` is which was pressed (0 main, 1 middle, 2 second) and
 * `buttons` the ones held after it, as bits (1 main, 2 second, 4 middle). */
int njs_dispatch_mouse(JSContext *ctx, int node, const char *type, int x, int y,
                       int button, int buttons, int mods) {
    njs_enter(JS_GetRuntime(ctx));
    void *host = JS_GetContextOpaque(ctx);
    JSValue ev = make_event(ctx, node, type);
    int page_y = y + njs_dom_scroll_y(host);
    JS_SetPropertyStr(ctx, ev, "clientX", JS_NewInt32(ctx, x));
    JS_SetPropertyStr(ctx, ev, "clientY", JS_NewInt32(ctx, y));
    JS_SetPropertyStr(ctx, ev, "x", JS_NewInt32(ctx, x));
    JS_SetPropertyStr(ctx, ev, "y", JS_NewInt32(ctx, y));
    JS_SetPropertyStr(ctx, ev, "pageX", JS_NewInt32(ctx, x));
    JS_SetPropertyStr(ctx, ev, "pageY", JS_NewInt32(ctx, page_y));
    JS_SetPropertyStr(ctx, ev, "screenX", JS_NewInt32(ctx, x));
    JS_SetPropertyStr(ctx, ev, "screenY", JS_NewInt32(ctx, y));
    /* From the target's own box, as laid out. */
    int bx = node >= 0 ? njs_dom_box(host, node, 0) : 0;
    int by = node >= 0 ? njs_dom_box(host, node, 1) : 0;
    JS_SetPropertyStr(ctx, ev, "offsetX", JS_NewInt32(ctx, x - bx));
    JS_SetPropertyStr(ctx, ev, "offsetY", JS_NewInt32(ctx, page_y - by));
    JS_SetPropertyStr(ctx, ev, "movementX", JS_NewInt32(ctx, 0));
    JS_SetPropertyStr(ctx, ev, "movementY", JS_NewInt32(ctx, 0));
    JS_SetPropertyStr(ctx, ev, "button", JS_NewInt32(ctx, button));
    JS_SetPropertyStr(ctx, ev, "buttons", JS_NewInt32(ctx, buttons));
    JS_SetPropertyStr(ctx, ev, "which", JS_NewInt32(ctx, button + 1));
    JS_SetPropertyStr(ctx, ev, "detail", JS_NewInt32(ctx, 1));
    JS_SetPropertyStr(ctx, ev, "relatedTarget", JS_NULL);
    set_mods(ctx, ev, mods);
    if (type[0] == 'p') {
        JS_SetPropertyStr(ctx, ev, "pointerId", JS_NewInt32(ctx, 1));
        JS_SetPropertyStr(ctx, ev, "pointerType", JS_NewString(ctx, "mouse"));
        JS_SetPropertyStr(ctx, ev, "isPrimary", JS_TRUE);
        JS_SetPropertyStr(ctx, ev, "width", JS_NewInt32(ctx, 1));
        JS_SetPropertyStr(ctx, ev, "height", JS_NewInt32(ctx, 1));
        JS_SetPropertyStr(ctx, ev, "pressure", JS_NewFloat64(ctx, buttons ? 0.5 : 0));
    }
    int fired = run_dispatch(ctx, node, type, ev);
    njs_leave();
    return fired;
}
