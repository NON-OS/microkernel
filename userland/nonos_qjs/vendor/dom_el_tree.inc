/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Element methods that move nodes and read or write attributes.
 */

static JSValue el_append_child(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    njs_dom_append(JS_GetContextOpaque(ctx), node_of(ctx, t), node_of(ctx, v[0]));
    return JS_DupValue(ctx, v[0]);
}

/* insertBefore(child, ref); a null ref appends, per the DOM contract. */
static JSValue el_insert_before(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    int before = (argc >= 2 && JS_IsObject(v[1])) ? node_of(ctx, v[1]) : -1;
    njs_dom_insert_before(JS_GetContextOpaque(ctx), node_of(ctx, t), node_of(ctx, v[0]), before);
    return JS_DupValue(ctx, v[0]);
}

static JSValue el_remove_attribute(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *k = JS_ToCString(ctx, v[0]);
    njs_dom_remove_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), k);
    JS_FreeCString(ctx, k);
    return JS_UNDEFINED;
}

static JSValue el_remove(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)argc; (void)v;
    int n = node_of(ctx, t);
    njs_dom_remove_child(JS_GetContextOpaque(ctx), njs_dom_parent(JS_GetContextOpaque(ctx), n), n);
    return JS_UNDEFINED;
}

static JSValue el_remove_child(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    njs_dom_remove_child(JS_GetContextOpaque(ctx), node_of(ctx, t), node_of(ctx, v[0]));
    return JS_DupValue(ctx, v[0]);
}

static JSValue el_set_attribute(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 2) return JS_UNDEFINED;
    const char *k = JS_ToCString(ctx, v[0]);
    const char *val = JS_ToCString(ctx, v[1]);
    njs_dom_set_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), k, val);
    JS_FreeCString(ctx, k);
    JS_FreeCString(ctx, val);
    return JS_UNDEFINED;
}

static JSValue el_get_attribute(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_NULL;
    const char *k = JS_ToCString(ctx, v[0]);
    char *r = njs_dom_get_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), k);
    JS_FreeCString(ctx, k);
    return str_or_null(ctx, r);
}
