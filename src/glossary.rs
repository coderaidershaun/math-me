//! What a lesson has to say about the terms and characters in its formulas.
//!
//! A lesson's own `explain()`/`explain_char()` entries come first; anything
//! they don't cover falls through the library's built-in [`crate::symbols`]
//! dictionary; anything that misses that too gets a description built from
//! its parts, so a reader is never told nothing.
//! [`crate::lesson::Lesson::audit`] watches for the two points where that
//! chain hits its floor.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::symbols;

/// The `name` a multi-atom term with no curated entry receives.
///
/// [`crate::lesson::Lesson::audit`] treats this as the signal that nothing —
/// neither the lesson's `explain()` nor the built-in dictionary — had
/// anything to say about the term as a whole.
pub(crate) const UNCURATED_TERM_NAME: &str = "Term";

/// The `meaning` a character nothing recognises receives.
///
/// [`crate::lesson::Lesson::audit`] treats this as the signal that a single
/// glyph fell all the way through the fallback chain.
pub(crate) const UNKNOWN_CHAR_MEANING: &str = "Not in the symbol dictionary yet.";

/// A symbol or term as it should be explained to a reader.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Description {
    /// The characters exactly as they were drawn on the page.
    pub display: String,
    /// What the term or character is, independent of any one lesson.
    pub name: String,
    /// What it means in the lesson that is asking.
    pub meaning: String,
}

/// A curated name and meaning, keyed either by a term's normalised Typst
/// source or by a single normalised character.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Entry {
    name: String,
    meaning: String,
}

/// What kind of linear-algebra object a character names; both kinds draw the
/// same sky blue accent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Role {
    Vector,
    Matrix,
}

/// What a lesson has taught about its own formulas.
///
/// Built by [`crate::lesson::LessonBuilder::explain`] and
/// [`crate::lesson::LessonBuilder::explain_char`] as a lesson is authored;
/// consulted by [`crate::terms::Term::describe`] while rendering, and by
/// [`crate::lesson::Lesson::audit`] to find what neither the lesson nor the
/// library's built-in dictionary can explain.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Glossary {
    terms: HashMap<String, Entry>,
    chars: HashMap<char, Entry>,
    /// `serde(default)` so lessons saved before roles existed still load.
    #[serde(default)]
    roles: HashMap<char, Role>,
}

impl Glossary {
    /// Record what a term means, keyed by its normalised Typst source.
    pub(crate) fn insert_term(&mut self, key: String, name: String, meaning: String) {
        self.terms.insert(key, Entry { name, meaning });
    }

    /// Record what a single character means, keyed by its normalised form so
    /// it matches whichever decorative Unicode variant Typst actually draws.
    pub(crate) fn insert_char(&mut self, ch: char, name: String, meaning: String) {
        self.chars.insert(symbols::normalize(ch), Entry { name, meaning });
    }

    pub(crate) fn insert_role(&mut self, ch: char, role: Role) {
        self.roles.insert(symbols::normalize(ch), role);
    }

    pub(crate) fn role(&self, ch: char) -> Option<Role> {
        self.roles.get(&symbols::normalize(ch)).copied()
    }

    /// Explain a term: the atoms it is made of, and the key
    /// [`crate::terms::key`] built from its normalised Typst source.
    ///
    /// Lookup chain: a curated term entry, then — for a term of one atom —
    /// the lesson's own character entry, then the built-in dictionary, then a
    /// description built from the term's parts. Nothing is ever left unsaid.
    pub(crate) fn describe(&self, key: &str, atoms: &[&str]) -> Description {
        if let Some(entry) = self.terms.get(key) {
            return Description {
                display: atoms.concat(),
                name: entry.name.clone(),
                meaning: entry.meaning.clone(),
            };
        }

        if let [atom] = atoms {
            return self.describe_char(atom);
        }

        let mut parts: Vec<String> = Vec::new();
        for atom in atoms {
            let name = self.describe_char(atom).name;
            if !parts.contains(&name) {
                parts.push(name);
            }
        }
        Description {
            display: key.to_owned(),
            name: UNCURATED_TERM_NAME.to_owned(),
            meaning: format!("A term built from {}.", list(&parts)),
        }
    }

    /// Explain one glyph's text, falling back gracefully on anything unlisted.
    fn describe_char(&self, raw: &str) -> Description {
        let normalized = symbols::normalize_str(raw);
        let display = raw.to_owned();
        let single = single_char(&normalized);

        if let Some(ch) = single
            && let Some(entry) = self.chars.get(&ch)
        {
            return Description {
                display,
                name: entry.name.clone(),
                meaning: entry.meaning.clone(),
            };
        }

        if let Some(ch) = single
            && let Some(entry) = symbols::char_entry(ch)
        {
            return Description {
                display,
                name: entry.name.to_owned(),
                meaning: entry.meaning.to_owned(),
            };
        }

        // Nothing curated: say what the character is, and admit the rest.
        let (name, meaning) = match normalized.chars().next() {
            Some(ch) if ch.is_ascii_uppercase() => {
                (format!("Latin capital {ch}"), "A variable or constant.".to_owned())
            }
            Some(ch) if ch.is_ascii_lowercase() => {
                (format!("Latin small {ch}"), "A variable or coefficient.".to_owned())
            }
            Some(ch) if ('\u{0370}'..='\u{03FF}').contains(&ch) => {
                (format!("Greek letter {ch}"), "A parameter.".to_owned())
            }
            Some(ch) => (format!("U+{:04X}", ch as u32), UNKNOWN_CHAR_MEANING.to_owned()),
            None => ("Unknown".to_owned(), "Nothing to say about this one.".to_owned()),
        };

        Description { display, name, meaning }
    }
}

/// `text`'s one character, if it has exactly one.
fn single_char(text: &str) -> Option<char> {
    let mut chars = text.chars();
    let first = chars.next()?;
    chars.next().is_none().then_some(first)
}

/// Join names the way a sentence would: "a, b and c".
fn list(parts: &[String]) -> String {
    match parts {
        [] => "nothing".to_owned(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curated_term_reports_its_curated_name_and_meaning() {
        let mut glossary = Glossary::default();
        glossary.insert_term("alpha epsilon_(t-1)^(2)".to_owned(), "The ARCH term".to_owned(), "the reaction".to_owned());

        let description = glossary.describe("alpha epsilon_(t-1)^(2)", &["α", "ε"]);

        assert_eq!(description.name, "The ARCH term");
        assert_eq!(description.meaning, "the reaction");
        assert_eq!(description.display, "αε");
    }

    #[test]
    fn single_atom_prefers_the_lessons_own_char_entry() {
        let mut glossary = Glossary::default();
        glossary.insert_char('中', "Custom name".to_owned(), "Custom meaning".to_owned());

        let description = glossary.describe("unrelated-key", &["中"]);

        assert_eq!(description.name, "Custom name");
        assert_eq!(description.meaning, "Custom meaning");
    }

    #[test]
    fn unrecognised_single_atom_falls_to_the_generic_floor() {
        let glossary = Glossary::default();

        // 中 is outside every range the built-in dictionary recognises, so
        // this exercises the true floor regardless of what that dictionary
        // curates.
        let description = glossary.describe("k", &["中"]);

        assert_eq!(description.name, "U+4E2D");
        assert_eq!(description.meaning, UNKNOWN_CHAR_MEANING);
    }

    #[test]
    fn ascii_letters_outside_the_built_in_dictionary_still_get_a_generic_name() {
        let glossary = Glossary::default();

        // Neither 'o' nor 'Q' is curated in symbols::char_entry — see the
        // ported POC dictionary, which has no catch-all for either.
        assert_eq!(glossary.describe("o", &["o"]).name, "Latin small o");
        assert_eq!(glossary.describe("Q", &["Q"]).name, "Latin capital Q");
    }

    #[test]
    fn uncurated_multi_atom_term_names_its_parts() {
        let glossary = Glossary::default();

        let description = glossary.describe("term-key", &["中", "文"]);

        assert_eq!(description.name, UNCURATED_TERM_NAME);
        assert_eq!(description.meaning, "A term built from U+4E2D and U+6587.");
        assert_eq!(description.display, "term-key");
    }
}
