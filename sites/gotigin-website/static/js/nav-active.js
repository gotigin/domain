(function () {
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
  if (!key) return;
  var links = document.querySelectorAll("#topnav a[data-nav]");
  for (var i = 0; i < links.length; i++) {
    if (links[i].getAttribute("data-nav") === key) {
      links[i].classList.add("active");
      links[i].setAttribute("aria-current", "page");
    }
  }
})();
