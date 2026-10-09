/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Selector questions about a single node, markup read back out, and the
 * attribute-backed collections a script expects on an element. Included by
 * dom_bindings.c after the statics these build on.
 */

extern int njs_dom_matches(void *host, int node, const char *sel);
extern int njs_dom_closest(void *host, int node, const char *sel);
extern char *njs_dom_get_inner_html(void *host, int node);
extern char *njs_dom_get_outer_html(void *host, int node);
extern int njs_dom_attr_count(void *host, int node);
extern char *njs_dom_attr_name_at(void *host, int node, int i);

static JSValue get_class_list(JSContext *ctx, JSValueConst t, int a, JSValueConst *v);
static JSValue get_dataset(JSContext *ctx, JSValueConst t, int a, JSValueConst *v);

static JSValue el_matches(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_FALSE;
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_FALSE;
    int m = njs_dom_matches(JS_GetContextOpaque(ctx), node_of(ctx, t), s);
    JS_FreeCString(ctx, s);
    return m ? JS_TRUE : JS_FALSE;
}

/* How a page turns a click on whatever was under the pointer into the row or
 * link the handler is about. A page that delegates its events and cannot
 * call this handles nothing at all. */
static JSValue el_closest(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_NULL;
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NULL;
    int n = njs_dom_closest(JS_GetContextOpaque(ctx), node_of(ctx, t), s);
    JS_FreeCString(ctx, s);
    return make_element(ctx, n);
}

static JSValue get_inner_html(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    char *s = njs_dom_get_inner_html(JS_GetContextOpaque(ctx), node_of(ctx, t));
    if (!s) return JS_NewString(ctx, "");
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}
static JSValue get_outer_html(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    char *s = njs_dom_get_outer_html(JS_GetContextOpaque(ctx), node_of(ctx, t));
    if (!s) return JS_NewString(ctx, "");
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}

static JSValue el_get_attribute_names(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    void *host = JS_GetContextOpaque(ctx);
    int n = node_of(ctx, t), count = njs_dom_attr_count(host, n);
    JSValue arr = JS_NewArray(ctx);
    for (int i = 0; i < count; i++) {
        char *k = njs_dom_attr_name_at(host, n, i);
        if (!k) continue;
        JS_SetPropertyUint32(ctx, arr, (uint32_t)i, JS_NewString(ctx, k));
        free(k);
    }
    return arr;
}
