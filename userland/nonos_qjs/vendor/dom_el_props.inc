/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Element properties backed by text, class, id, tag and markup.
 */

static JSValue set_text_content(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *s = JS_ToCString(ctx, v[0]);
    njs_dom_set_text(JS_GetContextOpaque(ctx), node_of(ctx, t), s);
    JS_FreeCString(ctx, s);
    return JS_UNDEFINED;
}
static JSValue get_text_content(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return str_or_null(ctx, njs_dom_get_text(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue set_class_name(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *s = JS_ToCString(ctx, v[0]);
    njs_dom_set_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), "class", s);
    JS_FreeCString(ctx, s);
    return JS_UNDEFINED;
}
static JSValue get_class_name(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return str_or_null(ctx, njs_dom_get_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), "class"));
}
static JSValue set_id(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *s = JS_ToCString(ctx, v[0]);
    njs_dom_set_attr(JS_GetContextOpaque(ctx), node_of(ctx, t), "id", s);
    JS_FreeCString(ctx, s);
    return JS_UNDEFINED;
}
static JSValue get_tag_name(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return str_or_null(ctx, njs_dom_get_tag(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue set_inner_html(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_UNDEFINED;
    const char *h = JS_ToCString(ctx, v[0]);
    njs_dom_set_inner_html(JS_GetContextOpaque(ctx), node_of(ctx, t), h);
    JS_FreeCString(ctx, h);
    return JS_UNDEFINED;
}
