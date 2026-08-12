//! Grouping a formula's glyphs into the terms a reader would name.
//!
//! Nobody reads an equation one character at a time. `ℓ(θ)` is a single thing —
//! the log-likelihood as a function of the parameters — and being told what a
//! left parenthesis is helps no one. So the hoverable unit is the term, not the
//! glyph.
//!
//! Typst already knows the structure: the same compile that lays out the page
//! parses the source into a syntax tree that knows where a fraction, an
//! attachment or a bracketed group begins and ends, and every glyph it lays out
//! carries the span of the source that produced it. Matching the two up gives
//! real terms for any formula rather than a lookup table of the ones on this
//! page. What the tree cannot give is bracket nesting — mitex writes every
//! parenthesis as a Typst *escape* (`\(`), so `(a + b)` arrives as five loose
//! siblings — hence the delimiter matching below.

use std::ops::Range;

use eframe::egui;
use typst::syntax::{Source, Span, SyntaxKind, SyntaxNode};

use crate::glossary::{Description, Glossary};
use crate::symbols;

/// One glyph as Typst laid it out, before it is assigned to a term.
pub struct RawGlyph {
    pub rect_pt: egui::Rect,
    pub text: String,
    pub span: Span,
}

/// One glyph of a term, in page-point coordinates.
pub struct GlyphBox {
    pub rect_pt: egui::Rect,
    pub text: String,
}

/// One hoverable unit: a whole term, or a lone operator between two terms.
pub struct Term {
    pub glyphs: Vec<GlyphBox>,
    /// The union of the glyph rectangles: the hit box, and what
    /// `MATH_ME_DEBUG_BOXES` outlines.
    pub rect_pt: egui::Rect,
    /// Where to clip the green repaint. Usually one rectangle — the union,
    /// which also catches ink no glyph owns, such as a fraction bar or an
    /// overline — but the glyph rectangles one by one where a union would
    /// reach over a glyph belonging to another term.
    pub highlight_pt: Vec<egui::Rect>,
    /// The term's normalised Typst source: the dictionary key.
    pub key: String,
}

impl Term {
    /// What to tell the reader about this term.
    pub(crate) fn describe(&self, glossary: &Glossary) -> Description {
        let atoms: Vec<&str> = self.glyphs.iter().map(|glyph| glyph.text.as_str()).collect();
        glossary.describe(&self.key, &atoms)
    }
}

/// Characters that end one term and begin the next. Each is a hoverable unit of
/// its own: `=` says something about the equation that neither side does.
const SEPARATORS: &str = "=+−±<>≤≥≠≈∼∈∝≡,;×⋅·";
/// Separators that can also be a sign rather than a split, when nothing
/// precedes them.
const SIGNS: &str = "+−±";
/// Operators whose limits and body are read as a unit of their own, and which
/// therefore stand between the terms around them rather than inside one.
const LARGE_OPERATORS: &str = "∑∏∫∮⋃⋂⨆⨁⨂";
const OPENING: &str = "([{";
const CLOSING: &str = ")]}";

/// Split a compiled formula into terms.
///
/// `source` must be the very source Typst compiled, so that the glyph spans
/// resolve against it.
pub fn partition(source: &Source, raw: Vec<RawGlyph>) -> Vec<Term> {
    let glyphs = resolve(source, raw);
    let ranges = match equation_body(source) {
        Some(body) => {
            let mut ranges = Vec::new();
            split(&children(body.node, body.range.start), &glyphs, &mut ranges);
            ranges
        }
        // No equation to read structure from: every glyph stands alone, which
        // is the behaviour this page had before terms existed.
        None => glyphs.iter().map(|glyph| glyph.range.clone()).collect(),
    };

    let mut terms = Vec::new();
    for (index, range) in ranges.iter().enumerate() {
        // A term owns every glyph from its own start up to the next term's,
        // so the whitespace between them cannot strand a glyph.
        let end = ranges.get(index + 1).map_or(usize::MAX, |next| next.start);
        let mine: Vec<&Resolved> = glyphs
            .iter()
            .filter(|glyph| (range.start..end).contains(&glyph.range.start))
            .collect();
        if mine.is_empty() {
            continue;
        }
        let rect_pt = mine
            .iter()
            .map(|glyph| glyph.rect_pt)
            .reduce(|a, b| a.union(b))
            .unwrap_or(egui::Rect::NOTHING);
        terms.push(Term {
            glyphs: mine
                .iter()
                .map(|glyph| GlyphBox {
                    rect_pt: glyph.rect_pt,
                    text: glyph.text.clone(),
                })
                .collect(),
            rect_pt,
            highlight_pt: Vec::new(),
            key: key(source.text().get(range.start..range.end.min(end)).unwrap_or_default()),
        });
    }

    debug_assert_eq!(
        terms.iter().map(|term| term.glyphs.len()).sum::<usize>(),
        glyphs.len(),
        "terms must tile the formula: every glyph in exactly one of them"
    );

    plan_highlights(&mut terms);
    terms
}

/// How far a glyph of another term may reach into a union before the union is
/// abandoned, in page points. Ink stays inside its glyph's rectangle, and
/// neighbouring rectangles tile edge to edge, so only a real overlap counts —
/// a shared edge is not a bleed.
const BLEED_PT: f32 = 0.5;

/// Decide, once per formula, how each term's highlight is clipped.
fn plan_highlights(terms: &mut [Term]) {
    let all: Vec<(usize, egui::Rect)> = terms
        .iter()
        .enumerate()
        .flat_map(|(index, term)| term.glyphs.iter().map(move |glyph| (index, glyph.rect_pt)))
        .collect();

    for (index, term) in terms.iter_mut().enumerate() {
        let union = term.rect_pt;
        let clean = all.iter().all(|&(owner, rect)| {
            if owner == index {
                return true;
            }
            let overlap = union.intersect(rect);
            overlap.width() <= BLEED_PT || overlap.height() <= BLEED_PT
        });
        term.highlight_pt = if clean {
            vec![union]
        } else {
            term.glyphs.iter().map(|glyph| glyph.rect_pt).collect()
        };
    }
}

/// A glyph with its span resolved to a byte range in the source.
struct Resolved {
    rect_pt: egui::Rect,
    text: String,
    range: Range<usize>,
    /// True when the range was borrowed from a neighbour rather than being the
    /// glyph's own. Good enough to put the glyph in the right term, but not to
    /// read structure from: the `l` and `n` of `ln(2π)` borrow the range of the
    /// `(`, and a bracket that appears to draw three characters is no longer
    /// recognisable as a bracket.
    borrowed: bool,
}

/// Resolve every glyph's span, in the order Typst laid the glyphs out.
///
/// Content the library generates rather than the source does — the letters of
/// an operator name such as `ln`, and the base of an accent — arrives with a
/// span that points nowhere. Such a glyph takes the range of the next glyph
/// that has one, which lands it in the right term because the missing cases are
/// all inside a term: an operator name is followed by the operand it applies
/// to, and an accent's base by the rest of the accented expression.
fn resolve(source: &Source, raw: Vec<RawGlyph>) -> Vec<Resolved> {
    let own: Vec<Option<Range<usize>>> = raw
        .iter()
        .map(|glyph| source.find(glyph.span).map(|node| node.range()))
        .collect();
    let mut ranges = own.clone();

    let mut next: Option<Range<usize>> = None;
    for range in ranges.iter_mut().rev() {
        match range {
            Some(known) => next = Some(known.clone()),
            None => range.clone_from(&next),
        }
    }
    let mut previous: Option<Range<usize>> = None;
    for range in ranges.iter_mut() {
        match range {
            Some(known) => previous = Some(known.clone()),
            None => range.clone_from(&previous),
        }
    }

    raw.into_iter()
        .zip(own)
        .zip(ranges)
        .map(|((glyph, own), range)| Resolved {
            rect_pt: glyph.rect_pt,
            text: glyph.text,
            range: range.unwrap_or(0..0),
            borrowed: own.is_none(),
        })
        .collect()
}

/// A syntax node with its byte range in the source.
#[derive(Clone)]
struct Node<'a> {
    node: &'a SyntaxNode,
    range: Range<usize>,
}

/// The `Math` node inside the document's one equation.
fn equation_body(source: &Source) -> Option<Node<'_>> {
    let equation = children(source.root(), 0)
        .into_iter()
        .find(|node| node.node.kind() == SyntaxKind::Equation)?;
    children(equation.node, equation.range.start)
        .into_iter()
        .find(|node| node.node.kind() == SyntaxKind::Math)
}

/// A node's children with their ranges, whitespace dropped.
fn children<'a>(parent: &'a SyntaxNode, start: usize) -> Vec<Node<'a>> {
    let mut out = Vec::new();
    let mut offset = start;
    for node in parent.children() {
        let range = offset..offset + node.len();
        offset = range.end;
        if !matches!(node.kind(), SyntaxKind::Space | SyntaxKind::Linebreak) {
            out.push(Node { node, range });
        }
    }
    out
}

/// What a factor does to the run of factors around it.
enum Kind<'a> {
    /// Part of the term being built: a variable, a fraction, an attachment.
    Atom { ident_like: bool },
    Separator { sign: bool, comma: bool },
    LargeOperator,
    Group {
        open: Range<usize>,
        close: Range<usize>,
        content: Vec<Node<'a>>,
    },
}

struct Factor<'a> {
    range: Range<usize>,
    kind: Kind<'a>,
}

/// Split one level of the equation into the byte ranges of its terms.
///
/// Terms tile the level: every glyph falls in exactly one of the ranges, and a
/// separator is a range of its own rather than being swallowed by a neighbour.
fn split(nodes: &[Node<'_>], glyphs: &[Resolved], out: &mut Vec<Range<usize>>) {
    let mut run: Option<Range<usize>> = None;
    let mut sign: Option<Range<usize>> = None;
    // True at the start of the level and straight after a separator: the two
    // places where a `-` is a sign on what follows rather than a subtraction.
    let mut operand_start = true;
    // A bracket straight after a summation is the body being summed, and the
    // reader wants that read term by term.
    let mut after_large_operator = false;
    // A bracket straight after a name is that function's argument — `ln(a + b)`
    // is one term however much lives inside the brackets.
    let mut applied = false;

    for factor in factors(nodes, glyphs) {
        match factor.kind {
            Kind::Separator { sign: is_sign, comma } => {
                if is_sign && run.is_none() && sign.is_none() && operand_start {
                    sign = Some(factor.range);
                    continue;
                }
                flush(&mut run, &mut sign, out);
                out.push(factor.range);
                operand_start = !comma;
                after_large_operator = false;
                applied = false;
            }
            Kind::LargeOperator => {
                flush(&mut run, &mut sign, out);
                out.push(factor.range);
                operand_start = false;
                after_large_operator = true;
                applied = false;
            }
            Kind::Group { open, close, content } => {
                // A bracket holding an operator of its own is a term in its own
                // right, and stands apart from whatever precedes it.
                if applied || !holds_operator(&content, glyphs) {
                    extend(&mut run, factor.range);
                } else {
                    flush(&mut run, &mut sign, out);
                    if after_large_operator {
                        out.push(open);
                        split(&content, glyphs, out);
                        out.push(close);
                    } else {
                        out.push(factor.range);
                    }
                }
                operand_start = false;
                after_large_operator = false;
                applied = false;
            }
            Kind::Atom { ident_like } => {
                extend(&mut run, factor.range);
                operand_start = false;
                after_large_operator = false;
                applied = ident_like;
            }
        }
    }
    flush(&mut run, &mut sign, out);
}

fn extend(run: &mut Option<Range<usize>>, range: Range<usize>) {
    match run {
        Some(open) => open.end = range.end,
        None => *run = Some(range),
    }
}

fn flush(
    run: &mut Option<Range<usize>>,
    sign: &mut Option<Range<usize>>,
    out: &mut Vec<Range<usize>>,
) {
    match (sign.take(), run.take()) {
        (Some(sign), Some(body)) => out.push(sign.start..body.end),
        (None, Some(body)) => out.push(body),
        (Some(sign), None) => out.push(sign),
        (None, None) => {}
    }
}

/// Classify one level's nodes, pairing up brackets as they are met.
fn factors<'a>(nodes: &[Node<'a>], glyphs: &[Resolved]) -> Vec<Factor<'a>> {
    let nodes = unwrap_stretch(nodes);
    let mut out = Vec::new();
    let mut index = 0;

    while index < nodes.len() {
        let node = &nodes[index];
        if opens(node, glyphs)
            && let Some(close) = closer(&nodes, index, glyphs)
        {
            out.push(Factor {
                range: node.range.start..nodes[close].range.end,
                kind: Kind::Group {
                    open: node.range.clone(),
                    close: nodes[close].range.clone(),
                    content: nodes[index + 1..close].to_vec(),
                },
            });
            index = close + 1;
            continue;
        }

        let mine = covered(&node.range, glyphs);
        let kind = if let [only] = mine[..]
            && let Some(ch) = sole_char(only)
            && SEPARATORS.contains(ch)
        {
            Kind::Separator {
                sign: SIGNS.contains(ch),
                comma: matches!(ch, ',' | ';'),
            }
        } else if mine
            .iter()
            .filter_map(|glyph| sole_char(glyph))
            .any(|ch| LARGE_OPERATORS.contains(ch))
        {
            Kind::LargeOperator
        } else {
            // A name is a name whether the source spells its letters out or,
            // like `ln`, has Typst generate them.
            Kind::Atom {
                ident_like: node.node.kind() == SyntaxKind::MathIdent
                    || (!mine.is_empty()
                        && mine
                            .iter()
                            .all(|glyph| glyph.text.chars().all(char::is_alphabetic))),
            }
        };
        out.push(Factor {
            range: node.range.clone(),
            kind,
        });
        index += 1;
    }
    out
}

/// Replace every `lr(...)` with its contents.
///
/// `lr` is how mitex asks Typst to stretch `\left`/`\right` delimiters. It says
/// nothing about meaning, and leaving it in place would hide the brackets it
/// wraps from the matching below.
fn unwrap_stretch<'a>(nodes: &[Node<'a>]) -> Vec<Node<'a>> {
    let mut out = Vec::new();
    for node in nodes {
        match stretch_body(node) {
            Some(body) => out.extend(children(body.node, body.range.start)),
            None => out.push(node.clone()),
        }
    }
    out
}

fn stretch_body<'a>(node: &Node<'a>) -> Option<Node<'a>> {
    if node.node.kind() != SyntaxKind::MathCall {
        return None;
    }
    let parts = children(node.node, node.range.start);
    let [callee, args] = &parts[..] else {
        return None;
    };
    if callee.node.kind() != SyntaxKind::MathIdent || callee.node.leaf_text() != "lr" {
        return None;
    }
    let mut inner = children(args.node, args.range.start)
        .into_iter()
        .filter(|node| node.node.kind() == SyntaxKind::Math);
    let body = inner.next()?;
    inner.next().is_none().then_some(body)
}

/// The glyphs this piece of source really drew, in source order.
///
/// Glyphs that only borrowed their range are left out: they say nothing about
/// what the source says, and counting them would make a bracket look like a
/// word.
fn covered<'a>(range: &Range<usize>, glyphs: &'a [Resolved]) -> Vec<&'a Resolved> {
    let mut mine: Vec<&Resolved> = glyphs
        .iter()
        .filter(|glyph| !glyph.borrowed && range.contains(&glyph.range.start))
        .collect();
    mine.sort_by_key(|glyph| glyph.range.start);
    mine
}

/// The one character a glyph draws, if it draws exactly one.
fn sole_char(glyph: &Resolved) -> Option<char> {
    let mut chars = symbols::normalize_str(&glyph.text).chars().collect::<Vec<_>>();
    (chars.len() == 1).then(|| chars.pop()).flatten()
}

/// True for a bare `(`, `[` or `{`: a node that is nothing but an opening
/// bracket, and so cannot be a self-contained expression that merely starts
/// with one.
fn opens(node: &Node<'_>, glyphs: &[Resolved]) -> bool {
    node.node.children().len() == 0
        && matches!(covered(&node.range, glyphs)[..], [only]
            if sole_char(only).is_some_and(|ch| OPENING.contains(ch)))
}

/// The node that closes the bracket opened at `from`.
///
/// A closing bracket need not be a node of its own: `(a + b)^2` parses as the
/// loose `(`, `a`, `+`, `b` and then an attachment whose *base* is the `)`, and
/// the exponent belongs to the bracketed group, so the whole attachment closes
/// it.
fn closer(nodes: &[Node<'_>], from: usize, glyphs: &[Resolved]) -> Option<usize> {
    let mut depth = 0usize;
    for (index, node) in nodes.iter().enumerate().skip(from) {
        let Some(ch) = leading_char(node, glyphs) else {
            continue;
        };
        if OPENING.contains(ch) {
            depth += 1;
        } else if CLOSING.contains(ch) {
            depth -= 1;
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

/// The first character the node's leftmost leaf draws.
fn leading_char(node: &Node<'_>, glyphs: &[Resolved]) -> Option<char> {
    let mut leaf = node.clone();
    while let Some(first) = children(leaf.node, leaf.range.start).into_iter().next() {
        leaf = first;
    }
    match covered(&leaf.range, glyphs)[..] {
        [only] => sole_char(only),
        _ => None,
    }
}

/// True if this level carries an operator of its own — a comma does not count,
/// since a comma makes a list rather than an expression.
fn holds_operator(nodes: &[Node<'_>], glyphs: &[Resolved]) -> bool {
    factors(nodes, glyphs).iter().any(|factor| {
        matches!(
            factor.kind,
            Kind::Separator { comma: false, .. } | Kind::LargeOperator
        )
    })
}

/// A term's Typst source, tidied into a dictionary key.
///
/// Typst escapes (mitex writes every parenthesis as `\(`) lose their backslash,
/// and the spacing mitex leaves behind is dropped — except where a letter is
/// involved on both sides, since there the space is the only thing separating
/// `alpha epsilon` from one long name. Between two digits it is dropped too,
/// because mitex sets `0.05` as four separate pieces.
///
/// `Glossary::explain()` reuses this on an author's LaTeX-turned-Typst
/// fragment so a lesson's curated key lands in the same space as the one a
/// compiled term produces.
pub(crate) fn key(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    let mut spaced = false;

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            spaced = true;
            continue;
        }
        if ch == '\\'
            && chars
                .peek()
                .is_some_and(|&next| !next.is_alphanumeric() && next != '\\')
        {
            continue;
        }
        if let Some(previous) = out.chars().next_back()
            && spaced
            && ch.is_alphanumeric()
            && previous.is_alphanumeric()
            && (ch.is_alphabetic() || previous.is_alphabetic())
        {
            out.push(' ');
        }
        spaced = false;
        out.push(ch);
    }
    out
}

/// The glossary key a lesson's `.explain()` fragment must use to match the
/// key [`partition`] gives the very same fragment when it appears inside a
/// compiled formula.
///
/// `typst_math` is Typst source — what [`crate::formula::to_typst_math`]
/// converts an author's LaTeX into, not the LaTeX itself. This parses it as
/// a detached equation and keys its top level through the identical
/// [`unwrap_stretch`] + [`key`] path `partition` walks, so a `\left`/`\right`
/// spelling and a plain-paren spelling of the same fragment always resolve
/// to one key. Falls back to keying `typst_math` verbatim if it does not
/// parse as an equation at all — `LessonBuilder::explain` only reaches this
/// on text already proven to parse, since it comes from a successful
/// [`crate::formula::to_typst_math`] call.
pub(crate) fn explain_key(typst_math: &str) -> String {
    let source = Source::detached(format!("${typst_math}$"));
    let top = equation_body(&source)
        .map(|body| unwrap_stretch(&children(body.node, body.range.start)))
        .unwrap_or_default();

    match (top.first(), top.last()) {
        (Some(first), Some(last)) => {
            key(source.text().get(first.range.start..last.range.end).unwrap_or(typst_math))
        }
        _ => key(typst_math),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typst(latex: &str) -> String {
        crate::formula::to_typst_math(latex).expect("mitex conversion")
    }

    /// The bug this function fixes: `\left`/`\right` compiles to an `lr(...)`
    /// wrapper that `partition` strips before keying, but a naive `key()` on
    /// the raw conversion does not — so the same fragment, spelled two ways,
    /// used to land under two different glossary entries.
    #[test]
    fn left_right_and_plain_parens_key_identically() {
        let left_right = typst(r"\left( \sigma_{t+1}^2 - \bar{\sigma}^2 \right)");
        let plain = typst(r"(\sigma_{t+1}^2 - \bar{\sigma}^2)");

        assert_eq!(explain_key(&left_right), explain_key(&plain));
    }

    /// A lesson's `explain()` entry and the compiled formula it is meant to
    /// annotate reach their key by different routes, and an entry filed under
    /// the wrong key matches nothing and says nothing — silently. So walk both
    /// routes over the three shapes a fragment comes in (juxtaposed atoms, a
    /// fraction, a stretched bracket) and pin where they meet.
    #[test]
    fn explain_key_agrees_with_the_key_partition_gives_the_same_fragment() {
        for (latex, expected) in [
            (r"\alpha \varepsilon_{t-1}^2", "alpha epsilon_(t-1)^(2)"),
            (r"\frac{\omega}{1 - \alpha - \beta}", "frac(omega,1-alpha-beta)"),
            (
                r"\left( \sigma_{t+1}^2 - \bar{\sigma}^2 \right)",
                "(sigma_(t+1)^(2)-macron(sigma)^(2))",
            ),
        ] {
            let compiled = crate::formula::compile(latex, true).expect("compile");
            let [term] = &compiled.terms[..] else {
                let keys: Vec<&str> = compiled.terms.iter().map(|term| term.key.as_str()).collect();
                panic!("{latex} is one whole term, but partition gave {keys:?}");
            };

            assert_eq!(term.key, expected, "partition keyed {latex}");
            assert_eq!(explain_key(&typst(latex)), expected, "explain() keyed {latex}");
        }
    }
}
