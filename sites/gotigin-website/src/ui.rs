// gotigin.com — company / house-mark Makepad SPA.
// Order: Trademark (Gotigin) → Company → Brand → Product.
// Wadhu is Memon *more* (Tools · Apps · More), not a messaging brand.
// Legal HTML shares this Hosting tree and the same top nav.
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // Palette from the Gotigin GTG mark (deep navy circle + white field).
    let Ink = #x12102a
    let Panel = #x1c1838
    let Steel = #x221e42
    let Line = #x3a3568
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
        font_size: 40.
        text_style_normal: theme.font_bold{font_size: 40. line_spacing: 1.06}
        text_style_bold: theme.font_bold{font_size: 40. line_spacing: 1.06}
        draw_text +: { color: Paper text_style: theme.font_bold{font_size: 40. line_spacing: 1.06} }
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
        font_size: 26.
        text_style_normal: theme.font_bold{font_size: 26. line_spacing: 1.15}
        text_style_bold: theme.font_bold{font_size: 26. line_spacing: 1.15}
        draw_text +: { color: Paper text_style: theme.font_bold{font_size: 26. line_spacing: 1.15} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    // One unbreakable piece of a heading. A row of these wraps between
    // pieces, so "Ghal Bol" stays whole at every width.
    let SubheadPiece = Markdown{
        selectable: true
        height: Fit
        width: Fit
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Paper
        font_size: 26.
        text_style_normal: theme.font_bold{font_size: 26. line_spacing: 1.15}
        text_style_bold: theme.font_bold{font_size: 26. line_spacing: 1.15}
        draw_text +: { color: Paper text_style: theme.font_bold{font_size: 26. line_spacing: 1.15} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let CardTitlePiece = Markdown{
        selectable: false
        height: Fit
        width: Fit
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: SoftAmber
        font_size: 22.
        text_style_normal: theme.font_bold{font_size: 22. line_spacing: 1.2}
        text_style_bold: theme.font_bold{font_size: 22. line_spacing: 1.2}
        draw_text +: { color: SoftAmber text_style: theme.font_bold{font_size: 22. line_spacing: 1.2} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let PieceRow = View{
        width: Fill height: Fit
        flow: Right{wrap: true}
        spacing: 8.
        wrap_spacing: 2.
    }
    let Lede = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 17.
        text_style_normal: theme.font_regular{font_size: 17. line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 17. line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 17. line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    // Normal body ~15.5 — align with Khaynjo/Moklanjo (phone pass 2026-11-22).
    let Body = Markdown{
        selectable: true
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 15.5
        text_style_normal: theme.font_regular{font_size: 15.5 line_spacing: 1.45}
        text_style_bold: theme.font_bold{font_size: 15.5 line_spacing: 1.45}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 15.5 line_spacing: 1.45} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let CardTitle = Markdown{
        selectable: false
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: SoftAmber
        font_size: 22.
        text_style_normal: theme.font_bold{font_size: 22. line_spacing: 1.2}
        text_style_bold: theme.font_bold{font_size: 22. line_spacing: 1.2}
        draw_text +: { color: SoftAmber text_style: theme.font_bold{font_size: 22. line_spacing: 1.2} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let ProductTitle = Markdown{
        selectable: false
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Paper
        font_size: 18.
        text_style_normal: theme.font_bold{font_size: 18. line_spacing: 1.2}
        text_style_bold: theme.font_bold{font_size: 18. line_spacing: 1.2}
        draw_text +: { color: Paper text_style: theme.font_bold{font_size: 18. line_spacing: 1.2} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let Kicker = Markdown{
        selectable: false
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 11.
        text_style_normal: theme.font_bold{font_size: 11. line_spacing: 1.2}
        text_style_bold: theme.font_bold{font_size: 11. line_spacing: 1.2}
        draw_text +: { color: Muted text_style: theme.font_bold{font_size: 11. line_spacing: 1.2} }
        draw_selection +: { color: Select }
        draw_block +: { selection_color: Select }
    }
    let CardBody = Markdown{
        selectable: false
        height: Fit
        width: Fill
        padding: 0.
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_color: Muted
        font_size: 15.
        text_style_normal: theme.font_regular{font_size: 15. line_spacing: 1.4}
        text_style_bold: theme.font_bold{font_size: 15. line_spacing: 1.4}
        draw_text +: { color: Muted text_style: theme.font_regular{font_size: 15. line_spacing: 1.4} }
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
    // Nav and quiet buttons are ≥44 px tall: a finger-sized target on every tier.
    // Horizontal pad stays compact so eight nav labels wrap on a phone
    // (2026-11-22: 5th chip was cropped when padding stayed at 14).
    let ActiveNav = Button{
        width: Fit height: Fit
        margin: 0.
        padding: Inset{left: 10. right: 10. top: 14. bottom: 14.}
        draw_bg +: { color: Amber }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 12.} }
    }
    let GhostBtn = Button{
        width: Fit height: Fit
        margin: 0.
        padding: Inset{left: 10. right: 10. top: 14. bottom: 14.}
        draw_bg +: { color: Panel }
        draw_text +: { color: Paper text_style: theme.font_bold{font_size: 12.} }
    }
    // Full-card hit target. Later sibling in an Overlay, so hover and click
    // cover the tile. Label is zero-size; the wash is the affordance.
    let CardHit = Button{
        width: Fill height: Fill
        padding: 0.
        text: ""
        grab_key_focus: false
        reset_hover_on_click: true
        label_walk: Walk{width: 0. height: 0.}
        icon_walk: Walk{width: 0. height: 0.}
        draw_bg +: {
            color: #x00000000
            color_hover: #xe8a31733
            color_down: #xe8a31755
            color_focus: #x00000000
            color_2: #x00000000
            color_2_hover: #xe8a31733
            color_2_down: #xe8a31755
            color_2_focus: #x00000000
            border_color: #x00000000
            border_color_hover: #xf0c14a99
            border_color_down: #xf0c14a
            border_color_focus: #x00000000
            border_radius: 12.
            border_size: 1.5
        }
        draw_text +: {
            color: #x00000000
            color_hover: #x00000000
            color_down: #x00000000
            color_focus: #x00000000
        }
    }
    let Band = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 72. bottom: 72.}
        draw_bg +: { color: Ink }
        flow: Down
        spacing: 16.
    }
    let BandAlt = SolidView{
        width: Fill height: Fit
        padding: Inset{left: 64. right: 64. top: 72. bottom: 72.}
        draw_bg +: { color: Steel }
        flow: Down
        spacing: 16.
    }
    let PanelCard = RoundedView{
        width: Fill height: Fit
        padding: Inset{left: 20. right: 20. top: 18. bottom: 18.}
        draw_bg +: { color: Panel border_radius: 12. border_size: 1. border_color: Line }
        flow: Down
        spacing: 8.
    }
    let ProductCard = View{
        width: Fill height: Fit
        flow: Overlay
    }
    let BrandMark = Image{
        width: 56 height: 56
        fit: ImageFit.Stretch
    }
    let AppMark = Image{
        width: 56 height: 56
        fit: ImageFit.Stretch
    }
    let ProductRow = View{
        width: Fill height: Fit
        flow: Right
        spacing: 14.
        align: Align{y: 0.0}
    }
    let ProductText = View{
        width: Fill height: Fit
        flow: Down
        spacing: 4.
    }
    let BrandBlock = View{
        width: Fill height: Fit
        flow: Down
        spacing: 12.
    }
    let BrandHead = RoundedView{
        width: Fill height: Fit
        padding: Inset{left: 20. right: 20. top: 16. bottom: 16.}
        draw_bg +: { color: Panel border_radius: 12. border_size: 1. border_color: Line }
        flow: Right
        spacing: 16.
        align: Align{y: 0.5}
    }
    let BrandHeadText = View{
        width: Fill height: Fit
        flow: Down
        spacing: 4.
    }
    let AppGrid = View{
        width: Fill height: Fit
        flow: Right
        spacing: 14.
        align: Align{x: 0.0 y: 0.0}
    }
    let CardLink = Markdown{
        selectable: false
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
                        // Default is the narrow bar (nav under the mark). Desk
                        // widths pull the nav onto one row in apply_responsive.
                        top := SolidView{
                            width: Fill height: Fit
                            padding: Inset{left: 16. right: 16. top: 12. bottom: 12.}
                            draw_bg +: { color: Steel }
                            flow: Down
                            spacing: 8.
                            align: Align{y: 0.5}
                            brand_row := View{
                                width: Fit height: Fit
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
                            }
                            top_spacer := View{
                                visible: false
                                width: Fill
                                height: 40.
                            }
                            // Phone: wrap so every chip stays visible (ScrollX
                            // previously cropped the 5th chip — 2026-11-22).
                            // Desk: one row (apply_responsive sets flow Right).
                            nav_row := View{
                                width: Fill height: Fit
                                flow: Right{wrap: true}
                                spacing: 6.
                                wrap_spacing: 6.
                                align: Align{y: 0.5}
                                nav_home := ActiveNav{ text: "Home" }
                                nav_about := GhostBtn{ text: "About" }
                                nav_privacy := GhostBtn{ text: "Privacy" }
                                nav_terms := GhostBtn{ text: "Terms" }
                                nav_cancellation := GhostBtn{ text: "Cancellation" }
                                nav_deletion := GhostBtn{ text: "Deletion" }
                                nav_help := GhostBtn{ text: "Help" }
                                nav_support := GhostBtn{ text: "Support" }
                            }
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
                                    Eyebrow{ body: "TRADEMARK" }
                                }
                                hero_title := Headline{ body: "Gotigin" }
                                hero_slogan := Subhead{ body: "Where Technology Meets Discovery." }
                                hero_lede := Lede{ body: "Memon for find it. Pronounced go-tee-geen. House mark of GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED." }
                                cta_row := View{
                                    width: Fill height: Fit
                                    flow: Right{wrap: true}
                                    spacing: 12.
                                    wrap_spacing: 10.
                                    cta_about := LinkBtn{ text: "About" }
                                    cta_privacy := GhostBtn{ text: "Privacy" }
                                    cta_help := GhostBtn{ text: "Help" }
                                }
                            }

                            brands := BandAlt{
                                Eyebrow{ body: "BRANDS" }
                                brands_title := PieceRow{
                                    brands_t1 := SubheadPiece{ body: "Kumkaj ·" }
                                    brands_t2 := SubheadPiece{ body: "Wadhu ·" }
                                    brands_t3 := SubheadPiece{ body: "Ghal Bol" }
                                }

                                brand_kumkaj := BrandBlock{
                                    kumkaj_head := ProductCard{
                                        kumkaj_head_bar := BrandHead{
                                            BrandMark{ src: crate_resource("self:resources/media/kumkaj-brand-logo.webp") }
                                            BrandHeadText{
                                                Kicker{ body: "BRAND" }
                                                CardTitle{ body: "Kumkaj" }
                                                CardBody{ body: "Business operating systems." }
                                                CardLink{ body: "kumkaj.com" }
                                            }
                                        }
                                        open_kumkaj := CardHit{}
                                    }
                                    kumkaj_row := AppGrid{
                                        p_khaynjo := ProductCard{
                                            PanelCard{
                                                ProductRow{
                                                    AppMark{ src: crate_resource("self:resources/media/khaynjo-app-icon.webp") }
                                                    ProductText{
                                                        Kicker{ body: "PRODUCT" }
                                                        ProductTitle{ body: "Khaynjo" }
                                                        CardBody{ body: "Restaurant OS. Tables, KOT, bills, day close." }
                                                        CardLink{ body: "khaynjo.com" }
                                                    }
                                                }
                                            }
                                            open_khaynjo := CardHit{}
                                        }
                                        p_moklanjo := ProductCard{
                                            PanelCard{
                                                ProductRow{
                                                    AppMark{ src: crate_resource("self:resources/media/moklanjo-app-icon.webp") }
                                                    ProductText{
                                                        Kicker{ body: "PRODUCT" }
                                                        ProductTitle{ body: "Moklanjo" }
                                                        CardBody{ body: "Courier desk. Booking, parties, day book." }
                                                        CardLink{ body: "moklanjo.com" }
                                                    }
                                                }
                                            }
                                            open_moklanjo := CardHit{}
                                        }
                                    }
                                    kumkaj_row2 := AppGrid{
                                        p_kirano := ProductCard{
                                            PanelCard{
                                                ProductRow{
                                                    AppMark{ src: crate_resource("self:resources/media/kirano-app-icon.webp") }
                                                    ProductText{
                                                        Kicker{ body: "PRODUCT" }
                                                        ProductTitle{ body: "Kirano" }
                                                        CardBody{ body: "Kirana. Catalogue, stock, khata, GST cash memo." }
                                                        CardLink{ body: "kirano.shop" }
                                                    }
                                                }
                                            }
                                            open_kirano := CardHit{}
                                        }
                                        p_tabiyat := ProductCard{
                                            PanelCard{
                                                ProductRow{
                                                    AppMark{ src: crate_resource("self:resources/media/tabiyat-app-icon.webp") }
                                                    ProductText{
                                                        Kicker{ body: "PRODUCT" }
                                                        ProductTitle{ body: "Tabiyat" }
                                                        CardBody{ body: "Pharmacy counter. Batch, expiry, stock, GST bills." }
                                                        CardLink{ body: "tabiyat.shop" }
                                                    }
                                                }
                                            }
                                            open_tabiyat := CardHit{}
                                        }
                                    }
                                }

                                brand_wadhu := BrandBlock{
                                    wadhu_head := ProductCard{
                                        wadhu_head_bar := BrandHead{
                                            BrandMark{ src: crate_resource("self:resources/media/wadhu-brand-logo.webp") }
                                            BrandHeadText{
                                                Kicker{ body: "BRAND" }
                                                CardTitle{ body: "Wadhu" }
                                                CardBody{ body: "Memon for more. Tools · Apps · More." }
                                                CardBody{ body: "Reusable tools, utils, and apps. Not a messenger." }
                                                CardLink{ body: "wadhu.com" }
                                            }
                                        }
                                        open_wadhu := CardHit{}
                                    }
                                    wadhu_row := AppGrid{
                                        p_sms := ProductCard{
                                            PanelCard{
                                                ProductRow{
                                                    AppMark{ src: crate_resource("self:resources/media/sms-gateway-app-icon.webp") }
                                                    ProductText{
                                                        Kicker{ body: "PRODUCT" }
                                                        ProductTitle{ body: "SMS Gateway" }
                                                        CardBody{ body: "Android-first SIM messaging. LAN and Bluetooth stay free." }
                                                        CardLink{ body: "wadhu.com" }
                                                    }
                                                }
                                            }
                                            open_sms := CardHit{}
                                        }
                                        p_relay := ProductCard{
                                            PanelCard{
                                                ProductRow{
                                                    AppMark{ src: crate_resource("self:resources/media/wadhu-brand-logo.webp") }
                                                    ProductText{
                                                        Kicker{ body: "PRODUCT" }
                                                        ProductTitle{ body: "SMS Relay" }
                                                        CardBody{ body: "Optional internet plane for paired gateways." }
                                                        CardLink{ body: "wadhu.com" }
                                                    }
                                                }
                                            }
                                            open_relay := CardHit{}
                                        }
                                        p_connect := ProductCard{
                                            PanelCard{
                                                ProductRow{
                                                    AppMark{ src: crate_resource("self:resources/media/wadhu-connect-app-icon.webp") }
                                                    ProductText{
                                                        Kicker{ body: "PRODUCT" }
                                                        ProductTitle{ body: "Wadhu Connect" }
                                                        CardBody{ body: "WhatsApp Cloud API bridge." }
                                                        CardLink{ body: "connect.wadhu.com" }
                                                    }
                                                }
                                            }
                                            open_connect := CardHit{}
                                        }
                                    }
                                }

                                brand_ghalbol := BrandBlock{
                                    ghalbol_head := ProductCard{
                                        ghalbol_head_bar := BrandHead{
                                            BrandMark{ src: crate_resource("self:resources/media/ghalbol-brand-logo.webp") }
                                            BrandHeadText{
                                                Kicker{ body: "BRAND" }
                                                CardTitle{ body: "Ghal Bol" }
                                                CardBody{ body: "Communication products." }
                                                CardLink{ body: "ghalbol.com" }
                                            }
                                        }
                                        open_ghalbol := CardHit{}
                                    }
                                    ghalbol_row := AppGrid{
                                        p_ghalbol_app := ProductCard{
                                            PanelCard{
                                                ProductRow{
                                                    AppMark{ src: crate_resource("self:resources/media/ghalbol-app-icon.webp") }
                                                    ProductText{
                                                        Kicker{ body: "PRODUCT" }
                                                        ProductTitle{ body: "Ghal Bol" }
                                                        CardBody{ body: "Encrypted messenger." }
                                                        CardLink{ body: "ghalbol.com" }
                                                    }
                                                }
                                            }
                                            open_ghalbol_app := CardHit{}
                                        }
                                    }
                                }
                            }

                            trust := Band{
                                Eyebrow{ body: "PAGES" }
                                trust_title := Subhead{ body: "Company pages" }
                                trust_row := AppGrid{
                                    ProductCard{
                                        PanelCard{
                                            Kicker{ body: "PAGE" }
                                            ProductTitle{ body: "Privacy" }
                                            CardBody{ body: "Company privacy policy." }
                                            CardLink{ body: "/privacy" }
                                        }
                                        open_pol_privacy := CardHit{}
                                    }
                                    ProductCard{
                                        PanelCard{
                                            Kicker{ body: "PAGE" }
                                            ProductTitle{ body: "Terms" }
                                            CardBody{ body: "Terms of use for gotigin.com." }
                                            CardLink{ body: "/terms" }
                                        }
                                        open_pol_terms := CardHit{}
                                    }
                                    ProductCard{
                                        PanelCard{
                                            Kicker{ body: "PAGE" }
                                            ProductTitle{ body: "Cancellation" }
                                            CardBody{ body: "Cancellation and refund policy." }
                                            CardLink{ body: "/cancellation" }
                                        }
                                        open_pol_cancellation := CardHit{}
                                    }
                                }
                                trust_row2 := AppGrid{
                                    ProductCard{
                                        PanelCard{
                                            Kicker{ body: "PAGE" }
                                            ProductTitle{ body: "Data deletion" }
                                            CardBody{ body: "How to ask for deletion." }
                                            CardLink{ body: "/data-deletion" }
                                        }
                                        open_pol_deletion := CardHit{}
                                    }
                                    ProductCard{
                                        PanelCard{
                                            Kicker{ body: "PAGE" }
                                            ProductTitle{ body: "Support" }
                                            CardBody{ body: "Email, phone, and product routing." }
                                            CardLink{ body: "/support" }
                                        }
                                        open_pol_support := CardHit{}
                                    }
                                }
                                trust_row3 := AppGrid{
                                    ProductCard{
                                        PanelCard{
                                            Kicker{ body: "PAGE" }
                                            ProductTitle{ body: "Help" }
                                            CardBody{ body: "Names, products, and sites." }
                                            CardLink{ body: "/help" }
                                        }
                                        open_pol_help := CardHit{}
                                    }
                                    ProductCard{
                                        PanelCard{
                                            Kicker{ body: "PAGE" }
                                            ProductTitle{ body: "About" }
                                            CardBody{ body: "Company registration and contacts." }
                                            CardLink{ body: "/about" }
                                        }
                                        open_pol_about := CardHit{}
                                    }
                                }
                                trust_row4 := AppGrid{
                                    ProductCard{
                                        PanelCard{
                                            Kicker{ body: "PAGE" }
                                            ProductTitle{ body: "Cookies" }
                                            CardBody{ body: "Cookies on gotigin.com." }
                                            CardLink{ body: "/cookies" }
                                        }
                                        open_pol_cookies := CardHit{}
                                    }
                                    ProductCard{
                                        PanelCard{
                                            Kicker{ body: "PAGE" }
                                            ProductTitle{ body: "Security" }
                                            CardBody{ body: "How to report a vulnerability." }
                                            CardLink{ body: "/security" }
                                        }
                                        open_pol_security := CardHit{}
                                    }
                                }
                            }

                            company := BandAlt{
                                Eyebrow{ body: "COMPANY" }
                                company_title := Subhead{ body: "GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                                company_lede := Lede{ body: "Private limited company. Dhoraji, Gujarat. Incorporated 24 June 2026." }
                                company_row := AppGrid{
                                    PanelCard{
                                        CardTitle{ body: "Registration" }
                                        Body{ body: "CIN" }
                                        Body{ body: "U62011GJ2026PTC179660" }
                                        Body{ body: "Incorporated 24 June 2026" }
                                        Body{ body: "Share capital ₹1,00,000" }
                                        Body{ body: "Limited by shares · Gujarat" }
                                    }
                                    PanelCard{
                                        CardTitle{ body: "Tax" }
                                        Body{ body: "PAN AANCG1628G" }
                                        Body{ body: "TAN RKTG08602G" }
                                        Body{ body: "GSTIN 24AANCG1628G1ZP" }
                                        Body{ body: "GST Regular · from 04 July 2026" }
                                    }
                                    PanelCard{
                                        CardTitle{ body: "Recognition" }
                                        Body{ body: "DPIIT DIPP271306" }
                                        Body{ body: "Issued 08 July 2026" }
                                        Body{ body: "Valid to 23 June 2036" }
                                        Body{ body: "Shops SR131000001 · Dhoraji" }
                                    }
                                }
                                company_row2 := AppGrid{
                                    PanelCard{
                                        CardTitle{ body: "Registered office" }
                                        Body{ body: "Gosiya Manzil, Darbargadh" }
                                        Body{ body: "Dhoraji, Rajkot" }
                                        Body{ body: "Gujarat 360410, India" }
                                    }
                                    PanelCard{
                                        CardTitle{ body: "Directors" }
                                        Body{ body: "Mearaj Abdulaziz Bhagad" }
                                        Body{ body: "Bhagad Senila Mearaj" }
                                    }
                                    PanelCard{
                                        CardTitle{ body: "Contact" }
                                        Body{ body: "support@gotigin.com" }
                                        Body{ body: "mearaj@gmail.com" }
                                        Body{ body: "+91 99677 17702" }
                                        Body{ body: "gotigin.com" }
                                    }
                                }
                                company_cta := View{
                                    width: Fill height: Fit
                                    flow: Right{wrap: true}
                                    spacing: 12.
                                    wrap_spacing: 10.
                                    cta_about_full := LinkBtn{ text: "About" }
                                    open_mail := GhostBtn{ text: "Email" }
                                    cta_contact := GhostBtn{ text: "Support" }
                                }
                            }

                            // Not `close`: that id is the window caption's close
                            // button, and a lookup by it never reaches this band.
                            closing := Band{
                                close_brand := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 12.
                                    align: Align{y: 0.5}
                                    Image{
                                        width: 48 height: 48
                                        fit: ImageFit.Stretch
                                        src: crate_resource("self:resources/media/gotigin-brand-logo.webp")
                                    }
                                    CardTitle{ body: "Gotigin" }
                                }
                                close_title := Subhead{ body: "Where Technology Meets Discovery." }
                                names := View{
                                    width: Fill height: Fit
                                    flow: Down
                                    spacing: 14.
                                    View{
                                        width: Fill height: Fit
                                        flow: Down
                                        spacing: 4.
                                        Eyebrow{ body: "TRADEMARK" }
                                        CardTitle{ body: "Gotigin" }
                                        Body{ body: "Memon for “find it” · pronounced go-tee-geen" }
                                    }
                                    View{
                                        width: Fill height: Fit
                                        flow: Down
                                        spacing: 4.
                                        Eyebrow{ body: "COMPANY" }
                                        CardTitle{ body: "GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                                        Body{ body: "support@gotigin.com · mearaj@gmail.com" }
                                    }
                                    View{
                                        width: Fill height: Fit
                                        flow: Down
                                        spacing: 4.
                                        Eyebrow{ body: "BRANDS" }
                                        PieceRow{
                                            CardTitlePiece{ body: "Kumkaj ·" }
                                            CardTitlePiece{ body: "Wadhu ·" }
                                            CardTitlePiece{ body: "Ghal Bol" }
                                        }
                                        Body{ body: "Business systems · tools and apps · communication" }
                                    }
                                }
                                close_legal := View{
                                    width: Fill height: Fit
                                    flow: Right{wrap: true}
                                    spacing: 6.
                                    wrap_spacing: 6.
                                    foot_privacy := GhostBtn{ text: "Privacy" }
                                    foot_terms := GhostBtn{ text: "Terms" }
                                    foot_cancellation := GhostBtn{ text: "Cancellation" }
                                    foot_deletion := GhostBtn{ text: "Deletion" }
                                    foot_support := GhostBtn{ text: "Support" }
                                    foot_about := GhostBtn{ text: "About" }
                                }
                                View{
                                    width: Fill height: Fit
                                    flow: Down
                                    spacing: 4.
                                    Body{ body: "© 2026 GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                                    Body{ body: "Gotigin · gotigin.com" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
