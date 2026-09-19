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

/// The house accent -- same value `STYLE.h2.color` already carries, so a
/// document with no brand default and no per-document override renders
/// pixel-identical to before `--pg-accent` existed. Bare hex, no `#`, same
/// convention as every other color in `STYLE` (`css()`/`hex_color()` add
/// the `#` at the point each renderer actually needs it).
pub const DEFAULT_ACCENT: &str = "2F5496";

/// BB26091501 -- accent resolution order for a generated document.
/// **Per-document override wins if present, otherwise the brand's own
/// default, otherwise the house default.** This is a pure, read-only
/// lookup: it never writes `brand_default` back, so calling it twice with
/// the same inputs and no override always returns the same value (two
/// briefs generated without touching the input are identical), and
/// supplying `document_override` for one call can never be observed by a
/// later call that omits it (overriding one document never mutates the
/// brand default).
pub fn resolve_accent(document_override: Option<&str>, brand_default: Option<&str>) -> String {
    document_override
        .filter(|s| !s.is_empty())
        .or_else(|| brand_default.filter(|s| !s.is_empty()))
        .unwrap_or(DEFAULT_ACCENT)
        .to_string()
}

#[cfg(test)]
mod accent_tests {
    use super::*;

    #[test]
    fn falls_back_to_the_house_default_when_neither_is_set() {
        assert_eq!(resolve_accent(None, None), DEFAULT_ACCENT);
    }

    #[test]
    fn uses_the_brand_default_when_no_override_is_given() {
        assert_eq!(resolve_accent(None, Some("1f7a44")), "1f7a44");
    }

    #[test]
    fn a_document_override_wins_over_the_brand_default() {
        assert_eq!(resolve_accent(Some("ff0000"), Some("1f7a44")), "ff0000");
    }

    #[test]
    fn empty_strings_are_treated_as_absent_at_every_level() {
        assert_eq!(resolve_accent(Some(""), Some("1f7a44")), "1f7a44");
        assert_eq!(resolve_accent(Some(""), Some("")), DEFAULT_ACCENT);
    }

    #[test]
    fn resolving_twice_with_no_override_is_identical_and_never_mutates_the_brand_default() {
        let brand_default = Some("1f7a44");
        let first = resolve_accent(None, brand_default);
        // Overriding a different, hypothetical document in between --
        // `brand_default` is a plain `&str` binding, so this line alone
        // already proves it cannot have been written back to; the repeat
        // resolution below is the behavioral half of that same guarantee.
        let _ = resolve_accent(Some("ff0000"), brand_default);
        let second = resolve_accent(None, brand_default);
        assert_eq!(first, second);
        assert_eq!(brand_default, Some("1f7a44"));
    }
}
