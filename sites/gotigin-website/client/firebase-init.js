// Firebase web app for gotigin.com — Analytics + Auth.
// Bundled into static/js/firebase-init.js.
import { initializeApp } from "firebase/app";
import { getAnalytics, isSupported } from "firebase/analytics";
import {
  getAuth,
  GoogleAuthProvider,
  signInWithPopup,
  signInWithEmailAndPassword,
  createUserWithEmailAndPassword,
} from "firebase/auth";

const firebaseConfig = {
  apiKey: "AIzaSyC5l2wOeUVAloLE6PmjtkROmDYyoeN2tyM",
  authDomain: "gotigin.com",
  projectId: "gotigin-com",
  storageBucket: "gotigin-com.firebasestorage.app",
  messagingSenderId: "1019994128565",
  appId: "1:1019994128565:web:3310a025fcdcea1f5d3efc",
};

const app = initializeApp(firebaseConfig);
const auth = getAuth(app);

let analytics = null;
if (firebaseConfig.measurementId) {
  isSupported()
    .then((ok) => {
      if (ok) analytics = getAnalytics(app);
    })
    .catch(() => {});
}

if (typeof window !== "undefined") {
  window.__SITE_AUTH__ = auth;
  window.__SITE_APP__ = app;
  window.__SITE_PRODUCT_NAME__ = "Gotigin";
  window.__SITE_ACCOUNT_HREF__ = window.__SITE_ACCOUNT_HREF__ || "#";
  window.__SITE_SIGN_IN_GOOGLE__ = () =>
    signInWithPopup(auth, new GoogleAuthProvider());
  window.__SITE_SIGN_IN_EMAIL__ = (email, password) =>
    signInWithEmailAndPassword(auth, email, password);
  window.__SITE_REGISTER_EMAIL__ = (email, password) =>
    createUserWithEmailAndPassword(auth, email, password);
}

export { app, auth, analytics };
