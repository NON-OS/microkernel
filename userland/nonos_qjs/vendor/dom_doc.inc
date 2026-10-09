/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * The document methods that create and find nodes.
 */

static JSValue doc_create_element(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NULL;
    const char *tag = JS_ToCString(ctx, v[0]);
    int node = njs_dom_create_element(JS_GetContextOpaque(ctx), tag);
    JS_FreeCString(ctx, tag);
    return make_element(ctx, node);
}
static JSValue doc_get_by_id(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NULL;
    const char *id = JS_ToCString(ctx, v[0]);
    int node = njs_dom_get_by_id(JS_GetContextOpaque(ctx), id);
    JS_FreeCString(ctx, id);
    return make_element(ctx, node);
}
static JSValue doc_query(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NULL;
    const char *sel = JS_ToCString(ctx, v[0]);
    int node = njs_dom_query(JS_GetContextOpaque(ctx), sel);
    JS_FreeCString(ctx, sel);
    return make_element(ctx, node);
}
static JSValue doc_query_all(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    JSValue arr = JS_NewArray(ctx);
    if (argc < 1) return arr;
    const char *sel = JS_ToCString(ctx, v[0]);
    if (!sel) return arr;
    int n = 0;
    int *ids = query_ids(ctx, sel, &n);
    JS_FreeCString(ctx, sel);
    if (!ids) {
        JS_FreeValue(ctx, arr);
        return JS_EXCEPTION;
    }
    for (int i = 0; i < n; i++)
        JS_SetPropertyUint32(ctx, arr, i, make_element(ctx, ids[i]));
    js_free(ctx, ids);
    return arr;
}

static JSValue doc_create_text(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NULL;
    const char *s = JS_ToCString(ctx, v[0]);
    int node = njs_dom_create_text(JS_GetContextOpaque(ctx), s);
    JS_FreeCString(ctx, s);
    return make_element(ctx, node);
}
