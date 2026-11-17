// gotigin.com — company / house-mark Makepad SPA (Khaynjo + Wadhu pattern).
// Gotigin = trademark; GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED = company.
// Brands: Kumkaj, Wadhu, Ghal Bol. Legal HTML lives beside this wasm on Hosting.
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let Ink = #x0a0e14
    let Panel = #x141a22
    let Steel = #x1a2332
    let Amber = #xe8a317
    let SoftAmber = #xf0c14a
    let Paper = #xf2f4f7
    let Muted = #x9aa3ad
    let Select = #x3d4a5cff

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
    let GhostBtn = Button{
        width: Fit height: Fit
        padding: Inset{left: 14. right: 14. top: 11. bottom: 11.}
        draw_bg +: { color: Steel }
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
                        top := SolidView{
                            width: Fill height: Fit
                            padding: Inset{left: 24. right: 24. top: 12. bottom: 12.}
                            draw_bg +: { color: Steel }
                            flow: Right
                            spacing: 10.
                            align: Align{y: 0.5}
                            brand_icon := Image{
                                width: 40 height: 40
                                fit: ImageFit.Stretch
                                src: crate_resource("self:resources/media/gotigin-brand-logo.webp")
                            }
                            brand_tag := Label{
                                width: Fit
                                text: "GOTIGIN · WHERE TECHNOLOGY MEETS DISCOVERY"
                                draw_text +: {
                                    color: SoftAmber
                                    text_style: theme.font_bold{font_size: 11.}
                                }
                            }
                            <View>{ width: Fill height: 1 }
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

                            hero := Band{
                                hero_tag := View{
                                    width: Fill height: Fit
                                    Eyebrow{ body: "GOTIGIN · COMPANY · HOUSE MARK" }
                                }
                                Headline{ body: "Gotigin." }
                                Lede{ body: "Where Technology Meets Discovery. The house mark and company site for GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED — parent of Kumkaj, Wadhu, Ghal Bol, and GotiginSwap." }
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
                                Lede{ body: "Each brand keeps its own domain, product story, and privacy policy. Gotigin is the father mark they share." }
                                brand_cards := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 16.
                                    kumkaj := PanelCard{
                                        CardTitle{ body: "Kumkaj" }
                                        Body{ body: "Business operating systems for real trades — restaurant, courier, kirana, pharmacy. Local-first books, parties, bills, and printing." }
                                        open_kumkaj := LinkBtn{ text: "kumkaj.com" }
                                    }
                                    wadhu := PanelCard{
                                        CardTitle{ body: "Wadhu" }
                                        Body{ body: "Reusable tools other brands (and anyone) can use — SMS Gateway, Relay, and Wadhu Connect for WhatsApp Cloud API." }
                                        open_wadhu := LinkBtn{ text: "wadhu.com" }
                                    }
                                    ghalbol := PanelCard{
                                        CardTitle{ body: "Ghal Bol" }
                                        Body{ body: "Communication products under the Ghal Bol brand — encrypted messaging for people who need a private channel." }
                                        open_ghalbol := LinkBtn{ text: "ghalbol.com" }
                                    }
                                }
                            }

                            products := Band{
                                Eyebrow{ body: "PRODUCTS" }
                                Subhead{ body: "What ships under the brands." }
                                Lede{ body: "Concrete apps and services — not a vague platform promise." }
                                product_row := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 14.
                                    p1 := PanelCard{
                                        CardTitle{ body: "Khaynjo" }
                                        Body{ body: "Restaurant OS — tables, KOT, bills, day close." }
                                        open_khaynjo := QuietBtn{ text: "khaynjo.com" }
                                    }
                                    p2 := PanelCard{
                                        CardTitle{ body: "Moklanjo" }
                                        Body{ body: "Courier desk — booking, parties, day book." }
                                        open_moklanjo := QuietBtn{ text: "moklanjo.com" }
                                    }
                                    p3 := PanelCard{
                                        CardTitle{ body: "Kirano · Tabiyat" }
                                        Body{ body: "Kirana and pharmacy counters — stock, khata, GST memos." }
                                        open_kirano := QuietBtn{ text: "kirano.shop" }
                                        open_tabiyat := QuietBtn{ text: "tabiyat.shop" }
                                    }
                                    p4 := PanelCard{
                                        CardTitle{ body: "Wadhu Connect" }
                                        Body{ body: "WhatsApp Cloud API bridge — their number, your software." }
                                        open_connect := QuietBtn{ text: "connect.wadhu.com" }
                                    }
                                }
                                Body{ body: "Also: SMS Gateway (Play / desktop), GotiginSwap (experimental). Product privacy lives on each brand domain; company verifiers use gotigin.com." }
                            }

                            trust := BandAlt{
                                Eyebrow{ body: "FOR STORES · META · PARTNERS" }
                                Subhead{ body: "Policies reviewers look for." }
                                Lede{ body: "Same-host legal pages — used for Google Play, Microsoft Store, Meta App Review, and company verification." }
                                trust_row := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 12.
                                    trust_privacy := LinkBtn{ text: "Privacy" }
                                    trust_terms := GhostBtn{ text: "Terms" }
                                    trust_deletion := GhostBtn{ text: "Data deletion" }
                                    trust_cookies := GhostBtn{ text: "Cookies" }
                                    trust_security := GhostBtn{ text: "Security" }
                                }
                                Body{ body: "Canonical URLs: gotigin.com/privacy · /terms · /data-deletion · /support · /help · /about · /cookies · /security" }
                            }

                            company := Band{
                                Eyebrow{ body: "COMPANY" }
                                Subhead{ body: "GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                                Body{ body: "CIN U62011GJ2026PTC179660 · Incorporated 24 June 2026 · GSTIN 24AANCG1628G1ZP · PAN AANCG1628G" }
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
