(function () {
  "use strict";
  try {
  var path = (location.pathname || "/").replace(/\/+$/, "") || "/";
  var map = {
    "/": "home",
    "/about": "about",
    "/privacy": "privacy",
    "/terms": "terms",
    "/cancellation": "cancellation",
    "/help": "help",
    "/support": "support",
    "/data-deletion": "deletion",
    "/cookies": "cookies",
    "/security": "security"
  };
  var key = map[path];
  var links = document.querySelectorAll("#topnav a[data-nav]");
  if (key) {
    for (var i = 0; i < links.length; i++) {
      if (links[i].getAttribute("data-nav") === key) {
        links[i].classList.add("active");
        links[i].setAttribute("aria-current", "page");
      }
    }
  }

  var bar = document.getElementById("topbar");
  var toggle = document.getElementById("nav-toggle");
  var nav = document.getElementById("topnav");
  if (!bar || !toggle || !nav) return;

  function setOpen(open) {
    bar.classList.toggle("nav-open", open);
    toggle.setAttribute("aria-expanded", open ? "true" : "false");
    toggle.textContent = open ? "Close" : "Menu";
  }

  toggle.addEventListener("click", function () {
    setOpen(!bar.classList.contains("nav-open"));
  });

  document.addEventListener("keydown", function (e) {
    if (e.key === "Escape") setOpen(false);
  });
  } catch (_) {}
})();
