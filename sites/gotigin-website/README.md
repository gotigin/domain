# gotigin.com — Makepad company website

Package `gotigin-website`. **One website**, one Firebase Hosting tree (same
shape as Khaynjo / Wadhu):

```text
public/
  index.html              ← Makepad SPA (scrolling company page)
  gotigin-website.wasm
  /privacy/…              ← formatted legal HTML (Play / Meta / Store)
  /terms/…
  /about/…
  /help/…
  /support/…
  /data-deletion/…
  /cookies/…
  /security/…
  /js/firebase-init.js
  /media/link-preview.jpg ← picture for link previews (see below)
  /build-id.txt           ← what is live
```

The SPA is one **scrolling** page: hero → brands and products → company
pages → company → close. Same-site paths (`/privacy`, `/about`, …) replace
the tab (`OpenUrlInPlace::Yes`). Brand sites open in a new tab. Legal HTML
is not an in-app Makepad route.

Legal Markdown lives in [`docs/legal/`](../../docs/legal/). Present with
`./sites/gotigin-website/scripts/present-legal.sh` (also run by deploy).

## Run

```bash
./sites/gotigin-website/scripts/run-desktop.sh   # native
./sites/gotigin-website/scripts/run-web.sh       # one tree → :8030
```

## Deploy

```bash
./sites/gotigin-website/scripts/deploy-firebase.sh
./sites/gotigin-website/scripts/deploy-firebase.sh --dry-run
```

Firebase project: **`gotigin-com`** (`.firebaserc`). Attach custom domain
**gotigin.com** in Hosting after first deploy.

## The shell around the wasm

Three things live in the HTML, because nothing can run before the wasm has
arrived and link crawlers never run it at all:

- **Start-up screen** — `shell/boot.{css,html,js}` and `shell/logo.webp`,
  inlined into `index.html` by `scripts/patch-web-shell.sh` (run by
  `run-web.sh` and by deploy). The logo sits in a ring that fills with the
  real wasm download, then the status reads "Starting" until the canvas has
  painted and the screen fades out. It says so when the connection is slow
  and offers Reload and the plain pages when the page cannot start.
- **Link preview** — `static/media/link-preview.jpg`, a 1200×630 JPEG of the
  real first screen, named by the Open Graph and Twitter tags on the home
  page and on every legal page. One set of tags serves WhatsApp, X,
  LinkedIn, Facebook, Telegram, Discord, Slack and iMessage. After the
  first screen changes: `run-web.sh` in one terminal, then
  `./sites/gotigin-website/scripts/make-link-preview.sh http://127.0.0.1:8030/`,
  commit the picture, deploy. Its address carries a hash of the file, so
  apps that cache previews fetch the new one.
- **Addresses that change per build** — the wasm and its loader get
  `?v=<build id>`, and the wasm is served `immutable`, so a returning
  visitor downloads it once per release. `/build-id.txt` says what is live.

What the page downloads (measured 2026-11-21, empty cache): the wasm is
1.3 MB compressed — the Makepad runtime, already built for size; the site's
own code adds almost nothing. `--split` was tried and sends more bytes, not
fewer (its second half stays on the start-up path). The pictures in
`resources/media/` are 192 px WebP because they are drawn at 40–56 px;
keep new ones that small.

## URLs

| Path | What |
| --- | --- |
| `/` | Makepad company SPA |
| `/privacy` | Company privacy policy |
| `/terms` | Terms of use |
| `/data-deletion` | Play / Meta deletion instructions |
| `/support` | Support contact |
| `/help` | Help centre / verification links |
| `/about` | Company + brand map |
| `/cookies` | Cookie policy |
| `/security` | Security & vulnerability reporting |
