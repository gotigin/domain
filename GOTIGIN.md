# Gotigin

**Gotigin** is a trademark. In Memon it means *find it* (*go-tee-geen*).
Slogan: *Where Technology Meets Discovery*. Visual mark: the circular
**GTG**.

**GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED** is authorized to use,
operate under, and represent the Gotigin trademark.

Identity order is always **Trademark → Company → Brand → Product**. See
[`docs/BRAND.md`](docs/BRAND.md). Registry and contact fields:
[`docs/COMPANY_IDENTITY.md`](docs/COMPANY_IDENTITY.md). Public about:
[`docs/legal/ABOUT.md`](docs/legal/ABOUT.md).

The company develops software and related support for everyday
**business and personal** use — and puts those tools in as many hands as
possible.

Products people need for day-to-day work and life should be available
without being forced to purchase, without ads that interrupt use, and
without depending on Gotigin’s cloud or a working internet connection to
operate. Where code is published, it is open source. Where the product
runs on the owner’s own devices, that use stays free.

Money, when charged, is for things that actually cost ongoing effort or
infrastructure: optional internet or cloud services, customer
assistance, visits, demos, and similar help. The product by itself is
not the thing being sold.

---

## Portfolio

| Name | Role |
| --- | --- |
| **Gotigin** | Trademark / house mark |
| **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED** | Legal company — authorized to use, operate under, and represent Gotigin |
| **Kumkaj** | Business operating systems: Khaynjo (restaurant), Moklanjo (courier desk), Kirano (kirana), Tabiyat (pharmacy) |
| **Wadhu** | Tools · Apps · More — reusable building blocks and utilities (not a messenger brand) |
| **Ghal Bol** | Encrypted communication |

Each Kumkaj product is a full business operating system for that trade —
money, parties, day books, and the ops that trade needs — not one
generic till marketed to everyone. Apps stay independent
(copy-then-diverge; no shared product core). Wadhu holds reusable Rust /
Makepad pieces other products (and outsiders) can use. Ghal Bol is a
separate communication line.

The legal name includes hardware: the company may sell and support
devices and accessories. In practice, products are built to work with
gear a shop already has (phones, printers, modems) and with optional
modules — not to require a Gotigin appliance before the day can start.

Public company site: [gotigin.com](https://gotigin.com). Brand and
product homes keep their own sites and policies.

---

## Free local, optional paid services

Across the portfolio the same trust model holds:

- **Local use is the complete product**, free, with no ads and no feature
  ceiling tied to a subscription.
- **Offline and own-network work is first-class.** Shop LAN, Bluetooth,
  and device-held books are designed so a dead internet link does not
  stop the day’s work.
- **Cloud and hosted planes are optional.** They add sync, backup,
  remote reach, or similar infrastructure — they do not unlock the core
  app. When a cloud term ends, local capability remains.
- **People cost money; the binary need not.** Assistance, demos, visits,
  and similar services may be charged separately.

Kumkaj states this as **free forever locally; cloud by usage** (yearly
storage and sync plans; billing must not hostage the till). Wadhu keeps
LAN and Bluetooth free; hosted relay and cloud bridges are the paid
layer. Product sites offer download of the full local product; demos and
walkthroughs are optional contact, not a paid unlock of the app.

Detail: Kumkaj `docs/KUMKAJ_PRODUCT_VISION_AND_REQUIREMENTS.md`,
`docs/CLOUD_PLANS.md`; Wadhu `docs/IDENTITY_AND_MONETIZATION_ALIGNMENT.md`.

---

## How the software is built

- **Local-first.** Ground truth on the owner’s devices; optional cloud
  replica later.
- **Open where published.** Public repositories under
  [github.com/gotigin](https://github.com/gotigin); Wadhu libraries are
  written to be usable outside Gotigin.
- **Rust** is the primary language for most Gotigin products.
- **Makepad** (Makepad 2) is the preferred framework / library for GUI —
  desktop, mobile, and many public sites (including wasm).
- **Trade-shaped, independent products** under each brand — not a single
  platform SKU that every user must bend themselves into.
- **Honest claims.** Ship what is real; unfinished work stays labelled;
  marketing does not invent customers, features, or results.

### Why Rust

Rust gives memory safety without a garbage collector, strong type
checking, and fearless concurrency. That matters for money-handling
books, long-running shop servers, messaging tools, and code that must
stay correct under load. One language spans native apps, services, and
wasm sites; performance stays close to C/C++ while whole classes of
crashes and data races are pushed out at compile time. The ecosystem
(Cargo, crates.io) keeps builds reproducible across Linux, Windows,
macOS, and Android.

### Why Makepad

Makepad is a Rust-native UI stack: immediate-mode style, GPU-backed
drawing, and the same application model on desktop, mobile, and web
(wasm). Gotigin prefers it so product UIs and marketing sites share one
craft — not a browser shell glued to a native core. It fits
local-first apps that must feel fast on modest hardware, stay
touch-friendly, and keep working offline. Vendored Makepad 2 is the
house platform for Kumkaj, Wadhu, Ghal Bol, and gotigin.com.

---

## Public company identifiers

Safe-to-publish registry and tax values (full detail:
[`docs/legal/ABOUT.md`](docs/legal/ABOUT.md),
[`docs/COMPANY_IDENTITY.md`](docs/COMPANY_IDENTITY.md)).

| | |
| --- | --- |
| Legal name | GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED |
| Constitution | Private Limited Company (limited by shares) |
| CIN | U62011GJ2026PTC179660 |
| Date of incorporation | 24 June 2026 (MCA Central Registration Centre) |
| Registered / mailing office | C/O Mearaj AB, Gosiya Manzil, Darbargadh, Dhoraji, Rajkot, Gujarat 360410, India |
| Share capital | ₹1,00,000 (10,000 equity shares × ₹10) |
| PAN | AANCG1628G |
| TAN | RKTG08602G |
| GSTIN | 24AANCG1628G1ZP |
| GST | Regular · liability from 04 July 2026 |
| D-U-N-S | 581757966 |
| DPIIT startup recognition | DIPP271306 · issued 08 July 2026 · valid to 23 June 2036 |
| Shops & Establishments | Dhoraji Nagarpalika · SR131000001 · commenced 02 July 2026 |
| Support | [support@gotigin.com](mailto:support@gotigin.com) |
| Phone | +91 99677 17702 |
| Website | [https://gotigin.com](https://gotigin.com) |

Do not invent further numbers. New public identifiers belong in
OfficialDocs first, then `COMPANY_IDENTITY.md` / `ABOUT.md`, then here.

---

## Spine

Everyday software that stays free on the user’s own devices, open where
published, independent of Gotigin’s cloud to keep working — with optional
paid cloud and human help for those who want them. Built primarily in
**Rust**, with **Makepad** for GUI.
