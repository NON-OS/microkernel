/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * document.cookie, read and written through the host's cookie jar. The
 * prelude defines the accessor; these are the two natives it calls. The
 * host applies the jar's rules: HttpOnly cookies are never read here and
 * never written from here, and the page reads only the jar of the network
 * it was loaded over.
 */

extern char *njs_dom_cookie_get(void *host);
extern void njs_dom_cookie_set(void *host, const char *line);

static JSValue js_cookie_get(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t; (void)argc; (void)v;
    char *s = njs_dom_cookie_get(JS_GetContextOpaque(ctx));
    if (!s) return JS_NewString(ctx, "");
    JSValue r = JS_NewString(ctx, s);
    free(s);
    return r;
}

static JSValue js_cookie_set(JSContext *ctx, JSValueConst t, int argc, JSValueConst *v) {
    (void)t;
    if (argc < 1) return JS_UNDEFINED;
    const char *line = JS_ToCString(ctx, v[0]);
    if (!line) return JS_UNDEFINED;
    njs_dom_cookie_set(JS_GetContextOpaque(ctx), line);
    JS_FreeCString(ctx, line);
    return JS_UNDEFINED;
}
