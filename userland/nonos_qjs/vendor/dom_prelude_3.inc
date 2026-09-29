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
    "})();";
