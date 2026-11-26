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
    // Phone uses a Menu control; chips live in the drawer (not a wrapped second row).
    let ActiveNav = Button{
        width: Fit height: Fit
        margin: 0.
        padding: Inset{left: 10. right: 10. top: 16. bottom: 16.}
        draw_bg +: { color: Amber }
        draw_text +: { color: Ink text_style: theme.font_bold{font_size: 12.} }
    }
    let GhostBtn = Button{
        width: Fit height: Fit
        margin: 0.
        padding: Inset{left: 10. right: 10. top: 16. bottom: 16.}
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

    // Script faces for translated copy (sites/_shared/i18n/site_i18n.rs).
    // One family per script: the Latin UI face, then that script's Noto face.
    // Only the hidden prototypes below use them, so a family (and its files)
    // loads only when a visitor picks a language in that script; the walker
    // gives keyed widgets the family and restores their own for Latin.
    let UiCyrillic = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
    }
    let UiGreek = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
    }
    let UiDevanagari = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansDevanagari-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiBengali = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansBengali-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiGurmukhi = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansGurmukhi-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiGujarati = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansGujarati-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiOriya = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansOriya-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiTamil = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansTamil-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiTelugu = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansTelugu-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiKannada = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansKannada-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiMalayalam = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansMalayalam-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiSinhala = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansSinhala-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiArabic = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansArabic-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiHebrew = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansHebrew-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiThaana = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansThaana-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiTibetan = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSerifTibetan-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiOlChiki = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansOlChiki-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiMeetei = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansMeeteiMayek-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiThai = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansThai-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiLao = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansLao-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiKhmer = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansKhmer-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiMyanmar = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansMyanmar-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiEthiopic = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansEthiopic-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiArmenian = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansArmenian-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiGeorgian = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSansGeorgian-Regular.ttf") asc: 0.0 desc: 0.0}
    }
    let UiCjkSc = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/SiteCJK-SC.otf") asc: 0.0 desc: 0.0}
    }
    let UiCjkTc = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/SiteCJK-TC.otf") asc: 0.0 desc: 0.0}
    }
    let UiCjkJp = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/SiteCJK-JP.otf") asc: 0.0 desc: 0.0}
    }
    let UiCjkKr = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/SiteCJK-KR.otf") asc: 0.0 desc: 0.0}
    }
    let UiLatinExt = FontFamily{
        latin := FontMember{res: crate_resource("self:resources/fonts/IBMPlexSans-Text.ttf") asc: -0.1 desc: 0.0}
        script := FontMember{res: crate_resource("self:resources/fonts/NotoSans-Regular.ttf") asc: 0.0 desc: 0.0}
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
                        // Phone: [mark · Menu]. Desk: [mark ····· chips] on one row.
                        // apply_responsive toggles Menu vs inline chips — never wrap
                        // eight links onto a second row under the brand.
                        top := SolidView{
                            width: Fill height: Fit
                            padding: Inset{left: 16. right: 16. top: 12. bottom: 12.}
                            draw_bg +: { color: Steel }
                            flow: Down
                            spacing: 8.
                            align: Align{y: 0.5}
                            top_chrome := View{
                                width: Fill height: Fit
                                flow: Right
                                spacing: 10.
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
                                        padding: Inset{left: 2. right: 4. top: 12. bottom: 12.}
                                        text: "Gotigin"
                                        draw_bg +: { color: #x00000000 }
                                        draw_text +: {
                                            color: SoftAmber
                                            text_style: theme.font_bold{font_size: 16.}
                                        }
                                    }
                                }
                                top_spacer := View{
                                    width: Fill
                                    height: 1.
                                }
                                nav_toggle := GhostBtn{ text: "Menu" }
                            }
                            nav_row := View{
                                visible: false
                                width: Fill height: Fit
                                flow: Down
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
                                nav_login := GhostBtn{ text: "Login" }
                                nav_github := View{
                                    width: 44 height: 44
                                    flow: Overlay
                                    align: Align{x: 0.5 y: 0.5}
                                    View{
                                        width: Fill height: Fill
                                        flow: Down
                                        align: Align{x: 0.5 y: 0.5}
                                        Image{
                                            width: 18 height: 18
                                            fit: ImageFit.Stretch
                                            src: crate_resource("self:resources/media/github-mark.webp")
                                        }
                                    }
                                    open_nav_github := CardHit{}
                                }
                                nav_lang := GhostBtn{ text: "EN" }
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
                                    Eyebrow{ body: "COMPANY" }
                                }
                                hero_title := Headline{ body: "Gotigin" }
                                hero_slogan := Subhead{ body: "Where Technology Meets Discovery." }
                                hero_lede := Lede{ body: "Everyday software for business and personal use — free on your devices." }
                            }

                            promise := BandAlt{
                                Eyebrow{ body: "PROMISE" }
                                promise_title := Subhead{ body: "Full products. Free to use." }
                                promise_lede := Lede{ body: "Download and run the complete product at no cost. We charge only for optional services you choose." }
                                promise_row := AppGrid{
                                    PanelCard{
                                        CardTitle{ body: "Free locally" }
                                        CardBody{ body: "The full product on your devices stays free. No paywall on the core tool." }
                                    }
                                    PanelCard{
                                        CardTitle{ body: "No forced ads" }
                                        CardBody{ body: "Use the software without ads that interrupt your work." }
                                    }
                                }
                                promise_row2 := AppGrid{
                                    PanelCard{
                                        CardTitle{ body: "Pay for services" }
                                        CardBody{ body: "Optional cloud, demos, visits, and assistance — only when you ask for them." }
                                    }
                                    p_github := ProductCard{
                                        PanelCard{
                                            CardTitle{ body: "Open where published" }
                                            CardBody{ body: "Public repositories. Local and LAN keep working offline." }
                                            CardLink{ body: "github.com/gotigin" }
                                        }
                                        open_github_pubs := CardHit{}
                                    }
                                }
                            }

                            tech := Band{
                                Eyebrow{ body: "BUILT WITH" }
                                tech_title := Subhead{ body: "Rust and Makepad" }
                                tech_lede := Lede{ body: "Most Gotigin products are written in Rust. GUIs use Makepad for fast apps and websites on desktop, mobile, and the web." }
                                tech_row := AppGrid{
                                    PanelCard{
                                        CardTitle{ body: "Rust" }
                                        CardBody{ body: "Safe, fast, and reliable — built for software that handles real work every day." }
                                    }
                                    PanelCard{
                                        CardTitle{ body: "Makepad" }
                                        CardBody{ body: "One Rust UI stack for native apps and wasm sites — performant and local-first." }
                                    }
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
                                        Body{ body: "D-U-N-S 581757966" }
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
                                        Body{ body: "C/O Mearaj AB" }
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
                                close_lede := Lede{ body: "Products free. Services optional." }
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
                                    foot_help := GhostBtn{ text: "Help" }
                                    foot_support := GhostBtn{ text: "Support" }
                                    foot_about := GhostBtn{ text: "About" }
                                    foot_cookies := GhostBtn{ text: "Cookies" }
                                    foot_security := GhostBtn{ text: "Security" }
                                }
                                View{
                                    width: Fill height: Fit
                                    flow: Down
                                    spacing: 4.
                                    Body{ body: "© 2026 GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED" }
                                    Body{ body: "Gotigin · gotigin.com" }
                                }
                            }
                            // Hidden font prototypes for translated copy (never drawn; see Ui* families).
                            i18n_fonts := View{
                                visible: false
                                width: 0 height: 0
                                font_cyrillic := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCyrillic}}}
                                font_greek := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiGreek}}}
                                font_devanagari := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiDevanagari}}}
                                font_bengali := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiBengali}}}
                                font_gurmukhi := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiGurmukhi}}}
                                font_gujarati := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiGujarati}}}
                                font_oriya := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiOriya}}}
                                font_tamil := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiTamil}}}
                                font_telugu := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiTelugu}}}
                                font_kannada := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiKannada}}}
                                font_malayalam := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiMalayalam}}}
                                font_sinhala := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiSinhala}}}
                                font_arabic := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiArabic}}}
                                font_hebrew := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiHebrew}}}
                                font_thaana := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiThaana}}}
                                font_tibetan := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiTibetan}}}
                                font_olchiki := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiOlChiki}}}
                                font_meetei := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiMeetei}}}
                                font_thai := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiThai}}}
                                font_lao := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiLao}}}
                                font_khmer := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiKhmer}}}
                                font_myanmar := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiMyanmar}}}
                                font_ethiopic := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiEthiopic}}}
                                font_armenian := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiArmenian}}}
                                font_georgian := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiGeorgian}}}
                                font_cjk_sc := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCjkSc}}}
                                font_cjk_tc := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCjkTc}}}
                                font_cjk_jp := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCjkJp}}}
                                font_cjk_kr := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiCjkKr}}}
                                font_latin_ext := Label{text: "" draw_text +: {text_style: theme.font_regular{font_family: UiLatinExt}}}
                            }
                        }
                    }
                }
            }
        }
    }
}
