# Domain docs

| Path | Purpose |
| --- | --- |
| [`BRAND.md`](BRAND.md) | Gotigin vs company vs product brands |
| [`COMPANY_IDENTITY.md`](COMPANY_IDENTITY.md) | Registry fields for forms & legal |
| [`legal/`](legal/) | Canonical Markdown for gotigin.com legal pages |

## Always responsive (gotigin.com)

**Responsive ≠ phone-only stacks.** Fit chip / short CTA toolbars that fit
on one line at 1280 / 1920 **must** be one line — forever-`Down` with empty
horizontal space is a bug. **Exceptions** exist (user-owned density and
other intentional stacks — mostly obvious; still take care; if unsure
leave alone). Same Gotigin-wide law as Kumkaj, Wadhu, and Ghal Bol.
Cursor: [`.cursor/rules/responsive-graceful-wide.mdc`](../.cursor/rules/responsive-graceful-wide.mdc).

**Android Chrome:** Desktop-site fake `innerWidth` must not drive Makepad
layout — `web_mobile_layout_width` + `shell/mobile-viewport-guard.js`.
Gate: `./scripts/check-web-mobile-layout.sh`.

Legal Markdown is presented to HTML by
`sites/gotigin-website/scripts/present-legal.sh`. Do not hand-edit generated
HTML under `sites/gotigin-website/static/`.

**Crawlable shell:** `sites/gotigin-website/shell/{json-ld.json,noscript.html}`
are inlined by `patch-web-shell.sh` (Organization JSON-LD + plain-HTML
summary). `static/robots.txt` and `static/sitemap.xml` ride the legal
overlay into the Hosting tree.
