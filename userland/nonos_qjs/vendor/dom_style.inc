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

/* el.style: a Proxy whose target carries the node, so each trap knows which
 * element it styles. A set replaces that property in the node's inline
 * style and an empty value removes it; a read answers what the inline style
 * gives the property, empty when it gives none. Reads used to answer
 * undefined, so the toggle on most menus, which reads `style.display`
 * before it writes it, could close a menu and never open it again; and
 * every set was appended, so code that moves an element each frame grew
 * its style attribute without end. setProperty, removeProperty and
 * getPropertyValue go the same way. */
extern char *njs_dom_get_style(void *host, int node, const char *prop);

static JSValue style_read(JSContext *ctx, int node, JSValueConst prop) {
    const char *p = JS_ToCString(ctx, prop);
    if (!p) return JS_EXCEPTION;
    char *s = njs_dom_get_style(JS_GetContextOpaque(ctx), node, p);
    JS_FreeCString(ctx, p);
    if (!s) return JS_NewString(ctx, "");
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}

static void style_write(JSContext *ctx, int node, JSValueConst prop, JSValueConst val) {
    const char *p = JS_ToCString(ctx, prop);
    /* null and undefined clear the property, as they do in a browser. */
    const char *v = (JS_IsNull(val) || JS_IsUndefined(val)) ? 0 : JS_ToCString(ctx, val);
    if (p) njs_dom_set_style(JS_GetContextOpaque(ctx), node, p, v ? v : "");
    JS_FreeCString(ctx, p);
    JS_FreeCString(ctx, v);
}

/* setProperty (magic 0), removeProperty (1) and getPropertyValue (2), with
 * the node as their data. removeProperty answers the value it removed. */
static JSValue style_method(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v, int magic,
                            JSValueConst *data) {
    (void)t;
    int node = -1;
    JS_ToInt32(ctx, &node, data[0]);
    if (argc < 1) return magic == 0 ? JS_UNDEFINED : JS_NewString(ctx, "");
    if (magic == 0) {
        style_write(ctx, node, v[0], argc > 1 ? v[1] : JS_UNDEFINED);
        return JS_UNDEFINED;
    }
    JSValue old = style_read(ctx, node, v[0]);
    if (magic == 1 && !JS_IsException(old)) style_write(ctx, node, v[0], JS_UNDEFINED);
    return old;
}

static const char *const g_style_methods[] = {"setProperty", "removeProperty", "getPropertyValue", 0};

static JSValue style_get_trap(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    /* A symbol (a Symbol.toPrimitive probe, say) names no property. */
    if (argc < 2 || !JS_IsString(v[1])) return JS_UNDEFINED;
    int node = node_of(ctx, v[0]);
    const char *p = JS_ToCString(ctx, v[1]);
    if (!p) return JS_EXCEPTION;
    int method = -1;
    for (int i = 0; g_style_methods[i]; i++)
        if (__builtin_strcmp(p, g_style_methods[i]) == 0) method = i;
    int hidden = __builtin_strcmp(p, "__node") == 0;
    JS_FreeCString(ctx, p);
    if (hidden) return JS_NewInt32(ctx, node);
    if (method >= 0) {
        JSValue data = JS_NewInt32(ctx, node);
        return JS_NewCFunctionData(ctx, style_method, method == 0 ? 2 : 1, method, 1, &data);
    }
    return style_read(ctx, node, v[1]);
}

static JSValue style_set_trap(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 3 || !JS_IsString(v[1])) return JS_TRUE;
    style_write(ctx, node_of(ctx, v[0]), v[1], v[2]);
    return JS_TRUE;
}

static JSValue get_style(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)argc; (void)v;
    JSValue target = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, target, "__node", JS_NewInt32(ctx, node_of(ctx, t)));
    JSValue handler = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, handler, "get", JS_NewCFunction(ctx, style_get_trap, "get", 3));
    JS_SetPropertyStr(ctx, handler, "set", JS_NewCFunction(ctx, style_set_trap, "set", 4));
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue proxy_ctor = JS_GetPropertyStr(ctx, global, "Proxy");
    JSValue args[2] = {target, handler};
    JSValue proxy = JS_CallConstructor(ctx, proxy_ctor, 2, args);
    JS_FreeValue(ctx, proxy_ctor);
    JS_FreeValue(ctx, global);
    JS_FreeValue(ctx, target);
    JS_FreeValue(ctx, handler);
    return proxy;
}
