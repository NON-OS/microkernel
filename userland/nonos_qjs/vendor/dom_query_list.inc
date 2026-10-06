/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Element-scoped lists by tag or class, and the query buffer every
 * list query fills.
 */

/* Every node a selector matches, into a buffer the caller js_frees. It holds
 * QUERY_MAX ids, more than the host DOM ever holds nodes (MAX_NODES is
 * 60,000 in capsule_browser/src/browser/dom/limits.rs), so no query is cut
 * short; raise both together. NULL, with the exception pending, when the
 * buffer cannot be had. */
#define QUERY_MAX 65536
static int *query_ids(JSContext *ctx, const char *sel, int *n) {
    *n = 0;
    int *buf = js_malloc(ctx, sizeof(int) * QUERY_MAX);
    if (!buf) return NULL;
    *n = njs_dom_query_all(JS_GetContextOpaque(ctx), sel, buf, QUERY_MAX);
    if (*n < 0 || *n > QUERY_MAX) *n = 0;
    return buf;
}

/* Descendants by tag or class. Both are the selector engine with the name
 * spelled as a selector, which is what the browser does with them anyway. */
static JSValue collect_sel(JSContext *ctx, JSValueConst t, const char *sel) {
    void *host = JS_GetContextOpaque(ctx);
    int root = node_of(ctx, t), n = 0;
    int *buf = query_ids(ctx, sel, &n);
    if (!buf) return JS_EXCEPTION;
    JSValue arr = JS_NewArray(ctx);
    uint32_t at = 0;
    for (int i = 0; i < n; i++)
        if (buf[i] != root && njs_dom_contains(host, root, buf[i]))
            JS_SetPropertyUint32(ctx, arr, at++, make_element(ctx, buf[i]));
    js_free(ctx, buf);
    return arr;
}
static JSValue el_by_tag(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_NewArray(ctx);
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NewArray(ctx);
    JSValue r = collect_sel(ctx, t, s);
    JS_FreeCString(ctx, s);
    return r;
}
static JSValue el_by_class(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    if (argc < 1) return JS_NewArray(ctx);
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NewArray(ctx);
    char sel[128];
    sel[0] = '.';
    int i = 0;
    for (; s[i] && i < 126; i++) sel[i + 1] = s[i] == ' ' ? '.' : s[i];
    sel[i + 1] = 0;
    JS_FreeCString(ctx, s);
    return collect_sel(ctx, t, sel);
}

static void install_query_ext(JSContext *ctx, JSValueConst el) {
    JS_SetPropertyStr(ctx, el, "matches", JS_NewCFunction(ctx, el_matches, "matches", 1));
    JS_SetPropertyStr(ctx, el, "closest", JS_NewCFunction(ctx, el_closest, "closest", 1));
    JS_SetPropertyStr(ctx, el, "getAttributeNames",
                      JS_NewCFunction(ctx, el_get_attribute_names, "getAttributeNames", 0));
    JS_SetPropertyStr(ctx, el, "getElementsByTagName",
                      JS_NewCFunction(ctx, el_by_tag, "getElementsByTagName", 1));
    JS_SetPropertyStr(ctx, el, "getElementsByClassName",
                      JS_NewCFunction(ctx, el_by_class, "getElementsByClassName", 1));
    accessor(ctx, el, "outerHTML", get_outer_html, 0);
    accessor(ctx, el, "classList", get_class_list, 0);
    accessor(ctx, el, "dataset", get_dataset, 0);
}
