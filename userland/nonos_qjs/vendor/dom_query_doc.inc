/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * classList and dataset through the prelude factories, and the
 * document-wide lists by tag or class.
 */

/* classList and dataset are built in JS from the element's own attribute
 * methods, because both are small and neither needs the host. The factories
 * live in the prelude; these hand the element to them.
 *
 * They are built per read rather than cached, which is what a script that
 * writes a class and reads it back in the same statement needs. */
static JSValue from_factory(JSContext *ctx, JSValueConst t, const char *factory) {
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue fn = JS_GetPropertyStr(ctx, global, factory);
    JSValue r = JS_UNDEFINED;
    if (JS_IsFunction(ctx, fn)) {
        JSValueConst arg = t;
        r = JS_Call(ctx, fn, JS_UNDEFINED, 1, &arg);
    }
    JS_FreeValue(ctx, fn);
    JS_FreeValue(ctx, global);
    return r;
}
static JSValue get_class_list(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return from_factory(ctx, t, "__njs_classlist");
}
static JSValue get_dataset(JSContext *ctx, JSValueConst t, int a, JSValueConst *v) {
    (void)a; (void)v;
    return from_factory(ctx, t, "__njs_dataset");
}

/* The elements a query buffer names, as an array; frees the buffer. */
static JSValue doc_list(JSContext *ctx, int *buf, int n) {
    if (!buf) return JS_EXCEPTION;
    JSValue arr = JS_NewArray(ctx);
    for (int i = 0; i < n; i++)
        JS_SetPropertyUint32(ctx, arr, (uint32_t)i, make_element(ctx, buf[i]));
    js_free(ctx, buf);
    return arr;
}

/* The document-wide forms. These search the whole tree rather than one
 * element's descendants, so they cannot share the element versions. */
static JSValue doc_by_tag(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NewArray(ctx);
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NewArray(ctx);
    int n = 0;
    int *buf = query_ids(ctx, s, &n);
    JS_FreeCString(ctx, s);
    return doc_list(ctx, buf, n);
}
static JSValue doc_by_class(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_NewArray(ctx);
    const char *s = JS_ToCString(ctx, v[0]);
    if (!s) return JS_NewArray(ctx);
    char sel[128];
    sel[0] = '.';
    int i = 0;
    for (; s[i] && i < 126; i++) sel[i + 1] = s[i] == ' ' ? '.' : s[i];
    sel[i + 1] = 0;
    JS_FreeCString(ctx, s);
    int n = 0;
    int *buf = query_ids(ctx, sel, &n);
    return doc_list(ctx, buf, n);
}
