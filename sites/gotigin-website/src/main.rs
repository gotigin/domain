//! gotigin.com — company / house-mark Makepad scrolling SPA.
//!
//! One Hosting tree: this wasm home + `/privacy` `/terms` `/about` `/help` …
//! HTML (from `docs/legal/`). Same-site nav replaces the current tab
//! (`OpenUrlInPlace::Yes`). External brand sites open in a new tab.

pub use makepad_widgets;

use makepad_widgets::*;

app_main!(App);

include!("ui.rs");

const PUBLIC_HOST: &str = "https://gotigin.com";

const PRIVACY_PATH: &str = "/privacy";
const TERMS_PATH: &str = "/terms";
const ABOUT_PATH: &str = "/about";
const HELP_PATH: &str = "/help";
const SUPPORT_PATH: &str = "/support";
const HOME_PATH: &str = "/";

const KUMKAJ_URL: &str = "https://kumkaj.com";
const WADHU_URL: &str = "https://wadhu.com";
const GHALBOL_URL: &str = "https://ghalbol.com";
const KHAYNJO_URL: &str = "https://khaynjo.com";
const MOKLANJO_URL: &str = "https://moklanjo.com";
const KIRANO_URL: &str = "https://kirano.shop";
const TABIYAT_URL: &str = "https://tabiyat.shop";
const CONNECT_URL: &str = "https://connect.wadhu.com/";
const MAILTO: &str = "mailto:mearaj@gotigin.com";

const BRAND_CARDS_SIDE_BY_SIDE_FROM: f64 = 780.0;
const PRODUCT_ROW_FROM: f64 = 900.0;
const MIN_WIDTH: f64 = 320.0;
const MAX_CONTENT: f64 = 1120.0;
const MAX_CONTENT_WIDE: f64 = 1320.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tier {
    Phone,
    Narrow,
    Tablet,
    Desk,
    Wide,
}

impl Tier {
    fn from_size(w: f64, h: f64) -> Self {
        if w < 480.0 {
            Self::Phone
        } else if w < 720.0 {
            Self::Narrow
        } else if w < 1024.0 {
            Self::Tablet
        } else if w >= 1600.0 && h >= 900.0 {
            Self::Wide
        } else {
            Self::Desk
        }
    }

    fn max_content(self) -> f64 {
        if self == Self::Wide {
            MAX_CONTENT_WIDE
        } else {
            MAX_CONTENT
        }
    }

    fn side_pad(self) -> f64 {
        match self {
            Self::Phone => 16.0,
            Self::Narrow => 24.0,
            Self::Tablet => 40.0,
            Self::Desk | Self::Wide => 64.0,
        }
    }

    fn band_pad_v(self) -> f64 {
        match self {
            Self::Phone => 40.0,
            Self::Narrow => 52.0,
            _ => 72.0,
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    laid_out: Option<(Tier, i64, i64)>,
}

fn open_site(cx: &mut Cx, url: &str) {
    // Same-tab navigation for gotigin.com paths (legal HTML + home).
    #[cfg(target_arch = "wasm32")]
    {
        cx.open_url(url, OpenUrlInPlace::Yes);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        open_external(cx, url);
    }
}

fn open_external(cx: &mut Cx, url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        cx.open_url(url, OpenUrlInPlace::No);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = cx;
        #[cfg(target_os = "windows")]
        let spawned = std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn();
        #[cfg(target_os = "macos")]
        let spawned = std::process::Command::new("open").arg(url).spawn();
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let spawned = std::process::Command::new("xdg-open").arg(url).spawn();
        if let Err(err) = spawned {
            log!("could not open {url}: {err}");
        }
    }
}

impl App {
    fn site_url(cx: &Cx, path: &str) -> String {
        if matches!(cx.os_type(), OsType::Web(_)) {
            path.to_string()
        } else {
            format!("{PUBLIC_HOST}{path}")
        }
    }

    fn apply_responsive(&mut self, cx: &mut Cx) {
        let size = self.ui.window(cx, ids!(main_window)).get_inner_size(cx);
        if size.x < 1.0 || size.y < 1.0 {
            return;
        }
        let layout_w = size.x.max(MIN_WIDTH);
        let tier = Tier::from_size(layout_w, size.y);
        let key = (tier, layout_w.round() as i64, size.y.round() as i64);
        if self.laid_out == Some(key) {
            return;
        }
        self.laid_out = Some(key);

        let min_side = tier.side_pad();
        let side = min_side.max((layout_w - tier.max_content()) / 2.0);
        let pad_v = tier.band_pad_v();
        let band_pad = Inset {
            left: side,
            right: side,
            top: pad_v,
            bottom: pad_v,
        };

        for path in [
            ids!(hero),
            ids!(brands),
            ids!(products),
            ids!(trust),
            ids!(company),
            ids!(close),
        ] {
            if let Some(mut view) = self.ui.view(cx, path).borrow_mut() {
                view.layout.padding = band_pad;
            }
        }
        if let Some(mut bar) = self.ui.view(cx, ids!(top)).borrow_mut() {
            let pad = if tier == Tier::Phone { 12.0 } else { 20.0 };
            bar.layout.padding = Inset {
                left: pad,
                right: pad,
                top: 12.0,
                bottom: 12.0,
            };
            // Always logo|title ---- nav (same as /about HTML topbar).
            bar.layout.flow = Flow::right();
        }

        if let Some(mut cards) = self.ui.view(cx, ids!(brand_cards)).borrow_mut() {
            cards.layout.flow = if layout_w >= BRAND_CARDS_SIDE_BY_SIDE_FROM {
                Flow::right()
            } else {
                Flow::Down
            };
        }
        let product_flow = if layout_w >= PRODUCT_ROW_FROM {
            Flow::right()
        } else {
            Flow::Down
        };
        for row_id in [ids!(kumkaj_row), ids!(wadhu_row)] {
            if let Some(mut row) = self.ui.view(cx, row_id).borrow_mut() {
                row.layout.flow = product_flow;
            }
        }
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.ui
            .window(cx, ids!(main_window))
            .set_title(cx, "Gotigin — Where Technology Meets Discovery");
        self.apply_responsive(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(nav_home)).clicked(actions)
            || self.ui.button(cx, ids!(brand_home)).clicked(actions)
        {
            open_site(cx, &Self::site_url(cx, HOME_PATH));
        }

        if self.ui.button(cx, ids!(nav_privacy)).clicked(actions)
            || self.ui.button(cx, ids!(cta_privacy)).clicked(actions)
        {
            open_site(cx, &Self::site_url(cx, PRIVACY_PATH));
        }
        if self.ui.button(cx, ids!(nav_terms)).clicked(actions) {
            open_site(cx, &Self::site_url(cx, TERMS_PATH));
        }
        if self.ui.button(cx, ids!(nav_about)).clicked(actions)
            || self.ui.button(cx, ids!(cta_about)).clicked(actions)
        {
            open_site(cx, &Self::site_url(cx, ABOUT_PATH));
        }
        if self.ui.button(cx, ids!(nav_help)).clicked(actions)
            || self.ui.button(cx, ids!(cta_help)).clicked(actions)
        {
            open_site(cx, &Self::site_url(cx, HELP_PATH));
        }
        if self.ui.button(cx, ids!(nav_support)).clicked(actions)
            || self.ui.button(cx, ids!(cta_contact)).clicked(actions)
        {
            open_site(cx, &Self::site_url(cx, SUPPORT_PATH));
        }
        if self.ui.button(cx, ids!(open_mail)).clicked(actions) {
            open_external(cx, MAILTO);
        }

        if self.ui.button(cx, ids!(open_kumkaj)).clicked(actions) {
            open_external(cx, KUMKAJ_URL);
        }
        if self.ui.button(cx, ids!(open_wadhu)).clicked(actions)
            || self.ui.button(cx, ids!(open_sms)).clicked(actions)
        {
            open_external(cx, WADHU_URL);
        }
        if self.ui.button(cx, ids!(open_ghalbol)).clicked(actions) {
            open_external(cx, GHALBOL_URL);
        }
        if self.ui.button(cx, ids!(open_khaynjo)).clicked(actions) {
            open_external(cx, KHAYNJO_URL);
        }
        if self.ui.button(cx, ids!(open_moklanjo)).clicked(actions) {
            open_external(cx, MOKLANJO_URL);
        }
        if self.ui.button(cx, ids!(open_kirano)).clicked(actions) {
            open_external(cx, KIRANO_URL);
        }
        if self.ui.button(cx, ids!(open_tabiyat)).clicked(actions) {
            open_external(cx, TABIYAT_URL);
        }
        if self.ui.button(cx, ids!(open_connect)).clicked(actions) {
            open_external(cx, CONNECT_URL);
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
        match event {
            Event::Startup | Event::WindowGeomChange(_) => self.apply_responsive(cx),
            _ => {}
        }
    }
}
