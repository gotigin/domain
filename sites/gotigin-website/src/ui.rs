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
    // Transparent full-card hit target (Khaynjo pattern). Hover/press wash
    // sits above the card content so the whole tile reads as clickable.
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
            border_color_hover: #xf0c14a66
            border_color_down: #xf0c14a99
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
        draw_bg +: { color: Panel border_radius: 12. }
        flow: Down
        spacing: 12.
    }
    // Product tile: content + full-area CardHit overlay (opens brand site).
    let ProductCard = View{
        width: Fill height: Fit
        flow: Overlay
    }
    let BrandMark = Image{
        width: 56 height: 56
        fit: ImageFit.Stretch
    }
    let AppMark = Image{
        width: 64 height: 64
        fit: ImageFit.Stretch
    }
    let BrandBlock = View{
        width: Fill height: Fit
        flow: Down
        spacing: 16.
    }
    let BrandHead = RoundedView{
        width: Fill height: Fit
        padding: Inset{left: 22. right: 22. top: 18. bottom: 18.}
        draw_bg +: { color: Panel border_radius: 12. }
        flow: Right
        spacing: 16.
        align: Align{y: 0.5}
    }
    let BrandHeadText = View{
        width: Fill height: Fit
        flow: Down
        spacing: 6.
    }
    let AppGrid = View{
        width: Fill height: Fit
        flow: Right
        spacing: 14.
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
                                    Eyebrow{ body: "GOTIGIN" }
                                }
                                Headline{ body: "Where Technology Meets Discovery." }
                                Lede{ body: "Gotigin is the trademark. We build brands and products for discovery." }
                                cta_row := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 14.
                                    cta_about := LinkBtn{ text: "About Gotigin" }
                                    cta_privacy := GhostBtn{ text: "Privacy" }
                                }
                            }

                            brands := BandAlt{
                                Eyebrow{ body: "BRANDS" }
                                Subhead{ body: "What we ship." }
                                Lede{ body: "Kumkaj, Wadhu, and Ghal Bol — each with its own mark and products." }

                                brand_kumkaj := BrandBlock{
                                    kumkaj_head := ProductCard{
                                        kumkaj_head_bar := BrandHead{
                                            BrandMark{ src: crate_resource("self:resources/media/kumkaj-brand-logo.webp") }
                                            BrandHeadText{
                                                CardTitle{ body: "Kumkaj" }
                                                Body{ body: "Business operating systems — local-first books, parties, bills, and printing." }
                                                CardLink{ body: "kumkaj.com" }
                                            }
                                        }
                                        open_kumkaj := CardHit{}
                                    }
                                    kumkaj_row := AppGrid{
                                        p_khaynjo := ProductCard{
                                            PanelCard{
                                                AppMark{ src: crate_resource("self:resources/media/khaynjo-app-icon.webp") }
                                                CardTitle{ body: "Khaynjo" }
                                                Body{ body: "Restaurant OS — tables, KOT, bills, day close." }
                                                CardLink{ body: "khaynjo.com" }
                                            }
                                            open_khaynjo := CardHit{}
                                        }
                                        p_moklanjo := ProductCard{
                                            PanelCard{
                                                AppMark{ src: crate_resource("self:resources/media/moklanjo-app-icon.webp") }
                                                CardTitle{ body: "Moklanjo" }
                                                Body{ body: "Courier desk — booking, parties, day book." }
                                                CardLink{ body: "moklanjo.com" }
                                            }
                                            open_moklanjo := CardHit{}
                                        }
                                        p_kirano := ProductCard{
                                            PanelCard{
                                                AppMark{ src: crate_resource("self:resources/media/kirano-app-icon.webp") }
                                                CardTitle{ body: "Kirano" }
                                                Body{ body: "Kirana / grocery — catalogue, stock, khata, GST cash memo." }
                                                CardLink{ body: "kirano.shop" }
                                            }
                                            open_kirano := CardHit{}
                                        }
                                        p_tabiyat := ProductCard{
                                            PanelCard{
                                                AppMark{ src: crate_resource("self:resources/media/tabiyat-app-icon.webp") }
                                                CardTitle{ body: "Tabiyat" }
                                                Body{ body: "Pharmacy counter — batch, expiry, stock, GST tax invoice." }
                                                CardLink{ body: "tabiyat.shop" }
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
                                                CardTitle{ body: "Wadhu" }
                                                Body{ body: "More — reusable tools, utils, and apps that other Gotigin products (and anyone) can use." }
                                                CardLink{ body: "wadhu.com" }
                                            }
                                        }
                                        open_wadhu := CardHit{}
                                    }
                                    wadhu_row := AppGrid{
                                        p_sms := ProductCard{
                                            PanelCard{
                                                AppMark{ src: crate_resource("self:resources/media/sms-gateway-app-icon.webp") }
                                                CardTitle{ body: "SMS Gateway" }
                                                Body{ body: "Android-first SIM gateway — LAN and Bluetooth stay free forever." }
                                                CardLink{ body: "wadhu.com" }
                                            }
                                            open_sms := CardHit{}
                                        }
                                        p_connect := ProductCard{
                                            PanelCard{
                                                AppMark{ src: crate_resource("self:resources/media/wadhu-connect-app-icon.webp") }
                                                CardTitle{ body: "Wadhu Connect" }
                                                Body{ body: "WhatsApp Cloud API bridge — their number, your software." }
                                                CardLink{ body: "connect.wadhu.com" }
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
                                                CardTitle{ body: "Ghal Bol" }
                                                Body{ body: "Communication products — encrypted messaging under the Ghal Bol brand." }
                                                CardLink{ body: "ghalbol.com" }
                                            }
                                        }
                                        open_ghalbol := CardHit{}
                                    }
                                    ghalbol_row := AppGrid{
                                        p_ghalbol_app := ProductCard{
                                            PanelCard{
                                                AppMark{ src: crate_resource("self:resources/media/ghalbol-app-icon.webp") }
                                                CardTitle{ body: "Ghal Bol" }
                                                Body{ body: "Encrypted messenger — private rooms and secure communication." }
                                                CardLink{ body: "ghalbol.com" }
                                            }
                                            open_ghalbol_app := CardHit{}
                                        }
                                    }
                                }

                            }

                            company := Band{
                                Eyebrow{ body: "COMPANY" }
                                Subhead{ body: "GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                                Lede{ body: "Private limited company in Dhoraji, Gujarat. Incorporated 24 June 2026. We build software and hardware, and support the products that run on them." }
                                company_row := AppGrid{
                                    PanelCard{
                                        CardTitle{ body: "Registration" }
                                        Body{ body: "CIN U62011GJ2026PTC179660" }
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
                                        Body{ body: "mearaj@gotigin.com" }
                                        Body{ body: "+91 99677 17702" }
                                        Body{ body: "gotigin.com" }
                                    }
                                }
                                company_cta := View{
                                    width: Fill height: Fit
                                    flow: Right
                                    spacing: 14.
                                    cta_about_full := LinkBtn{ text: "About" }
                                    open_mail := GhostBtn{ text: "Email" }
                                    cta_contact := GhostBtn{ text: "Support" }
                                }
                            }

                            close := BandAlt{
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
                                Subhead{ body: "Where Technology Meets Discovery." }
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
                                        Body{ body: "mearaj@gotigin.com" }
                                    }
                                    View{
                                        width: Fill height: Fit
                                        flow: Down
                                        spacing: 4.
                                        Eyebrow{ body: "BRANDS" }
                                        CardTitle{ body: "Kumkaj · Wadhu · Ghal Bol" }
                                        Body{ body: "Each brand ships its own products" }
                                    }
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
