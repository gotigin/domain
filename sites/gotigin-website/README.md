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
