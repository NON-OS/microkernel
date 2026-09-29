/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Prelude text, second part: location, history, events.
 */

    /* location is built from the address the page was actually fetched from.
     * It used to say http://localhost/ whatever had been loaded, so a page
     * that reads its own path to decide what to show, which is every page
     * with a router in it, was told something that was never true. */
    "globalThis.__njs_mkloc=function(u){"
    "var m=/^([a-z]+:)\\/\\/([^\\/?#]*)([^?#]*)(\\?[^#]*)?(#.*)?$/i.exec(u||'')||[];"
    "var proto=m[1]||'http:',hostport=m[2]||'',path=m[3]||'/';"
    "var at=hostport.lastIndexOf(':');"
    "var hasport=at>0&&/^[0-9]+$/.test(hostport.slice(at+1));"
    "return{href:u||'',protocol:proto,host:hostport,"
    "hostname:hasport?hostport.slice(0,at):hostport,"
    "port:hasport?hostport.slice(at+1):'',"
    "pathname:path||'/',search:m[4]||'',hash:m[5]||'',"
    "origin:proto+'//'+hostport,"
    "toString:function(){return this.href;},"
    "assign:function(h){globalThis.__njs_navigate(String(h));},"
    "replace:function(h){globalThis.__njs_navigate(String(h));},"
    "reload:function(){globalThis.__njs_navigate(this.href);}};};"
    "globalThis.location=globalThis.__njs_mkloc(globalThis.__njs_base||'');"
    /* A page that pushes state is telling the reader it moved. The address
     * has to follow, or the next relative link resolves against the old one
     * and code reading the path after a route change sees the previous one. */
    "globalThis.history={length:1,state:null,scrollRestoration:'auto',"
    "pushState:function(s,t,u){this.state=s;if(u)globalThis.location="
    "globalThis.__njs_mkloc(globalThis.__njs_resolve(String(u)));this.length++;},"
    "replaceState:function(s,t,u){this.state=s;if(u)globalThis.location="
    "globalThis.__njs_mkloc(globalThis.__njs_resolve(String(u)));},"
    "back:sink,forward:sink,go:sink};"
    "if(globalThis.document){document.baseURI=globalThis.__njs_base||'';"
    "document.URL=document.baseURI;document.location=globalThis.location;}"
    "globalThis.matchMedia=function(){return{matches:false,media:'',addListener:sink,"
    "removeListener:sink,addEventListener:sink,removeEventListener:sink};};"
    "globalThis.getComputedStyle=function(){return{getPropertyValue:function(){return'';}};};"
    "globalThis.Node=globalThis.Node||function(){};"
    "globalThis.Element=globalThis.Element||function(){};"
    "globalThis.HTMLElement=globalThis.HTMLElement||function(){};"
    "globalThis.HTMLIFrameElement=globalThis.HTMLIFrameElement||function(){};"
    "globalThis.SVGElement=globalThis.SVGElement||function(){};"
    "globalThis.Document=globalThis.Document||function(){};"
    "globalThis.Text=globalThis.Text||function(){};"
    "globalThis.Comment=globalThis.Comment||function(){};"
    /* An event a script builds has to carry what it was built with. These
     * were empty functions, so `new CustomEvent('x',{detail:d})` produced an
     * object with no type and no detail, and the handler that received it
     * could not tell what had happened. */
    "globalThis.Event=function(t,o){o=o||{};this.type=t;this.bubbles=!!o.bubbles;"
    "this.cancelable=!!o.cancelable;this.defaultPrevented=false;this.target=null;"
    "this.currentTarget=null;this.timeStamp=0;this.isTrusted=false;"
    "this.preventDefault=function(){if(this.cancelable)this.defaultPrevented=true;};"
    "this.stopPropagation=function(){this.__stop=true;};"
    "this.stopImmediatePropagation=function(){this.__stop=true;this.__stopnow=true;};};"
    "globalThis.CustomEvent=function(t,o){o=o||{};Event.call(this,t,o);this.detail="
    "o.detail===undefined?null:o.detail;};"
    "globalThis.MouseEvent=globalThis.KeyboardEvent=globalThis.PointerEvent="
    "globalThis.InputEvent=globalThis.FocusEvent=Event;"
