//! Figure SVGs flattened for the screen: egui's resvg loader has no fonts, so
//! raw `<text>` vanishes silently while the PDF path (Typst) draws it fine.
//! Text is converted to paths here, against fonts this crate controls.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use usvg::fontdb;

const ATKINSON: &[u8] = include_bytes!("../assets/AtkinsonHyperlegible-Regular.ttf");
const ATKINSON_BOLD: &[u8] = include_bytes!("../assets/AtkinsonHyperlegible-Bold.ttf");

fn fonts() -> Arc<fontdb::Database> {
    static FONTS: OnceLock<Arc<fontdb::Database>> = OnceLock::new();
    Arc::clone(FONTS.get_or_init(|| {
        let mut db = fontdb::Database::new();
        db.load_font_data(ATKINSON.to_vec());
        db.load_font_data(ATKINSON_BOLD.to_vec());
        // STIX covers the Greek and math characters Atkinson lacks.
        db.load_font_data(crate::formula::STIX_MATH.to_vec());
        db.load_system_fonts();
        db.set_sans_serif_family("Atkinson Hyperlegible");
        Arc::new(db)
    }))
}

/// Shared by the viewer's flattening and [`crate::lesson::Lesson::audit`].
pub(crate) fn tree(svg: &str) -> Result<usvg::Tree, usvg::Error> {
    let options = usvg::Options {
        fontdb: fonts(),
        ..usvg::Options::default()
    };
    usvg::Tree::from_str(svg, &options)
}

/// `svg` with text converted to paths, cached under `uri`. An unparsable SVG
/// passes through unchanged; the audit is where that gets reported.
pub(crate) fn flattened(uri: &str, svg: &str) -> Arc<[u8]> {
    static CACHE: OnceLock<Mutex<HashMap<String, Arc<[u8]>>>> = OnceLock::new();
    let mut cache = CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .expect("figure cache poisoned");
    Arc::clone(cache.entry(uri.to_owned()).or_insert_with(|| {
        let flat = match tree(svg) {
            Ok(tree) => tree.to_string(&usvg::WriteOptions::default()),
            Err(_) => svg.to_owned(),
        };
        Arc::from(flat.into_bytes())
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_is_flattened_to_paths() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 40" font-family="sans-serif" font-size="11"><text x="10" y="20">1,200</text></svg>"##;
        let flat = String::from_utf8(flattened("bytes://figure-test.svg", svg).to_vec()).unwrap();
        assert!(!flat.contains("<text"), "text nodes must not survive: {flat}");
        assert!(flat.contains("<path"), "the label must come back as paths: {flat}");
    }

    #[test]
    fn greek_labels_get_glyphs() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 40" font-family="sans-serif" font-size="11"><text x="10" y="20">&#949;</text></svg>"##;
        let flat = String::from_utf8(flattened("bytes://figure-test-greek.svg", svg).to_vec()).unwrap();
        assert!(flat.contains("<path"), "epsilon must resolve to a glyph path: {flat}");
    }

    #[test]
    fn unparsable_svg_is_left_alone() {
        let svg = "<svg not even close";
        assert!(tree(svg).is_err());
        let flat = flattened("bytes://figure-test-broken.svg", svg);
        assert_eq!(&flat[..], svg.as_bytes());
    }
}
