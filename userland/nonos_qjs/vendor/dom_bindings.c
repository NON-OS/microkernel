/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * The QuickJS <-> DOM glue. A DOM element is a JS object carrying a hidden
 * __node integer (its index in the host DOM). Mutating methods and property
 * accessors route through njs_dom_* callbacks the host implements over its real
 * node tree. The host pointer travels as the context opaque.
 */
#include "quickjs.h"

void free(void *);

#include "dom_host.inc"
#include "dom_el_tree.inc"
#include "dom_el_props.inc"
#include "dom_listen.inc"
#include "dom_event_obj.inc"
#include "dom_dispatch.inc"
#include "dom_ui_event.inc"
#include "dom_style.inc"
#include "dom_nav.inc"
#include "dom_ext.inc"
#include "dom_query.inc"
#include "dom_query_list.inc"
#include "dom_query_doc.inc"
#include "dom_cookie.inc"
#include "dom_dialog.inc"
#include "dom_events.inc"
#include "dom_make.inc"
#include "dom_doc.inc"
#include "dom_prelude.inc"
#include "dom_install.inc"
