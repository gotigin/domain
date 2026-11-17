(()=>{var Te=()=>{};var xe=function(e){let t=[],n=0;for(let r=0;r<e.length;r++){let i=e.charCodeAt(r);i<128?t[n++]=i:i<2048?(t[n++]=i>>6|192,t[n++]=i&63|128):(i&64512)===55296&&r+1<e.length&&(e.charCodeAt(r+1)&64512)===56320?(i=65536+((i&1023)<<10)+(e.charCodeAt(++r)&1023),t[n++]=i>>18|240,t[n++]=i>>12&63|128,t[n++]=i>>6&63|128,t[n++]=i&63|128):(t[n++]=i>>12|224,t[n++]=i>>6&63|128,t[n++]=i&63|128)}return t},St=function(e){let t=[],n=0,r=0;for(;n<e.length;){let i=e[n++];if(i<128)t[r++]=String.fromCharCode(i);else if(i>191&&i<224){let s=e[n++];t[r++]=String.fromCharCode((i&31)<<6|s&63)}else if(i>239&&i<365){let s=e[n++],o=e[n++],a=e[n++],c=((i&7)<<18|(s&63)<<12|(o&63)<<6|a&63)-65536;t[r++]=String.fromCharCode(55296+(c>>10)),t[r++]=String.fromCharCode(56320+(c&1023))}else{let s=e[n++],o=e[n++];t[r++]=String.fromCharCode((i&15)<<12|(s&63)<<6|o&63)}}return t.join("")},ke={byteToCharMap_:null,charToByteMap_:null,byteToCharMapWebSafe_:null,charToByteMapWebSafe_:null,ENCODED_VALS_BASE:"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",get ENCODED_VALS(){return this.ENCODED_VALS_BASE+"+/="},get ENCODED_VALS_WEBSAFE(){return this.ENCODED_VALS_BASE+"-_."},HAS_NATIVE_SUPPORT:typeof atob=="function",encodeByteArray(e,t){if(!Array.isArray(e))throw Error("encodeByteArray takes an array as a parameter");this.init_();let n=t?this.byteToCharMapWebSafe_:this.byteToCharMap_,r=[];for(let i=0;i<e.length;i+=3){let s=e[i],o=i+1<e.length,a=o?e[i+1]:0,c=i+2<e.length,l=c?e[i+2]:0,f=s>>2,h=(s&3)<<4|a>>4,I=(a&15)<<2|l>>6,p=l&63;c||(p=64,o||(I=64)),r.push(n[f],n[h],n[I],n[p])}return r.join("")},encodeString(e,t){return this.HAS_NATIVE_SUPPORT&&!t?btoa(e):this.encodeByteArray(xe(e),t)},decodeString(e,t){return this.HAS_NATIVE_SUPPORT&&!t?atob(e):St(this.decodeStringToByteArray(e,t))},decodeStringToByteArray(e,t){this.init_();let n=t?this.charToByteMapWebSafe_:this.charToByteMap_,r=[];for(let i=0;i<e.length;){let s=n[e.charAt(i++)],a=i<e.length?n[e.charAt(i)]:0;++i;let l=i<e.length?n[e.charAt(i)]:64;++i;let h=i<e.length?n[e.charAt(i)]:64;if(++i,s==null||a==null||l==null||h==null)throw new W;let I=s<<2|a>>4;if(r.push(I),l!==64){let p=a<<4&240|l>>2;if(r.push(p),h!==64){let At=l<<6&192|h;r.push(At)}}}return r},init_(){if(!this.byteToCharMap_){this.byteToCharMap_={},this.charToByteMap_={},this.byteToCharMapWebSafe_={},this.charToByteMapWebSafe_={};for(let e=0;e<this.ENCODED_VALS.length;e++)this.byteToCharMap_[e]=this.ENCODED_VALS.charAt(e),this.charToByteMap_[this.byteToCharMap_[e]]=e,this.byteToCharMapWebSafe_[e]=this.ENCODED_VALS_WEBSAFE.charAt(e),this.charToByteMapWebSafe_[this.byteToCharMapWebSafe_[e]]=e,e>=this.ENCODED_VALS_BASE.length&&(this.charToByteMap_[this.ENCODED_VALS_WEBSAFE.charAt(e)]=e,this.charToByteMapWebSafe_[this.ENCODED_VALS.charAt(e)]=e)}}},W=class extends Error{constructor(){super(...arguments),this.name="DecodeBase64StringError"}},Dt=function(e){let t=xe(e);return ke.encodeByteArray(t,!0)},q=function(e){return Dt(e).replace(/\./g,"")},Re=function(e){try{return ke.decodeString(e,!0)}catch(t){console.error("base64Decode failed: ",t)}return null};function Tt(){if(typeof self<"u")return self;if(typeof window<"u")return window;if(typeof global<"u")return global;throw new Error("Unable to locate global object.")}var Ot=()=>Tt().__FIREBASE_DEFAULTS__,xt=()=>{if(typeof process>"u"||typeof process.env>"u")return;let e=process.env.__FIREBASE_DEFAULTS__;if(e)return JSON.parse(e)},kt=()=>{if(typeof document>"u")return;let e;try{e=document.cookie.match(/__FIREBASE_DEFAULTS__=([^;]+)/)}catch{return}let t=e&&Re(e[1]);return t&&JSON.parse(t)},Rt=()=>{try{return Te()||Ot()||xt()||kt()}catch(e){console.info(`Unable to get __FIREBASE_DEFAULTS__ due to: ${e}`);return}};var G=()=>{var e;return(e=Rt())===null||e===void 0?void 0:e.config};var N=class{constructor(){this.reject=()=>{},this.resolve=()=>{},this.promise=new Promise((t,n)=>{this.resolve=t,this.reject=n})}wrapCallback(t){return(n,r)=>{n?this.reject(n):this.resolve(r),typeof t=="function"&&(this.promise.catch(()=>{}),t.length===1?t(n):t(n,r))}}};function K(){let e=typeof chrome=="object"?chrome.runtime:typeof browser=="object"?browser.runtime:void 0;return typeof e=="object"&&e.id!==void 0}function R(){try{return typeof indexedDB=="object"}catch{return!1}}function B(){return new Promise((e,t)=>{try{let n=!0,r="validate-browser-context-for-indexeddb-analytics-module",i=self.indexedDB.open(r);i.onsuccess=()=>{i.result.close(),n||self.indexedDB.deleteDatabase(r),e(!0)},i.onupgradeneeded=()=>{n=!1},i.onerror=()=>{var s;t(((s=i.error)===null||s===void 0?void 0:s.message)||"")}}catch(n){t(n)}})}function J(){return!(typeof navigator>"u"||!navigator.cookieEnabled)}var Bt="FirebaseError",b=class e extends Error{constructor(t,n,r){super(n),this.code=t,this.customData=r,this.name=Bt,Object.setPrototypeOf(this,e.prototype),Error.captureStackTrace&&Error.captureStackTrace(this,v.prototype.create)}},v=class{constructor(t,n,r){this.service=t,this.serviceName=n,this.errors=r}create(t,...n){let r=n[0]||{},i=`${this.service}/${t}`,s=this.errors[t],o=s?Mt(s,r):"Error",a=`${this.serviceName}: ${o} (${i}).`;return new b(i,a,r)}};function Mt(e,t){return e.replace(Pt,(n,r)=>{let i=t[r];return i!=null?String(i):`<${r}?>`})}var Pt=/\{\$([^}]+)}/g;function O(e,t){if(e===t)return!0;let n=Object.keys(e),r=Object.keys(t);for(let i of n){if(!r.includes(i))return!1;let s=e[i],o=t[i];if(Oe(s)&&Oe(o)){if(!O(s,o))return!1}else if(s!==o)return!1}for(let i of r)if(!n.includes(i))return!1;return!0}function Oe(e){return e!==null&&typeof e=="object"}var Nt=1e3,Lt=2,$t=14400*1e3,Ft=.5;function Y(e,t=Nt,n=Lt){let r=t*Math.pow(n,e),i=Math.round(Ft*r*(Math.random()-.5)*2);return Math.min($t,r+i)}function Z(e){return e&&e._delegate?e._delegate:e}var m=class{constructor(t,n,r){this.name=t,this.instanceFactory=n,this.type=r,this.multipleInstances=!1,this.serviceProps={},this.instantiationMode="LAZY",this.onInstanceCreated=null}setInstantiationMode(t){return this.instantiationMode=t,this}setMultipleInstances(t){return this.multipleInstances=t,this}setServiceProps(t){return this.serviceProps=t,this}setInstanceCreatedCallback(t){return this.onInstanceCreated=t,this}};var A="[DEFAULT]";var X=class{constructor(t,n){this.name=t,this.container=n,this.component=null,this.instances=new Map,this.instancesDeferred=new Map,this.instancesOptions=new Map,this.onInitCallbacks=new Map}get(t){let n=this.normalizeInstanceIdentifier(t);if(!this.instancesDeferred.has(n)){let r=new N;if(this.instancesDeferred.set(n,r),this.isInitialized(n)||this.shouldAutoInitialize())try{let i=this.getOrInitializeService({instanceIdentifier:n});i&&r.resolve(i)}catch{}}return this.instancesDeferred.get(n).promise}getImmediate(t){var n;let r=this.normalizeInstanceIdentifier(t?.identifier),i=(n=t?.optional)!==null&&n!==void 0?n:!1;if(this.isInitialized(r)||this.shouldAutoInitialize())try{return this.getOrInitializeService({instanceIdentifier:r})}catch(s){if(i)return null;throw s}else{if(i)return null;throw Error(`Service ${this.name} is not available`)}}getComponent(){return this.component}setComponent(t){if(t.name!==this.name)throw Error(`Mismatching Component ${t.name} for Provider ${this.name}.`);if(this.component)throw Error(`Component for ${this.name} has already been provided`);if(this.component=t,!!this.shouldAutoInitialize()){if(Ht(t))try{this.getOrInitializeService({instanceIdentifier:A})}catch{}for(let[n,r]of this.instancesDeferred.entries()){let i=this.normalizeInstanceIdentifier(n);try{let s=this.getOrInitializeService({instanceIdentifier:i});r.resolve(s)}catch{}}}}clearInstance(t=A){this.instancesDeferred.delete(t),this.instancesOptions.delete(t),this.instances.delete(t)}async delete(){let t=Array.from(this.instances.values());await Promise.all([...t.filter(n=>"INTERNAL"in n).map(n=>n.INTERNAL.delete()),...t.filter(n=>"_delete"in n).map(n=>n._delete())])}isComponentSet(){return this.component!=null}isInitialized(t=A){return this.instances.has(t)}getOptions(t=A){return this.instancesOptions.get(t)||{}}initialize(t={}){let{options:n={}}=t,r=this.normalizeInstanceIdentifier(t.instanceIdentifier);if(this.isInitialized(r))throw Error(`${this.name}(${r}) has already been initialized`);if(!this.isComponentSet())throw Error(`Component ${this.name} has not been registered yet`);let i=this.getOrInitializeService({instanceIdentifier:r,options:n});for(let[s,o]of this.instancesDeferred.entries()){let a=this.normalizeInstanceIdentifier(s);r===a&&o.resolve(i)}return i}onInit(t,n){var r;let i=this.normalizeInstanceIdentifier(n),s=(r=this.onInitCallbacks.get(i))!==null&&r!==void 0?r:new Set;s.add(t),this.onInitCallbacks.set(i,s);let o=this.instances.get(i);return o&&t(o,i),()=>{s.delete(t)}}invokeOnInitCallbacks(t,n){let r=this.onInitCallbacks.get(n);if(r)for(let i of r)try{i(t,n)}catch{}}getOrInitializeService({instanceIdentifier:t,options:n={}}){let r=this.instances.get(t);if(!r&&this.component&&(r=this.component.instanceFactory(this.container,{instanceIdentifier:jt(t),options:n}),this.instances.set(t,r),this.instancesOptions.set(t,n),this.invokeOnInitCallbacks(r,t),this.component.onInstanceCreated))try{this.component.onInstanceCreated(this.container,t,r)}catch{}return r||null}normalizeInstanceIdentifier(t=A){return this.component?this.component.multipleInstances?t:A:t}shouldAutoInitialize(){return!!this.component&&this.component.instantiationMode!=="EXPLICIT"}};function jt(e){return e===A?void 0:e}function Ht(e){return e.instantiationMode==="EAGER"}var L=class{constructor(t){this.name=t,this.providers=new Map}addComponent(t){let n=this.getProvider(t.name);if(n.isComponentSet())throw new Error(`Component ${t.name} has already been registered with ${this.name}`);n.setComponent(t)}addOrOverwriteComponent(t){this.getProvider(t.name).isComponentSet()&&this.providers.delete(t.name),this.addComponent(t)}getProvider(t){if(this.providers.has(t))return this.providers.get(t);let n=new X(t,this);return this.providers.set(t,n),n}getProviders(){return Array.from(this.providers.values())}};var zt=[],u;(function(e){e[e.DEBUG=0]="DEBUG",e[e.VERBOSE=1]="VERBOSE",e[e.INFO=2]="INFO",e[e.WARN=3]="WARN",e[e.ERROR=4]="ERROR",e[e.SILENT=5]="SILENT"})(u||(u={}));var Ut={debug:u.DEBUG,verbose:u.VERBOSE,info:u.INFO,warn:u.WARN,error:u.ERROR,silent:u.SILENT},Vt=u.INFO,Wt={[u.DEBUG]:"log",[u.VERBOSE]:"log",[u.INFO]:"info",[u.WARN]:"warn",[u.ERROR]:"error"},qt=(e,t,...n)=>{if(t<e.logLevel)return;let r=new Date().toISOString(),i=Wt[t];if(i)console[i](`[${r}]  ${e.name}:`,...n);else throw new Error(`Attempted to log a message with an invalid logType (value: ${t})`)},x=class{constructor(t){this.name=t,this._logLevel=Vt,this._logHandler=qt,this._userLogHandler=null,zt.push(this)}get logLevel(){return this._logLevel}set logLevel(t){if(!(t in u))throw new TypeError(`Invalid value "${t}" assigned to \`logLevel\``);this._logLevel=t}setLogLevel(t){this._logLevel=typeof t=="string"?Ut[t]:t}get logHandler(){return this._logHandler}set logHandler(t){if(typeof t!="function")throw new TypeError("Value assigned to `logHandler` must be a function");this._logHandler=t}get userLogHandler(){return this._userLogHandler}set userLogHandler(t){this._userLogHandler=t}debug(...t){this._userLogHandler&&this._userLogHandler(this,u.DEBUG,...t),this._logHandler(this,u.DEBUG,...t)}log(...t){this._userLogHandler&&this._userLogHandler(this,u.VERBOSE,...t),this._logHandler(this,u.VERBOSE,...t)}info(...t){this._userLogHandler&&this._userLogHandler(this,u.INFO,...t),this._logHandler(this,u.INFO,...t)}warn(...t){this._userLogHandler&&this._userLogHandler(this,u.WARN,...t),this._logHandler(this,u.WARN,...t)}error(...t){this._userLogHandler&&this._userLogHandler(this,u.ERROR,...t),this._logHandler(this,u.ERROR,...t)}};var Gt=(e,t)=>t.some(n=>e instanceof n),Be,Me;function Kt(){return Be||(Be=[IDBDatabase,IDBObjectStore,IDBIndex,IDBCursor,IDBTransaction])}function Jt(){return Me||(Me=[IDBCursor.prototype.advance,IDBCursor.prototype.continue,IDBCursor.prototype.continuePrimaryKey])}var Pe=new WeakMap,ee=new WeakMap,Ne=new WeakMap,Q=new WeakMap,ne=new WeakMap;function Yt(e){let t=new Promise((n,r)=>{let i=()=>{e.removeEventListener("success",s),e.removeEventListener("error",o)},s=()=>{n(w(e.result)),i()},o=()=>{r(e.error),i()};e.addEventListener("success",s),e.addEventListener("error",o)});return t.then(n=>{n instanceof IDBCursor&&Pe.set(n,e)}).catch(()=>{}),ne.set(t,e),t}function Zt(e){if(ee.has(e))return;let t=new Promise((n,r)=>{let i=()=>{e.removeEventListener("complete",s),e.removeEventListener("error",o),e.removeEventListener("abort",o)},s=()=>{n(),i()},o=()=>{r(e.error||new DOMException("AbortError","AbortError")),i()};e.addEventListener("complete",s),e.addEventListener("error",o),e.addEventListener("abort",o)});ee.set(e,t)}var te={get(e,t,n){if(e instanceof IDBTransaction){if(t==="done")return ee.get(e);if(t==="objectStoreNames")return e.objectStoreNames||Ne.get(e);if(t==="store")return n.objectStoreNames[1]?void 0:n.objectStore(n.objectStoreNames[0])}return w(e[t])},set(e,t,n){return e[t]=n,!0},has(e,t){return e instanceof IDBTransaction&&(t==="done"||t==="store")?!0:t in e}};function Le(e){te=e(te)}function Xt(e){return e===IDBDatabase.prototype.transaction&&!("objectStoreNames"in IDBTransaction.prototype)?function(t,...n){let r=e.call($(this),t,...n);return Ne.set(r,t.sort?t.sort():[t]),w(r)}:Jt().includes(e)?function(...t){return e.apply($(this),t),w(Pe.get(this))}:function(...t){return w(e.apply($(this),t))}}function Qt(e){return typeof e=="function"?Xt(e):(e instanceof IDBTransaction&&Zt(e),Gt(e,Kt())?new Proxy(e,te):e)}function w(e){if(e instanceof IDBRequest)return Yt(e);if(Q.has(e))return Q.get(e);let t=Qt(e);return t!==e&&(Q.set(e,t),ne.set(t,e)),t}var $=e=>ne.get(e);function F(e,t,{blocked:n,upgrade:r,blocking:i,terminated:s}={}){let o=indexedDB.open(e,t),a=w(o);return r&&o.addEventListener("upgradeneeded",c=>{r(w(o.result),c.oldVersion,c.newVersion,w(o.transaction),c)}),n&&o.addEventListener("blocked",c=>n(c.oldVersion,c.newVersion,c)),a.then(c=>{s&&c.addEventListener("close",()=>s()),i&&c.addEventListener("versionchange",l=>i(l.oldVersion,l.newVersion,l))}).catch(()=>{}),a}var en=["get","getKey","getAll","getAllKeys","count"],tn=["put","add","delete","clear"],re=new Map;function $e(e,t){if(!(e instanceof IDBDatabase&&!(t in e)&&typeof t=="string"))return;if(re.get(t))return re.get(t);let n=t.replace(/FromIndex$/,""),r=t!==n,i=tn.includes(n);if(!(n in(r?IDBIndex:IDBObjectStore).prototype)||!(i||en.includes(n)))return;let s=async function(o,...a){let c=this.transaction(o,i?"readwrite":"readonly"),l=c.store;return r&&(l=l.index(a.shift())),(await Promise.all([l[n](...a),i&&c.done]))[0]};return re.set(t,s),s}Le(e=>({...e,get:(t,n,r)=>$e(t,n)||e.get(t,n,r),has:(t,n)=>!!$e(t,n)||e.has(t,n)}));var se=class{constructor(t){this.container=t}getPlatformInfoString(){return this.container.getProviders().map(n=>{if(nn(n)){let r=n.getImmediate();return`${r.library}/${r.version}`}else return null}).filter(n=>n).join(" ")}};function nn(e){let t=e.getComponent();return t?.type==="VERSION"}var oe="@firebase/app",Fe="0.13.2";var _=new x("@firebase/app"),rn="@firebase/app-compat",sn="@firebase/analytics-compat",on="@firebase/analytics",an="@firebase/app-check-compat",cn="@firebase/app-check",ln="@firebase/auth",un="@firebase/auth-compat",fn="@firebase/database",dn="@firebase/data-connect",hn="@firebase/database-compat",pn="@firebase/functions",mn="@firebase/functions-compat",gn="@firebase/installations",bn="@firebase/installations-compat",yn="@firebase/messaging",wn="@firebase/messaging-compat",In="@firebase/performance",vn="@firebase/performance-compat",_n="@firebase/remote-config",En="@firebase/remote-config-compat",Cn="@firebase/storage",An="@firebase/storage-compat",Sn="@firebase/firestore",Dn="@firebase/ai",Tn="@firebase/firestore-compat",On="firebase";var ae="[DEFAULT]",xn={[oe]:"fire-core",[rn]:"fire-core-compat",[on]:"fire-analytics",[sn]:"fire-analytics-compat",[cn]:"fire-app-check",[an]:"fire-app-check-compat",[ln]:"fire-auth",[un]:"fire-auth-compat",[fn]:"fire-rtdb",[dn]:"fire-data-connect",[hn]:"fire-rtdb-compat",[pn]:"fire-fn",[mn]:"fire-fn-compat",[gn]:"fire-iid",[bn]:"fire-iid-compat",[yn]:"fire-fcm",[wn]:"fire-fcm-compat",[In]:"fire-perf",[vn]:"fire-perf-compat",[_n]:"fire-rc",[En]:"fire-rc-compat",[Cn]:"fire-gcs",[An]:"fire-gcs-compat",[Sn]:"fire-fst",[Tn]:"fire-fst-compat",[Dn]:"fire-vertex","fire-js":"fire-js",[On]:"fire-js-all"};var j=new Map,kn=new Map,ce=new Map;function je(e,t){try{e.container.addComponent(t)}catch(n){_.debug(`Component ${t.name} failed to register with FirebaseApp ${e.name}`,n)}}function E(e){let t=e.name;if(ce.has(t))return _.debug(`There were multiple attempts to register component ${t}.`),!1;ce.set(t,e);for(let n of j.values())je(n,e);for(let n of kn.values())je(n,e);return!0}function k(e,t){let n=e.container.getProvider("heartbeat").getImmediate({optional:!0});return n&&n.triggerHeartbeat(),e.container.getProvider(t)}var Rn={"no-app":"No Firebase App '{$appName}' has been created - call initializeApp() first","bad-app-name":"Illegal App name: '{$appName}'","duplicate-app":"Firebase App named '{$appName}' already exists with different options or config","app-deleted":"Firebase App named '{$appName}' already deleted","server-app-deleted":"Firebase Server App has been deleted","no-options":"Need to provide options, when not being deployed to hosting via source.","invalid-app-argument":"firebase.{$appName}() takes either no argument or a Firebase App instance.","invalid-log-argument":"First argument to `onLog` must be null or a function.","idb-open":"Error thrown when opening IndexedDB. Original error: {$originalErrorMessage}.","idb-get":"Error thrown when reading from IndexedDB. Original error: {$originalErrorMessage}.","idb-set":"Error thrown when writing to IndexedDB. Original error: {$originalErrorMessage}.","idb-delete":"Error thrown when deleting from IndexedDB. Original error: {$originalErrorMessage}.","finalization-registry-not-supported":"FirebaseServerApp deleteOnDeref field defined but the JS runtime does not support FinalizationRegistry.","invalid-server-app-environment":"FirebaseServerApp is not for use in browser environments."},C=new v("app","Firebase",Rn);var le=class{constructor(t,n,r){this._isDeleted=!1,this._options=Object.assign({},t),this._config=Object.assign({},n),this._name=n.name,this._automaticDataCollectionEnabled=n.automaticDataCollectionEnabled,this._container=r,this.container.addComponent(new m("app",()=>this,"PUBLIC"))}get automaticDataCollectionEnabled(){return this.checkDestroyed(),this._automaticDataCollectionEnabled}set automaticDataCollectionEnabled(t){this.checkDestroyed(),this._automaticDataCollectionEnabled=t}get name(){return this.checkDestroyed(),this._name}get options(){return this.checkDestroyed(),this._options}get config(){return this.checkDestroyed(),this._config}get container(){return this._container}get isDeleted(){return this._isDeleted}set isDeleted(t){this._isDeleted=t}checkDestroyed(){if(this.isDeleted)throw C.create("app-deleted",{appName:this._name})}};function de(e,t={}){let n=e;typeof t!="object"&&(t={name:t});let r=Object.assign({name:ae,automaticDataCollectionEnabled:!0},t),i=r.name;if(typeof i!="string"||!i)throw C.create("bad-app-name",{appName:String(i)});if(n||(n=G()),!n)throw C.create("no-options");let s=j.get(i);if(s){if(O(n,s.options)&&O(r,s.config))return s;throw C.create("duplicate-app",{appName:i})}let o=new L(i);for(let c of ce.values())o.addComponent(c);let a=new le(n,r,o);return j.set(i,a),a}function he(e=ae){let t=j.get(e);if(!t&&e===ae&&G())return de();if(!t)throw C.create("no-app",{appName:e});return t}function y(e,t,n){var r;let i=(r=xn[e])!==null&&r!==void 0?r:e;n&&(i+=`-${n}`);let s=i.match(/\s|\//),o=t.match(/\s|\//);if(s||o){let a=[`Unable to register library "${i}" with version "${t}":`];s&&a.push(`library name "${i}" contains illegal characters (whitespace or "/")`),s&&o&&a.push("and"),o&&a.push(`version name "${t}" contains illegal characters (whitespace or "/")`),_.warn(a.join(" "));return}E(new m(`${i}-version`,()=>({library:i,version:t}),"VERSION"))}var Bn="firebase-heartbeat-database",Mn=1,M="firebase-heartbeat-store",ie=null;function Ve(){return ie||(ie=F(Bn,Mn,{upgrade:(e,t)=>{switch(t){case 0:try{e.createObjectStore(M)}catch(n){console.warn(n)}}}}).catch(e=>{throw C.create("idb-open",{originalErrorMessage:e.message})})),ie}async function Pn(e){try{let n=(await Ve()).transaction(M),r=await n.objectStore(M).get(We(e));return await n.done,r}catch(t){if(t instanceof b)_.warn(t.message);else{let n=C.create("idb-get",{originalErrorMessage:t?.message});_.warn(n.message)}}}async function He(e,t){try{let r=(await Ve()).transaction(M,"readwrite");await r.objectStore(M).put(t,We(e)),await r.done}catch(n){if(n instanceof b)_.warn(n.message);else{let r=C.create("idb-set",{originalErrorMessage:n?.message});_.warn(r.message)}}}function We(e){return`${e.name}!${e.options.appId}`}var Nn=1024,Ln=30,ue=class{constructor(t){this.container=t,this._heartbeatsCache=null;let n=this.container.getProvider("app").getImmediate();this._storage=new fe(n),this._heartbeatsCachePromise=this._storage.read().then(r=>(this._heartbeatsCache=r,r))}async triggerHeartbeat(){var t,n;try{let i=this.container.getProvider("platform-logger").getImmediate().getPlatformInfoString(),s=ze();if(((t=this._heartbeatsCache)===null||t===void 0?void 0:t.heartbeats)==null&&(this._heartbeatsCache=await this._heartbeatsCachePromise,((n=this._heartbeatsCache)===null||n===void 0?void 0:n.heartbeats)==null)||this._heartbeatsCache.lastSentHeartbeatDate===s||this._heartbeatsCache.heartbeats.some(o=>o.date===s))return;if(this._heartbeatsCache.heartbeats.push({date:s,agent:i}),this._heartbeatsCache.heartbeats.length>Ln){let o=Fn(this._heartbeatsCache.heartbeats);this._heartbeatsCache.heartbeats.splice(o,1)}return this._storage.overwrite(this._heartbeatsCache)}catch(r){_.warn(r)}}async getHeartbeatsHeader(){var t;try{if(this._heartbeatsCache===null&&await this._heartbeatsCachePromise,((t=this._heartbeatsCache)===null||t===void 0?void 0:t.heartbeats)==null||this._heartbeatsCache.heartbeats.length===0)return"";let n=ze(),{heartbeatsToSend:r,unsentEntries:i}=$n(this._heartbeatsCache.heartbeats),s=q(JSON.stringify({version:2,heartbeats:r}));return this._heartbeatsCache.lastSentHeartbeatDate=n,i.length>0?(this._heartbeatsCache.heartbeats=i,await this._storage.overwrite(this._heartbeatsCache)):(this._heartbeatsCache.heartbeats=[],this._storage.overwrite(this._heartbeatsCache)),s}catch(n){return _.warn(n),""}}};function ze(){return new Date().toISOString().substring(0,10)}function $n(e,t=Nn){let n=[],r=e.slice();for(let i of e){let s=n.find(o=>o.agent===i.agent);if(s){if(s.dates.push(i.date),Ue(n)>t){s.dates.pop();break}}else if(n.push({agent:i.agent,dates:[i.date]}),Ue(n)>t){n.pop();break}r=r.slice(1)}return{heartbeatsToSend:n,unsentEntries:r}}var fe=class{constructor(t){this.app=t,this._canUseIndexedDBPromise=this.runIndexedDBEnvironmentCheck()}async runIndexedDBEnvironmentCheck(){return R()?B().then(()=>!0).catch(()=>!1):!1}async read(){if(await this._canUseIndexedDBPromise){let n=await Pn(this.app);return n?.heartbeats?n:{heartbeats:[]}}else return{heartbeats:[]}}async overwrite(t){var n;if(await this._canUseIndexedDBPromise){let i=await this.read();return He(this.app,{lastSentHeartbeatDate:(n=t.lastSentHeartbeatDate)!==null&&n!==void 0?n:i.lastSentHeartbeatDate,heartbeats:t.heartbeats})}else return}async add(t){var n;if(await this._canUseIndexedDBPromise){let i=await this.read();return He(this.app,{lastSentHeartbeatDate:(n=t.lastSentHeartbeatDate)!==null&&n!==void 0?n:i.lastSentHeartbeatDate,heartbeats:[...i.heartbeats,...t.heartbeats]})}else return}};function Ue(e){return q(JSON.stringify({version:2,heartbeats:e})).length}function Fn(e){if(e.length===0)return-1;let t=0,n=e[0].date;for(let r=1;r<e.length;r++)e[r].date<n&&(n=e[r].date,t=r);return t}function jn(e){E(new m("platform-logger",t=>new se(t),"PRIVATE")),E(new m("heartbeat",t=>new ue(t),"PRIVATE")),y(oe,Fe,e),y(oe,Fe,"esm2017"),y("fire-js","")}jn("");var Hn="firebase",zn="11.10.0";y(Hn,zn,"app");var Ke="@firebase/installations",be="0.6.18";var Je=1e4,Ye=`w:${be}`,Ze="FIS_v2",Un="https://firebaseinstallations.googleapis.com/v1",Vn=3600*1e3,Wn="installations",qn="Installations";var Gn={"missing-app-config-values":'Missing App configuration value: "{$valueName}"',"not-registered":"Firebase Installation is not registered.","installation-not-found":"Firebase Installation not found.","request-failed":'{$requestName} request failed with error "{$serverCode} {$serverStatus}: {$serverMessage}"',"app-offline":"Could not process request. Application offline.","delete-pending-registration":"Can't delete installation while there is a pending registration request."},D=new v(Wn,qn,Gn);function Xe(e){return e instanceof b&&e.code.includes("request-failed")}function Qe({projectId:e}){return`${Un}/projects/${e}/installations`}function et(e){return{token:e.token,requestStatus:2,expiresIn:Jn(e.expiresIn),creationTime:Date.now()}}async function tt(e,t){let r=(await t.json()).error;return D.create("request-failed",{requestName:e,serverCode:r.code,serverMessage:r.message,serverStatus:r.status})}function nt({apiKey:e}){return new Headers({"Content-Type":"application/json",Accept:"application/json","x-goog-api-key":e})}function Kn(e,{refreshToken:t}){let n=nt(e);return n.append("Authorization",Yn(t)),n}async function rt(e){let t=await e();return t.status>=500&&t.status<600?e():t}function Jn(e){return Number(e.replace("s","000"))}function Yn(e){return`${Ze} ${e}`}async function Zn({appConfig:e,heartbeatServiceProvider:t},{fid:n}){let r=Qe(e),i=nt(e),s=t.getImmediate({optional:!0});if(s){let l=await s.getHeartbeatsHeader();l&&i.append("x-firebase-client",l)}let o={fid:n,authVersion:Ze,appId:e.appId,sdkVersion:Ye},a={method:"POST",headers:i,body:JSON.stringify(o)},c=await rt(()=>fetch(r,a));if(c.ok){let l=await c.json();return{fid:l.fid||n,registrationStatus:2,refreshToken:l.refreshToken,authToken:et(l.authToken)}}else throw await tt("Create Installation",c)}function it(e){return new Promise(t=>{setTimeout(t,e)})}function Xn(e){return btoa(String.fromCharCode(...e)).replace(/\+/g,"-").replace(/\//g,"_")}var Qn=/^[cdef][\w-]{21}$/,ge="";function er(){try{let e=new Uint8Array(17);(self.crypto||self.msCrypto).getRandomValues(e),e[0]=112+e[0]%16;let n=tr(e);return Qn.test(n)?n:ge}catch{return ge}}function tr(e){return Xn(e).substr(0,22)}function z(e){return`${e.appName}!${e.appId}`}var st=new Map;function ot(e,t){let n=z(e);at(n,t),nr(n,t)}function at(e,t){let n=st.get(e);if(n)for(let r of n)r(t)}function nr(e,t){let n=rr();n&&n.postMessage({key:e,fid:t}),ir()}var S=null;function rr(){return!S&&"BroadcastChannel"in self&&(S=new BroadcastChannel("[Firebase] FID Change"),S.onmessage=e=>{at(e.data.key,e.data.fid)}),S}function ir(){st.size===0&&S&&(S.close(),S=null)}var sr="firebase-installations-database",or=1,T="firebase-installations-store",pe=null;function ye(){return pe||(pe=F(sr,or,{upgrade:(e,t)=>{switch(t){case 0:e.createObjectStore(T)}}})),pe}async function H(e,t){let n=z(e),i=(await ye()).transaction(T,"readwrite"),s=i.objectStore(T),o=await s.get(n);return await s.put(t,n),await i.done,(!o||o.fid!==t.fid)&&ot(e,t.fid),t}async function ct(e){let t=z(e),r=(await ye()).transaction(T,"readwrite");await r.objectStore(T).delete(t),await r.done}async function U(e,t){let n=z(e),i=(await ye()).transaction(T,"readwrite"),s=i.objectStore(T),o=await s.get(n),a=t(o);return a===void 0?await s.delete(n):await s.put(a,n),await i.done,a&&(!o||o.fid!==a.fid)&&ot(e,a.fid),a}async function we(e){let t,n=await U(e.appConfig,r=>{let i=ar(r),s=cr(e,i);return t=s.registrationPromise,s.installationEntry});return n.fid===ge?{installationEntry:await t}:{installationEntry:n,registrationPromise:t}}function ar(e){let t=e||{fid:er(),registrationStatus:0};return lt(t)}function cr(e,t){if(t.registrationStatus===0){if(!navigator.onLine){let i=Promise.reject(D.create("app-offline"));return{installationEntry:t,registrationPromise:i}}let n={fid:t.fid,registrationStatus:1,registrationTime:Date.now()},r=lr(e,n);return{installationEntry:n,registrationPromise:r}}else return t.registrationStatus===1?{installationEntry:t,registrationPromise:ur(e)}:{installationEntry:t}}async function lr(e,t){try{let n=await Zn(e,t);return H(e.appConfig,n)}catch(n){throw Xe(n)&&n.customData.serverCode===409?await ct(e.appConfig):await H(e.appConfig,{fid:t.fid,registrationStatus:0}),n}}async function ur(e){let t=await qe(e.appConfig);for(;t.registrationStatus===1;)await it(100),t=await qe(e.appConfig);if(t.registrationStatus===0){let{installationEntry:n,registrationPromise:r}=await we(e);return r||n}return t}function qe(e){return U(e,t=>{if(!t)throw D.create("installation-not-found");return lt(t)})}function lt(e){return fr(e)?{fid:e.fid,registrationStatus:0}:e}function fr(e){return e.registrationStatus===1&&e.registrationTime+Je<Date.now()}async function dr({appConfig:e,heartbeatServiceProvider:t},n){let r=hr(e,n),i=Kn(e,n),s=t.getImmediate({optional:!0});if(s){let l=await s.getHeartbeatsHeader();l&&i.append("x-firebase-client",l)}let o={installation:{sdkVersion:Ye,appId:e.appId}},a={method:"POST",headers:i,body:JSON.stringify(o)},c=await rt(()=>fetch(r,a));if(c.ok){let l=await c.json();return et(l)}else throw await tt("Generate Auth Token",c)}function hr(e,{fid:t}){return`${Qe(e)}/${t}/authTokens:generate`}async function Ie(e,t=!1){let n,r=await U(e.appConfig,s=>{if(!ut(s))throw D.create("not-registered");let o=s.authToken;if(!t&&gr(o))return s;if(o.requestStatus===1)return n=pr(e,t),s;{if(!navigator.onLine)throw D.create("app-offline");let a=yr(s);return n=mr(e,a),a}});return n?await n:r.authToken}async function pr(e,t){let n=await Ge(e.appConfig);for(;n.authToken.requestStatus===1;)await it(100),n=await Ge(e.appConfig);let r=n.authToken;return r.requestStatus===0?Ie(e,t):r}function Ge(e){return U(e,t=>{if(!ut(t))throw D.create("not-registered");let n=t.authToken;return wr(n)?Object.assign(Object.assign({},t),{authToken:{requestStatus:0}}):t})}async function mr(e,t){try{let n=await dr(e,t),r=Object.assign(Object.assign({},t),{authToken:n});return await H(e.appConfig,r),n}catch(n){if(Xe(n)&&(n.customData.serverCode===401||n.customData.serverCode===404))await ct(e.appConfig);else{let r=Object.assign(Object.assign({},t),{authToken:{requestStatus:0}});await H(e.appConfig,r)}throw n}}function ut(e){return e!==void 0&&e.registrationStatus===2}function gr(e){return e.requestStatus===2&&!br(e)}function br(e){let t=Date.now();return t<e.creationTime||e.creationTime+e.expiresIn<t+Vn}function yr(e){let t={requestStatus:1,requestTime:Date.now()};return Object.assign(Object.assign({},e),{authToken:t})}function wr(e){return e.requestStatus===1&&e.requestTime+Je<Date.now()}async function Ir(e){let t=e,{installationEntry:n,registrationPromise:r}=await we(t);return r?r.catch(console.error):Ie(t).catch(console.error),n.fid}async function vr(e,t=!1){let n=e;return await _r(n),(await Ie(n,t)).token}async function _r(e){let{registrationPromise:t}=await we(e);t&&await t}function Er(e){if(!e||!e.options)throw me("App Configuration");if(!e.name)throw me("App Name");let t=["projectId","apiKey","appId"];for(let n of t)if(!e.options[n])throw me(n);return{appName:e.name,projectId:e.options.projectId,apiKey:e.options.apiKey,appId:e.options.appId}}function me(e){return D.create("missing-app-config-values",{valueName:e})}var ft="installations",Cr="installations-internal",Ar=e=>{let t=e.getProvider("app").getImmediate(),n=Er(t),r=k(t,"heartbeat");return{app:t,appConfig:n,heartbeatServiceProvider:r,_delete:()=>Promise.resolve()}},Sr=e=>{let t=e.getProvider("app").getImmediate(),n=k(t,ft).getImmediate();return{getId:()=>Ir(n),getToken:i=>vr(n,i)}};function Dr(){E(new m(ft,Ar,"PUBLIC")),E(new m(Cr,Sr,"PRIVATE"))}Dr();y(Ke,be);y(Ke,be,"esm2017");var V="analytics",Tr="firebase_id",Or="origin",xr=60*1e3,kr="https://firebase.googleapis.com/v1alpha/projects/-/apps/{app-id}/webConfig",De="https://www.googletagmanager.com/gtag/js";var d=new x("@firebase/analytics");var Rr={"already-exists":"A Firebase Analytics instance with the appId {$id}  already exists. Only one Firebase Analytics instance can be created for each appId.","already-initialized":"initializeAnalytics() cannot be called again with different options than those it was initially called with. It can be called again with the same options to return the existing instance, or getAnalytics() can be used to get a reference to the already-initialized instance.","already-initialized-settings":"Firebase Analytics has already been initialized.settings() must be called before initializing any Analytics instanceor it will have no effect.","interop-component-reg-failed":"Firebase Analytics Interop Component failed to instantiate: {$reason}","invalid-analytics-context":"Firebase Analytics is not supported in this environment. Wrap initialization of analytics in analytics.isSupported() to prevent initialization in unsupported environments. Details: {$errorInfo}","indexeddb-unavailable":"IndexedDB unavailable or restricted in this environment. Wrap initialization of analytics in analytics.isSupported() to prevent initialization in unsupported environments. Details: {$errorInfo}","fetch-throttle":"The config fetch request timed out while in an exponential backoff state. Unix timestamp in milliseconds when fetch request throttling ends: {$throttleEndTimeMillis}.","config-fetch-failed":"Dynamic config fetch failed: [{$httpStatus}] {$responseMessage}","no-api-key":'The "apiKey" field is empty in the local Firebase config. Firebase Analytics requires this field tocontain a valid API key.',"no-app-id":'The "appId" field is empty in the local Firebase config. Firebase Analytics requires this field tocontain a valid app ID.',"no-client-id":'The "client_id" field is empty.',"invalid-gtag-resource":"Trusted Types detected an invalid gtag resource: {$gtagURL}."},g=new v("analytics","Analytics",Rr);function Br(e){if(!e.startsWith(De)){let t=g.create("invalid-gtag-resource",{gtagURL:e});return d.warn(t.message),""}return e}function yt(e){return Promise.all(e.map(t=>t.catch(n=>n)))}function Mr(e,t){let n;return window.trustedTypes&&(n=window.trustedTypes.createPolicy(e,t)),n}function Pr(e,t){let n=Mr("firebase-js-sdk-policy",{createScriptURL:Br}),r=document.createElement("script"),i=`${De}?l=${e}&id=${t}`;r.src=n?n?.createScriptURL(i):i,r.async=!0,document.head.appendChild(r)}function Nr(e){let t=[];return Array.isArray(window[e])?t=window[e]:window[e]=t,t}async function Lr(e,t,n,r,i,s){let o=r[i];try{if(o)await t[o];else{let c=(await yt(n)).find(l=>l.measurementId===i);c&&await t[c.appId]}}catch(a){d.error(a)}e("config",i,s)}async function $r(e,t,n,r,i){try{let s=[];if(i&&i.send_to){let o=i.send_to;Array.isArray(o)||(o=[o]);let a=await yt(n);for(let c of o){let l=a.find(h=>h.measurementId===c),f=l&&t[l.appId];if(f)s.push(f);else{s=[];break}}}s.length===0&&(s=Object.values(t)),await Promise.all(s),e("event",r,i||{})}catch(s){d.error(s)}}function Fr(e,t,n,r){async function i(s,...o){try{if(s==="event"){let[a,c]=o;await $r(e,t,n,a,c)}else if(s==="config"){let[a,c]=o;await Lr(e,t,n,r,a,c)}else if(s==="consent"){let[a,c]=o;e("consent",a,c)}else if(s==="get"){let[a,c,l]=o;e("get",a,c,l)}else if(s==="set"){let[a]=o;e("set",a)}else e(s,...o)}catch(a){d.error(a)}}return i}function jr(e,t,n,r,i){let s=function(...o){window[r].push(arguments)};return window[i]&&typeof window[i]=="function"&&(s=window[i]),window[i]=Fr(s,e,t,n),{gtagCore:s,wrappedGtag:window[i]}}function Hr(e){let t=window.document.getElementsByTagName("script");for(let n of Object.values(t))if(n.src&&n.src.includes(De)&&n.src.includes(e))return n;return null}var zr=30,Ur=1e3,_e=class{constructor(t={},n=Ur){this.throttleMetadata=t,this.intervalMillis=n}getThrottleMetadata(t){return this.throttleMetadata[t]}setThrottleMetadata(t,n){this.throttleMetadata[t]=n}deleteThrottleMetadata(t){delete this.throttleMetadata[t]}},wt=new _e;function Vr(e){return new Headers({Accept:"application/json","x-goog-api-key":e})}async function Wr(e){var t;let{appId:n,apiKey:r}=e,i={method:"GET",headers:Vr(r)},s=kr.replace("{app-id}",n),o=await fetch(s,i);if(o.status!==200&&o.status!==304){let a="";try{let c=await o.json();!((t=c.error)===null||t===void 0)&&t.message&&(a=c.error.message)}catch{}throw g.create("config-fetch-failed",{httpStatus:o.status,responseMessage:a})}return o.json()}async function qr(e,t=wt,n){let{appId:r,apiKey:i,measurementId:s}=e.options;if(!r)throw g.create("no-app-id");if(!i){if(s)return{measurementId:s,appId:r};throw g.create("no-api-key")}let o=t.getThrottleMetadata(r)||{backoffCount:0,throttleEndTimeMillis:Date.now()},a=new Ee;return setTimeout(async()=>{a.abort()},n!==void 0?n:xr),It({appId:r,apiKey:i,measurementId:s},o,a,t)}async function It(e,{throttleEndTimeMillis:t,backoffCount:n},r,i=wt){var s;let{appId:o,measurementId:a}=e;try{await Gr(r,t)}catch(c){if(a)return d.warn(`Timed out fetching this Firebase app's measurement ID from the server. Falling back to the measurement ID ${a} provided in the "measurementId" field in the local Firebase config. [${c?.message}]`),{appId:o,measurementId:a};throw c}try{let c=await Wr(e);return i.deleteThrottleMetadata(o),c}catch(c){let l=c;if(!Kr(l)){if(i.deleteThrottleMetadata(o),a)return d.warn(`Failed to fetch this Firebase app's measurement ID from the server. Falling back to the measurement ID ${a} provided in the "measurementId" field in the local Firebase config. [${l?.message}]`),{appId:o,measurementId:a};throw c}let f=Number((s=l?.customData)===null||s===void 0?void 0:s.httpStatus)===503?Y(n,i.intervalMillis,zr):Y(n,i.intervalMillis),h={throttleEndTimeMillis:Date.now()+f,backoffCount:n+1};return i.setThrottleMetadata(o,h),d.debug(`Calling attemptFetch again in ${f} millis`),It(e,h,r,i)}}function Gr(e,t){return new Promise((n,r)=>{let i=Math.max(t-Date.now(),0),s=setTimeout(n,i);e.addEventListener(()=>{clearTimeout(s),r(g.create("fetch-throttle",{throttleEndTimeMillis:t}))})})}function Kr(e){if(!(e instanceof b)||!e.customData)return!1;let t=Number(e.customData.httpStatus);return t===429||t===500||t===503||t===504}var Ee=class{constructor(){this.listeners=[]}addEventListener(t){this.listeners.push(t)}abort(){this.listeners.forEach(t=>t())}};var Ce;async function Jr(e,t,n,r,i){if(i&&i.global){e("event",n,r);return}else{let s=await t,o=Object.assign(Object.assign({},r),{send_to:s});e("event",n,o)}}var Ae;function Yr(e){Ae=e}function Zr(e){Ce=e}async function Xr(){if(R())try{await B()}catch(e){return d.warn(g.create("indexeddb-unavailable",{errorInfo:e?.toString()}).message),!1}else return d.warn(g.create("indexeddb-unavailable",{errorInfo:"IndexedDB is not available in this environment."}).message),!1;return!0}async function Qr(e,t,n,r,i,s,o){var a;let c=qr(e);c.then(p=>{n[p.measurementId]=p.appId,e.options.measurementId&&p.measurementId!==e.options.measurementId&&d.warn(`The measurement ID in the local Firebase config (${e.options.measurementId}) does not match the measurement ID fetched from the server (${p.measurementId}). To ensure analytics events are always sent to the correct Analytics property, update the measurement ID field in the local config or remove it from the local config.`)}).catch(p=>d.error(p)),t.push(c);let l=Xr().then(p=>{if(p)return r.getId()}),[f,h]=await Promise.all([c,l]);Hr(s)||Pr(s,f.measurementId),Ae&&(i("consent","default",Ae),Yr(void 0)),i("js",new Date);let I=(a=o?.config)!==null&&a!==void 0?a:{};return I[Or]="firebase",I.update=!0,h!=null&&(I[Tr]=h),i("config",f.measurementId,I),Ce&&(i("set",Ce),Zr(void 0)),f.measurementId}var Se=class{constructor(t){this.app=t}_delete(){return delete P[this.app.options.appId],Promise.resolve()}},P={},dt=[],ht={},ve="dataLayer",ei="gtag",pt,vt,mt=!1;function ti(){let e=[];if(K()&&e.push("This is a browser extension environment."),J()||e.push("Cookies are not available."),e.length>0){let t=e.map((r,i)=>`(${i+1}) ${r}`).join(" "),n=g.create("invalid-analytics-context",{errorInfo:t});d.warn(n.message)}}function ni(e,t,n){ti();let r=e.options.appId;if(!r)throw g.create("no-app-id");if(!e.options.apiKey)if(e.options.measurementId)d.warn(`The "apiKey" field is empty in the local Firebase config. This is needed to fetch the latest measurement ID for this Firebase app. Falling back to the measurement ID ${e.options.measurementId} provided in the "measurementId" field in the local Firebase config.`);else throw g.create("no-api-key");if(P[r]!=null)throw g.create("already-exists",{id:r});if(!mt){Nr(ve);let{wrappedGtag:s,gtagCore:o}=jr(P,dt,ht,ve,ei);vt=s,pt=o,mt=!0}return P[r]=Qr(e,dt,ht,t,pt,ve,n),new Se(e)}function _t(e=he()){e=Z(e);let t=k(e,V);return t.isInitialized()?t.getImmediate():ri(e)}function ri(e,t={}){let n=k(e,V);if(n.isInitialized()){let i=n.getImmediate();if(O(t,n.getOptions()))return i;throw g.create("already-initialized")}return n.initialize({options:t})}async function Et(){if(K()||!J()||!R())return!1;try{return await B()}catch{return!1}}function ii(e,t,n,r){e=Z(e),Jr(vt,P[e.app.options.appId],t,n,r).catch(i=>d.error(i))}var gt="@firebase/analytics",bt="0.10.17";function si(){E(new m(V,(t,{options:n})=>{let r=t.getProvider("app").getImmediate(),i=t.getProvider("installations-internal").getImmediate();return ni(r,i,n)},"PUBLIC")),E(new m("analytics-internal",e,"PRIVATE")),y(gt,bt),y(gt,bt,"esm2017");function e(t){try{let n=t.getProvider(V).getImmediate();return{logEvent:(r,i,s)=>ii(n,r,i,s)}}catch(n){throw g.create("interop-component-reg-failed",{reason:n})}}}si();var Ct={apiKey:"AIzaSyC5l2wOeUVAloLE6PmjtkROmDYyoeN2tyM",authDomain:"gotigin-com.firebaseapp.com",projectId:"gotigin-com",storageBucket:"gotigin-com.firebasestorage.app",messagingSenderId:"1019994128565",appId:"1:1019994128565:web:3310a025fcdcea1f5d3efc"},oi=de(Ct),ai=null;Ct.measurementId&&Et().then(e=>{e&&(ai=_t(oi))}).catch(()=>{});})();
/*! Bundled license information:

@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
@firebase/logger/dist/esm/index.esm2017.js:
  (**
   * @license
   * Copyright 2017 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)

@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
  (**
   * @license
   * Copyright 2022 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)

@firebase/util/dist/index.esm2017.js:
  (**
   * @license
   * Copyright 2017 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)
  (**
   * @license
   * Copyright 2025 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)

@firebase/util/dist/index.esm2017.js:
@firebase/util/dist/index.esm2017.js:
  (**
   * @license
   * Copyright 2021 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)

@firebase/util/dist/index.esm2017.js:
@firebase/installations/dist/esm/index.esm2017.js:
@firebase/analytics/dist/esm/index.esm2017.js:
  (**
   * @license
   * Copyright 2019 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)
  (**
   * @license
   * Copyright 2020 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)

@firebase/component/dist/esm/index.esm2017.js:
@firebase/app/dist/esm/index.esm2017.js:
@firebase/app/dist/esm/index.esm2017.js:
@firebase/app/dist/esm/index.esm2017.js:
@firebase/installations/dist/esm/index.esm2017.js:
@firebase/installations/dist/esm/index.esm2017.js:
@firebase/installations/dist/esm/index.esm2017.js:
@firebase/installations/dist/esm/index.esm2017.js:
  (**
   * @license
   * Copyright 2019 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)

@firebase/app/dist/esm/index.esm2017.js:
  (**
   * @license
   * Copyright 2019 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)
  (**
   * @license
   * Copyright 2023 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)

@firebase/app/dist/esm/index.esm2017.js:
  (**
   * @license
   * Copyright 2021 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)
  (**
   * @license
   * Copyright 2019 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)

firebase/app/dist/esm/index.esm.js:
@firebase/installations/dist/esm/index.esm2017.js:
  (**
   * @license
   * Copyright 2020 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)

@firebase/analytics/dist/esm/index.esm2017.js:
  (**
   * @license
   * Copyright 2020 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)
  (**
   * @license
   * Copyright 2019 Google LLC
   *
   * Licensed under the Apache License, Version 2.0 (the "License");
   * you may not use this file except in compliance with the License.
   * You may obtain a copy of the License at
   *
   *   http://www.apache.org/licenses/LICENSE-2.0
   *
   * Unless required by applicable law or agreed to in writing, software
   * distributed under the License is distributed on an "AS IS" BASIS,
   * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   * See the License for the specific language governing permissions and
   * limitations under the License.
   *)
*/
