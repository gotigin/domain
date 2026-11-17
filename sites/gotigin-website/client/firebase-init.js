// Firebase web app for gotigin.com — bundled into static/js/firebase-init.js.
// Analytics loads only when measurementId is present (enable GA4 in console).
import { initializeApp } from "firebase/app";
import { getAnalytics, isSupported } from "firebase/analytics";

const firebaseConfig = {
  apiKey: "AIzaSyC5l2wOeUVAloLE6PmjtkROmDYyoeN2tyM",
  authDomain: "gotigin-com.firebaseapp.com",
  projectId: "gotigin-com",
  storageBucket: "gotigin-com.firebasestorage.app",
  messagingSenderId: "1019994128565",
  appId: "1:1019994128565:web:3310a025fcdcea1f5d3efc",
  // measurementId: "G-XXXXXXXXXX", // add after enabling Google Analytics
};

const app = initializeApp(firebaseConfig);

let analytics = null;
if (firebaseConfig.measurementId) {
  isSupported()
    .then((ok) => {
      if (ok) analytics = getAnalytics(app);
    })
    .catch(() => {});
}

export { app, analytics };
