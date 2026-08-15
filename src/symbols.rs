//! The generic, page-agnostic character dictionary — the floor
//! `Glossary::describe` falls back to once a lesson's own `explain()` and
//! `explain_char()` entries are exhausted.
//!
//! Typst sets maths variables in the Mathematical Alphanumeric Symbols block
//! (italic 𝑥 is U+1D465, not U+0078), so every lookup normalises back to plain
//! ASCII/Greek first and only then consults the dictionary.

/// A character's curated name and generic mathematical meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CharEntry {
    pub name: &'static str,
    pub meaning: &'static str,
}

/// True for characters that should never reach a tooltip or a hit box:
/// variation selectors (Typst picks the script 𝒩 with U+FE00), zero-width
/// and invisible-operator characters, and combining marks. egui does no
/// Unicode shaping, so any of these would draw as a tofu box.
pub(crate) fn is_invisible(ch: char) -> bool {
    matches!(
        ch,
        '\u{FE00}'..='\u{FE0F}'         // variation selectors
            | '\u{200B}'..='\u{200F}'   // zero-width space/joiners, marks
            | '\u{2060}'..='\u{2064}'   // word joiner, invisible operators
            | '\u{0300}'..='\u{036F}'   // combining marks (bar, hat, ...)
    )
}

/// True for characters egui cannot draw in a tooltip.
///
/// egui does no Unicode shaping, and none of the three embedded fonts covers
/// the subscript block, combining marks or the fraction slash, so each of those
/// comes out as a tofu box wherever a tooltip quotes it back.
///
/// `Lesson::audit()` scans a lesson's curated text with this at runtime, so a
/// dictionary or `explain()` entry cannot quietly reintroduce the bug.
pub(crate) fn is_unrenderable(ch: char) -> bool {
    matches!(
        ch,
        '\u{2070}'..='\u{209F}' | '\u{0300}'..='\u{036F}' | '\u{2044}'
    )
}

/// Fold decorative Unicode maths variants back to their base character.
///
/// Handles the whole U+1D400–U+1D7FF block (bold/italic/script/fraktur/
/// double-struck/sans/monospace latin, the five Greek blocks and the digit
/// blocks) plus the letterlike singletons that live outside it because their
/// slots in the block are reserved.
pub(crate) fn normalize(ch: char) -> char {
    // Letterlike Symbols that stand in for holes in the maths block.
    match ch {
        'ℎ' => return 'h', // U+210E, the italic h of Planck's constant
        'ℯ' => return 'e',
        'ℊ' => return 'g',
        'ℬ' | 'ℭ' => return 'B',
        'ℰ' => return 'E',
        'ℱ' => return 'F',
        'ℋ' | 'ℌ' | 'ℍ' => return 'H',
        'ℐ' | 'ℑ' => return 'I',
        'ℒ' => return 'L',
        'ℳ' => return 'M',
        'ℛ' | 'ℜ' | 'ℝ' => return 'R',
        'ℨ' | 'ℤ' => return 'Z',
        'ℂ' => return 'C',
        'ℕ' => return 'N',
        'ℙ' => return 'P',
        'ℚ' => return 'Q',
        // Greek variant shapes, folded onto the plain letter.
        'ϵ' => return 'ε',
        'ϑ' => return 'θ',
        'ϰ' => return 'κ',
        'ϕ' => return 'φ',
        'ϱ' => return 'ρ',
        'ϖ' => return 'π',
        'ϴ' => return 'Θ',
        _ => {}
    }

    let code = ch as u32;
    if !(0x1D400..=0x1D7FF).contains(&code) {
        return ch;
    }

    // Latin blocks: 26 capitals then 26 smalls, in that order.
    const LATIN_BLOCKS: [u32; 13] = [
        0x1D400, // bold
        0x1D434, // italic
        0x1D468, // bold italic
        0x1D49C, // script
        0x1D4D0, // bold script
        0x1D504, // fraktur
        0x1D538, // double-struck
        0x1D56C, // bold fraktur
        0x1D5A0, // sans-serif
        0x1D5D4, // sans-serif bold
        0x1D608, // sans-serif italic
        0x1D63C, // sans-serif bold italic
        0x1D670, // monospace
    ];
    for start in LATIN_BLOCKS {
        if (start..start + 52).contains(&code) {
            let offset = code - start;
            let base = if offset < 26 { b'A' } else { b'a' };
            return char::from(base + (offset % 26) as u8);
        }
    }

    // Greek blocks: 58 slots each, laid out capitals, nabla, smalls, partial,
    // then six variant shapes.
    const GREEK_BLOCKS: [u32; 5] = [
        0x1D6A8, // bold
        0x1D6E2, // italic
        0x1D71C, // bold italic
        0x1D756, // sans-serif bold
        0x1D790, // sans-serif bold italic
    ];
    for start in GREEK_BLOCKS {
        if (start..start + 58).contains(&code) {
            let base = match code - start {
                offset @ 0..=16 => 0x391 + offset,  // Alpha..Rho
                17 => 0x398,                        // capital theta symbol
                offset @ 18..=24 => 0x3A3 + offset - 18, // Sigma..Omega
                25 => 0x2207,                       // nabla
                offset @ 26..=50 => 0x3B1 + offset - 26, // alpha..omega
                51 => 0x2202,                       // partial differential
                52 => 0x3B5,                        // epsilon variant
                53 => 0x3B8,                        // theta variant
                54 => 0x3BA,                        // kappa variant
                55 => 0x3C6,                        // phi variant
                56 => 0x3C1,                        // rho variant
                _ => 0x3C0,                         // pi variant
            };
            return char::from_u32(base).unwrap_or(ch);
        }
    }

    // Digit blocks: bold, double-struck, sans, sans bold, monospace.
    for start in [0x1D7CE, 0x1D7D8, 0x1D7E2, 0x1D7EC, 0x1D7F6] {
        if (start..start + 10).contains(&code) {
            return char::from(b'0' + (code - start) as u8);
        }
    }

    ch
}

/// Normalise every character of a glyph's text.
pub(crate) fn normalize_str(text: &str) -> String {
    text.chars().map(normalize).collect()
}

/// Map a Typst math identifier back to the character it draws.
///
/// mitex spells a Greek letter as a name rather than the character itself —
/// `\varepsilon` becomes the identifier `epsilon`, `\beta` becomes `beta` —
/// so [`crate::emphasis::bolden`] needs this to resolve an identifier to the
/// character a declared role is keyed by. A single-letter identifier is
/// already the character it names; anything else is looked up by its stem,
/// the part before any `.` variant modifier (`theta.alt` is still theta),
/// which lands a LaTeX letter and its `\var…` cousin on the one character
/// [`normalize`] already folds both of their glyphs onto.
pub(crate) fn ident_char(ident: &str) -> Option<char> {
    let mut chars = ident.chars();
    let first = chars.next()?;
    if chars.next().is_none() {
        return Some(first);
    }

    let stem = ident.split('.').next().unwrap_or(ident);
    Some(match stem {
        "alpha" => 'α',
        "beta" => 'β',
        "gamma" => 'γ',
        "delta" => 'δ',
        "epsilon" => 'ε',
        "zeta" => 'ζ',
        "eta" => 'η',
        "theta" => 'θ',
        "iota" => 'ι',
        "kappa" => 'κ',
        "lambda" => 'λ',
        "mu" => 'μ',
        "nu" => 'ν',
        "xi" => 'ξ',
        "pi" => 'π',
        "rho" => 'ρ',
        "sigma" => 'σ',
        "tau" => 'τ',
        "upsilon" => 'υ',
        "phi" => 'φ',
        "chi" => 'χ',
        "psi" => 'ψ',
        "omega" => 'ω',
        "Gamma" => 'Γ',
        "Delta" => 'Δ',
        "Theta" => 'Θ',
        "Lambda" => 'Λ',
        "Xi" => 'Ξ',
        "Pi" => 'Π',
        "Sigma" => 'Σ',
        "Upsilon" => 'Υ',
        "Phi" => 'Φ',
        "Psi" => 'Ψ',
        "Omega" => 'Ω',
        "ell" => 'ℓ',
        "planck" => 'ℏ',
        "aleph" => 'ℵ',
        "nabla" => '∇',
        "diff" => '∂',
        "oo" => '∞',
        _ => return None,
    })
}

/// The inverse of [`ident_char`]'s Greek and letterlike table: the canonical
/// mitex identifier a character was named by.
///
/// [`crate::emphasis::strip_bold`] needs this because bolding writes the
/// literal character rather than the name — `bold(ε)`, not
/// `bold(epsilon)` — to work around a Typst span-tracking gap (see
/// [`crate::emphasis::bolden`]'s doc comment). Undoing that has to restore
/// the name, or a compiled term's key would read `ε` while a lesson's
/// `.explain(r"\varepsilon", ...)` entry — never bolded, so never rewritten
/// — still keys as `epsilon`, and the two would silently stop matching.
/// `\epsilon` and `\varepsilon` both draw glyphs [`normalize`] folds onto
/// `ε`, but only one spelling can be canonical here: the one this crate's
/// lessons write, `\varepsilon`.
pub(crate) fn char_ident(ch: char) -> Option<&'static str> {
    Some(match ch {
        'α' => "alpha",
        'β' => "beta",
        'γ' => "gamma",
        'δ' => "delta",
        'ε' => "epsilon",
        'ζ' => "zeta",
        'η' => "eta",
        'θ' => "theta",
        'ι' => "iota",
        'κ' => "kappa",
        'λ' => "lambda",
        'μ' => "mu",
        'ν' => "nu",
        'ξ' => "xi",
        'π' => "pi",
        'ρ' => "rho",
        'σ' => "sigma",
        'τ' => "tau",
        'υ' => "upsilon",
        'φ' => "phi",
        'χ' => "chi",
        'ψ' => "psi",
        'ω' => "omega",
        'Γ' => "Gamma",
        'Δ' => "Delta",
        'Θ' => "Theta",
        'Λ' => "Lambda",
        'Ξ' => "Xi",
        'Π' => "Pi",
        'Σ' => "Sigma",
        'Υ' => "Upsilon",
        'Φ' => "Phi",
        'Ψ' => "Psi",
        'Ω' => "Omega",
        'ℓ' => "ell",
        'ℏ' => "planck",
        'ℵ' => "aleph",
        '∇' => "nabla",
        '∂' => "diff",
        '∞' => "oo",
        _ => return None,
    })
}

/// Look up the curated entry for a character, once normalised.
///
/// This is the crate's built-in, page-agnostic dictionary: the final
/// fallback `Glossary::describe` consults after a lesson's own `explain()`
/// and `explain_char()` entries. Meanings here are deliberately generic —
/// this module has no idea what lesson, if any, is asking, so nothing here
/// may talk about a particular model.
pub(crate) fn char_entry(ch: char) -> Option<CharEntry> {
    let (name, meaning) = match normalize(ch) {
        // ---- Greek ----------------------------------------------------
        'σ' => (
            "Greek small sigma",
            "Standard deviation. Often carries a subscript for a value that varies over time, e.g. σ_t; its square σ² is then the corresponding variance.",
        ),
        'ω' => (
            "Greek small omega",
            "Commonly a baseline or constant term in a model equation, often required to stay positive.",
        ),
        'α' => (
            "Greek small alpha",
            "Commonly a model coefficient — often a weight on how strongly one term reacts to another.",
        ),
        'β' => (
            "Greek small beta",
            "Commonly a model coefficient — often a weight on how much of a previous value carries forward.",
        ),
        'ε' => (
            "Greek small epsilon",
            "An error, shock or residual: the part of an observed value a model's other terms do not explain.",
        ),
        'μ' => (
            "Greek small mu",
            "A mean or expected value — often the baseline a deviation is measured against.",
        ),
        'θ' => (
            "Greek small theta",
            "A parameter, or a vector of parameters — commonly what an estimation procedure such as maximum likelihood searches over.",
        ),
        'π' => (
            "Greek small pi",
            "The circle constant, 3.14159…. It appears as ln(2π) in the normal log-density.",
        ),
        'λ' => ("Greek small lambda", "A rate or weight parameter."),
        'γ' => (
            "Greek small gamma",
            "Commonly a generic coefficient; in some models, an asymmetry or leverage parameter that upweights one class of outcomes over another.",
        ),
        'δ' => ("Greek small delta", "A small change, or a difference."),
        'ρ' => ("Greek small rho", "A correlation coefficient."),
        'τ' => ("Greek small tau", "A second time index, used when t is taken."),
        'ν' => ("Greek small nu", "Degrees of freedom, e.g. for a Student-t distribution."),
        'φ' => ("Greek small phi", "An autoregressive coefficient."),
        'η' => ("Greek small eta", "An innovation or noise term."),
        'κ' => ("Greek small kappa", "Kurtosis, or a mean-reversion speed."),
        'χ' => ("Greek small chi", "Used for the chi-squared distribution, e.g. in hypothesis testing."),
        'ψ' => ("Greek small psi", "An infinite-order moving-average weight."),
        'ξ' => ("Greek small xi", "A generic random variable."),
        'ζ' => ("Greek small zeta", "A generic parameter."),
        'ι' => ("Greek small iota", "A vector of ones."),
        'Σ' => (
            "Greek capital sigma",
            "A covariance matrix, or (as ∑) a summation over a set of terms.",
        ),
        'Ω' => ("Greek capital omega", "The information set — everything known at a point in time."),
        'Θ' => ("Greek capital theta", "The parameter space that θ is searched over."),
        'Δ' => ("Greek capital delta", "A change from one period to the next."),
        'Γ' => ("Greek capital gamma", "The gamma function, or an autocovariance matrix."),
        'Λ' => ("Greek capital lambda", "A matrix of eigenvalues."),
        'Φ' => ("Greek capital phi", "The standard normal cumulative distribution function."),
        'Π' => ("Greek capital pi", "A product over terms."),

        // ---- Latin letters ----------------------------------------------
        'r' => (
            "Latin small r",
            "Commonly used for a return or rate; r_t denotes its value at time t in an observed sequence.",
        ),
        't' => ("Latin small t", "The time index — one observation in a sequence, such as a day or period."),
        'T' => ("Latin capital T", "The sample size: how many observations a data set contains."),
        'h' => ("Latin small h", "The forecast horizon, measured in steps ahead of today."),
        'z' => (
            "Latin small z",
            "Commonly a standardised variable, z = ε / σ — a raw value with its scale divided out — often assumed to follow a standard normal distribution.",
        ),
        'E' => (
            "Latin capital E",
            "Expectation — the average or best forecast of a random variable given the information available.",
        ),
        'N' => (
            "Latin capital N",
            "The normal (Gaussian) distribution. N(0,1) is the standard normal: mean 0, variance 1.",
        ),
        'ℓ' => (
            "Script small l",
            "The log-likelihood: the objective a numerical optimiser maximises to fit a model's parameters.",
        ),
        'L' => ("Latin capital L", "The likelihood, before logs are taken."),
        'e' => ("Latin small e", "Euler's number, 2.71828…, the base of the natural logarithm."),
        'i' => ("Latin small i", "A generic index."),
        'j' => ("Latin small j", "A second generic index."),
        'k' => ("Latin small k", "A lag length, or a count of parameters."),
        'n' => ("Latin small n", "A count of observations."),
        'p' => (
            "Latin small p",
            "Commonly a model order — how many terms of one kind (e.g. autoregressive lags) enter a recursive definition.",
        ),
        'q' => (
            "Latin small q",
            "Commonly a model order — how many terms of another kind (e.g. moving-average lags) enter a recursive definition.",
        ),
        's' => ("Latin small s", "A standard deviation, or a second time index."),
        'x' => ("Latin small x", "A generic variable, often used for a horizontal-axis quantity."),
        'y' => ("Latin small y", "A generic variable, often used for a vertical-axis quantity."),
        'a' | 'b' | 'c' | 'd' | 'f' | 'g' | 'm' | 'u' | 'v' | 'w' => {
            ("Latin small letter", "A variable or coefficient.")
        }

        // ---- Operators and relations -----------------------------------
        '∑' => (
            "N-ary summation",
            "Add up every term as the index below runs to the limit above.",
        ),
        '∏' => ("N-ary product", "Multiply every term over the index range."),
        '=' => ("Equals sign", "The two sides are the same quantity."),
        '+' => ("Plus sign", "Addition."),
        '−' | '-' => ("Minus sign", "Subtraction, or a negative quantity."),
        '±' => ("Plus-minus sign", "Both the positive and the negative case."),
        '×' => ("Multiplication sign", "Multiplication."),
        '⋅' | '·' => ("Dot operator", "Multiplication."),
        '/' => ("Solidus", "Division."),
        '<' => (
            "Less-than sign",
            "Strictly smaller. Often used to state a boundary condition, such as a stability or stationarity constraint.",
        ),
        '>' => ("Greater-than sign", "Strictly larger. Often used to state a positivity or other boundary constraint on a parameter."),
        '≤' => ("Less-than or equal to", "Smaller than, or the same as."),
        '≥' => (
            "Greater-than or equal to",
            "At least as large. Often used to state a non-negativity constraint an optimiser must respect.",
        ),
        '≠' => ("Not equal to", "The two sides differ."),
        '≈' => ("Almost equal to", "Approximately the same."),
        '∼' | '~' => (
            "Tilde operator",
            "\"is distributed as\". x ∼ N(0,1) says the quantity follows a standard normal distribution.",
        ),
        '∈' => ("Element of", "Belongs to the set that follows."),
        '∞' => ("Infinity", "Grows without bound."),
        '∂' => ("Partial differential", "The derivative with respect to one variable, holding the rest fixed."),
        '∇' => ("Nabla", "The gradient: the vector of partial derivatives."),
        '√' => ("Square root", "The positive number that squares to what follows."),
        '∫' => ("Integral", "The area under the function that follows."),

        // ---- Punctuation and decoration --------------------------------
        '(' => ("Left parenthesis", "Opens a group; everything inside is evaluated first."),
        ')' => ("Right parenthesis", "Closes a group."),
        '[' => ("Left square bracket", "Opens a group."),
        ']' => ("Right square bracket", "Closes a group."),
        '{' => ("Left brace", "Opens a set or a group."),
        '}' => ("Right brace", "Closes a set or a group."),
        '|' => ("Vertical bar", "\"given\" in a conditional, or an absolute value."),
        ',' => ("Comma", "Separates items in a list or a tuple."),
        '.' => ("Full stop", "A decimal point, or the end of the sentence."),
        ':' => ("Colon", "Introduces a definition."),
        ';' => ("Semicolon", "Separates parameters from data."),
        '!' => ("Exclamation mark", "The factorial of the number before it."),
        '¯' | '‾' | '\u{0304}' | '\u{0305}' => (
            "Macron (overbar)",
            "A long-run or sample average — the bar marks a mean taken over time or over a sample.",
        ),
        '^' => ("Circumflex", "An estimate of the quantity underneath."),
        '\'' | '′' => ("Prime", "A transpose, or a derivative."),

        // ---- Digits -----------------------------------------------------
        '0' => ("Digit zero", "Zero — commonly the mean of a standard normal distribution, or an additive identity."),
        '1' => (
            "Digit one",
            "One. Commonly a stability or stationarity boundary for a sum of coefficients, and the variance of the standard normal.",
        ),
        '2' => (
            "Digit two",
            "Two. As a superscript it commonly means squared, e.g. a variance written as σ² or a squared residual ε².",
        ),
        '3' => ("Digit three", "Three."),
        '4' => ("Digit four", "Four."),
        '5' => ("Digit five", "Five."),
        '6' => ("Digit six", "Six."),
        '7' => ("Digit seven", "Seven."),
        '8' => ("Digit eight", "Eight."),
        '9' => ("Digit nine", "Nine."),

        _ => return None,
    };
    Some(CharEntry { name, meaning })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_maths_alphanumerics() {
        assert_eq!(normalize('\u{1D465}'), 'x'); // italic x
        assert_eq!(normalize('\u{1D6FC}'), 'α'); // italic alpha
        assert_eq!(normalize('\u{1D70E}'), 'σ'); // italic sigma
        assert_eq!(normalize('\u{1D7D0}'), '2'); // bold digit two
        assert_eq!(normalize('ℎ'), 'h');
        assert_eq!(normalize('ℓ'), 'ℓ'); // its own symbol, left alone
    }

    #[test]
    fn known_characters_have_curated_entries() {
        let entry = char_entry('σ').expect("sigma is curated");
        assert_eq!(entry.name, "Greek small sigma");

        // Normalisation runs first, so the italic maths-alphanumeric variant
        // finds the same entry as the plain letter.
        assert_eq!(char_entry('\u{1D70E}'), char_entry('σ'));
    }

    #[test]
    fn uncurated_characters_have_no_entry() {
        assert!(char_entry('\u{2A0C}').is_none());
    }

    /// Every Greek command this lesson engine is likely to meet, compiled for
    /// real and checked against what `ident_char` says the same mitex
    /// identifier means — so the table above cannot silently drift from what
    /// mitex actually emits.
    #[test]
    fn ident_char_matches_what_normalize_folds_the_compiled_glyph_to() {
        for latex in [
            r"\alpha", r"\beta", r"\gamma", r"\delta", r"\epsilon", r"\varepsilon", r"\zeta",
            r"\eta", r"\theta", r"\vartheta", r"\iota", r"\kappa", r"\lambda", r"\mu", r"\nu",
            r"\xi", r"\pi", r"\rho", r"\sigma", r"\tau", r"\upsilon", r"\phi", r"\varphi",
            r"\chi", r"\psi", r"\omega", r"\Gamma", r"\Delta", r"\Theta", r"\Lambda", r"\Xi",
            r"\Pi", r"\Sigma", r"\Upsilon", r"\Phi", r"\Psi", r"\Omega",
        ] {
            let typst = crate::formula::to_typst_math(latex).expect("mitex");
            let rendered = crate::formula::compile(latex, false, &crate::glossary::Glossary::default())
                .expect("compile");
            let glyphs: Vec<&str> = rendered
                .terms
                .iter()
                .flat_map(|term| &term.glyphs)
                .map(|glyph| glyph.text.as_str())
                .collect();
            let [glyph] = glyphs[..] else {
                panic!("{latex} drew {glyphs:?}, expected exactly one glyph");
            };
            let drawn = glyph.chars().next().expect("a character");

            let resolved =
                ident_char(&typst).unwrap_or_else(|| panic!("no mapping for {typst:?} ({latex})"));
            assert_eq!(normalize(drawn), resolved, "{latex} compiled to typst {typst:?}, drew {drawn:?}");
        }
    }

    /// The tooltip text is drawn by egui, which does no Unicode shaping and
    /// has no font covering subscript letters, combining marks or the
    /// fraction slash — they all come out as tofu boxes (the epsilon-sub-t bug).
    /// Scan this source file so no dictionary entry can reintroduce one.
    /// (Keys written as escapes, like "\u{0304}", are exempt by construction:
    /// the scan sees source bytes, and keys are never rendered anyway.)
    #[test]
    fn dictionary_avoids_unrenderable_characters() {
        let source = include_str!("symbols.rs");
        for (index, line) in source.lines().enumerate() {
            for ch in line.chars() {
                assert!(
                    !is_unrenderable(ch),
                    "unrenderable {ch:?} on line {}: {line}",
                    index + 1
                );
            }
        }
    }
}
