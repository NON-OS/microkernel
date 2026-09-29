/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * The one JS object per DOM node.
 */

/* One JS object per DOM node, cached in a global registry. Frameworks hang
 * expando state off DOM nodes and compare them by identity, so every lookup
 * for the same node must return the same object. */
static JSValue make_element(JSContext *ctx, int node) {
    if (node < 0) return JS_NULL;
    JSValue global = JS_GetGlobalObject(ctx);
    JSValue reg = JS_GetPropertyStr(ctx, global, "__njs_nodes");
    if (!JS_IsObject(reg)) {
        JS_FreeValue(ctx, reg);
        reg = JS_NewObject(ctx);
        JS_SetPropertyStr(ctx, global, "__njs_nodes", JS_DupValue(ctx, reg));
    }
    JSValue cached = JS_GetPropertyUint32(ctx, reg, (uint32_t)node);
    if (JS_IsObject(cached)) {
        JS_FreeValue(ctx, reg);
        JS_FreeValue(ctx, global);
        return cached;
    }
    JS_FreeValue(ctx, cached);
    JSValue el = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, el, "__node", JS_NewInt32(ctx, node));
    /* Elements carry the on<event> IDL properties as null: frameworks probe
     * `'onclick' in dom` to decide event-name normalization. */
    static const char *onprops[] = {
        "onclick", "ondblclick", "oninput", "onchange", "onsubmit", "onkeydown", "onkeyup",
        "onkeypress", "onmousedown", "onmouseup", "onmousemove", "onmouseover", "onmouseout",
        "onmouseenter", "onmouseleave", "onfocus", "onblur", "onscroll", "ontouchstart",
        "ontouchend", "ontouchmove", "onpointerdown", "onpointerup", "onpointermove", 0,
    };
    for (int i = 0; onprops[i]; i++)
        JS_SetPropertyStr(ctx, el, onprops[i], JS_NULL);
    JS_SetPropertyStr(ctx, el, "appendChild", JS_NewCFunction(ctx, el_append_child, "appendChild", 1));
    JS_SetPropertyStr(ctx, el, "insertBefore", JS_NewCFunction(ctx, el_insert_before, "insertBefore", 2));
    JS_SetPropertyStr(ctx, el, "removeChild", JS_NewCFunction(ctx, el_remove_child, "removeChild", 1));
    JS_SetPropertyStr(ctx, el, "remove", JS_NewCFunction(ctx, el_remove, "remove", 0));
    JS_SetPropertyStr(ctx, el, "setAttribute", JS_NewCFunction(ctx, el_set_attribute, "setAttribute", 2));
    JS_SetPropertyStr(ctx, el, "getAttribute", JS_NewCFunction(ctx, el_get_attribute, "getAttribute", 1));
    JS_SetPropertyStr(ctx, el, "removeAttribute", JS_NewCFunction(ctx, el_remove_attribute, "removeAttribute", 1));
    JS_SetPropertyStr(ctx, el, "addEventListener", JS_NewCFunction(ctx, el_add_listener, "addEventListener", 2));
    accessor(ctx, el, "textContent", get_text_content, set_text_content);
    accessor(ctx, el, "data", get_text_content, set_text_content);
    accessor(ctx, el, "nodeValue", get_text_content, set_text_content);
    accessor(ctx, el, "className", get_class_name, set_class_name);
    accessor(ctx, el, "tagName", get_tag_name, 0);
    accessor(ctx, el, "localName", get_local_name, 0);
    accessor(ctx, el, "nodeName", get_node_name, 0);
    accessor(ctx, el, "nodeType", get_node_type, 0);
    accessor(ctx, el, "parentNode", get_parent_node, 0);
    accessor(ctx, el, "ownerDocument", get_owner_document, 0);
    accessor(ctx, el, "firstChild", get_first_child, 0);
    accessor(ctx, el, "nextSibling", get_next_sibling, 0);
    accessor(ctx, el, "childNodes", get_child_nodes, 0);
    accessor(ctx, el, "style", get_style, 0);
    accessor(ctx, el, "innerHTML", get_inner_html, set_inner_html);
    install_element_ext(ctx, el);
    install_query_ext(ctx, el);
    install_event_ext(ctx, el);
    JS_SetPropertyUint32(ctx, reg, (uint32_t)node, JS_DupValue(ctx, el));
    JS_FreeValue(ctx, reg);
    return el;
}
