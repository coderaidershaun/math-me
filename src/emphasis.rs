//! Baking a lesson's declared vectors and matrices into bold, in the one
//! Typst compile the viewer and the PDF exporter both go through.
//!
//! `\mathbf` is unavailable (mitex emits `mitexmathbf`, which the crate's
//! prelude-free Typst document never defines), so bolding is done here
//! instead, on the mitex-produced Typst source, by wrapping every declared
//! letter in `bold(...)`.

use crate::glossary::Glossary;
use crate::symbols;

/// Wrap every declared vector or matrix letter in `bold(...)`.
///
/// `typst_math` is mitex output, which this scans rather than reparses —
/// mitex writes every parenthesis as an escape (`\(`, `\)`), so a `\`
/// unconditionally passes its next character through untouched. An
/// identifier is skipped when it is immediately followed by `(` (it is a
/// call — `frac`, `lr`, `macron`, `bb`, `upright`, `sum`, … — not a name) or
/// by `_` (a subscript names one entry of the object, not the object
/// itself: `x_(t-1)` stays plain even when `x` is declared a vector).
/// Otherwise, if the identifier resolves to a character the glossary has a
/// role for, the wrapped content is that literal character, not the
/// identifier's name — `bold(ε)`, not `bold(epsilon)`. Typst's math grammar
/// parses a single character as literal content but a multi-letter name as
/// an identifier to be evaluated, and evaluating a name like `epsilon`
/// inside a call resolves it to a symbol *value* that carries no source
/// span; the glyph Typst then draws for it is invisible to
/// [`crate::terms::partition`], which relies on that span to place the
/// glyph in a term. A single Latin letter such as `x` already parses as
/// literal content, so this changes nothing for it — `ident_char` returns
/// the very character that was scanned.
pub(crate) fn bolden(typst_math: &str, glossary: &Glossary) -> String {
    let chars: Vec<char> = typst_math.chars().collect();
    let mut out = String::with_capacity(typst_math.len());
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if ch == '\\' {
            out.push(ch);
            if let Some(&next) = chars.get(i + 1) {
                out.push(next);
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }

        if is_ident_start(ch) {
            let start = i;
            i = ident_end(&chars, i);
            let ident: String = chars[start..i].iter().collect();

            // mitex pads its output with spaces (`x _(t )`, not `x_(t)`), so
            // the lookahead for a call or a subscript has to see past them.
            let mut after = i;
            while chars.get(after) == Some(&' ') {
                after += 1;
            }
            let bold = (!matches!(chars.get(after), Some('(') | Some('_')))
                .then(|| symbols::ident_char(&ident))
                .flatten()
                .filter(|&ch| glossary.role(ch).is_some());
            match bold {
                Some(ch) => {
                    out.push_str("bold(");
                    out.push(ch);
                    out.push(')');
                }
                None => out.push_str(&ident),
            }
            continue;
        }

        out.push(ch);
        i += 1;
    }

    out
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

/// The index just past the identifier starting at `start`, including one
/// `.suffix` if there is one (`epsilon.alt`).
fn ident_end(chars: &[char], start: usize) -> usize {
    let mut i = start + 1;
    while i < chars.len() && is_ident_continue(chars[i]) {
        i += 1;
    }
    if chars.get(i) == Some(&'.') && chars.get(i + 1).is_some_and(|&ch| is_ident_start(ch)) {
        i += 1;
        while i < chars.len() && is_ident_continue(chars[i]) {
            i += 1;
        }
    }
    i
}

/// Undo [`bolden`]: remove every `bold(...)` wrapper, keeping its content.
///
/// Used by [`crate::terms::key`] so a term's dictionary key is stable
/// whether or not the glyphs that produced it were bolded — otherwise
/// bolding would turn the key `a L x` into `bold(a)bold(L)bold(x)` and every
/// `.explain()` entry in a lesson would silently stop matching.
///
/// A wrapper's content is restored via [`symbols::char_ident`] where that
/// resolves: `bolden` writes a Greek letter's *character*, not its mitex
/// name, so undoing it has to spell the name back out — `bold(ε)` must
/// become `epsilon`, not `ε`, to match the key an un-bolded `\varepsilon`
/// produces elsewhere. A single Latin letter has no such entry and comes
/// back unchanged, since `bold(x)` already wraps the letter itself.
pub(crate) fn strip_bold(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;

    while i < chars.len() {
        let word_boundary = i == 0 || !is_ident_continue(chars[i - 1]);
        if word_boundary && chars[i..].starts_with(&['b', 'o', 'l', 'd', '(']) {
            let open = i + 5;
            let mut depth = 1;
            let mut j = open;
            while j < chars.len() && depth > 0 {
                match chars[j] {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => {}
                }
                j += 1;
            }
            let close = if depth == 0 { j - 1 } else { j };
            let inner = strip_bold(&chars[open..close].iter().collect::<String>());

            let mut inner_chars = inner.chars();
            let restored = match (inner_chars.next(), inner_chars.next()) {
                (Some(ch), None) => symbols::char_ident(ch).map(str::to_owned).unwrap_or(inner),
                _ => inner,
            };
            out.push_str(&restored);
            i = j;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glossary::Role;

    fn glossary(roles: &[(char, Role)]) -> Glossary {
        let mut glossary = Glossary::default();
        for &(ch, role) in roles {
            glossary.insert_role(ch, role);
        }
        glossary
    }

    #[test]
    fn wraps_a_declared_letter_and_a_declared_greek_name() {
        let glossary = glossary(&[('x', Role::Vector), ('L', Role::Matrix), ('ε', Role::Vector)]);
        // The Greek name wraps as its literal character, `bold(ε)`, not
        // `bold(epsilon)` — see the doc comment on `bolden` for why: Typst
        // evaluates a multi-letter name used as a call argument, and the
        // resulting symbol value carries no span for `terms::partition` to
        // find the glyph by.
        assert_eq!(
            bolden("x = a L x + epsilon", &glossary),
            "bold(x) = a bold(L) bold(x) + bold(ε)"
        );
    }

    #[test]
    fn skips_a_subscript_base_and_a_call_name() {
        let glossary = glossary(&[('x', Role::Vector), ('L', Role::Matrix)]);
        assert_eq!(bolden("x_(t-1) = a x_(t-2)", &glossary), "x_(t-1) = a x_(t-2)");
        assert_eq!(bolden("frac(x,L)", &glossary), "frac(bold(x),bold(L))");
        assert_eq!(bolden("lr(x)", &glossary), "lr(bold(x))");
        assert_eq!(bolden("macron(x)", &glossary), "macron(bold(x))");
    }

    #[test]
    fn a_superscript_still_bolds() {
        let glossary = glossary(&[('L', Role::Matrix)]);
        assert_eq!(bolden("L^(-1)", &glossary), "bold(L)^(-1)");
    }

    #[test]
    fn leaves_escaped_parens_untouched() {
        let glossary = Glossary::default();
        assert_eq!(bolden(r"\(a+b\)", &glossary), r"\(a+b\)");
    }

    #[test]
    fn is_a_no_op_for_an_empty_glossary() {
        let glossary = Glossary::default();
        assert_eq!(bolden("x = a L x + epsilon", &glossary), "x = a L x + epsilon");
    }

    #[test]
    fn strip_bold_removes_wrappers_and_leaves_plain_text_untouched() {
        assert_eq!(strip_bold("bold(a) bold(L) bold(x)"), "a L x");
        assert_eq!(strip_bold("a bold(L) bold(x)"), "a L x");
        assert_eq!(strip_bold("a L x"), "a L x");
        assert_eq!(strip_bold("frac(bold(x),bold(L))"), "frac(x,L)");
    }

    /// The round trip `bolden` and `strip_bold` must agree on: a declared
    /// Greek letter's stripped key must read as the mitex *name* an
    /// un-bolded occurrence of the same letter produces, not the literal
    /// character `bolden` actually wrapped.
    #[test]
    fn strip_bold_restores_a_greek_letters_mitex_name() {
        assert_eq!(strip_bold("bold(ε)"), "epsilon");
        assert_eq!(strip_bold("a + bold(ε)"), "a + epsilon");
    }

    /// mitex pads its output with a space before `_` (`x _(t )`, not
    /// `x_(t)`) — real conversion output, not a hand-typed fixture, is what
    /// catches that a naive "is the very next character `_`?" check misses.
    /// This is the lesson's own opening contrast: `x_t` must stay plain
    /// even though `x` is declared a vector.
    #[test]
    fn a_subscript_stays_plain_even_with_mitexs_space_before_the_underscore() {
        let glossary = glossary(&[('x', Role::Vector), ('ε', Role::Vector)]);
        let typst = crate::formula::to_typst_math(r"x_t = a x_{t-1} + \varepsilon_t").expect("mitex");

        assert_eq!(bolden(&typst, &glossary), typst, "a subscripted base must not be wrapped");
    }
}
