/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * The host callbacks the glue calls, the listener table, and the node
 * helpers every other piece builds on. Included by dom_bindings.c first.
 */

extern int njs_dom_create_element(void *host, const char *tag);
extern void njs_dom_append(void *host, int parent, int child);
extern void njs_dom_remove_child(void *host, int parent, int child);
extern void njs_dom_set_attr(void *host, int node, const char *k, const char *v);
extern char *njs_dom_get_attr(void *host, int node, const char *k);
extern void njs_dom_set_text(void *host, int node, const char *text);
extern char *njs_dom_get_text(void *host, int node);
extern char *njs_dom_get_tag(void *host, int node);
extern int njs_dom_get_by_id(void *host, const char *id);
extern int njs_dom_query(void *host, const char *sel);
extern int njs_dom_query_all(void *host, const char *sel, int *out, int max);
extern int njs_dom_body(void *host);
extern void njs_dom_set_style(void *host, int node, const char *prop, const char *val);
extern void njs_dom_set_inner_html(void *host, int node, const char *html);
extern int njs_dom_create_text(void *host, const char *text);
extern void njs_dom_insert_before(void *host, int parent, int child, int before);
extern int njs_dom_parent(void *host, int node);
extern int njs_dom_child_count(void *host, int node);
extern int njs_dom_child_at(void *host, int node, int i);
extern int njs_dom_next_sibling(void *host, int node);
extern int njs_dom_node_kind(void *host, int node);
extern void njs_dom_remove_attr(void *host, int node, const char *k);

/* Event listeners are held here with a duplicated function reference so the GC
 * keeps them alive between the script run and later dispatch. One table per
 * process is correct: a capsule runs one page context. */
#define MAX_LISTENERS 1024
static struct {
    int node;
    char type[24];
    JSValue fn;
    int live;
} g_listeners[MAX_LISTENERS];
static int g_lcount = 0;

static int node_of(JSContext *ctx, JSValueConst obj) {
    JSValue v = JS_GetPropertyStr(ctx, obj, "__node");
    int n = -1;
    JS_ToInt32(ctx, &n, v);
    JS_FreeValue(ctx, v);
    return n;
}

static JSValue str_or_null(JSContext *ctx, char *s) {
    if (!s) return JS_NULL;
    JSValue v = JS_NewString(ctx, s);
    free(s);
    return v;
}

static JSValue make_element(JSContext *ctx, int node);
