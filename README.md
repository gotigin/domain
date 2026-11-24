# Domain — Gotigin public domains

Public code for **gotigin.com** and related company web surfaces.

| | |
| --- | --- |
| **Company** | GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED |
| **Trademark** | Gotigin — *Where Technology Meets Discovery* |
| **Site crate** | [`sites/gotigin-website`](sites/gotigin-website) |
| **Legal source** | [`docs/legal/`](docs/legal) |
| **Firebase** | `gotigin-com` |
| **GitHub** | [gotigin/domain](https://github.com/gotigin/domain) |

**Gotigin** is a trademark (Memon *find it*). **GOTIGIN SOFTWARE & HARDWARE
PRIVATE LIMITED** is authorized to use, operate under, and represent it.
Brands are **Kumkaj**,
**Wadhu** (Memon *more* — Tools · Apps · More; not a messenger), and
**Ghal Bol**. Store listings, Meta
App Review, and other verifiers expect mature same-host pages here:
`/privacy`, `/terms`, `/cancellation`, `/about`, `/support`, `/help`,
`/data-deletion`, `/cookies`, `/security`.

## Setup

```bash
./scripts/setup-makepad.sh
# Wasm packaging also needs vendor/makepad-full (same rev as Wadhu/Kumkaj).
# If missing after setup, symlink or rsync from another Gotigin workspace's
# vendor/makepad-full, or source android-env-style ensure_makepad_full.
```

## Run

```bash
./sites/gotigin-website/scripts/run-desktop.sh
./sites/gotigin-website/scripts/run-web.sh      # http://127.0.0.1:8030
```

## Deploy

```bash
./sites/gotigin-website/scripts/deploy-firebase.sh
./sites/gotigin-website/scripts/deploy-firebase.sh --dry-run
```

Attach custom domain **gotigin.com** in Firebase Hosting for project
`gotigin-com` after the first deploy.
