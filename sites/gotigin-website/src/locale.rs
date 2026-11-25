//! gotigin.com words. English is the source (`EN`, keyed by a stable id);
//! every other catalogue language is `translations/<code>.tsv` in this crate
//! — one `key<TAB>text` line per `EN` key, in this order, `{placeholders}`
//! kept. The runtime (catalogue, `#l=` bridge, packs, the widget walker) is
//! `sites/_shared/i18n/site_i18n.rs`.
//!
//! **Page copy** in `ui.rs` is matched by its English text: a Label /
//! Markdown / Button whose text is an `EN` value shows that key's
//! translation. Text the site sets from code goes through `App::tr`.
//! `tests::every_ui_string_is_keyed_or_skipped` fails when `ui.rs` gains
//! copy that is neither here nor in `UI_SKIP` (names, numbers, ids).
//!
//! Legal page bodies stay English. Account chrome is `data-i18n` / `T()`.
//!
//! **New or changed English:** edit `EN`, run
//! `./scripts/scaffold-site-translations.sh gotigin` (adds the key to every
//! pack as the English source), then translate for real. Hindi and Urdu
//! must not stay English; other packs may until someone translates them.
//! `@@EN@@` stand-ins fail the pack test.

use std::collections::HashMap;
use std::sync::OnceLock;

/// Every site key, in pack order.
pub const EN: &[(&str, &str)] = &[
    ("menu", "Menu"),
    ("close", "Close"),
    ("home", "Home"),
    ("about", "About"),
    ("privacy", "Privacy"),
    ("terms", "Terms"),
    ("cancellation", "Cancellation"),
    ("deletion", "Deletion"),
    ("help", "Help"),
    ("support", "Support"),
    ("login", "Login"),
    ("email", "Email"),
    ("trademark", "TRADEMARK"),
    ("window_title", "Gotigin — Where Technology Meets Discovery"),
    ("slogan", "Where Technology Meets Discovery."),
    ("hero_lede", "Everyday software for business and personal use — free on your devices."),
    ("promise", "PROMISE"),
    ("promise_title", "Full products. Free to use."),
    ("promise_lede", "Download and run the complete product at no cost. We charge only for optional services you choose."),
    ("promise_free_title", "Free locally"),
    ("promise_free_body", "The full product on your devices stays free. No paywall on the core tool."),
    ("promise_ads_title", "No forced ads"),
    ("promise_ads_body", "Use the software without ads that interrupt your work."),
    ("promise_charge_title", "Pay for services"),
    ("promise_charge_body", "Optional cloud, demos, visits, and assistance — only when you ask for them."),
    ("promise_open_title", "Open where published"),
    ("promise_open_body", "Public code under github.com/gotigin. Local and LAN keep working offline."),
    ("built_with", "BUILT WITH"),
    ("tech_title", "Rust and Makepad"),
    ("tech_lede", "Most Gotigin products are written in Rust. GUIs use Makepad for fast apps and websites on desktop, mobile, and the web."),
    ("tech_rust_body", "Safe, fast, and reliable — built for software that handles real work every day."),
    ("tech_makepad_body", "One Rust UI stack for native apps and wasm sites — performant and local-first."),
    ("close_lede", "Products free. Services optional."),
    ("brands", "BRANDS"),
    ("brand", "BRAND"),
    ("product", "PRODUCT"),
    ("kumkaj_blurb", "Business operating systems."),
    ("khaynjo_blurb", "Restaurant OS. Tables, KOT, bills, day close."),
    ("moklanjo_blurb", "Courier desk. Booking, parties, day book."),
    ("kirano_blurb", "Kirana. Catalogue, stock, khata, GST cash memo."),
    ("tabiyat_blurb", "Pharmacy counter. Batch, expiry, stock, GST bills."),
    ("wadhu_blurb", "Memon for more. Tools · Apps · More."),
    ("wadhu_note", "Reusable tools, utils, and apps. Not a messenger."),
    ("sms_gateway_blurb", "Android-first SIM messaging. LAN and Bluetooth stay free."),
    ("sms_relay_blurb", "Optional internet plane for paired gateways."),
    ("connect_blurb", "WhatsApp Cloud API bridge."),
    ("ghalbol_blurb", "Communication products."),
    ("ghalbol_app_blurb", "Encrypted messenger."),
    ("pages", "PAGES"),
    ("company_pages", "Company pages"),
    ("page", "PAGE"),
    ("privacy_blurb", "Company privacy policy."),
    ("terms_blurb", "Terms of use for gotigin.com."),
    ("cancellation_blurb", "Cancellation and refund policy."),
    ("data_deletion", "Data deletion"),
    ("deletion_blurb", "How to ask for deletion."),
    ("support_blurb", "Email, phone, and product routing."),
    ("help_blurb", "Names, products, and sites."),
    ("about_blurb", "Company registration and contacts."),
    ("cookies", "Cookies"),
    ("cookies_blurb", "Cookies on gotigin.com."),
    ("security", "Security"),
    ("security_blurb", "How to report a vulnerability."),
    ("company", "COMPANY"),
    ("company_lede", "Private limited company. Dhoraji, Gujarat. Incorporated 24 June 2026."),
    ("registration", "Registration"),
    ("cin", "CIN"),
    ("incorporated", "Incorporated 24 June 2026"),
    ("share_capital", "Share capital ₹1,00,000"),
    ("limited_by_shares", "Limited by shares · Gujarat"),
    ("tax", "Tax"),
    ("gst_regular", "GST Regular · from 04 July 2026"),
    ("recognition", "Recognition"),
    ("issued", "Issued 08 July 2026"),
    ("valid_to", "Valid to 23 June 2036"),
    ("registered_office", "Registered office"),
    ("directors", "Directors"),
    ("contact", "Contact"),
    ("find_it", "Memon for “find it” · pronounced go-tee-geen"),
    ("brands_line", "Business systems · tools and apps · communication"),
    ("account", "Account"),
    ("acct_page_title", "Account — Gotigin"),
    ("acct_lede", "Sign in for Gotigin company site features. Same options as on the product sites — email, providers, or phone."),
    ("acct_continue_with", "Continue with email, a provider, or phone."),
    ("acct_name_optional", "Name (optional)"),
    ("acct_password", "Password"),
    ("acct_continue", "Continue"),
    ("acct_or_continue", "or continue with"),
    ("acct_federated", "Federated sign-in"),
    ("acct_phone", "Phone"),
    ("acct_mobile", "Mobile number"),
    ("acct_phone_placeholder", "+91…"),
    ("acct_send_sms", "Send SMS code"),
    ("acct_sms_code", "SMS code"),
    ("acct_confirm_code", "Confirm code"),
    ("acct_signed_in", "Signed in"),
    ("acct_signed_in_as", "Signed in as"),
    ("acct_status_label", "Status"),
    ("acct_resend_verify", "Resend verification email"),
    ("acct_sign_out", "Sign out"),
    ("acct_home_aria", "Gotigin home"),
    ("acct_nav_aria", "Primary"),
    ("err_invalid_email", "Enter a valid email address."),
    ("err_weak_password", "Use a password with at least 6 characters."),
    ("err_wrong_password", "Wrong password."),
    ("err_invalid_credential", "Email or password is wrong."),
    ("err_popup_closed", "Sign-in window was closed."),
    ("err_cancelled", "Sign-in was cancelled."),
    ("err_unauthorized_domain", "This domain is not authorized for Firebase Auth."),
    ("err_not_allowed", "That sign-in method is not enabled yet."),
    ("err_too_many", "Too many attempts. Try again later."),
    ("err_invalid_phone", "Use a full phone number with country code (e.g. +91…)."),
    ("err_bad_code", "That SMS code is wrong or expired."),
    ("err_generic", "Something went wrong."),
    ("acct_no_email", "(no email)"),
    ("acct_email_verified", "Email verified"),
    ("acct_email_unverified", "Email not verified yet — check your inbox"),
    ("acct_phone_signed_in", "Signed in with phone"),
    ("acct_you_are_in", "You’re signed in."),
    ("acct_verify_sent", "Verification email sent — check your inbox"),
];

/// `ui.rs` text that is not site copy: brand and product names, ids, URLs,
/// addresses, and the unused LinkBtn template label.
#[cfg_attr(not(test), allow(dead_code))]
pub const UI_SKIP: &[&str] = &[
    "Open",
    "EN",
    "Gotigin",
    "Rust",
    "Makepad",
    "Kumkaj",
    "Wadhu",
    "Ghal Bol",
    "Khaynjo",
    "Moklanjo",
    "Kirano",
    "Tabiyat",
    "SMS Gateway",
    "SMS Relay",
    "Wadhu Connect",
    "Kumkaj ·",
    "Wadhu ·",
    "kumkaj.com",
    "khaynjo.com",
    "moklanjo.com",
    "kirano.shop",
    "tabiyat.shop",
    "wadhu.com",
    "ghalbol.com",
    "connect.wadhu.com",
    "gotigin.com",
    "/privacy",
    "/terms",
    "/cancellation",
    "/data-deletion",
    "/support",
    "/help",
    "/about",
    "/cookies",
    "/security",
    "GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED",
    "U62011GJ2026PTC179660",
    "D-U-N-S 581757966",
    "PAN AANCG1628G",
    "TAN RKTG08602G",
    "GSTIN 24AANCG1628G1ZP",
    "DPIIT DIPP271306",
    "Shops SR131000001 · Dhoraji",
    "C/O Mearaj AB",
    "Gosiya Manzil, Darbargadh",
    "Dhoraji, Rajkot",
    "Gujarat 360410, India",
    "Mearaj Abdulaziz Bhagad",
    "Bhagad Senila Mearaj",
    "support@gotigin.com",
    "mearaj@gmail.com",
    "+91 99677 17702",
    "support@gotigin.com · mearaj@gmail.com",
    "© 2026 GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED",
    "Gotigin · gotigin.com",
];

/// English for `key` ("" for an unknown key — tests forbid that in code).
pub fn en(key: &str) -> &'static str {
    EN.iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
        .unwrap_or("")
}

/// The key whose English is exactly `english` (page copy lookup).
pub fn key_for(english: &str) -> Option<&'static str> {
    static REVERSE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    REVERSE
        .get_or_init(|| EN.iter().map(|(k, v)| (*v, *k)).collect())
        .get(english)
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::site_i18n::checks;

    const DIR: &str = env!("CARGO_MANIFEST_DIR");

    fn keys() -> Vec<&'static str> {
        EN.iter().map(|(k, _)| *k).collect()
    }

    #[test]
    fn keys_are_unique_and_english_is_unique() {
        let mut seen_k = std::collections::HashSet::new();
        let mut seen_v = std::collections::HashSet::new();
        for (k, v) in EN {
            assert!(seen_k.insert(*k), "duplicate key {k}");
            assert!(seen_v.insert(*v), "duplicate English {v:?} (two keys would fight over one widget)");
            assert!(!v.trim().is_empty(), "{k}: empty English");
        }
    }

    /// Every table language has a pack with every key, in `EN` order, with
    /// the same placeholders and no `@@EN@@` stand-in. Untranslated packs
    /// may still hold the English source; Hindi and Urdu may not.
    #[test]
    fn every_catalogue_language_translates_every_key() {
        checks::every_language_translates_every_key(DIR, &keys(), &en);
    }

    #[test]
    fn hi_and_ur_are_real_translations() {
        let mut problems = Vec::new();
        for code in ["hi", "ur"] {
            let pack = checks::read_pack(DIR, code).unwrap_or_else(|| {
                panic!("{code}: translations/{code}.tsv missing");
            });
            for (key, english) in EN {
                let Some(value) = pack.get(*key) else {
                    problems.push(format!("{code}/{key}: missing"));
                    continue;
                };
                if value.starts_with(crate::site_i18n::STAND_IN) {
                    problems.push(format!("{code}/{key}: still @@EN@@"));
                }
                if value == english {
                    problems.push(format!("{code}/{key}: still English"));
                }
            }
        }
        assert!(problems.is_empty(), "hi/ur packs:\n{}", problems.join("\n"));
    }

    #[test]
    fn every_translation_draws_with_the_site_fonts() {
        checks::every_pack_draws_with_its_fonts(DIR, &crate::script_font_files);
    }

    #[test]
    fn words_are_marked_where_the_script_has_no_spaces() {
        checks::words_are_marked_in_scripts_without_spaces(DIR);
    }

    #[test]
    fn every_ui_string_is_keyed_or_skipped() {
        checks::every_ui_string_is_keyed_or_skipped(include_str!("ui.rs"), &key_for, UI_SKIP);
    }

    /// Account page (`data-i18n`) and the account script (`T("key", "English")`).
    /// Legal bodies stay English and are not in this list.
    #[test]
    fn html_and_script_copy_match_en() {
        checks::html_and_script_copy_match_en(
            DIR,
            &["static/account/index.html"],
            &["client/account-app.js"],
            &en,
        );
    }
}
