import{r as v,c as l,S as w,a as c,o as g,i as I,b as S,t as N,v as b,d as f,s as L}from"./vendor.dab25925.js";const P=function(){const n=document.createElement("link").relList;if(n&&n.supports&&n.supports("modulepreload"))return;for(const e of document.querySelectorAll('link[rel="modulepreload"]'))r(e);new MutationObserver(e=>{for(const t of e)if(t.type==="childList")for(const s of t.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&r(s)}).observe(document,{childList:!0,subtree:!0});function i(e){const t={};return e.integrity&&(t.integrity=e.integrity),e.referrerpolicy&&(t.referrerPolicy=e.referrerpolicy),e.crossorigin==="use-credentials"?t.credentials="include":e.crossorigin==="anonymous"?t.credentials="omit":t.credentials="same-origin",t}function r(e){if(e.ep)return;e.ep=!0;const t=i(e);fetch(e.href,t)}};P();const m=N("<div></div>"),O="ws://localhost:8080/socket";function u(o,n){o.send(JSON.stringify(n))}const E=o=>{const[n,i]=c(!1),r=b();let e;return g(()=>{e=new YT.Player(r,{height:"360",width:"640",videoId:o.videoId,playerVars:{controls:0,disablekb:1},events:{onReady:()=>{i(!0)},onStateChange:t=>{console.log(t.data),t.data===1&&u(o.socket,{type:"ping"}),t.data===0&&u(o.socket,{type:"feed",pointer:o.pointer+1})}}})}),f(()=>{!n()||(console.log(1234),e.loadVideoById(o.videoId))}),f(()=>{!n()||(console.log(o.duration),e.seekTo(o.duration))}),(()=>{const t=m.cloneNode(!0);return L(t,"id",r),t})()},_=o=>{const[n,i]=c(!1),[r,e]=c(""),[t,s]=c(0),[p,h]=c(0);return g(()=>{o.socket.addEventListener("message",a=>{console.log(a);const d=JSON.parse(a.data);if(d.pointer===null)return;const k=d.queue[d.pointer];e(k.id),h(d.duration),s(d.pointer),i(!0)}),u(o.socket,{type:"ping"})}),(()=>{const a=m.cloneNode(!0);return I(a,l(S,{get when(){return n()},get children(){return l(E,{get videoId(){return r()},get duration(){return p()},get socket(){return o.socket},get pointer(){return t()}})}})),a})()},y=new WebSocket(O);await new Promise(o=>y.addEventListener("open",o));await window.jukerYtLoadingPromise;v(()=>l(w,{get children(){return l(_,{socket:y})}}),document.getElementById("root"));