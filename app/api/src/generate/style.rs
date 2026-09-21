//! The house style spec -- one shared token table every archetype and every
//! renderer reads from. Ported verbatim (same numbers/colors) from iSconl
//! `scope/lib/generate/style.js`. Plain data, no logic.

pub struct Font {
    pub family: &'static str,
}

pub struct TextStyle {
    pub size: f32,
    pub bold: bool,
    pub color: &'static str,
    pub border_color: Option<&'static str>,
}

pub struct Page {
    pub size: &'static str,
    pub width_twips: u32,
    pub height_twips: u32,
    pub margin_top_twips: u32,
    pub margin_bottom_twips: u32,
    pub margin_left_twips: u32,
    pub margin_right_twips: u32,
    pub header_twips: u32,
    pub footer_twips: u32,
}

pub struct Spacing {
    pub body_after_twips: u32,
    pub body_line: f32,
    pub h2_before_twips: u32,
    pub h2_after_twips: u32,
}

pub struct Style {
    pub font: Font,
    pub h1: TextStyle,
    pub h2: TextStyle,
    pub body: TextStyle,
    pub meta: TextStyle,
    pub page: Page,
    pub spacing: Spacing,
}

pub const STYLE: Style = Style {
    font: Font { family: "Calibri" },
    h1: TextStyle { size: 15.0, bold: true, color: "1F3864", border_color: None },
    h2: TextStyle { size: 8.5, bold: true, color: "2F5496", border_color: Some("B4C6E7") },
    body: TextStyle { size: 9.0, bold: false, color: "000000", border_color: None },
    meta: TextStyle { size: 7.5, bold: false, color: "808080", border_color: None },
    page: Page {
        size: "A4",
        width_twips: 11906,
        height_twips: 16838,
        margin_top_twips: 737,
        margin_bottom_twips: 737,
        margin_left_twips: 907,
        margin_right_twips: 907,
        header_twips: 720,
        footer_twips: 720,
    },
    spacing: Spacing {
        body_after_twips: 70,
        body_line: 1.14,
        h2_before_twips: 140,
        h2_after_twips: 40,
    },
};
