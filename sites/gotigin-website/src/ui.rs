// gotigin.com — company / house-mark Makepad SPA.
// Gotigin = trademark + GTG mark; company = GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED.
// Brands: Kumkaj, Wadhu, Ghal Bol. Legal HTML shares the same Hosting tree + top nav.
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // Palette from the Gotigin GTG mark (deep navy circle + white field).
    let Ink = #x12102a
    let Panel = #x1c1838
    let Steel = #x221e42
    let Amber = #xe8a317
    let SoftAmber = #xf0c14a
    let Paper = #xf7f6fb
    let Muted = #xb0adc4
    let Select = #x3a3568ff

    mod.widgets.Label.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Button.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Markdown.draw_text.sdf_luma_bias = 0.0
    mod.widgets.Markdown.selectable = true
    mod.widgets.Markdown.draw_selection.color = Select
    mod.widgets.TextFlow.draw_selection.color = Select

    let Eyebrow = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: SoftAmber
        font_size: 13.
        text_style_normal: theme.font_bold{font_size: 13. line_spacing: 1.3}
        text_style_bold: theme.font_bold{font_size: 13. line_spacing: 1.3}
        draw_text +: { color: SoftAmber text_style: theme.font_bold{font_size: 13. line_spacing: 1.3} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Headline = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Paper
        font_size: 46.
        text_style_normal: theme.font_bold{font_size: 46. line_spacing: 1.06}
        text_style_bold: theme.font_bold{font_size: 46. line_spacing: 1.06}
        draw_text +: { color: Paper text_style: theme.font_bold{font_size: 46. line_spacing: 1.06} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Subhead = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Paper
        font_size: 28.
        text_style_normal: theme.font_bold{font_size: 28. line_spacing: 1.15}
        text_style_bold: theme.font_bold{font_size: 28. line_spacing: 1.15}
        draw_text +: { color: Paper text_style: theme.font_bold{font_size: 28. line_spacing: 1.15} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Lede = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 19.
        text_style_normal: theme.font_regular{font_size: 19. line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 19. line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 19. line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Body = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 16.
        text_style_normal: theme.font_regular{font_size: 16. line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 16. line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 16. line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let CardTitle = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: SoftAmber
        font_size: 22.
        text_style_normal: theme.font_bold{font_size: 22. line_spacing: 1.25}
        text_style_bold: theme.font_bold{font_size: 22. line_spacing: 1.25}
        draw_text +: { color: SoftAmber text_style: theme.font_bold{font_size: 22. line_spacing: 1.25} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let LinkBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 18. right: 18. top: 13. bottom: 13.}
        text: "Open"
        draw_bg +: { color: Amber }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 14.} }
    }
    let ActiveNav = Button{
        width: Fit height: Fit
        margin: 0.
        padding: Inset{left: 14. right: 14. top: 11. bottom: 11.}
        draw_bg +: { color: Amber }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 13.} }
    }
    let GhostBtn = Button{
        width: Fit height: Fit
        margin: 0.
        padding: Inset{left: 14. right: 14. top: 11. bottom: 11.}
        draw_bg +: { color: Panel }
        draw_text +: { color: Paper text_style: theme.font_bold{font_size: 13.} }
    }
    let QuietBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 12. right: 12. top: 10. bottom: 10.}
        draw_bg +: { color: Panel }
        draw_text +: { color: Muted text_style: theme.font_bold{font_size: 12.} }
    }
    let Band = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 72. bottom: 72.}
        draw_bg +: { color: Ink }
        flow: Down
        spacing: 18.
    }
    let BandAlt = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 72. bottom: 72.}
        draw_bg +: { color: Steel }
        flow: Down
        spacing: 18.
    }
    let PanelCard = RoundedView{
        width: Fill height: Fit
        padding: Inset{left: 24. right: 24. top: 22. bottom: 22.}
        draw_bg +: { color: Panel }
        flow: Down
        spacing: 12.
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "Gotigin — Where Technology Meets Discovery"
                window.inner_size: vec2(1320, 860)
                pass.clear_color: Ink
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    shell := SolidView{
                        width: Fill height: Fill
                        draw_bg +: { color: Ink }
                        flow: Down
                        // Match /about HTML topbar: [logo + Gotigin] .... [nav items]
                        top := SolidView{
                            width: Fill height: Fit
                            padding: Inset{left: 20. right: 20. top: 12. bottom: 12.}
                            draw_bg +: { color: Steel }
                            flow: Right
                            spacing: 10.
                            align: Align{y: 0.5}
                            brand_icon := Image{
                                width: 40 height: 40
                                fit: ImageFit.Stretch
                                src: crate_resource("self:resources/media/gotigin-brand-logo.webp")
                            }
                            brand_home := Button{
                                width: Fit height: Fit
                                margin: 0.
                                padding: Inset{left: 2. right: 4. top: 8. bottom: 8.}
                                text: "Gotigin"
                                draw_bg +: { color: #x00000000 }
                                draw_text +: {
                                    color: SoftAmber
                                    text_style: theme.font_bold{font_size: 16.}
                                }
                            }
                            top_spacer := View{
                                width: Fill
                                height: 40.
                            }
                            nav_home := ActiveNav{ text: "Home" }
                            nav_about := GhostBtn{ text: "About" }
                            nav_privacy := GhostBtn{ text: "Privacy" }
                            nav_terms := GhostBtn{ text: "Terms" }
                            nav_help := GhostBtn{ text: "Help" }
                            nav_support := GhostBtn{ text: "Support" }
                        }
                        page := ScrollYView{
                            width: Fill
                            height: Fill
                            flow: Down
                            selection_root: true
                            select_from_gaps: true

                            // Logo lives in the topbar only (same as wadhu.com).
                            hero := Band{
                                hero_tag := View{
                                    width: Fill height: Fit
                                    Eyebrow{ body: "GOTIGIN · COMPANY · HOUSE MARK · GTG" }
                                }
                                Headline{ body: "Gotigin." }
                                Lede{ body: "Where Technology Meets Discovery. Company site for GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED — the house mark behind Kumkaj, Wadhu, Ghal Bol, and GotiginSwap. The GTG emblem is Gotigin." }
                                cta_row := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 14.
                                    cta_about := LinkBtn{ text: "About Gotigin" }
                                    cta_privacy := GhostBtn{ text: "Privacy policy" }
                                    cta_help := GhostBtn{ text: "Help centre" }
                                }
                            }

                            brands := BandAlt{
                                Eyebrow{ body: "BRANDS" }
                                Subhead{ body: "One company. Distinct brands." }
                                Lede{ body: "Each brand has its own domain and privacy policy. Gotigin is the shared company mark above them." }
                                brand_cards := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 16.
                                    kumkaj := PanelCard{
                                        CardTitle{ body: "Kumkaj" }
                                        Body{ body: "Business operating systems — four products: Khaynjo, Moklanjo, Kirano, Tabiyat. Local-first books, parties, bills, and printing." }
                                        open_kumkaj := LinkBtn{ text: "kumkaj.com" }
                                    }
                                    wadhu := PanelCard{
                                        CardTitle{ body: "Wadhu" }
                                        Body{ body: "Reusable tools — two products: SMS Gateway and Wadhu Connect." }
                                        open_wadhu := LinkBtn{ text: "wadhu.com" }
                                    }
                                    ghalbol := PanelCard{
                                        CardTitle{ body: "Ghal Bol" }
                                        Body{ body: "Communication products — encrypted messaging under the Ghal Bol brand." }
                                        open_ghalbol := LinkBtn{ text: "ghalbol.com" }
                                    }
                                }
                            }

                            products := Band{
                                Eyebrow{ body: "PRODUCTS BY BRAND" }
                                Subhead{ body: "Kumkaj — four products" }
                                Lede{ body: "Concrete business operating systems for real trades." }
                                kumkaj_row := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 14.
                                    p_khaynjo := PanelCard{
                                        CardTitle{ body: "Khaynjo" }
                                        Body{ body: "Restaurant OS — tables, KOT, bills, day close." }
                                        open_khaynjo := QuietBtn{ text: "khaynjo.com" }
                                    }
                                    p_moklanjo := PanelCard{
                                        CardTitle{ body: "Moklanjo" }
                                        Body{ body: "Courier desk — booking, parties, day book." }
                                        open_moklanjo := QuietBtn{ text: "moklanjo.com" }
                                    }
                                    p_kirano := PanelCard{
                                        CardTitle{ body: "Kirano" }
                                        Body{ body: "Kirana / grocery counter — catalogue, stock, khata, GST cash memo." }
                                        open_kirano := QuietBtn{ text: "kirano.shop" }
                                    }
                                    p_tabiyat := PanelCard{
                                        CardTitle{ body: "Tabiyat" }
                                        Body{ body: "Pharmacy counter — batch, expiry, stock, GST tax invoice." }
                                        open_tabiyat := QuietBtn{ text: "tabiyat.shop" }
                                    }
                                }
                                Subhead{ body: "Wadhu — two products" }
                                Lede{ body: "Messaging tools other Gotigin brands (and anyone) can use." }
                                wadhu_row := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 14.
                                    p_sms := PanelCard{
                                        CardTitle{ body: "SMS Gateway" }
                                        Body{ body: "Android-first SIM gateway — LAN and Bluetooth stay free forever." }
                                        open_sms := QuietBtn{ text: "wadhu.com" }
                                    }
                                    p_connect := PanelCard{
                                        CardTitle{ body: "Wadhu Connect" }
                                        Body{ body: "WhatsApp Cloud API bridge — their number, your software." }
                                        open_connect := QuietBtn{ text: "connect.wadhu.com" }
                                    }
                                }
                                Body{ body: "Also under Gotigin: GotiginSwap (experimental). Product privacy lives on each brand domain; company verifiers use gotigin.com." }
                            }

                            trust := BandAlt{
                                Eyebrow{ body: "FOR STORES · META · PARTNERS" }
                                Subhead{ body: "Policies live in the top bar." }
                                Lede{ body: "Privacy, Terms, Help, Support, and About are always available from the navigation. Additional pages: /data-deletion · /cookies · /security." }
                                Body{ body: "Use these same-host URLs for Google Play, Microsoft Store, Meta App Review, and company verification." }
                            }

                            company := Band{
                                Eyebrow{ body: "COMPANY" }
                                Subhead{ body: "GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                                Body{ body: "CIN: U62011GJ2026PTC179660" }
                                Body{ body: "Incorporation date: 24 June 2026" }
                                Body{ body: "GSTIN: 24AANCG1628G1ZP" }
                                Body{ body: "PAN: AANCG1628G" }
                                Body{ body: "DPIIT: DIPP271306" }
                                Body{ body: "Registered office: Gosiya Manzil, Darbargadh, Dhoraji, Rajkot, Gujarat 360410, India" }
                                Body{ body: "Directors: Mearaj Abdulaziz Bhagad; Bhagad Senila Mearaj" }
                                Body{ body: "Business email: mearaj@gotigin.com" }
                                open_mail := LinkBtn{ text: "Email Gotigin" }
                            }

                            close := BandAlt{
                                Eyebrow{ body: "NEXT" }
                                Subhead{ body: "Build with Gotigin." }
                                Lede{ body: "Explore the brands, read the policies, or write to us. This site is the company source of truth for identity and legal links." }
                                cta_contact := LinkBtn{ text: "Contact / support" }
                                Body{ body: "© 2026 GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED. Trademark: Gotigin." }
                            }
                        }
                    }
                }
            }
        }
    }
}
