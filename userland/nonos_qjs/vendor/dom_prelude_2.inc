/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Prelude text, second part: location, history, events.
 */

    /* location is built from the address the page was actually fetched from.
     * It used to say http://localhost/ whatever had been loaded, so a page
     * that reads its own path to decide what to show, which is every page
     * with a router in it, was told something that was never true.
     *
     * Writing to it goes somewhere, as it does in every browser:
     * `location.href = url`, `location = url`, and a new pathname or search
     * navigate, and a new hash moves the address within the page and fires
     * hashchange. They were plain properties, so the most common way a
     * script sends the reader on, `location.href = url`, did nothing. */
    "globalThis.__njs_mkloc=function(u){u=u||'';"
    "var m=/^([a-z]+:)\\/\\/([^\\/?#]*)([^?#]*)(\\?[^#]*)?(#.*)?$/i.exec(u)||[];"
    "var proto=m[1]||'http:',hostport=m[2]||'',path=m[3]||'/',search=m[4]||'',hash=m[5]||'';"
    "var at=hostport.lastIndexOf(':');"
    "var hasport=at>0&&/^[0-9]+$/.test(hostport.slice(at+1));"
    "var go=function(h){globalThis.__njs_navigate(String(h));};"
    "var base=function(){return proto+'//'+hostport;};"
    "var l={protocol:proto,host:hostport,"
    "hostname:hasport?hostport.slice(0,at):hostport,"
    "port:hasport?hostport.slice(at+1):'',"
    "origin:proto+'//'+hostport,"
    "toString:function(){return u;},"
    "assign:go,replace:go,reload:function(){go(u);}};"
    "Object.defineProperty(l,'href',{enumerable:true,get:function(){return u;},set:go});"
    "Object.defineProperty(l,'pathname',{enumerable:true,get:function(){return path||'/';},"
    "set:function(p){p=String(p);go(base()+(p.charAt(0)==='/'?p:'/'+p)+search+hash);}});"
    "Object.defineProperty(l,'search',{enumerable:true,get:function(){return search;},"
    "set:function(q){q=String(q);if(q&&q.charAt(0)!=='?')q='?'+q;if(q==='?')q='';"
    "go(base()+(path||'/')+q+hash);}});"
    "Object.defineProperty(l,'hash',{enumerable:true,get:function(){return hash;},"
    "set:function(h){h=String(h);if(h&&h.charAt(0)!=='#')h='#'+h;if(h==='#')h='';"
    "if(h===hash)return;var old=u;hash=h;u=u.replace(/#.*$/,'')+h;"
    "if(typeof globalThis.dispatchEvent==='function'){var e=new Event('hashchange');"
    "e.oldURL=old;e.newURL=u;globalThis.dispatchEvent(e);}}});"
    "return l;};"
    "var loc=globalThis.__njs_mkloc(globalThis.__njs_base||'');"
    /* The page's own record of where it is, which pushState moves without
     * going anywhere; assigning location itself navigates. */
    "globalThis.__njs_setloc=function(l){loc=l;};"
    "Object.defineProperty(globalThis,'location',{configurable:true,enumerable:true,"
    "get:function(){return loc;},set:function(h){globalThis.__njs_navigate(String(h));}});"
    /* A page that pushes state is telling the reader it moved. The address
     * has to follow, or the next relative link resolves against the old one
     * and code reading the path after a route change sees the previous one. */
    "var pushed=0;"
    "globalThis.history={state:null,scrollRestoration:'auto',"
    "pushState:function(s,t,u){this.state=s;if(u)loc="
    "globalThis.__njs_mkloc(globalThis.__njs_resolve(String(u)));pushed++;},"
    "replaceState:function(s,t,u){this.state=s;if(u)loc="
    "globalThis.__njs_mkloc(globalThis.__njs_resolve(String(u)));},"
    /* back, forward and go step through the reader's history as the
     * toolbar's buttons do; go() with no step loads the page again. */
    "back:function(){globalThis.__njs_history_go(-1);},"
    "forward:function(){globalThis.__njs_history_go(1);},"
    "go:function(n){n=Math.trunc(+n||0);if(n)globalThis.__njs_history_go(n);"
    "else loc.reload();}};"
    /* length is the reader's session history as the browser keeps it
     * (__njs_history: 0 its entries, 1 the one shown), and each pushState
     * after the one shown, as a push drops what was ahead. It was 1 on
     * every page, so a page offering its own Back button only when there
     * is somewhere to go back to never offered it. */
    "Object.defineProperty(globalThis.history,'length',{enumerable:true,get:function(){"
    "var n=+globalThis.__njs_history(0)||0,i=+globalThis.__njs_history(1)||0;"
    "return Math.max(1,pushed?i+1+pushed:n);}});"
    "if(globalThis.document){document.baseURI=globalThis.__njs_base||'';"
    "Object.defineProperty(document,'URL',{configurable:true,get:function(){return loc.href;}});"
    "Object.defineProperty(document,'location',{configurable:true,"
    "get:function(){return loc;},set:function(h){globalThis.__njs_navigate(String(h));}});}"
    /* matchMedia answers from the browser's own @media judge at the page's
     * viewport, and `matches` is read afresh each time. It answered false to
     * everything, so a page on a wide window drew its phone layout. A list
     * with a listener is kept so a resize can tell it (__njs_viewport_changed);
     * one without is not, as pages ask matchMedia in loops. */
    "var mqls=[];"
    "globalThis.matchMedia=function(q){q=String(q);var l=[];var m={media:q,"
    "addListener:function(f){if(typeof f!=='function'&&!(f&&f.handleEvent))return;"
    "if(l.indexOf(f)<0)l.push(f);if(mqls.indexOf(m)<0){m.__was=m.matches;mqls.push(m);}},"
    "removeListener:function(f){var k=l.indexOf(f);if(k>=0)l.splice(k,1);},"
    "addEventListener:function(t,f){if(String(t)==='change')m.addListener(f);},"
    "removeEventListener:function(t,f){if(String(t)==='change')m.removeListener(f);},"
    "dispatchEvent:function(){return true;},__l:l};"
    "Object.defineProperty(m,'matches',{enumerable:true,"
    "get:function(){return !!globalThis.__njs_media_matches(q);}});"
    "Object.defineProperty(m,'onchange',{enumerable:true,"
    "get:function(){return m.__on||null;},set:function(f){m.__on=f;"
    "if(mqls.indexOf(m)<0){m.__was=m.matches;mqls.push(m);}}});"
    "return m;};"
    /* getComputedStyle reads the host's cascade: the common properties as
     * the page was styled and laid out, any other as the element's style
     * attribute declares it, else empty. It answered empty for everything.
     * Both spellings read: style.backgroundColor and
     * getPropertyValue('background-color'). */
    "globalThis.getComputedStyle=function(el){"
    "var n=el&&typeof el.__node==='number'?el.__node:-1;"
    "var read=function(p){return n<0?'':String(globalThis.__njs_computed(n,String(p)));};"
    "var o={getPropertyValue:read,getPropertyPriority:function(){return'';},"
    "item:function(){return'';},length:0,cssText:'',parentRule:null};"
    "return new Proxy(o,{get:function(t,p){"
    "if(typeof p!=='string'||p in t)return t[p];return read(p);}});};"
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
    "this.currentTarget=null;this.timeStamp=globalThis.performance?performance.now():0;"
    "this.isTrusted=false;"
    "this.preventDefault=function(){if(this.cancelable)this.defaultPrevented=true;};"
    "this.stopPropagation=function(){this.__stop=true;};"
    "this.stopImmediatePropagation=function(){this.__stop=true;this.__stopnow=true;};};"
    "globalThis.CustomEvent=function(t,o){o=o||{};Event.call(this,t,o);this.detail="
    "o.detail===undefined?null:o.detail;};"
    "globalThis.MouseEvent=globalThis.KeyboardEvent=globalThis.PointerEvent="
    "globalThis.InputEvent=globalThis.FocusEvent=Event;"
