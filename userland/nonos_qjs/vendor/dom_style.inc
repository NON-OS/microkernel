/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Accessor installation and the el.style proxy.
 */

static void accessor(JSContext *ctx, JSValueConst el, const char *name, JSCFunction *get, JSCFunction *set) {
    JSAtom a = JS_NewAtom(ctx, name);
    JSValue g = get ? JS_NewCFunction(ctx, get, name, 0) : JS_UNDEFINED;
    JSValue s = set ? JS_NewCFunction(ctx, set, name, 1) : JS_UNDEFINED;
    JS_DefinePropertyGetSet(ctx, el, a, g, s, JS_PROP_C_W_E);
    JS_FreeAtom(ctx, a);
}

/* el.style.<prop> = <val>: a Proxy whose target carries the node, so the set
 * trap knows which element to style. Individual property assignment is what
 * frameworks emit; it accumulates into the node's inline style. */
static JSValue style_set_trap(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 3) return JS_TRUE;
    const char *prop = JS_ToCString(ctx, v[1]);
    const char *val = JS_ToCString(ctx, v[2]);
    njs_dom_set_style(JS_GetContextOpaque(ctx), node_of(ctx, v[0]), prop, val);
    JS_FreeCString(ctx, prop);
    JS_FreeCString(ctx, val);
    return JS_TRUE;
}

static JSValue get_style(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)argc; (void)v;
    JSValue target = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, target, "__node", JS_NewInt32(ctx, node_of(ctx, t)));
    JSValue handler = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, handler, "set", JS_NewCFunction(ctx, style_set_trap, "set", 4));
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue proxy_ctor = JS_GetPropertyStr(ctx, global, "Proxy");
    JSValue args[2] = {target, handler};
    JSValue proxy = JS_CallConstructor(ctx, proxy_ctor, 2, args);
    JS_FreeValue(ctx, proxy_ctor);
    JS_FreeValue(ctx, target);
    JS_FreeValue(ctx, handler);
    return proxy;
}
