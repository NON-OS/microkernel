/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Installing the document object and running the prelude.
 */

void njs_install_dom(JSContext *ctx, void *host) {
    JS_SetContextOpaque(ctx, host);
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue doc = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, doc, "createElement", JS_NewCFunction(ctx, doc_create_element, "createElement", 1));
    JS_SetPropertyStr(ctx, doc, "createDocumentFragment",
                      JS_NewCFunction(ctx, doc_create_fragment, "createDocumentFragment", 0));
    JS_SetPropertyStr(ctx, doc, "createTextNode", JS_NewCFunction(ctx, doc_create_text, "createTextNode", 1));
    JS_SetPropertyStr(ctx, doc, "getElementById", JS_NewCFunction(ctx, doc_get_by_id, "getElementById", 1));
    JS_SetPropertyStr(ctx, doc, "querySelector", JS_NewCFunction(ctx, doc_query, "querySelector", 1));
    JS_SetPropertyStr(ctx, doc, "querySelectorAll", JS_NewCFunction(ctx, doc_query_all, "querySelectorAll", 1));
    JS_SetPropertyStr(ctx, doc, "addEventListener", JS_NewCFunction(ctx, el_add_listener, "addEventListener", 2));
    JS_SetPropertyStr(ctx, doc, "removeEventListener",
                      JS_NewCFunction(ctx, el_remove_listener, "removeEventListener", 2));
    JS_SetPropertyStr(ctx, doc, "getElementsByTagName",
                      JS_NewCFunction(ctx, doc_by_tag, "getElementsByTagName", 1));
    JS_SetPropertyStr(ctx, doc, "getElementsByClassName",
                      JS_NewCFunction(ctx, doc_by_class, "getElementsByClassName", 1));
    JS_SetPropertyStr(ctx, doc, "body", make_element(ctx, njs_dom_body(host)));
    JS_SetPropertyStr(ctx, doc, "documentElement", make_element(ctx, njs_dom_query(host, "html")));
    JS_SetPropertyStr(ctx, doc, "head", make_element(ctx, njs_dom_query(host, "head")));
    JS_SetPropertyStr(ctx, global, "document", doc);
    /* The page's own address goes on before the prelude runs, because the
     * prelude builds location out of it. */
    install_page_address(ctx, global, host);
    JS_FreeValue(ctx, global);
    JSValue r = JS_Eval(ctx, PRELUDE, __builtin_strlen(PRELUDE), "<prelude>", JS_EVAL_TYPE_GLOBAL);
    JS_FreeValue(ctx, r);
}
