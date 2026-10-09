/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Tree navigation properties: parent, children, siblings, names.
 */

static JSValue get_parent_node(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return make_element(ctx, njs_dom_parent(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue get_first_child(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return make_element(ctx, njs_dom_child_at(JS_GetContextOpaque(ctx), node_of(ctx, t), 0));
}
static JSValue get_next_sibling(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return make_element(ctx, njs_dom_next_sibling(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue get_node_type(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return JS_NewInt32(ctx, njs_dom_node_kind(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
static JSValue get_local_name(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return str_or_null(ctx, njs_dom_get_tag(JS_GetContextOpaque(ctx), node_of(ctx, t)));
}
/* nodeName: elements report their tag uppercased, text nodes #text. */
static JSValue get_node_name(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    int n = node_of(ctx, t);
    if (njs_dom_node_kind(host, n) == 3) return JS_NewString(ctx, "#text");
    char *s = njs_dom_get_tag(host, n);
    if (!s) return JS_NULL;
    for (int i = 0; s[i]; i++)
        if (s[i] >= 'a' && s[i] <= 'z') s[i] -= 32;
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}
/* ownerDocument hands back the shared document object; event delegation
 * roots (React) listen there. */
static JSValue get_owner_document(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)t; (void)a; (void)v;
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue doc = JS_GetPropertyStr(ctx, global, "document");
    return doc;
}
static JSValue get_child_nodes(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    int n = node_of(ctx, t);
    JSValue arr = JS_NewArray(ctx);
    int count = njs_dom_child_count(host, n);
    for (int i = 0; i < count; i++)
        JS_SetPropertyUint32(ctx, arr, (uint32_t)i, make_element(ctx, njs_dom_child_at(host, n, i)));
    return arr;
}
