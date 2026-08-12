//! A tiny infix expression language, so that a tunable curve can be *data*.
//!
//! A lesson holds no closures — that is what lets it be saved and replayed
//! (see `.scratch/plan.md`) — so a curve the reader can retune has to arrive
//! as text: `"omega + alpha * x^2"`, plus the parameters that name its knobs.
//! This module parses that text once and evaluates it a few hundred times per
//! redraw.
//!
//! The language is deliberately not programmable: arithmetic, unary minus,
//! `^`, a fixed function table and named variables. There is no assignment,
//! no comparison and no control flow, so an expression is always a number and
//! evaluation always terminates.

/// A parsed expression, ready to evaluate over a set of named values.
#[derive(Clone, Debug)]
pub(crate) struct Expr {
    root: Node,
}

impl Expr {
    /// Parse `source` into an expression.
    ///
    /// # Errors
    /// [`ExprError`] describing the first thing in `source` the grammar could
    /// not accept, with the position it was found at.
    pub(crate) fn parse(source: &str) -> Result<Self, ExprError> {
        let mut parser = Parser::new(source);
        let root = parser.expression()?;
        parser.skip_whitespace();
        match parser.peek() {
            Some(character) => Err(ExprError::Unexpected { character, position: parser.position }),
            None => Ok(Self { root }),
        }
    }

    /// Every name the expression reads, in the order it first reads them.
    pub(crate) fn variables(&self) -> Vec<&str> {
        let mut names = Vec::new();
        self.root.collect_variables(&mut names);
        names
    }

    /// Evaluate the expression, reading each name from `bindings`.
    ///
    /// A name with no binding evaluates to `NaN` rather than failing: a
    /// caller plotting the result skips non-finite samples anyway, and
    /// [`Self::variables`] is how an undeclared name is caught up front.
    pub(crate) fn eval(&self, bindings: &[(&str, f64)]) -> f64 {
        self.root.eval(bindings)
    }
}

/// Why an expression could not be parsed.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub(crate) enum ExprError {
    #[error("unexpected {character:?} at position {position}")]
    Unexpected { character: char, position: usize },

    #[error("expected {expected} at position {position}")]
    Expected { expected: &'static str, position: usize },

    #[error("expression ended early, expected {expected}")]
    EndedEarly { expected: &'static str },

    /// Two values with no operator between them, as in `2x` or `3(x + 1)`.
    #[error("missing operator at position {position}; a product has to be written out, as in `2 * x`")]
    MissingOperator { position: usize },

    #[error("unknown function {name:?}; the ones that exist are {}", function_names())]
    UnknownFunction { name: String },

    #[error("{name} takes {expected} argument(s), given {given}")]
    Arity { name: &'static str, expected: usize, given: usize },

    #[error("{text:?} is not a number")]
    NotANumber { text: String },

    #[error("expression nests deeper than {limit} brackets")]
    TooDeep { limit: usize },
}

#[derive(Clone, Debug)]
enum Node {
    Number(f64),
    Variable(String),
    Negate(Box<Node>),
    Binary { operator: Operator, left: Box<Node>, right: Box<Node> },
    Call { function: &'static Function, arguments: Vec<Node> },
}

impl Node {
    fn eval(&self, bindings: &[(&str, f64)]) -> f64 {
        match self {
            Self::Number(value) => *value,
            Self::Variable(name) => bindings
                .iter()
                .find(|(bound, _)| bound == name)
                .map_or(f64::NAN, |(_, value)| *value),
            Self::Negate(inner) => -inner.eval(bindings),
            Self::Binary { operator, left, right } => operator.apply(left.eval(bindings), right.eval(bindings)),
            Self::Call { function, arguments } => {
                // Arity is checked while parsing, so the unused slot of a
                // one-argument call is never read.
                let mut values = [f64::NAN; MAX_ARITY];
                for (slot, argument) in values.iter_mut().zip(arguments) {
                    *slot = argument.eval(bindings);
                }
                (function.apply)(values[0], values[1])
            }
        }
    }

    fn collect_variables<'a>(&'a self, names: &mut Vec<&'a str>) {
        match self {
            Self::Number(_) => {}
            Self::Variable(name) => {
                if !names.contains(&name.as_str()) {
                    names.push(name);
                }
            }
            Self::Negate(inner) => inner.collect_variables(names),
            Self::Binary { left, right, .. } => {
                left.collect_variables(names);
                right.collect_variables(names);
            }
            Self::Call { arguments, .. } => {
                for argument in arguments {
                    argument.collect_variables(names);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl Operator {
    fn apply(self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Subtract => left - right,
            Self::Multiply => left * right,
            Self::Divide => left / right,
            Self::Power => left.powf(right),
        }
    }
}

/// The most arguments any [`Function`] takes.
const MAX_ARITY: usize = 2;

/// One of the functions an expression may call. A one-argument function is
/// handed [`f64::NAN`] as its second argument and ignores it.
#[derive(Debug)]
struct Function {
    name: &'static str,
    arity: usize,
    apply: fn(f64, f64) -> f64,
}

/// The whole function table. Adding a function is a line here and a row in
/// the README's list; nothing else in the crate knows what exists — the error
/// a typo earns reads the table too.
const FUNCTIONS: &[Function] = &[
    Function { name: "sin", arity: 1, apply: |value, _| value.sin() },
    Function { name: "cos", arity: 1, apply: |value, _| value.cos() },
    Function { name: "tan", arity: 1, apply: |value, _| value.tan() },
    Function { name: "exp", arity: 1, apply: |value, _| value.exp() },
    Function { name: "ln", arity: 1, apply: |value, _| value.ln() },
    Function { name: "log", arity: 1, apply: |value, _| value.log10() },
    Function { name: "sqrt", arity: 1, apply: |value, _| value.sqrt() },
    Function { name: "abs", arity: 1, apply: |value, _| value.abs() },
    Function { name: "floor", arity: 1, apply: |value, _| value.floor() },
    Function { name: "ceil", arity: 1, apply: |value, _| value.ceil() },
    Function { name: "pow", arity: 2, apply: f64::powf },
    Function { name: "min", arity: 2, apply: f64::min },
    Function { name: "max", arity: 2, apply: f64::max },
];

fn lookup(name: &str) -> Option<&'static Function> {
    FUNCTIONS.iter().find(|function| function.name == name)
}

fn function_names() -> String {
    FUNCTIONS.iter().map(|function| function.name).collect::<Vec<_>>().join(", ")
}

/// A recursive-descent parser over the expression's characters.
///
/// Characters rather than bytes, so `position` counts what an author counts
/// when reading the error back against what they wrote, and so a Greek
/// parameter name costs nothing extra.
struct Parser {
    characters: Vec<char>,
    position: usize,
    depth: usize,
}

/// How deeply brackets and signs may nest. Far more than an author writing
/// maths by hand will reach, and low enough that a pathological expression —
/// which can arrive through [`crate::Lesson::load`], not just from a keyboard
/// — is refused rather than run out of stack part way down.
const MAX_DEPTH: usize = 64;

impl Parser {
    fn new(source: &str) -> Self {
        Self { characters: source.chars().collect(), position: 0, depth: 0 }
    }

    /// Parse one level deeper, at every point the grammar recurses.
    fn nested<T>(&mut self, parse: impl FnOnce(&mut Self) -> Result<T, ExprError>) -> Result<T, ExprError> {
        if self.depth == MAX_DEPTH {
            return Err(ExprError::TooDeep { limit: MAX_DEPTH });
        }
        self.depth += 1;
        let parsed = parse(self);
        self.depth -= 1;
        parsed
    }

    fn peek(&self) -> Option<char> {
        self.characters.get(self.position).copied()
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.position += 1;
        }
    }

    /// Consume the next non-space character if it is `wanted`.
    fn eat(&mut self, wanted: char) -> bool {
        self.skip_whitespace();
        let found = self.peek() == Some(wanted);
        if found {
            self.position += 1;
        }
        found
    }

    fn expression(&mut self) -> Result<Node, ExprError> {
        let mut left = self.term()?;
        loop {
            self.skip_whitespace();
            let operator = match self.peek() {
                Some('+') => Operator::Add,
                Some('-') => Operator::Subtract,
                _ => return Ok(left),
            };
            self.position += 1;
            let right = self.term()?;
            left = Node::Binary { operator, left: Box::new(left), right: Box::new(right) };
        }
    }

    fn term(&mut self) -> Result<Node, ExprError> {
        let mut left = self.unary()?;
        loop {
            self.skip_whitespace();
            let operator = match self.peek() {
                Some('*') => Operator::Multiply,
                Some('/') => Operator::Divide,
                // A second value where an operator belongs is `2x` or
                // `3(x + 1)`: the one mistake worth naming, since a reader
                // writing maths by hand leaves the sign out without thinking.
                Some(character) if starts_a_value(character) => {
                    return Err(ExprError::MissingOperator { position: self.position });
                }
                _ => return Ok(left),
            };
            self.position += 1;
            let right = self.unary()?;
            left = Node::Binary { operator, left: Box::new(left), right: Box::new(right) };
        }
    }

    /// Unary sign binds looser than `^`, so `-x^2` is `-(x^2)` as it is on
    /// paper, while `2^-1` still reads as an exponent of minus one.
    fn unary(&mut self) -> Result<Node, ExprError> {
        self.skip_whitespace();
        match self.peek() {
            Some('-') => {
                self.position += 1;
                Ok(Node::Negate(Box::new(self.nested(Self::unary)?)))
            }
            Some('+') => {
                self.position += 1;
                self.nested(Self::unary)
            }
            _ => self.power(),
        }
    }

    fn power(&mut self) -> Result<Node, ExprError> {
        let base = self.atom()?;
        if !self.eat('^') {
            return Ok(base);
        }
        let exponent = self.unary()?;
        Ok(Node::Binary { operator: Operator::Power, left: Box::new(base), right: Box::new(exponent) })
    }

    fn atom(&mut self) -> Result<Node, ExprError> {
        self.skip_whitespace();
        let Some(character) = self.peek() else {
            return Err(ExprError::EndedEarly { expected: "a number, a name or `(`" });
        };

        if character == '(' {
            self.position += 1;
            let inner = self.nested(Self::expression)?;
            if !self.eat(')') {
                return Err(ExprError::Expected { expected: "`)`", position: self.position });
            }
            return Ok(inner);
        }
        if character.is_ascii_digit() || character == '.' {
            return self.number();
        }
        if is_name_start(character) {
            return self.name_or_call();
        }
        Err(ExprError::Unexpected { character, position: self.position })
    }

    fn number(&mut self) -> Result<Node, ExprError> {
        let start = self.position;
        while self.peek().is_some_and(|character| character.is_ascii_digit() || character == '.') {
            self.position += 1;
        }
        self.take_exponent();

        let text: String = self.characters[start..self.position].iter().collect();
        text.parse().map(Node::Number).map_err(|_| ExprError::NotANumber { text })
    }

    /// Extend a number over a trailing `e12`/`E-3`, but only when digits
    /// really follow — otherwise `2e` would swallow a variable named `e`.
    fn take_exponent(&mut self) {
        if !self.peek().is_some_and(|character| character == 'e' || character == 'E') {
            return;
        }
        let signed = matches!(self.characters.get(self.position + 1), Some('+' | '-'));
        let digits_at = self.position + 1 + usize::from(signed);
        if !self.characters.get(digits_at).is_some_and(char::is_ascii_digit) {
            return;
        }
        self.position = digits_at;
        while self.peek().is_some_and(|character| character.is_ascii_digit()) {
            self.position += 1;
        }
    }

    fn name_or_call(&mut self) -> Result<Node, ExprError> {
        let start = self.position;
        while self.peek().is_some_and(is_name_continuation) {
            self.position += 1;
        }
        let name: String = self.characters[start..self.position].iter().collect();

        if !self.eat('(') {
            return Ok(Node::Variable(name));
        }
        let Some(function) = lookup(&name) else {
            return Err(ExprError::UnknownFunction { name });
        };

        let mut arguments = Vec::new();
        if !self.eat(')') {
            loop {
                arguments.push(self.nested(Self::expression)?);
                if self.eat(')') {
                    break;
                }
                if !self.eat(',') {
                    return Err(ExprError::Expected { expected: "`,` or `)`", position: self.position });
                }
            }
        }
        if arguments.len() != function.arity {
            return Err(ExprError::Arity {
                name: function.name,
                expected: function.arity,
                given: arguments.len(),
            });
        }
        Ok(Node::Call { function, arguments })
    }
}

/// Greek counts: a lesson that writes `ω` in its prose should be able to name
/// the slider `ω` too.
fn is_name_start(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

fn is_name_continuation(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// Whether `character` could begin a value rather than continue an expression.
fn starts_a_value(character: char) -> bool {
    is_name_start(character) || character.is_ascii_digit() || matches!(character, '.' | '(')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(source: &str) -> f64 {
        Expr::parse(source)
            .unwrap_or_else(|error| panic!("`{source}` should parse: {error}"))
            .eval(&[])
    }

    #[test]
    fn evaluates_arithmetic_precedence_and_the_function_table() {
        let cases: &[(&str, f64)] = &[
            ("1 + 2 * 3", 7.0),
            ("(1 + 2) * 3", 9.0),
            ("10 / 4", 2.5),
            ("1 - 2 - 3", -4.0),
            ("2 ^ 3 ^ 2", 512.0),
            ("-2 ^ 2", -4.0),
            ("(-2) ^ 2", 4.0),
            ("2 ^ -1", 0.5),
            ("2 * -3", -6.0),
            ("--3", 3.0),
            ("1e3", 1000.0),
            ("1.5e-2", 0.015),
            (".5", 0.5),
            ("sin(0)", 0.0),
            ("cos(0)", 1.0),
            ("tan(0)", 0.0),
            ("exp(0)", 1.0),
            ("ln(1)", 0.0),
            ("log(100)", 2.0),
            ("sqrt(9)", 3.0),
            ("abs(-3)", 3.0),
            ("floor(1.7)", 1.0),
            ("ceil(1.2)", 2.0),
            ("pow(2, 10)", 1024.0),
            ("min(3, 5)", 3.0),
            ("max (3, 5)", 5.0),
        ];

        for (source, expected) in cases {
            let found = eval(source);
            assert!((found - expected).abs() < 1e-9, "`{source}` gave {found}, expected {expected}");
        }
    }

    #[test]
    fn reads_named_values_and_reports_the_names_it_reads() {
        let expr = Expr::parse("omega + alpha * x^2 + beta * x").expect("should parse");

        assert_eq!(expr.variables(), ["omega", "alpha", "x", "beta"]);
        assert_eq!(expr.eval(&[("omega", 1.0), ("alpha", 2.0), ("beta", 3.0), ("x", 4.0)]), 45.0);
    }

    #[test]
    fn a_greek_name_is_one_variable() {
        let expr = Expr::parse("ω + α * x").expect("should parse");

        assert_eq!(expr.variables(), ["ω", "α", "x"]);
        assert_eq!(expr.eval(&[("ω", 1.0), ("α", 2.0), ("x", 3.0)]), 7.0);
    }

    #[test]
    fn an_unbound_name_evaluates_to_nan_rather_than_failing() {
        let expr = Expr::parse("x + gamma").expect("should parse");

        assert!(expr.eval(&[("x", 1.0)]).is_nan());
    }

    #[test]
    fn a_pole_evaluates_to_a_non_finite_number_rather_than_panicking() {
        assert!(eval("1 / 0").is_infinite());
        assert!(eval("ln(-1)").is_nan());
        assert!(eval("sqrt(-1)").is_nan());
    }

    #[test]
    fn malformed_expressions_are_rejected_with_where_and_what() {
        let cases: &[(&str, ExprError)] = &[
            ("1 + ", ExprError::EndedEarly { expected: "a number, a name or `(`" }),
            ("1 @ 2", ExprError::Unexpected { character: '@', position: 2 }),
            ("(1 + 2", ExprError::Expected { expected: "`)`", position: 6 }),
            ("wobble(3)", ExprError::UnknownFunction { name: "wobble".to_owned() }),
            ("sin(1, 2)", ExprError::Arity { name: "sin", expected: 1, given: 2 }),
            ("min(1)", ExprError::Arity { name: "min", expected: 2, given: 1 }),
            ("1.2.3", ExprError::NotANumber { text: "1.2.3".to_owned() }),
            ("2x", ExprError::MissingOperator { position: 1 }),
            ("3(x + 1)", ExprError::MissingOperator { position: 1 }),
            ("2 * (1 + x) x", ExprError::MissingOperator { position: 12 }),
            ("sin(2 x)", ExprError::MissingOperator { position: 6 }),
        ];

        for (source, expected) in cases {
            assert_eq!(Expr::parse(source).err().as_ref(), Some(expected), "for `{source}`");
        }
    }

    #[test]
    fn a_typo_is_told_which_functions_exist() {
        let error = Expr::parse("tanh(x)").expect_err("tanh is not in the table");

        let message = error.to_string();
        for name in FUNCTIONS {
            assert!(message.contains(name.name), "`{}` missing from: {message}", name.name);
        }
    }

    #[test]
    fn nesting_past_the_limit_is_an_error_rather_than_a_lost_stack() {
        // Reachable from `Lesson::load`, so it has to fail as data, not as a
        // stack overflow the caller cannot catch.
        for source in ["(".repeat(10_000), "-".repeat(10_000)] {
            assert_eq!(
                Expr::parse(&source).err(),
                Some(ExprError::TooDeep { limit: MAX_DEPTH }),
                "for {} of `{}`",
                source.len(),
                &source[..1]
            );
        }
        assert!(
            Expr::parse(&format!("{}x{}", "(".repeat(MAX_DEPTH), ")".repeat(MAX_DEPTH))).is_ok(),
            "the limit itself should still parse"
        );
    }
}
