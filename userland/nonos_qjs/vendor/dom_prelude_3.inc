/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * Prelude text, last part: classList, dataset, observers.
 */

    /* classList and dataset, built from the element's own attribute methods
     * so neither needs the host. classList is on nearly every page that has
     * a script at all, and dataset is how markup hands a value to one. */
    "globalThis.__njs_classlist=function(el){"
    "var g=function(){return String(el.className||'').split(/\\s+/).filter(Boolean);};"
    "var s=function(a){el.className=a.join(' ');};"
    "var o={contains:function(c){return g().indexOf(c)>=0;},"
    "add:function(){var a=g();for(var i=0;i<arguments.length;i++)"
    "if(a.indexOf(arguments[i])<0)a.push(arguments[i]);s(a);},"
    "remove:function(){var a=g();for(var i=0;i<arguments.length;i++){"
    "var k=a.indexOf(arguments[i]);if(k>=0)a.splice(k,1);}s(a);},"
    "toggle:function(c,f){var a=g(),k=a.indexOf(c);"
    "var on=f===undefined?k<0:!!f;if(on&&k<0)a.push(c);else if(!on&&k>=0)a.splice(k,1);"
    "s(a);return on;},"
    "replace:function(x,y){var a=g(),k=a.indexOf(x);if(k<0)return false;a[k]=y;s(a);return true;},"
    "item:function(i){return g()[i]||null;},"
    "toString:function(){return g().join(' ');}};"
    "Object.defineProperty(o,'length',{get:function(){return g().length;}});"
    "Object.defineProperty(o,'value',{get:function(){return g().join(' ');}});"
    "return o;};"
    /* dataset reads and writes through the attributes, so a value a script
     * sets is the one markup shows, and the kebab spelling is derived rather
     * than stored twice. */
    "globalThis.__njs_dataset=function(el){"
    "var kebab=function(p){return 'data-'+p.replace(/[A-Z]/g,function(c){"
    "return '-'+c.toLowerCase();});};"
    "var camel=function(a){return a.slice(5).replace(/-([a-z])/g,function(m,c){"
    "return c.toUpperCase();});};"
    "return new Proxy({},{get:function(t,p){if(typeof p!=='string')return undefined;"
    "var v=el.getAttribute(kebab(p));return v===null?undefined:v;},"
    "set:function(t,p,v){el.setAttribute(kebab(p),String(v));return true;},"
    "has:function(t,p){return el.hasAttribute(kebab(p));},"
    "deleteProperty:function(t,p){el.removeAttribute(kebab(p));return true;},"
    "ownKeys:function(){return el.getAttributeNames()"
    ".filter(function(a){return a.indexOf('data-')===0;}).map(camel);},"
    "getOwnPropertyDescriptor:function(t,p){return{enumerable:true,configurable:true,"
    "value:el.getAttribute(kebab(p))};}});};"
    /* Storage a page can actually use. Backed by a plain object, so it lasts
     * as long as the page rather than across a boot: a script that stores a
     * preference and reads it back in the same visit works, and nothing is
     * written to disk that the reader did not ask to keep. */
    "var mkstore=function(){var m={};return{getItem:function(k){"
    "return Object.prototype.hasOwnProperty.call(m,String(k))?m[String(k)]:null;},"
    "setItem:function(k,v){m[String(k)]=String(v);},"
    "removeItem:function(k){delete m[String(k)];},"
    "clear:function(){m={};},"
    "key:function(i){return Object.keys(m)[i]||null;},"
    "get length(){return Object.keys(m).length;}};};"
    "globalThis.localStorage=mkstore();globalThis.sessionStorage=mkstore();"
    "globalThis.MutationObserver=globalThis.MutationObserver||function(){"
    "this.observe=sink;this.disconnect=sink;this.takeRecords=function(){return[];};};"
    "if(globalThis.document){document.defaultView=globalThis;document.nodeType=9;}"
    /* window's own listeners, and the document's for the events of its
     * life. window had no addEventListener at all, so the most common line
     * of setup code there is, window.addEventListener('load', ...), threw
     * and took the rest of its script with it. The document's lifecycle
     * events are kept here too because they are fired from here; every
     * other type still goes to the native table that UI dispatch reads. */
    "var wl={},dl={},life={domcontentloaded:1,readystatechange:1,load:1,pageshow:1,"
    "visibilitychange:1};"
    "var keep=function(e){(globalThis.__njs_errors=globalThis.__njs_errors||[])"
    ".push(String(e)+' @ '+String(e&&e.stack||''));};"
    "var addTo=function(m,t,f,o){if(!f)return;var k=String(t).toLowerCase();"
    "var a=m[k]||(m[k]=[]);for(var i=0;i<a.length;i++)if(a[i].f===f)return;"
    "a.push({f:f,once:!!(o&&typeof o==='object'&&o.once)});};"
    "var dropFrom=function(m,t,f){var a=m[String(t).toLowerCase()];if(!a)return;"
    "for(var i=0;i<a.length;i++)if(a[i].f===f){a.splice(i,1);return;}};"
    "var fire=function(m,self,ev){var a=(m[String(ev.type).toLowerCase()]||[]).slice();"
    "ev.currentTarget=self;for(var i=0;i<a.length&&!ev.__stopnow;i++){var l=a[i];"
    "if(l.once)dropFrom(m,ev.type,l.f);"
    "try{if(typeof l.f==='function')l.f.call(self,ev);"
    "else if(l.f&&typeof l.f.handleEvent==='function')l.f.handleEvent(ev);}catch(e){keep(e);}}};"
    "var onprop=function(self,ev){var h=self['on'+String(ev.type).toLowerCase()];"
    "if(typeof h==='function'){try{h.call(self,ev);}catch(e){keep(e);}}};"
    /* The window was resized: a kept media query list whose answer changed
     * hears change, then window hears resize. */
    "globalThis.__njs_viewport_changed=function(){"
    "for(var i=0;i<mqls.length;i++){var m=mqls[i],now=m.matches;if(now===m.__was)continue;"
    "m.__was=now;var ev=new Event('change');ev.matches=now;ev.media=m.media;ev.target=m;"
    "ev.currentTarget=m;var a=m.__l.slice();"
    "for(var k=0;k<a.length;k++){try{if(typeof a[k]==='function')a[k].call(m,ev);"
    "else a[k].handleEvent(ev);}catch(e){keep(e);}}"
    "if(typeof m.onchange==='function'){try{m.onchange.call(m,ev);}catch(e){keep(e);}}}"
    "var r=new Event('resize');r.target=globalThis;fire(wl,globalThis,r);onprop(globalThis,r);};"
    /* How far the page is scrolled: down as the reader scrolled it, never
     * across, as the browser scrolls a page only up and down. */
    "Object.defineProperty(globalThis,'scrollY',{configurable:true,"
    "get:function(){return globalThis.__njs_scroll_y();}});"
    "Object.defineProperty(globalThis,'pageYOffset',{configurable:true,"
    "get:function(){return globalThis.__njs_scroll_y();}});"
    "globalThis.scrollX=0;globalThis.pageXOffset=0;"
    /* scrollTo(x, y), scroll(x, y), scrollBy(dx, dy), each also with
     * {top, left}: the host moves the page down to y (dom::script_scroll)
     * and scrollY reads it at once; the window follows when the script
     * returns, and scroll fires. They were not defined, so a "back to top"
     * button threw. Across is ignored, as this browser scrolls a page only
     * up and down; a call with no y moves nothing. */
    "var scrollAt=function(a,b,by){var y;"
    "if(a&&typeof a==='object'){if(a.top===undefined)return;y=+a.top;}"
    "else if(b===undefined)return;else y=+b;"
    "if(y!==y)y=0;y=Math.round(y);"
    "globalThis.__njs_scroll_to(by?globalThis.__njs_scroll_y()+y:y);};"
    "globalThis.scrollTo=globalThis.scroll=function(a,b){scrollAt(a,b,false);};"
    "globalThis.scrollBy=function(a,b){scrollAt(a,b,true);};"
    /* The root element scrolls the page, as in a standards mode document:
     * document.documentElement.scrollTop reads and sets scrollY. */
    "if(globalThis.document&&document.documentElement){var root=document.documentElement;"
    "document.scrollingElement=root;"
    "Object.defineProperty(root,'scrollTop',{configurable:true,"
    "get:function(){return globalThis.__njs_scroll_y();},"
    "set:function(v){scrollAt(0,v,false);}});}"
    /* alert, confirm and prompt cannot wait for the reader: a script runs
     * to its end in one go, on a time budget. Each hands the browser what
     * it asked (dom_dialog.inc), which says it to the reader, and answers
     * at once: alert nothing, confirm false and prompt null. Never yes, as
     * the reader agreed to nothing. A message is a string of anything. */
    "var asked=function(k,m){globalThis.__njs_dialog(k,m===undefined?'':String(m));};"
    "globalThis.alert=function(m){asked(0,m);};"
    "globalThis.confirm=function(m){asked(1,m);return false;};"
    "globalThis.prompt=function(m){asked(2,m);return null;};"
    "globalThis.addEventListener=function(t,f,o){addTo(wl,t,f,o);};"
    "globalThis.removeEventListener=function(t,f){dropFrom(wl,t,f);};"
    "globalThis.dispatchEvent=function(ev){if(!ev||!ev.type)return true;"
    "if(!ev.target)ev.target=globalThis;fire(wl,globalThis,ev);onprop(globalThis,ev);"
    "return !ev.defaultPrevented;};"
    /* readyState, and document.cookie read and written through the host's
     * jar, which keeps HttpOnly cookies out of a script's reach. */
    "if(globalThis.document){var d=document,nadd=d.addEventListener,"
    "nrem=d.removeEventListener,rs='loading';"
    "d.addEventListener=function(t,f,o){if(life[String(t).toLowerCase()])addTo(dl,t,f,o);"
    "else if(nadd)nadd.call(d,t,f,o);};"
    "d.removeEventListener=function(t,f,o){if(life[String(t).toLowerCase()])dropFrom(dl,t,f);"
    "else if(nrem)nrem.call(d,t,f,o);};"
    "Object.defineProperty(d,'readyState',{configurable:true,get:function(){return rs;}});"
    "Object.defineProperty(d,'cookie',{configurable:true,"
    "get:function(){return typeof globalThis.__njs_cookie_get==='function'?"
    "String(globalThis.__njs_cookie_get()):'';},"
    "set:function(v){if(typeof globalThis.__njs_cookie_set==='function')"
    "globalThis.__njs_cookie_set(String(v));}});"
    "d.hidden=false;d.visibilityState='visible';"
    /* The end of a UI event's bubble, after the document's own listeners
     * (njs_dispatch_event): document.on<type>, then window's listeners and
     * window.on<type>. Answers how many it called. */
    "globalThis.__njs_bubble_out=function(ev){var t=String(ev.type).toLowerCase(),k='on'+t,n=0;"
    "if(typeof d[k]==='function'){n++;ev.currentTarget=d;try{d[k].call(d,ev);}catch(e){keep(e);}}"
    "if(ev.__stop)return n;n+=(wl[t]||[]).length;if(typeof globalThis[k]==='function')n++;"
    "fire(wl,globalThis,ev);onprop(globalThis,ev);return n;};"
    "var mk=function(t,b){var e=new Event(t,{bubbles:b});e.target=d;return e;};"
    /* Called by the browser once every script in the page has run, then
     * again for load. Each step happens once and in order, and answers 1
     * when it did, so the browser knows listeners may have changed the
     * page. */
    "globalThis.__njs_ready=function(s){"
    "if(s==='interactive'&&rs==='loading'){rs='interactive';"
    "var r=mk('readystatechange',false);fire(dl,d,r);onprop(d,r);"
    "var c=mk('DOMContentLoaded',true);fire(dl,d,c);if(!c.__stop)fire(wl,globalThis,c);"
    "return 1;}"
    "if(s==='complete'&&rs==='interactive'){rs='complete';"
    "var r2=mk('readystatechange',false);fire(dl,d,r2);onprop(d,r2);"
    "var l=mk('load',false);fire(wl,globalThis,l);onprop(globalThis,l);"
    "var p=mk('pageshow',false);p.persisted=false;fire(wl,globalThis,p);onprop(globalThis,p);"
    "return 1;}"
    "return 0;};}"
    "})();";
