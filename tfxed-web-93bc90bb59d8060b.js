let J=1,N=`string`,C=`undefined`,G=0,P=4,F=null,K=3,M=`number`,D=`utf-8`,O=`Object`,I=`function`,E=Error,L=FinalizationRegistry,S=Object,R=Object.getPrototypeOf,Q=globalThis,H=undefined;var y=((a,b)=>{});var n=(()=>{if(m===F||m.buffer.detached===!0||m.buffer.detached===H&&m.buffer!==a.memory.buffer){m=new DataView(a.memory.buffer)};return m});var q=(a=>{const b=typeof a;if(b==M||b==`boolean`||a==F){return `${a}`};if(b==N){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==F){return `Symbol`}else{return `Symbol(${b})`}};if(b==I){const b=a.name;if(typeof b==N&&b.length>G){return `Function(${b})`}else{return `Function`}};if(Array.isArray(a)){const b=a.length;let c=`[`;if(b>G){c+=q(a[G])};for(let d=J;d<b;d++){c+=`, `+ q(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c&&c.length>J){d=c[J]}else{return toString.call(a)};if(d==O){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return O}};if(a instanceof E){return `${a.name}: ${a.message}\n${a.stack}`};return d});var p=((b,c,d,e)=>{const f={a:b,b:c,cnt:J,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=G;try{return e(c,f.b,...b)}finally{if(--f.cnt===G){a.__wbindgen_export_5.get(f.dtor)(c,f.b);o.unregister(f)}else{f.a=c}}};g.original=f;o.register(g,f,f);return g});var u=((b,c)=>{a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h720fa791d94c831a(b,c)});function g(b,c){try{return b.apply(this,c)}catch(b){const c=f(b);a.__wbindgen_exn_store(c)}}var e=((a,c)=>{a=a>>>G;return b.decode(d().subarray(a,a+ c))});var x=(()=>{const b={};b.wbg={};b.wbg.__wbg_addEventListener_90e553fdce254421=function(){return g(((a,b,c,d)=>{a.addEventListener(e(b,c),d)}),arguments)};b.wbg.__wbg_altKey_c33c03aed82e4275=(a=>{const b=a.altKey;return b});b.wbg.__wbg_appendChild_8204974b7328bf98=function(){return g(((a,b)=>{const c=a.appendChild(b);return c}),arguments)};b.wbg.__wbg_body_942ea927546a04ba=(a=>{const b=a.body;return h(b)?G:f(b)});b.wbg.__wbg_call_672a4d21634d4a24=function(){return g(((a,b)=>{const c=a.call(b);return c}),arguments)};b.wbg.__wbg_createElement_8c9931a732ee2fea=function(){return g(((a,b,c)=>{const d=a.createElement(e(b,c));return d}),arguments)};b.wbg.__wbg_ctrlKey_1e826e468105ac11=(a=>{const b=a.ctrlKey;return b});b.wbg.__wbg_document_d249400bd7bd996d=(a=>{const b=a.document;return h(b)?G:f(b)});b.wbg.__wbg_getElementById_f827f0d6648718a8=((a,b,c)=>{const d=a.getElementById(e(b,c));return h(d)?G:f(d)});b.wbg.__wbg_height_614ba187d8cae9ca=function(){return g((a=>{const b=a.height;return b}),arguments)};b.wbg.__wbg_innerHeight_05f4225d754a7929=function(){return g((a=>{const b=a.innerHeight;return b}),arguments)};b.wbg.__wbg_innerWidth_7e0498dbd876d498=function(){return g((a=>{const b=a.innerWidth;return b}),arguments)};b.wbg.__wbg_instanceof_Window_def73ea0955fc569=(a=>{let b;try{b=a instanceof Window}catch(a){b=!1}const c=b;return c});b.wbg.__wbg_key_7b5c6cb539be8e13=((b,c)=>{const d=c.key;const e=l(d,a.__wbindgen_malloc,a.__wbindgen_realloc);const f=i;n().setInt32(b+ P*J,f,!0);n().setInt32(b+ P*G,e,!0)});b.wbg.__wbg_navigator_1577371c070c8947=(a=>{const b=a.navigator;return b});b.wbg.__wbg_newnoargs_105ed471475aaf50=((a,b)=>{const c=new Function(e(a,b));return c});b.wbg.__wbg_now_2c95c9de01293173=(a=>{const b=a.now();return b});b.wbg.__wbg_now_807e54c39636c349=(()=>{const a=Date.now();return a});b.wbg.__wbg_performance_7a3ffd0b17f663ad=(a=>{const b=a.performance;return b});b.wbg.__wbg_requestAnimationFrame_d7fd890aaefc3246=function(){return g(((a,b)=>{const c=a.requestAnimationFrame(b);return c}),arguments)};b.wbg.__wbg_screen_8edf8699f70d98bc=function(){return g((a=>{const b=a.screen;return b}),arguments)};b.wbg.__wbg_setAttribute_2704501201f15687=function(){return g(((a,b,c,d,f)=>{a.setAttribute(e(b,c),e(d,f))}),arguments)};b.wbg.__wbg_setinnerHTML_31bde41f835786f7=((a,b,c)=>{a.innerHTML=e(b,c)});b.wbg.__wbg_setonresize_376a647b3cba80a2=((a,b)=>{a.onresize=b});b.wbg.__wbg_shiftKey_86e737105bab1a54=(a=>{const b=a.shiftKey;return b});b.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07=(()=>{const a=typeof global===C?F:global;return h(a)?G:f(a)});b.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0=(()=>{const a=typeof Q===C?F:Q;return h(a)?G:f(a)});b.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819=(()=>{const a=typeof self===C?F:self;return h(a)?G:f(a)});b.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40=(()=>{const a=typeof window===C?F:window;return h(a)?G:f(a)});b.wbg.__wbg_userAgent_12e9d8e62297563f=function(){return g(((b,c)=>{const d=c.userAgent;const e=l(d,a.__wbindgen_malloc,a.__wbindgen_realloc);const f=i;n().setInt32(b+ P*J,f,!0);n().setInt32(b+ P*G,e,!0)}),arguments)};b.wbg.__wbg_width_679079836447b4b7=function(){return g((a=>{const b=a.width;return b}),arguments)};b.wbg.__wbindgen_cb_drop=(a=>{const b=a.original;if(b.cnt--==J){b.a=G;return !0};const c=!1;return c});b.wbg.__wbindgen_closure_wrapper1584=((a,b,c)=>{const d=p(a,b,1004,v);return d});b.wbg.__wbindgen_closure_wrapper51=((a,b,c)=>{const d=p(a,b,P,t);return d});b.wbg.__wbindgen_closure_wrapper52=((a,b,c)=>{const d=p(a,b,P,u);return d});b.wbg.__wbindgen_debug_string=((b,c)=>{const d=q(c);const e=l(d,a.__wbindgen_malloc,a.__wbindgen_realloc);const f=i;n().setInt32(b+ P*J,f,!0);n().setInt32(b+ P*G,e,!0)});b.wbg.__wbindgen_init_externref_table=(()=>{const b=a.__wbindgen_export_2;const c=b.grow(P);b.set(G,H);b.set(c+ G,H);b.set(c+ J,F);b.set(c+ 2,!0);b.set(c+ K,!1)});b.wbg.__wbindgen_is_undefined=(a=>{const b=a===H;return b});b.wbg.__wbindgen_number_get=((a,b)=>{const c=b;const d=typeof c===M?c:H;n().setFloat64(a+ 8*J,h(d)?G:d,!0);n().setInt32(a+ P*G,!h(d),!0)});b.wbg.__wbindgen_throw=((a,b)=>{throw new E(e(a,b))});return b});var t=((b,c,d)=>{a.closure6_externref_shim(b,c,d)});var v=((b,c,d)=>{a.closure1003_externref_shim(b,c,d)});var A=(b=>{if(a!==H)return a;if(typeof b!==C){if(R(b)===S.prototype){({module:b}=b)}else{console.warn(`using deprecated parameters for \`initSync()\`; pass a single object instead`)}};const c=x();y(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return z(d,b)});var l=((a,b,c)=>{if(c===H){const c=j.encode(a);const e=b(c.length,J)>>>G;d().subarray(e,e+ c.length).set(c);i=c.length;return e};let e=a.length;let f=b(e,J)>>>G;const g=d();let h=G;for(;h<e;h++){const b=a.charCodeAt(h);if(b>127)break;g[f+ h]=b};if(h!==e){if(h!==G){a=a.slice(h)};f=c(f,e,e=h+ a.length*K,J)>>>G;const b=d().subarray(f+ h,f+ e);const g=k(a,b);h+=g.written;f=c(f,e,h,J)>>>G};i=h;return f});var w=(async(a,b)=>{if(typeof Response===I&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===I){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve Wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var z=((b,d)=>{a=b.exports;B.__wbindgen_wasm_module=d;m=F;c=F;a.__wbindgen_start();return a});var d=(()=>{if(c===F||c.byteLength===G){c=new Uint8Array(a.memory.buffer)};return c});var B=(async(b)=>{if(a!==H)return a;if(typeof b!==C){if(R(b)===S.prototype){({module_or_path:b}=b)}else{console.warn(`using deprecated parameters for the initialization function; pass a single object instead`)}};if(typeof b===C){b=new URL(`tfxed-web_bg.wasm`,import.meta.url)};const c=x();if(typeof b===N||typeof Request===I&&b instanceof Request||typeof URL===I&&b instanceof URL){b=fetch(b)};y(c);const {instance:d,module:e}=await w(await b,c);return z(d,e)});var h=(a=>a===H||a===F);var f=(b=>{const c=a.__externref_table_alloc();a.__wbindgen_export_2.set(c,b);return c});let a;const b=typeof TextDecoder!==C?new TextDecoder(D,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw E(`TextDecoder not available`)}};if(typeof TextDecoder!==C){b.decode()};let c=F;let i=G;const j=typeof TextEncoder!==C?new TextEncoder(D):{encode:()=>{throw E(`TextEncoder not available`)}};const k=typeof j.encodeInto===I?((a,b)=>j.encodeInto(a,b)):((a,b)=>{const c=j.encode(a);b.set(c);return {read:a.length,written:c.length}});let m=F;const o=typeof L===C?{register:()=>{},unregister:()=>{}}:new L(b=>{a.__wbindgen_export_5.get(b.dtor)(b.a,b.b)});function r(b){const c=l(b,a.__wbindgen_malloc,a.__wbindgen_realloc);const d=i;a.compile_dsl(c,d)}function s(b){const c=l(b,a.__wbindgen_malloc,a.__wbindgen_realloc);const d=i;a.update_canvas(c,d)}export default B;export{r as compile_dsl,s as update_canvas,A as initSync}