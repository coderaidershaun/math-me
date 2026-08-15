//! Limits from zero to hero: what a limit actually asks, why it deliberately
//! refuses to look at the point it names, the moves that answer a 0/0
//! question, and the payoff — the derivative, which is a limit and nothing
//! else, and which every worked 0/0 here secretly was. Finance-flavoured
//! throughout: instantaneous growth rates, announcement jumps, option deltas,
//! continuous compounding. Cross-links: lesson-exponents (the natural-base
//! debt, settled in practice) and lesson-algebra-to-linear (the kinked
//! payoff). Each section function is named after the heading it renders and
//! chained in document order.
//!
//! Prerequisites:
//! - School algebra: factorising x^2 - 9, cancelling a common factor,
//!   rearranging an inequality.
//! - Function notation: reading f(x) as "what the rule f returns at x".
//! - No calculus assumed — the derivative is built here, from zero.
//!
//! Run it: cargo run --release --bin lesson-limits

use math_me::prelude::*;

fn main() -> math_me::Result<()> {
    let lesson = build();

    // The audit call belongs inside the assert: left outside it, a release
    // build would still compile every formula and then throw the answer away.
    debug_assert!(
        lesson.audit().is_empty(),
        "math errors, unexplained terms or unusable curves: {:?}",
        lesson.audit()
    );

    lesson.show()
}

fn build() -> Lesson {
    let b = Lesson::builder("Limits");
    let b = a_question_arithmetic_cannot_answer(b);
    let b = saying_it_in_notation(b);
    let b = the_hole_is_the_whole_point(b);
    let b = from_both_sides_or_not_at_all(b);
    let b = how_close_is_close_enough(b);
    let b = when_you_may_simply_plug_in(b);
    let b = four_moves_for_a_zero_over_zero(b);
    let b = pointing_the_arrow_outward(b);
    let b = every_derivative_is_a_limit(b);
    let b = practice(b);
    let b = letter_overrides(b);
    b.build()
}

fn a_question_arithmetic_cannot_answer(b: LessonBuilder) -> LessonBuilder {
    b.heading("A question arithmetic cannot answer")
        .note("Hover any term in a formula to see what it means here. Two plots below have sliders — drag them and the curves follow.")
        .para(|p| p
            .text("A fund opened at $100 million and has run for two years. Its value, in millions, has tracked the rule"))
        .display(r"V(t) = 100 + 8t - t^2")
        .explain(r"V(t)", "The fund's value at time t",
            "How many millions the fund is worth t years after launch. Everything in this lesson is asked about this one rule.")
        .explain("100", "The launch value",
            "What the fund was worth at t = 0, in millions of dollars.")
        .explain("8 t", "The early drift",
            "Eight million a year of steady growth — the part of the story that never fades.")
        .explain(r"t^2", "The drag",
            "A subtraction that grows with the square of time: small at first, then dominant. It is what eventually turns the fund over — and, in the derivative algebra later, the one term of V that does not cancel cleanly against its windowed copy.")
        .para(|p| p
            .text("At two years it is worth 100 plus 16 minus 4 — $112 million. Now the question every investor in it actually wants answered: how fast is it growing right now, at the two-year mark? Not on average since launch. Now."))
        .para(|p| p
            .text("You know how to answer a nearby question. Growth over a stretch of time is the change in value divided by the time it took:"))
        .display(r"m = \frac{V(t + h) - V(t)}{h}")
        .explain(r"\frac{V(t + h) - V(t)}{h}", "Average growth over a window of length h",
            "How much the fund gained between t and t + h, divided by how long that took: dollars per year, averaged across the window.")
        .para(|p| p
            .text("Set ")
            .math("t")
            .text(" to 2 and shrink the window. Over the next full year the fund goes from 112 to 115, so it averaged 3 million a year. Over the next half year: 3.5. Over a tenth of a year: 3.9. Over a hundredth: 3.99. Over a thousandth: 3.999. The answers are not wandering — they are marching on 4, and getting closer with every squeeze."))
        .para(|p| p
            .text("So set ")
            .math("h")
            .text(" to zero and read off the answer. Except:"))
        .display(r"\frac{V(2) - V(2)}{0} = \frac{0}{0}")
        .explain(r"\frac{V(2) - V(2)}{0}", "The window closed all the way",
            "A window of zero length: the fund gained nothing, over no time at all. The formula that worked for every other window collapses here.")
        .explain(r"\frac{0}{0}", "Nothing divided by nothing",
            "Not a number and not an error — a question with the answer torn off. Any number times zero is zero, so nothing about this expression singles one out.")
        .para(|p| p
            .text("Nothing divided by nothing is not 0, and not 1, and not undefined-in-the-boring-sense. It is a question that has been asked badly. Six times two is twelve, so twelve divided by two is six; that is what division means. But every number times zero is zero, so the question \"what times zero gives zero?\" has every number as an answer, which is the same as having none."))
        .para(|p| p
            .text("Here is the awkward fact this lesson exists to fix. The number 4 is real. The fund genuinely is growing at $4 million a year at the two-year mark; you can watch the averages close in on it. But the formula that computes it is undefined at exactly the value of ")
            .math("h")
            .text(" you care about. Every instantaneous rate in finance has this shape — a spot yield, an option's delta, a portfolio's exposure — and every one of them is quoted as a number in the same breath as the formula for it collapses."))
        .para(|p| p
            .text("The limit is the machine that reads the march without ever arriving. By the end of this lesson you will state what it asks precisely, know the four moves that answer it when the arithmetic breaks, know exactly when a limit fails to exist, and build the derivative out of it from scratch."))
}

fn saying_it_in_notation(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Saying it in notation")
        .para(|p| p
            .text("The sentence \"as ")
            .math("x")
            .text(" gets close to ")
            .math("a")
            .text(", the values ")
            .math("f(x)")
            .text(" close in on ")
            .math("L")
            .text("\" is written:"))
        .display(r"\lim_{x \to a} f(x) = L")
        .explain(r"\lim_{x \to a} f(x)", "The limit of f as x approaches a",
            "Not a value of f. It names the single number the outputs f(x) crowd around as the inputs x are driven toward a — a number f may never actually take. Two things the notation quietly assumes: that such a number exists, and that only one does. Uniqueness is a two-line theorem; existence rests on the completeness of the real numbers, and fails in the rationals.")
        .para(|p| p
            .text("Read it aloud, because the reading is the definition: \"the limit, as ")
            .math("x")
            .text(" approaches ")
            .math("a")
            .text(", of ")
            .math("f(x)")
            .text(", is ")
            .math("L")
            .text("\". Three parts, each doing separate work. The moving input ")
            .math("x")
            .text(" is what travels. The target ")
            .math("a")
            .text(" is where it travels toward — a place on the input axis, never a value of the function. And ")
            .math("L")
            .text(" is the answer: a value on the output axis."))
        .para(|p| p
            .text("Note what the notation withholds. It says nothing whatever about ")
            .math("f(a)")
            .text(". It does not say ")
            .math("f")
            .text(" reaches ")
            .math("L")
            .text(", nor that ")
            .math("x")
            .text(" reaches ")
            .math("a")
            .text(". The arrow is a direction of travel, and travel is all it promises. The next section makes that omission the centre of the lesson, because it is the reason limits are worth inventing at all."))
        .explain(r"f(a)", "The value of f at the target",
            "What the rule f actually returns when handed a. A separate number from the limit, and often not a number at all.")
        .para(|p| p
            .text("Before G. H. Hardy standardised the arrow in 1908, people wrote things like \"lim, x = a\", with an equals sign where the arrow now sits — and that equals sign was exactly the confusion the arrow was invented to kill, because ")
            .math("x")
            .text(" never does equal ")
            .math("a")
            .text("."))
}

fn the_hole_is_the_whole_point(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("The hole is the whole point")
        .para(|p| p
            .text("Take a formula that is broken at exactly one place:"))
        .display(r"f(x) = \frac{x^2 - 9}{x - 3}")
        .explain(r"\frac{x^2 - 9}{x - 3}", "A formula with one bad input",
            "Perfectly well behaved for every x except 3, where the bottom is zero and the whole expression collapses to 0/0.")
        .para(|p| p
            .text("At ")
            .math("x")
            .text(" = 3 the bottom is zero and the top is zero: the same 0/0 the fund handed us. At every other input it is fine. So walk up to 3 and watch."))
        .figure(Figure::new(HOLE_SVG,
            "The same function twice: as a table of approaches and as a graph. From the left the outputs climb 5.9, 5.99, 5.999; from the right they fall 6.1, 6.01, 6.001. Both sides squeeze on 6. At x = 3 itself there is no value at all — the graph is a straight line with a single point punched out of it, and the punched-out point sits exactly where both approaches were pointing."))
        .para(|p| p
            .text("Why the values behave so tamely is pure algebra. The top factors, and the factor that appears on top also appears underneath:"))
        .display(r"\frac{x^2 - 9}{x - 3} = \frac{(x-3)(x+3)}{x - 3} = x + 3")
        .explain(r"\frac{(x-3)(x+3)}{x - 3}", "The same formula, factored",
            "The difference of two squares split into its factors, revealing the x - 3 that the bottom will cancel.")
        .explain(r"x+3", "What is left after the cancellation",
            "The formula's true behaviour everywhere except at x = 3, where the cancellation was not licensed.")
        .para(|p| p
            .text("Now the move that matters, and the reason it is legal. Cancelling ")
            .math(r"(x-3)")
            .text(" from top and bottom requires dividing by ")
            .math(r"(x-3)")
            .text(", which is forbidden when that quantity is zero — that is, when ")
            .math("x")
            .text(" is 3. And ")
            .math("x")
            .text(" is never 3. The limit sends ")
            .math("x")
            .text(" toward 3, through 2.9 and 2.99 and 2.999 and 3.001, and at no point on that journey is ")
            .math("x")
            .text(" equal to 3. So the cancellation is licensed at every single input the limit ever inspects."))
        .explain(r"(x-3)", "The factor that is zero at 3",
            "Zero exactly when x is 3 and nowhere else. In the hole example it is the factor being cancelled — legal because x is never 3 on the approach. In move one, later, it is the factor that survives the cancellation and delivers the answer.")
        .para(|p| p
            .text("That is the trick, and it is the same trick every time. The limit is allowed to cancel because the limit never arrives. So on the approach the function is simply the line ")
            .math("x + 3")
            .text(", and a line has no surprises:"))
        .display(r"\lim_{x \to 3} \frac{x^2 - 9}{x - 3} = 6")
        .explain(r"\lim_{x \to 3} \frac{x^2 - 9}{x - 3}", "The limit of the broken formula at its broken point",
            "Six. Reached by cancelling the shared factor — legal because x is never actually 3 on the approach — and then reading the line x + 3 at 3.")
        .rule()
        .para(|p| p
            .text("Now hold on to the strangeness. The value ")
            .math("f(3)")
            .text(" does not exist. The limit at 3 is 6. Both statements are true at once, and neither contradicts the other, because they are answers to different questions: the limit asks about the journey, and only about the journey. Where the traveller ends up is not its business."))
        .explain(r"f(3)", "The value at the broken input",
            "It does not exist: at x = 3 the formula is 0/0. That non-existence is entirely compatible with the limit at 3 being 6.")
        .note("Three functions can agree everywhere except at one point and be wildly different there — undefined, or equal to 6, or equal to minus 400 — and all three have the identical limit of 6, because the limit never looks at that point.")
        .para(|p| p
            .text("This is the primary idea of the lesson, and everything after it is machinery. A limit reports where a function is heading, not where it lands. Which is precisely why it can answer the fund's question: a formula can be undefined at the one input you care about and still point unambiguously at a number, and the limit is how you read the pointing."))
}

fn from_both_sides_or_not_at_all(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("From both sides, or not at all")
        .para(|p| p
            .text("In the table, the approach from below and the approach from above agreed. They do not have to. Because the two directions can disagree, each gets its own name and its own notation — a small raised minus for the approach from below, a raised plus for the approach from above:"))
        .display(r"\lim_{x \to a^{-}} f(x) \quad \lim_{x \to a^{+}} f(x)")
        .explain(r"\lim_{x \to a^{-}} f(x)", "The left-hand limit",
            "Where f is heading as x closes in on a from below — through inputs smaller than a only. The raised minus marks the side, not a negative number.")
        .explain(r"\lim_{x \to a^{+}} f(x)", "The right-hand limit",
            "Where f is heading as x closes in on a from above — through inputs larger than a only.")
        .explain(r"\lim_{x \to a^{-}} f(x) \quad \lim_{x \to a^{+}} f(x)", "The two one-sided limits",
            "The same function and the same target, approached from each side separately. Each is computed blind to the other — and nothing forces them to agree.")
        .para(|p| p
            .text("These are the one-sided limits, and the rule joining them to the plain two-sided limit asks for no new idea: the limit exists exactly when both one-sided limits exist and are equal, and then it equals their common value."))
        .display(r"\lim_{x \to a^{-}} f(x) = \lim_{x \to a^{+}} f(x) = L")
        .explain(r"\lim_{x \to a^{+}} f(x) = L", "Both sides agreeing on L",
            "When the approach from above and the approach from below arrive at the same L, that shared number is the limit. When they do not, there is no limit — not a disputed one, none. Once the tolerance game arrives, this rule turns out to be a one-line theorem: the punctured window 0 < |x − a| < δ is exactly the two one-sided windows glued together.")
        .para(|p| p
            .text("Finance supplies the disagreement for free. A company reports earnings at nine in the morning. Right before the announcement the stock is trading around $100 and drifting quietly. Right after, it is $108. Between those two states there is no path, only a discontinuity in the tape."))
        .figure(Figure::new(JUMP_SVG,
            "An earnings announcement, drawn as a function of time. Approaching the announcement from before, the price heads for 100; approaching from after, it heads for 108. The auction that reopened the stock printed 104. Three different numbers at one instant — the left limit, the right limit, and the actual value — and because the first two disagree, the price has no limit at that instant, whatever the third one says."))
        .para(|p| p
            .text("Three separate numbers live at that one moment: the left limit of 100, the right limit of 108, and the actual print of 104. Because the first two disagree, there is no two-sided limit. Notice how little the print of 104 has to do with it: even if the auction had printed exactly 100, or exactly 108, the two-sided limit would still not exist. The value at the point can neither create a limit nor destroy one."))
        .rule()
        .para(|p| p
            .text("Limits fail in a handful of recognisable ways, and it is worth being able to name the one in front of you."))
        .para(|p| p
            .text("It jumps. The two sides head for different numbers, as in the announcement. Anything with a scheduled discrete event does this: a bond at a coupon date, a fund at a distribution, an option at expiry."))
        .para(|p| p
            .text("It blows up. The values do not settle on any number because they grow without bound — ")
            .math(r"1/x^2")
            .text(" as ")
            .math("x")
            .text(" nears zero climbs past every number you can name. There is no ")
            .math("L")
            .text(" because no finite ")
            .math("L")
            .text(" could be the answer."))
        .explain(r"1/x^2", "One over x squared",
            "Positive on both sides of zero and unbounded as x nears it: the standard blow-up. Its two-sided limit at 0 does not exist.")
        .para(|p| p
            .text("It oscillates. This one is the least intuitive and the most instructive. Consider ")
            .math(r"\sin(1/x)")
            .text(" as ")
            .math("x")
            .text(" approaches zero. The values stay politely between minus one and one — nothing blows up — but as ")
            .math("x")
            .text(" shrinks, ")
            .math("1/x")
            .text(" races off, and the sine sweeps its full range faster and faster. In any interval around zero, however microscopic, the function takes the value 1 infinitely often and the value minus 1 infinitely often. It settles nowhere, so it has no limit. Bounded is not the same as convergent, and that distinction has ruined more than one intuition about noisy price data."))
        .explain(r"\sin(1/x)", "Sine of one over x",
            "Bounded between minus one and one, but oscillating infinitely fast as x nears zero. It never settles, so it has no limit at 0 — bounded and convergent are different things. Sample it only at inputs of the form one over a whole multiple of pi and every reading is exactly zero: a table can make it look flat. The practice section springs that trap properly.")
        .explain(r"1/x", "One over x",
            "Small inputs make it enormous. As x is driven toward zero this races off without bound, which is what drives the oscillation.")
}

fn how_close_is_close_enough(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("How close is close enough?")
        .para(|p| p
            .text("Everything so far has leaned on the words \"close\" and \"heading toward\", which are not mathematics. Mathematicians ran on that intuition for a hundred and fifty years and it mostly worked, until it stopped working and nobody could say which arguments were sound. The repair, and it is a genuinely beautiful one, is to recast the whole thing as a game between two players."))
        .para(|p| p
            .text("You claim a limit is ")
            .math("L")
            .text(". A sceptic tests the claim. The sceptic moves first, and names a tolerance: a band around ")
            .math("L")
            .text(" so narrow that landing inside it would be impressive. Call the half-width of that band ")
            .math(r"\varepsilon")
            .text(", the Greek epsilon, for \"error\". The claim survives if the outputs eventually stay inside:"))
        .explain_char('ε', "Greek small epsilon",
            "The tolerance the sceptic demands: how close the outputs must come to L. The sceptic picks it, and may pick it as small as they like. Because it can be made smaller than any gap you name, it is also what makes two rival limits impossible.")
        .display(r"L - \varepsilon < f(x) < L + \varepsilon")
        .explain(r"f(x)", "The output being tested",
            "Where the function actually is, for a particular input x. The sceptic demands this land inside the band around L.")
        .para(|p| p
            .text("Now you answer. You cannot control ")
            .math(r"\varepsilon")
            .text(" — the sceptic already chose it. What you control is how close to ")
            .math("a")
            .text(" you insist the inputs come. You name a closeness ")
            .math(r"\delta")
            .text(", the Greek delta, for \"distance\", and promise that every input inside your window does the job:"))
        .explain_char('δ', "Greek small delta",
            "The closeness you supply in reply: how near to a the inputs must be for the outputs to land inside the sceptic's band. You get to see ε before choosing it.")
        .display(r"a - \delta < x < a + \delta")
        .para(|p| p
            .text("With one exclusion, which by now you will expect: the input ")
            .math("a")
            .text(" itself is struck out of that window. The promise covers every input near ")
            .math("a")
            .text(" except ")
            .math("a")
            .text(". Textbooks write the two conditions with vertical bars — 0 < |x − a| < δ implies |f(x) − L| < ε — where the bars mean distance and the leading 0 < is exactly this exclusion."))
        .figure(Figure::new(EPSILON_SVG,
            "The game, drawn. The sceptic shades a horizontal band of half-width epsilon around L. You reply with a vertical stripe of half-width delta around a, chosen so the curve inside your stripe never escapes the band. The sceptic tightens to the narrow dashed band; you tighten to the narrow dashed stripe. The claim is true only if you can answer every band, however narrow — and note that the point above a is punched out, so the value there is never tested."))
        .para(|p| p
            .text("The limit is ")
            .math("L")
            .text(" precisely when you can answer every challenge: for every ")
            .math(r"\varepsilon")
            .text(" the sceptic names, however brutally small, some ")
            .math(r"\delta")
            .text(" exists that keeps the promise."))
        .note("The order of the two players is the entire content of the definition. Epsilon is chosen first, in ignorance of your reply; delta is chosen second, knowing epsilon. Swap the order and you have defined something else — and something false.")
        .para(|p| p
            .text("Run it once with numbers, on a function simple enough to see through. Let ")
            .math("f(x) = 3x + 1")
            .text(", with ")
            .math("a")
            .text(" = 2, and claim the limit is 7. The sceptic demands the output land within 0.03 of 7 — between 6.97 and 7.03. Since the output is ")
            .math("3x + 1")
            .text(", it sits within 0.03 of 7 exactly when ")
            .math("3x")
            .text(" sits within 0.03 of 6, which happens exactly when ")
            .math("x")
            .text(" sits within 0.01 of 2. So answer 0.01 and the promise holds."))
        .explain(r"f(x) = 3x + 1", "A straight line to test the game on",
            "Chosen because the algebra is transparent: whatever tolerance the sceptic demands on the output, dividing by the slope 3 converts it into the closeness required on the input.")
        .explain(r"3x + 1", "Three times the input, plus one",
            "The output rule. Its slope of 3 is what magnifies input error into output error, and dividing by it is what undoes the magnification.")
        .explain(r"3x", "Three times the input",
            "The part of the output that moves. Keeping it within 0.03 of 6 is the same demand as keeping x within 0.01 of 2.")
        .para(|p| p
            .text("But answering one challenge proves nothing — the sceptic gets to keep going. So answer all of them at once. Whatever tolerance is named, divide it by the slope:"))
        .display(r"\delta = \frac{\varepsilon}{3}")
        .explain(r"\frac{\varepsilon}{3}", "The reply, as a formula",
            "Divide the demanded output tolerance by the slope 3 to get the required input closeness. A formula in ε rather than a single number is what wins the game outright: it answers every challenge at once.")
        .para(|p| p
            .text("That formula, not any particular number, is the proof. It is also the intuition behind the whole apparatus: a steeper function magnifies input error into output error, so a steeper function demands a tighter ")
            .math(r"\delta")
            .text(" for the same ")
            .math(r"\varepsilon")
            .text(". Slope is the exchange rate between the two tolerances — which is a hint about where this lesson is going. And a straight line hides one thing: its exchange rate is the same at every point, so a single formula settles every challenge everywhere. A curve's is not — it changes as you move along it — and the payoff section returns to collect exactly this. The plot below lets you feel the difference before the algebra arrives."))
        .plot(Plot::new(-0.5..=0.5)
            .curve("how far the fund's value strays from V(a)", "abs((8 - 2*a)*x - x^2)")
            .curve("what the sceptic allows", "epsilon")
            .param("a", 0.0..=5.0, 2.0)
            .param("epsilon", 0.05..=2.0, 0.5)
            .vline(0.0)
            .x_label("how far the input strays from a, in years")
            .y_label("how far the output strays, in millions")
            .height(300.0)
            .caption("The game, made playable, on the fund instead of a straight line. The wedge is how far an input straying from a pushes the value away from V(a); the flat line is the sceptic's tolerance. You win with any delta whose window holds the wedge under the line the whole way across, so the winning delta is the first crossing on either side, read straight off the bottom axis. The vertical line is a itself, the one input the window strikes out. Drag a: the crossing sits near 0.06 at launch and near 0.12 at a = 2 — twice the room for the same tolerance, because the fund is half as steep there. At a = 4, the peak, the arms flatten so far that at this tolerance the wedge stays under the line right across the window. But tighten epsilon and even the peak has to answer: with the exchange rate gone the wedge is just the square of the stray, so the winning delta there is the square root of the tolerance rather than a fraction of it — punishingly generous by comparison, and the first sign of what a vanishing derivative buys. The steepness of the arms is the size of 8 minus 2a, the exchange rate this section promised."))
        .para(|p| p
            .text("The formulation is not decoration; it is what rescued calculus. Bernard Bolzano wrote it down in 1817 and was ignored. Augustin-Louis Cauchy used the letters in the 1820s but never quite tied ")
            .math(r"\delta")
            .text(" to ")
            .math(r"\varepsilon")
            .text(". Karl Weierstrass, lecturing in Berlin in the 1860s, stated it in the form above, and with that the arguments about who was reasoning correctly ended, because there was finally something to check. In practice you will almost never run the game by hand — you will use the laws of the next section. But when a limit is contested, this is the court of appeal, and there is no higher one."))
        .para(|p| p
            .text("One assumption underneath all of it deserves its name, because the game never supplies it. The tolerance game tests a claimed destination: hand it an ")
            .math("L")
            .text(" and it rules on ")
            .math("L")
            .text(". Nothing in it produces one — so the opening section's confident \"the answers are marching on 4\" leaned on something extra: that a march must have somewhere to land. Among the fractions alone, it need not. The sequence 1, 1.4, 1.41, 1.414 bunches up exactly as convincingly as the fund's averages did, and converges to nothing, because the number it is aiming at is the square root of two and no fraction squares to two."))
        .para(|p| p
            .text("What supplies the destination is an axiom, assumed rather than proved — completeness: every non-empty collection of real numbers with a ceiling has a least ceiling. The reals are the rationals with every missing destination deliberately built in, a construction Dedekind and Cantor finally gave in 1872, because pressing for rigour about limits had forced the deeper question of what a real number is. Marches arrive in the reals because the reals were built so that marches arrive."))
        .note("And the limit, not a limit: two rival answers cannot both survive the game. Let the sceptic demand a tolerance of half the gap between them — the two bands no longer overlap, yet the outputs would eventually have to sit inside both at once. The definite article is a theorem, not a habit.")
}

fn when_you_may_simply_plug_in(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("When you may simply plug in")
        .para(|p| p
            .text("Nobody runs the tolerance game on live problems, because a handful of laws do the work. Each is provable from the game, and each says the same reassuring thing: limits pass straight through ordinary arithmetic. The limit of a sum is the sum of the limits; likewise for differences, products, and constant multiples."))
        .display(r"\lim_{x \to a} [f(x) + g(x)] = \lim_{x \to a} f(x) + \lim_{x \to a} g(x)")
        .explain(r"\lim_{x \to a}", "The limit operation itself",
            "The instruction to send x toward a and read where the values head. The laws say this instruction passes through sums, differences and products untouched.")
        .explain(r"[f(x) + g(x)]", "Two functions added before the limit is taken",
            "Add first, then take the limit. The sum law says this gives the same answer as taking both limits and adding them.")
        .explain(r"\lim_{x \to a} g(x)", "The limit of the second function",
            "Where g is heading as x approaches a — computed on its own, then added.")
        .para(|p| p
            .text("The reason is the tolerance game, and it is worth seeing once. If you need the sum to land within ")
            .math(r"\varepsilon")
            .text(" of its target, ask each piece to land within half of ")
            .math(r"\varepsilon")
            .text(" of its own target; two errors of at most half an epsilon cannot add to more than an epsilon. Then take the tighter of the two deltas that come back. The law is not an assumption — it is a budget, split in two."))
        .para(|p| p
            .text("Division gets a law too, with one condition attached that is the whole reason this lesson exists: the limit of a quotient is the quotient of the limits, provided the limit underneath is not zero. Strike that condition and you are back at 0/0, which is exactly the case the laws cannot handle and the next section is about."))
        .rule()
        .para(|p| p
            .text("Laws that pass limits through arithmetic need something to pass them through from, and there are exactly two starting points, both immediate. A constant does not move, so its limit is itself. And ")
            .math("x")
            .text(" is being driven at ")
            .math("a")
            .text(" by the very instruction the notation gives, so its limit is ")
            .math("a")
            .text(". Chain the laws over those two and a polynomial — built from constants and copies of ")
            .math("x")
            .text(" by adding and multiplying — has limits you get by substitution, with no game played at any step. The same holds for sines, cosines, exponentials and logarithms on their domains. Functions with that property have a name:"))
        .display(r"\lim_{x \to a} f(x) = f(a)")
        .explain(r"\lim_{x \to a} f(x) = f(a)", "The definition of continuity at a",
            "The journey and the destination agree. Everything the approach predicts, the value at a delivers. This is a property some functions have at some points — never something a limit guarantees. Note that the theorems continuity buys lean on completeness too: among the fractions alone, x squared minus 2 is continuous, negative at 1, positive at 2, and never zero, because the crossing it owes is at a number the fractions do not contain.")
        .para(|p| p
            .text("A function is continuous at ")
            .math("a")
            .text(" when the limit there exists, the value there exists, and the two are equal. Continuity is not what a limit is; continuity is the special case where the journey and the destination happen to agree, and \"just plug in\" is a licence you get only from it. Every worked example in this lesson that could not be plugged in was a point of discontinuity."))
        .para(|p| p
            .text("Which makes the failures easy to catalogue, and the cut that organises them is a single question: does the limit survive? If it does, the discontinuity is removable, and there is nothing else it can be. The broken formula ")
            .math(r"(x^2-9)/(x-3)")
            .text(" has limit 6 at 3 and no value there, and defining the value to be 6 removes it — as it would if the value existed but disagreed, which is the same repair with a different starting mess. If the limit does not survive, the names on offer describe how it failed rather than partitioning anything: the earnings announcement has both one-sided limits but no two-sided one, a jump; ")
            .math(r"1/x^2")
            .text(" has no finite limit at all, an infinite discontinuity; and ")
            .math(r"\sin(1/x)")
            .text(" settles nowhere, an oscillating one."))
        .explain(r"(x^2-9)/(x-3)", "The broken formula, written inline",
            "Limit 6 at x = 3, no value at x = 3. Filling in the missing value repairs it completely, which is what makes the discontinuity removable.")
        .explain(r"(x^2-9)", "The top of the broken formula",
            "A difference of squares, zero at x = 3 — vanishing there together with the bottom is what makes the 0/0.")
        .note("Removable is the interesting case, and the one to remember: the function is not merely fixable, it is fixable in exactly one way, because the limit names the only value that would make it continuous. Every derivative in the next section is a removable discontinuity, repaired.")
        .note("And one failure has no drama at all: nothing on one side to approach. An option's time to expiry runs down to zero from above only, so the only limit on offer there is one-sided — not because the function misbehaves but because the domain has one side.")
        .rule()
        .para(|p| p
            .text("Continuity also buys more than a licence to substitute. If a continuous function is negative somewhere and positive somewhere else, it must be zero in between — crossing without touching would be a jump, and continuity is exactly the promise of no jumps. That is the intermediate value theorem, proved by Bolzano in 1817 in the very paper the previous section credits, and it is why a solver can find a number nobody can write down. A five-year bond paying a 4% coupon on $100 of face is worth $120.00 at a yield of 0% and $77.26 at 10%, and its price falls continuously in between — so if it trades at $100, some yield prices it exactly, guaranteed before any calculation is done, and halving the interval walks you there: too low at 5%, too high at 2.5%, trapped near 4.00% four steps later. That is bisection — the theorem run as an algorithm — and every yield to maturity, implied volatility and internal rate of return on a screen was found this way or by a faster relative. The exponents lesson names these root-hunts among the doors it deliberately leaves shut; continuity is the hinge they hang on."))
        .note("Two honesty clauses. The theorem promises a crossing exists, never that only one does — the exponents lesson's cash flow with two internal rates of return, 25% and 400%, is the standing warning. And it needs the continuity it was given: bracket a function across a jump and the sign can change with no root between, which is how a solver run over a discontinuous payoff returns an answer that is confidently wrong.")
}

fn four_moves_for_a_zero_over_zero(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Four moves for a 0/0")
        .para(|p| p
            .text("Substitution answers most limits. The ones it cannot answer announce themselves by returning 0/0 — or its cousins, infinity minus infinity and infinity over infinity. These are the indeterminate forms, and the name is precise: it does not mean the limit fails to exist, it means the form alone determines nothing. The fund's growth was a 0/0 whose answer was 4. Every derivative you will ever compute is a 0/0. The form is not a verdict; it is an instruction to do algebra."))
        .para(|p| p
            .text("Four moves cover almost everything you will meet, and the first has already been used."))
        .rule()
        .para(|p| p
            .text("Move one: factor and cancel. Whenever the top and bottom both vanish at ")
            .math("a")
            .text(", both contain a factor that vanishes at ")
            .math("a")
            .text(", and it can be divided out — legally, because the limit never arrives. Consider"))
        .display(r"\lim_{x \to 2} \frac{x^2 - 5x + 6}{x - 2}")
        .explain(r"\lim_{x \to 2} \frac{x^2 - 5x + 6}{x - 2}", "Move one's worked example",
            "Substituting 2 returns 0/0, which is an instruction: factor the top, cancel the shared factor, and read the survivor at 2. The answer is minus 1.")
        .explain(r"\frac{x^2 - 5x + 6}{x - 2}", "A 0/0 waiting to be factored",
            "Top and bottom both vanish at x = 2, so both carry a factor of x - 2. Cancel it and the limit falls out.")
        .para(|p| p
            .text("The top factors as ")
            .math(r"(x-2)(x-3)")
            .text(", the ")
            .math(r"(x-2)")
            .text(" cancels, and what remains is ")
            .math("x - 3")
            .text(", which at 2 is minus 1."))
        .explain(r"(x-2)(x-3)", "The top, factored",
            "Two roots, at 2 and 3. The first matches the bottom and cancels; the second survives and delivers the answer.")
        .explain(r"(x-2)", "The vanishing factor",
            "Zero exactly at x = 2. It appears top and bottom, and dividing it out is legal at every input the limit visits.")
        .explain(r"x - 3", "What the cancellation leaves",
            "A line, continuous everywhere, so its limit at 2 is simply its value there: minus 1.")
        .rule()
        .para(|p| p
            .text("Move two: multiply by the conjugate. Fractional powers block factoring, and the fix is to multiply top and bottom by the same expression with the middle sign flipped, which turns a difference into a difference of squares. Here is a version with a story attached. A fund gains a fraction ")
            .math("x")
            .text(" over two years. The annualised gain — the steady yearly rate that would compound to the same total — is ")
            .math(r"(1+x)^{1/2} - 1")
            .text(". What fraction of the total gain does one year get?"))
        .display(r"\lim_{x \to 0} \frac{(1+x)^{1/2} - 1}{x}")
        .explain(r"\frac{(1+x)^{1/2} - 1}{x}", "Annualised gain as a share of total gain",
            "The two-year total gain x, converted to its annual equivalent, divided by the total. At x = 0 both top and bottom vanish: a 0/0.")
        .explain(r"(1+x)^{1/2} - 1", "The annualised gain",
            "The square root of the growth factor, minus one: the yearly rate that compounds over two years to a total gain of x.")
        .explain(r"\lim_{x \to 0} \frac{(1+x)^{1/2} - 1}{x}", "The small-gain limit",
            "What share of a vanishingly small total gain one year takes. The conjugate move opens it, and the answer is exactly one half.")
        .explain(r"(1+x)^{1/2}", "The annual growth factor",
            "The square root of the two-year growth factor: what a single year multiplies the fund by, if both years match.")
        .explain(r"(1+x)", "The two-year growth factor",
            "One plus the fractional gain x: what the fund's starting value is multiplied by over the full two years.")
        .para(|p| p
            .text("Multiply top and bottom by ")
            .math(r"(1+x)^{1/2} + 1")
            .text(". The top becomes ")
            .math(r"(1+x) - 1")
            .text(", which is just ")
            .math("x")
            .text(", and that ")
            .math("x")
            .text(" cancels the one underneath, leaving one over ")
            .math(r"(1+x)^{1/2} + 1")
            .text(". At zero that is one over two. So for small gains, each year takes almost exactly half — a 2% two-year gain annualises to 0.995%, near enough half of 2%. For large ones it does not: a 21% gain annualises to 10%, which is under half, because compounding lets the first year's gain earn in the second. The limit is the small-move approximation every desk uses, and the algebra shows exactly when it stops being safe."))
        .explain(r"(1+x)^{1/2} + 1", "The conjugate",
            "The same expression with the middle sign flipped. Multiplying by it turns the difference on top into a difference of squares, which clears the fractional power.")
        .explain(r"(1+x) - 1", "The top after conjugating",
            "The square root squared, minus one squared: the fractional power is gone and what remains is x, ready to cancel.")
        .rule()
        .para(|p| p
            .text("Move three: divide by the dominant term. This is the one for infinity over infinity, and it is nothing but a change of units. In"))
        .display(r"\lim_{x \to \infty} \frac{3x^2 + 5x}{2x^2 - 7}")
        .explain(r"\lim_{x \to \infty} \frac{3x^2 + 5x}{2x^2 - 7}", "Move three's worked example",
            "Infinity over infinity, settled by dividing top and bottom by the dominant x squared. The answer is 3/2 — the ratio of the leading coefficients.")
        .explain(r"\frac{3x^2 + 5x}{2x^2 - 7}", "A ratio of two runaway quantities",
            "Top and bottom both grow without bound, so the form is infinity over infinity. Dividing both by the highest power present settles it.")
        .para(|p| p
            .text("both parts run away, so measure everything against the fastest-growing piece present, ")
            .math(r"x^2")
            .text(". Divide top and bottom by it: the top becomes ")
            .math("3 + 5/x")
            .text(", the bottom ")
            .math(r"2 - 7/x^2")
            .text(", and every term with an ")
            .math("x")
            .text(" underneath collapses to zero as ")
            .math("x")
            .text(" runs off. What is left is three over two. In the long run only the leading terms matter — which is why a fee that grows linearly is irrelevant against a book that grows quadratically, however large the fee looks today."))
        .explain(r"x^2", "x squared",
            "In the long-run example, the fastest-growing piece anywhere in the fraction — dividing through by it is a change of units that leaves the ratio unchanged and the answer visible. In the payoff section it returns as the rule whose slope the lesson's first broken formula was secretly computing.")
        .explain(r"3 + 5/x", "The top, rescaled",
            "After dividing by x squared. The 5/x fades to nothing as x runs off, leaving 3.")
        .explain(r"5/x", "Five over x",
            "The linear term measured against the dominant one. As x runs off it fades to nothing, which is why the 5x never mattered.")
        .explain(r"7/x^2", "Seven over x squared",
            "The constant term measured against the dominant one — fading even faster. In the long run only leading terms speak.")
        .explain(r"2 - 7/x^2", "The bottom, rescaled",
            "After dividing by x squared. The 7 over x squared fades even faster, leaving 2.")
        .rule()
        .para(|p| p
            .text("Move four: squeeze it. When a function is too wild to evaluate, trap it between two functions that are not, and if the trap closes on a single number the function has nowhere else to go. The oscillating troublemaker from earlier makes the point. On its own ")
            .math(r"\sin(1/x)")
            .text(" has no limit at zero. But multiply it by ")
            .math("x")
            .text(" and the product is pinned between two walls that are closing:"))
        .display(r"-|x| \le x \sin(1/x) \le |x|")
        .explain(r"x \sin(1/x)", "The oscillation, damped",
            "Still swinging infinitely fast, but between walls that are closing. The oscillation never stops; the room it has to oscillate in runs out.")
        .explain(r"-|x|", "The lower wall",
            "Minus the distance from zero. Below it the product cannot go, because the sine never drops below minus one. Written with absolute values so it holds on both sides of zero at once — multiplying an inequality by a negative x would flip it, and a one-sided sandwich could only ever license a one-sided conclusion.")
        .explain(r"|x|", "The upper wall",
            "The distance from zero. Above it the product cannot go, because the sine never rises above one. Both walls close on zero, and the function between them has no choice but to follow.")
        .para(|p| p
            .text("because the sine never leaves the range from minus one to one. The absolute values are doing real work: multiplying an inequality by a negative number reverses it, so below zero the walls would swap places — writing both as a distance from zero states the two cases at once, in a lesson that has spent a whole section insisting both sides must agree. Both walls head for zero, so the function trapped between them heads for zero too, even though it never stops oscillating on the way. A position being wound down behaves like this: the underlying keeps thrashing, but the size is cut on a schedule, and the profit and loss is trapped between plus and minus the size times the day's move. The walls close because the size does — and the analogy breaks exactly where the squeeze's hypothesis does: the walls only close if the day's move stays bounded, so a position wound down through a gap does not obey it."))
        .para(|p| p
            .text("That example is a toy. Here is the squeeze doing work nothing else can do. What is ")
            .math(r"\frac{\sin x}{x}")
            .text(" as ")
            .math("x")
            .text(" goes to zero? Both parts vanish — a 0/0 with nothing to factor and nothing to conjugate. Trap it instead. On the unit circle a small positive angle pins the sine below the arc and the arc below the tangent segment; divide through by the sine, turn everything over, and the walls appear:"))
        .display(r"\cos x \le \frac{\sin x}{x} \le 1")
        .explain(r"\cos x", "The lower wall here",
            "One at zero, and rising back toward one as the angle closes. The trapped function has nowhere to go but 1.")
        .explain(r"\frac{\sin x}{x}", "The sine of an angle, divided by the angle",
            "A 0/0 at zero that no factoring or conjugate opens — there is nothing to cancel. It is also the difference quotient of the sine at zero, so its limit of 1 is the sine's slope at the origin, and every trigonometric derivative descends from it. Note the circularity it escapes: you cannot get this limit by differentiating, because the differentiating depends on it. The squeeze is the only door in.")
        .para(|p| p
            .text("That argument ran on positive angles, and this lesson does not get to spend a whole section demanding both sides agree and then check only one. It survives the crossing for free: flipping the sign of ")
            .math("x")
            .text(" flips the sign of the sine with it, so the ratio is unchanged and the walls stand where they were. Both go to 1, so the trapped ratio does too: 0.998334 at 0.1, 0.999983 at 0.01."))
        .note("This is why angles are measured in radians. The arc on the unit circle equals x only because a radian is defined as the angle subtending an arc of one radius. Measure in degrees and the very same limit comes out at pi over 180, about 0.0174533 — and every trigonometric derivative would drag that factor around forever. Radians are not a fact of nature. They are a convention, chosen to make this number 1.")
        .rule()
        .para(|p| p
            .text("There is a fifth move, the one every calculus student eventually reaches for first: when top and bottom both vanish at the target, differentiate each separately and take the limit of the new fraction. On move one's example it is instant — the top of ")
            .math(r"(x^2-5x+6)/(x-2)")
            .text(" differentiates to ")
            .math("2x - 5")
            .text(", the bottom to 1, and at 2 that reads minus 1, matching the factoring exactly. That is L'Hôpital's rule, and this lesson has deliberately kept it for last: it spends derivatives, and a derivative is itself a 0/0 limit, so using the rule to build the derivative would be a circle. The four algebraic moves are the ones that work before you own a derivative — the only position from which the derivative can honestly be built. One warning rides with it: the rule is licensed by 0/0 and infinity over infinity and by nothing else — point it at a fraction that is not indeterminate and it answers a different question, confidently."))
        .explain(r"2x - 5", "The top, differentiated",
            "The derivative of x squared minus 5x plus 6 — the payoff section builds what that means. At 2 it reads minus 1, which is move one's answer arriving by a faster road.")
        .explain("2 x", "Twice the input",
            "The squared term differentiated — the general fact that the squaring rule grows at twice its input, which the payoff section derives and the practice section confirms.")
        .note("Try each move in order when a limit returns 0/0: can it be factored, conjugated, rescaled, or trapped? One of the four almost always opens it — and once derivatives are in hand, the fifth move opens most of them at once.")
}

fn pointing_the_arrow_outward(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Pointing the arrow outward")
        .para(|p| p
            .text("So far the arrow has aimed at a finite target. Aim it at infinity instead and nothing about the machinery changes — only the reading of the arrow does. Where ")
            .math(r"x \to a")
            .text(" meant \"x gets arbitrarily close to a\", the symbol ")
            .math(r"x \to \infty")
            .text(" means \"x grows past every bound\". Infinity is not a place being approached; it is shorthand for a direction of travel with no destination."))
        .explain(r"x \to a", "x approaches a",
            "The input is driven arbitrarily close to the number a, without ever being set equal to it.")
        .explain(r"x \to \infty", "x grows without bound",
            "Not an approach to a place — a direction. Infinity is not a number and cannot be substituted; the notation is shorthand for \"however large a bound you name, x eventually passes it\".")
        .para(|p| p
            .text("The workhorse example is the one that makes all the others go:"))
        .display(r"\lim_{x \to \infty} \frac{1}{x} = 0")
        .explain(r"\lim_{x \to \infty} \frac{1}{x}", "One over x, out at infinity",
            "Zero. Name any tolerance and every x past its reciprocal keeps 1/x inside — the tolerance game, played out at infinity.")
        .para(|p| p
            .text("A limit at infinity that comes out finite is a horizontal asymptote: a level the function approaches but, generally, never reaches. Finance is built on them. A perpetuity — a stream paying ")
            .math("C")
            .text(" a year forever — is priced by taking the price of an ")
            .math("n")
            .text("-year annuity and letting ")
            .math("n")
            .text(" run off:"))
        .display(r"\lim_{n \to \infty} C \frac{1 - (1+r)^{-n}}{r} = \frac{C}{r}")
        .explain(r"C \frac{1 - (1+r)^{-n}}{r}", "The price of an n-year annuity",
            "What a stream of C a year for n years is worth today at discount rate r. The whole n-dependence sits in the term being subtracted.")
        .explain(r"\lim_{n \to \infty} C \frac{1 - (1+r)^{-n}}{r}", "The annuity price, sent to forever",
            "The n-year price with n driven past every bound. The discount factor dies geometrically, the subtraction vanishes, and C over r is all that survives.")
        .explain(r"\frac{C}{r}", "The perpetuity price",
            "The annuity price with n sent to infinity. At 5% a payment of $5 a year forever is worth $100 — the far-off payments contribute so little that the total stops moving.")
        .para(|p| p
            .text("The reason it converges is visible in the formula: the discount factor ")
            .math(r"(1+r)^{-n}")
            .text(" shrinks geometrically, takes the whole subtracted term with it, and only ")
            .math("C")
            .text(" over ")
            .math("r")
            .text(" survives. That an infinite stream of payments has a finite price is not an accounting convention; it is a limit, and it converges because distant money is worth almost nothing."))
        .explain(r"(1+r)^{-n}", "The discount factor n years out",
            "What a dollar n years away is worth today. With r positive it shrinks geometrically toward zero, which is exactly why the infinite sum stays finite.")
        .rule()
        .para(|p| p
            .text("The most famous limit in finance hides inside everyday arithmetic. Invest $1 at an annual rate ")
            .math("r")
            .text(", compounded ")
            .math("n")
            .text(" times a year. After a year you hold ")
            .math(r"(1 + r/n)^n")
            .text(": each compounding chops the rate into smaller slices and applies more of them, and the two effects nearly cancel — but not quite, and the residue is worth money."))
        .explain(r"(1 + r/n)^n", "One dollar, compounded n times a year",
            "Split the annual rate into n slices and apply each one to whatever the previous slices produced. More frequent compounding always beats less, but by an ever-shrinking margin.")
        .para(|p| p
            .text("At 5%, compounding once gives 1.05. Twelve times gives 1.051162. Every day gives 1.051267. The gains are collapsing, and they collapse onto a number:"))
        .display(r"\lim_{n \to \infty} (1 + r/n)^n = e^r")
        .explain(r"\lim_{n \to \infty} (1 + r/n)^n", "Compounding, taken to the limit",
            "What a dollar grows to in a year if the interest is applied continuously rather than at intervals. The ceiling that ever-finer compounding approaches and never passes. That it converges at all is completeness again: the value climbs with every extra compounding and never passes the ceiling, and in the reals a climb with a ceiling always has a destination.")
        .explain(r"\lim_{n \to \infty}", "The limit as n runs off",
            "The same machine as x approaching a, aimed at a direction instead of a place: however large a bound you name, the count n eventually passes it.")
        .explain(r"e^r", "The continuous growth factor",
            "Euler's number raised to the rate. At r = 0.05 it is 1.05127110 — the value daily compounding is already within a millionth of.")
        .para(|p| p
            .text("This is where continuous compounding comes from. It is not an approximation to daily compounding and not a modelling convenience; it is a limit — the exact ceiling that finer and finer compounding approaches. And the number ")
            .math("e")
            .text(" is not chosen; it falls out. Set ")
            .math("r")
            .text(" to 1 and the same limit defines ")
            .math("e")
            .text(" itself, at 2.71828 and onward. The exponents lesson owns the economics of this ladder — why halving the rate slice and doubling the steps so nearly cancels, and what the leftover is worth — so this lesson adds only the part that is purely about limits."))
        .explain_char('e', "Euler's number",
            "About 2.71828. Not chosen but produced: it is the value of the compounding limit at a rate of 1, and it is why continuous compounding wears exponentials.")
        .para(|p| p
            .text("Look at which indeterminate form it is. The base heads for 1 and the exponent for infinity: one to the infinity, a member of the same family as 0/0 that the four-moves section did not list, and one whose two naive answers — 1 and infinity — are both wrong. What settles it is a 0/0 in disguise: write the compound as an exponential of ")
            .math("n")
            .text(" times the logarithm of ")
            .math("1 + r/n")
            .text(", substitute ")
            .math("u = r/n")
            .text(", and everything turns on one limit:"))
        .display(r"\lim_{u \to 0} \frac{\ln(1+u)}{u} = 1")
        .explain(r"\lim_{u \to 0} \frac{\ln(1+u)}{u}", "The logarithm's difference quotient at 1",
            "The top is ln(1+u) minus ln(1), and ln(1) is zero — so this is rise over run for the logarithm at 1, and its limit is that curve's slope there. That the answer is exactly 1 is what makes e the natural base: run the same quotient in base 2 and it converges to a conversion factor of about 0.6931 instead.")
        .explain(r"r/n", "One slice of the annual rate",
            "The rate divided into n pieces. It heads for zero as n grows, dragging the base toward 1 — half of what makes the form indeterminate. The exponent running off is the other half.")
        .explain(r"u = r/n", "The substitution",
            "One name for the shrinking slice. Sending n to infinity is the same journey as sending u to zero.")
        .para(|p| p
            .text("It reads 0.99503 at ")
            .math("u")
            .text(" = 0.01 and 0.99950 at 0.001. And that 1 is what closes the argument, because the exponent is that quotient in disguise: ")
            .math("n")
            .text(" times the logarithm of ")
            .math("1 + r/n")
            .text(" is exactly ")
            .math("r")
            .text(" times it. So the exponent heads for ")
            .math("r")
            .text(" times 1, and the compound heads for ")
            .math("e^r")
            .text(" — the ceiling was never guessed from the table of 1.05, 1.051162, 1.051267; it was derived. Hold on to the quotient's shape — a vanishing change up top, divided by the vanishing change that caused it; one section from now that shape gets a name and becomes the whole subject. It is also how the exponents lesson's recorded debt — that the exponential is its own rate of change, \"the entire content of the word natural\" — finally gets paid. The practice section settles it."))
        .rule()
        .para(|p| p
            .text("One more direction to name. When values grow without bound as the input approaches a finite point, the convention is to write the limit as infinity — as ")
            .math("x")
            .text(" nears zero from above, ")
            .math("1/x")
            .text(" exceeds every bound, and that is written as a limit equal to infinity."))
        .display(r"\lim_{x \to 0^{+}} 1/x = \infty")
        .explain(r"\lim_{x \to 0^{+}} 1/x", "One over x, approaching zero from above",
            "Written as infinity, which is a statement of failure dressed as an answer: it says no finite limit exists, and says how it fails.")
        .explain_char('∞', "The infinity symbol",
            "Not a number. In a limit it records a direction of unbounded growth, and an equals sign in front of it is a description of how the limit fails rather than a value it takes.")
        .note("An equation ending in infinity is describing a failure, not reporting a value. It says the limit does not exist, and adds the useful detail that it fails by growing rather than by jumping or oscillating.")
}

fn every_derivative_is_a_limit(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("The payoff: every derivative is a limit")
        .para(|p| p
            .text("Return to the fund, now armed. The average growth over a window of length ")
            .math("h")
            .text(" was a perfectly ordinary fraction; the instantaneous growth is what that fraction is heading for as the window closes. Which is a limit, and now there is a notation for it:"))
        .display(r"V'(t) = \lim_{h \to 0} \frac{V(t + h) - V(t)}{h}")
        .explain(r"V'(t)", "The derivative of V at time t",
            "The fund's instantaneous growth rate, in millions per year. The prime mark is shorthand for the entire limit on the right.")
        .explain(r"\lim_{h \to 0} \frac{V(t + h) - V(t)}{h}", "The difference quotient, taken to the limit",
            "Average growth over a window of length h, with the window closed to nothing. Undefined at h = 0 and perfectly well-defined as a limit — the removable discontinuity, repaired.")
        .para(|p| p
            .text("That fraction has a name, the difference quotient, and note what it is: a 0/0 by construction. Not by accident, not for awkward functions — every derivative of every function is a 0/0, because the top and bottom are both built to vanish together. The derivative is the limit concept's reason for existing, and the fund's question is answered by move one, factor and cancel."))
        .para(|p| p
            .text("Work it. With ")
            .math("V(t) = 100 + 8t - t^2")
            .text(", expanding ")
            .math("V(t+h)")
            .text(" gives ")
            .math(r"100 + 8t + 8h - t^2 - 2th - h^2")
            .text(". Subtract ")
            .math("V(t)")
            .text(" and the 100, the ")
            .math("8t")
            .text(" and the ")
            .math(r"t^2")
            .text(" all cancel, leaving ")
            .math(r"8h - 2th - h^2")
            .text(" — every surviving term carrying a factor of ")
            .math("h")
            .text(". That is the cancellation the whole lesson has been building toward:"))
        .display(r"\frac{V(t+h) - V(t)}{h} = 8 - 2t - h")
        .explain(r"8 - 2t - h", "The difference quotient, after cancelling h",
            "Legal because h is never zero on the approach. What remains is an ordinary polynomial in h — continuous, so its limit is its value.")
        .explain(r"V(t+h)", "The fund's value one window later",
            "The same rule evaluated at t + h. Expanding it is the only algebra the derivative of this fund requires.")
        .explain(r"8h - 2th - h^2", "What survives the subtraction",
            "Every term carries a factor of h, which is why the h underneath can be cancelled and the 0/0 opened.")
        .explain(r"8t", "The drift term",
            "Eight million a year. It appears identically in V(t+h) and V(t), so it cancels in the subtraction and contributes nothing to the difference.")
        .explain(r"100 + 8t + 8h - t^2 - 2th - h^2", "V(t+h), expanded",
            "The fund's rule with t + h substituted and the square multiplied out, ready for the subtraction to knock out everything that does not carry an h.")
        .explain(r"8 h", "The drift across the window",
            "Eight million a year, acting for a window of length h. Only V(t+h) contains it, so it survives the subtraction.")
        .explain(r"2 t h", "The cross term",
            "The drag linking the fund's age t to the window h. Divided by h it leaves the minus 2t in the growth rate.")
        .explain(r"h^2", "The window squared",
            "The drag's own-window piece. Even after dividing by h one factor of h remains, so closing the window kills it — the one term the limit removes.")
        .explain(r"2 t", "Twice the time",
            "The drag's mark on the growth rate: two million a year of slowdown for every year the fund has run.")
        .para(|p| p
            .text("Now the limit is trivial, because ")
            .math("8 - 2t - h")
            .text(" is a polynomial in ")
            .math("h")
            .text(" and polynomials are continuous: substitute ")
            .math("h")
            .text(" = 0. The ")
            .math("h")
            .text(" vanishes and what remains is a formula for the growth rate at every instant of the fund's life:"))
        .display(r"V'(t) = 8 - 2t")
        .explain(r"8 - 2t", "The fund's growth rate at any time t",
            "Eight million a year at launch, falling by two million for every year that passes. At t = 2 it is 4, which is the number the shrinking windows were marching on. It is also the fund's local exchange rate in the tolerance game — and at the peak, where it is zero, the game turns free: the output barely moves however the input jiggles, so almost any delta wins.")
        .para(|p| p
            .text("At ")
            .math("t")
            .text(" = 2 that is 8 minus 4: exactly the 4 the shrinking windows marched on in the opening section, now derived rather than guessed. And the formula says more than the guess could. It goes to zero at ")
            .math("t")
            .text(" = 4 — the fund peaks at $116 million in its fourth year — and turns negative after, the drag having overtaken the drift. One limit, and the fund's whole future is legible."))
        .para(|p| p
            .text("That formula also settles the tolerance game's debt. The game left slope as the exchange rate between the two tolerances — ")
            .math(r"\varepsilon / 3")
            .text(" for the line of slope 3, the tighter ")
            .math(r"\varepsilon / 5")
            .text(" for the steeper one — and warned that a curve's rate moves. Here is the rate moving: the fund's exchange rate is 8 at launch, 4 at two years, 0 at the peak, minus 4 at six. The derivative is the local exchange rate — the number that says how hard an input error is punished right here — and that is what the hint was pointing at."))
        .rule()
        .para(|p| p
            .text("The picture makes the same argument with no algebra. Pick the point on the curve at ")
            .math("t")
            .text(" = 2, pick a second point a window away, and draw the straight line through both. That line is a secant, and its slope is precisely the difference quotient — rise over run, value change over time change. Now slide the second point in."))
        .figure(Figure::new(SECANT_SVG,
            "The fund's value curve with the point at t = 2 fixed and the second point sliding in. The secant through t = 4 has slope 2; through t = 3, slope 3; through t = 2.5, slope 3.5. Each is the difference quotient for that window, and each equals 4 minus h exactly. As the second point closes on the first the secants pivot, and the line they pivot toward — slope 4 — is the tangent. The tangent is the one line the secants never quite become, which is exactly the relationship a limit describes."))
        .para(|p| p
            .text("Every secant needs two distinct points; a line through one point is not defined. So the tangent is a line the construction can never actually produce — and yet the secants aim at it unmistakably. That is the limit's whole job, restated geometrically. The derivative is the slope of the tangent, and the tangent is the limit of the secants."))
        .plot(Plot::new(0.0..=5.0)
            .curve("fund value V(t)", "100 + 8*x - x^2")
            .curve("secant from t = 2 across a window h", "112 + (4 - h)*(x - 2)")
            .curve("tangent at t = 2", "112 + 4*(x - 2)")
            .param("h", 0.05..=3.0, 2.5)
            .vline(2.0)
            .x_label("years since launch")
            .y_label("fund value, millions of dollars")
            .height(300.0)
            .caption("The same three lines as the figure above, in the same three colours, now under your hand. Drag h down and watch the blue secant pivot onto the amber tangent it can never reach. Its slope is 4 minus h, so the gap to the tangent's slope of 4 is exactly h — the error is the window, which is why closing the window is the same act as taking the limit. Drag h up instead and the secant sags below the curve: a wide window reports the average, and the average understates a fund that is still growing."))
        .rule()
        .para(|p| p
            .text("One change of clothes, and then the lesson can open its envelope. Nothing obliges you to name the difference quotient's second input by the size of the window: call it ")
            .math("x")
            .text(" instead of ")
            .math("t + h")
            .text(", and closing the window means sending ")
            .math("x")
            .text(" to ")
            .math("t")
            .text(". The same limit reads:"))
        .display(r"f'(a) = \lim_{x \to a} \frac{f(x) - f(a)}{x - a}")
        .explain(r"f'(a)", "The derivative of f at a",
            "The same object V'(t) names for the fund — the limit of the difference quotient — at one chosen point a of any rule f.")
        .explain(r"\lim_{x \to a} \frac{f(x) - f(a)}{x - a}", "The derivative, with the second input named directly",
            "The same limit as the h-form: put x = a + h, and h going to 0 becomes x going to a. This is the form in which this lesson's worked 0/0 examples are visibly derivatives.")
        .explain(r"\frac{f(x) - f(a)}{x - a}", "Rise over run between two points of f",
            "The second point named x rather than a + h. Undefined at x = a — which is the only place its limit is ever taken.")
        .para(|p| p
            .text("Now look at that fraction. Then look back at the first broken formula in this lesson."))
        .display(r"\frac{x^2 - 9}{x - 3}")
        .para(|p| p
            .text("Nine is three squared. So the top is the squaring rule at ")
            .math("x")
            .text(" minus the squaring rule at 3, the bottom is ")
            .math("x - 3")
            .text(", and the whole thing is the difference quotient of ")
            .math("x^2")
            .text(" at 3, written in exactly the form above. Its limit was 6. It was never a curiosity about a hole. It was a derivative, and you computed it before you had a name for one."))
        .para(|p| p
            .text("Nor was it alone. Move one's example, ")
            .math(r"(x^2-5x+6)/(x-2)")
            .text(", is the same form at 2 — the subtraction on top is invisible only because that polynomial's value at 2 happens to be zero — and its answer of minus 1 is that parabola's growth rate at 2. The conjugate example, ")
            .math(r"\frac{(1+x)^{1/2} - 1}{x}")
            .text(", is the h-form with the window called ")
            .math("x")
            .text(": the derivative of the square root at 1, climbing at one half. Even the sine snuck in — move four's ")
            .math(r"\frac{\sin x}{x}")
            .text(" is the sine's difference quotient at zero. Four worked 0/0s, four disguises, every one a derivative. You have been differentiating for three sections without being told. Move three is the one that got away, and its alibi is worth hearing: it is a limit at infinity, and a derivative needs a point to stand at. There is no \"there\" out at infinity to take a slope at — which is the cleanest statement of what separates the two kinds of arrow this lesson uses."))
        .explain(r"(x^2-5x+6)", "The parabola on top",
            "Zero at 2 and at 3. Vanishing at the target together with the bottom is what made it a 0/0 — and what makes it a difference quotient in disguise, since the subtracted value at 2 is zero and leaves no trace.")
        .para(|p| p
            .text("So take the first one as a prediction rather than a fact. The squaring rule at 3 grows at 6 — twice 3. If that is a pattern and not a coincidence, the squaring rule at 4 must grow at 8, and the first practice question below happens to be exactly that limit. Check it there. If the prediction holds, what you are holding is not a number attached to a point but a new function — ")
            .math("x^2")
            .text(" grows at twice ")
            .math("x")
            .text(", everywhere — assembled by taking one limit at every point at once. That is what a derivative is, and ")
            .math("V'(t) = 8 - 2t")
            .text(" is the fund's."))
        .para(|p| p
            .text("One quieter consequence, retrospective. The hole in this lesson's very first picture was a derivative too: that graph plots the quotient itself, so its height is a secant slope, and the punched-out point at 3 that both approaches were aiming at is the tangent slope — 6. Every removable discontinuity in this lesson was a difference quotient waiting to be read."))
        .rule()
        .para(|p| p
            .text("If the derivative is a limit, then everything that can go wrong with a limit can go wrong with a derivative. Take a call option at expiry, struck at $100: it pays nothing below the strike and the excess above, so the payoff is perfectly continuous at 100 — left limit, right limit and value all zero. Its difference quotient is another matter. For a positive window the payoff gains exactly ")
            .math("h")
            .text(", so the quotient is 1; for a negative window it gains nothing, so the quotient is 0. Two one-sided limits, and they disagree — which by the rule of the earlier section means no limit, and therefore no derivative. The option has no delta at its own strike at expiry. Not a delta that is hard to compute — one that is not there. This is the announcement's jump one level up: it has moved out of the value and into the slope. A corner, not a jump — and the absolute-value function is the same shape at zero, with one-sided quotients of minus one and plus one."))
        .figure(Figure::new(CORNER_SVG,
            "A call struck at $100, at expiry. On the left, the payoff itself: continuous at the strike, with left limit, right limit and value all zero, so nothing the earlier sections tested would object. On the right, its difference quotient — 0 for every negative h, 1 for every positive h. The one-sided limits disagree, so the derivative at the strike does not exist. Read the circles across the two panels: the single filled dot on the left says the value at the strike exists, and the two open ones on the right say the slope there does not."))
        .note("Differentiable implies continuous — if the quotient is to settle, its top must vanish with h, and a vanishing top is continuity. The converse buys nothing: a jump kills the limit of the value, while a corner leaves the value's limit intact and kills the slope's. Nor are corners exotica: a stop-loss kinks the profit and loss at the stop, a tiered fee schedule kinks the bill at each breakpoint, and the linear-algebra lesson meets this very payoff as the clerk that replaces negative cells with zero and finds it will not add — one corner, breaking linearity there and differentiability here. It is also why traders speak of gamma exploding into expiry: as the smooth option price collapses onto the kinked payoff, the delta near the strike swings from 0 to 1 across an ever-narrower band, and the rate of that swing grows without bound.")
        .rule()
        .para(|p| p
            .text("One coda, because the opening table is the first thing most people reach for. It was exact arithmetic: 3, then 3.5, 3.9, 3.99, 3.999, marching on 4. Ask a computer to continue the march and it obeys — for a while. At a window of ten to the minus 7 it reports 3.9999999, the closest it will ever come. One step further and it reports 4.0000003, which is not merely inaccurate but impossible: the exact quotient is 4 minus h, strictly below 4 for every window there is, so every digit of that answer is noise. By ten to the minus 12 it says 4.0075; at ten to the minus 15 it returns exactly 0, forever after. The culprit is the subtraction on top: V(2+h) and V(2) agree in ever more leading digits, subtracting nearly equal numbers destroys exactly the digits the answer depends on, and eventually the fund's real gain — billionths of a dollar across nanoseconds — is smaller than the smallest change a stored number the size of 112 million can record. The top rounds to exactly zero."))
        .note("Shrinking h has two competing effects — it closes the gap to the instantaneous rate, and it magnifies the rounding error in the subtraction. The best window sits near the square root of the machine's precision scaled by the size of the numbers subtracted, and no further shrinking improves on it. A machine cannot take a limit: it gets close, then worse, then nothing. Sending h to zero is a move only algebra can make — which is why the rest of calculus is a body of symbolic rules, not a numerical recipe.")
        .rule()
        .para(|p| p
            .text("It is worth knowing that this was contested, bitterly, and that the objection was a good one. Newton and Leibniz got calculus working in the 1600s using quantities that were treated as non-zero when it came time to divide by them and as zero when it came time to discard them. In 1734 Bishop George Berkeley attacked exactly that step: quantities that are neither finite, nor infinitely small, nor nothing — \"may we not call them the ghosts of departed quantities?\""))
        .para(|p| p
            .text("He was right, and for a century nobody had a clean answer. The answer, when it came, was the limit. Nothing is ever divided by zero and nothing is ever discarded: ")
            .math("h")
            .text(" stays honestly non-zero throughout the cancellation, and then the limit — with the tolerance game underneath it — asks a different question entirely, about where the surviving expression heads. Berkeley's ghost was exorcised not by better arithmetic but by a better question. Which is why this lesson spent its first half on what a limit asks, and not on how to compute one."))
        .para(|p| p
            .text("One relic of the losing side is still in daily use. Leibniz wrote the derivative as ")
            .math(r"dV/dt")
            .text(" and meant it literally — ")
            .math("dV")
            .text(" and ")
            .math("dt")
            .text(" were infinitely small quantities and their quotient an honest fraction, precisely the ghosts Berkeley was pointing at. The reasoning was thrown out; the handwriting was kept, because the chain rule and change of variables look like cancelling fractions when written this way — a theorem proved each time, not the fraction doing what fractions do. And that is the doorway out. The next step is a handful of rules that turn differentiation into arithmetic: a power rule that is this section's cancellation done once in general, and sum, product, quotient and chain rules, each provable by running the limit laws from earlier over difference quotients. The two stray limits this lesson has already banked — the logarithm's quotient at 1, and the sine's at 0 — become the derivatives of the exponential and the sine. All of it is this one limit, taken at every point at once."))
        .explain(r"dV/dt", "Leibniz notation for the derivative",
            "Read as one symbol, not a fraction: the limit of the difference quotient. It looks like a ratio because Leibniz meant it as one, and the appearance outlived the reasoning.")
        .explain("dV", "An infinitely small change in the fund's value",
            "Leibniz's original quantity, and one of Berkeley's ghosts. In modern usage the d carries no independent meaning inside dV/dt.")
        .explain("dt", "An infinitely small change in time",
            "The other half of Leibniz's ratio. Not a number, and not zero — which was exactly the objection.")
}

fn practice(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Practice")
        .para(|p| p
            .text("Seven questions, all built from this lesson's own examples. Commit to an answer before reading past each rule — the guess is where the learning is."))
        .para(|p| p
            .text("First, one worked in full, so the moves stay visible. Evaluate"))
        .display(r"\lim_{x \to 4} \frac{x^2 - 16}{x - 4}")
        .explain(r"\frac{x^2 - 16}{x - 4}", "A 0/0 at x = 4",
            "The same shape as the lesson's opening broken formula, moved to 4. Top and bottom both vanish there.")
        .para(|p| p
            .text("Substituting gives 0/0, so that is an instruction, not an answer. Move one: the top is a difference of squares, ")
            .math(r"(x-4)(x+4)")
            .text(". The ")
            .math(r"(x-4)")
            .text(" cancels — legal because ")
            .math("x")
            .text(" is never 4 on the approach — leaving ")
            .math("x + 4")
            .text(", which at 4 is 8. Sanity check from the side: at ")
            .math("x")
            .text(" = 3.99 the original formula gives 7.99, and at 4.01 it gives 8.01. And 8 is twice 4 — the prediction the payoff section made, confirmed at a second point: the squaring rule grows at twice its input, wherever you stand on it."))
        .explain(r"(x-4)(x+4)", "The top, factored",
            "A difference of squares. One factor matches the bottom and cancels; the other survives to give the answer.")
        .explain(r"\lim_{x \to 4} \frac{x^2 - 16}{x - 4}", "The worked practice limit",
            "A 0/0 at 4 — the opening broken formula, relocated. Factor, cancel the vanishing factor, and the survivor delivers 8.")
        .explain(r"(x+4)", "The surviving factor",
            "Continuous at 4, so its limit there is simply its value: 8.")
        .explain(r"(x-4)", "The vanishing factor",
            "Zero exactly at 4 — never zero anywhere the limit actually looks.")
        .explain(r"x + 4", "What the cancellation leaves",
            "A line, continuous, so its limit at 4 is its value there: 8.")
        .rule()
        .para(|p| p
            .text("Now you. A table, of the kind this lesson opened with. Here are five values of ")
            .math(r"\sin(\pi/x)")
            .text(", walking in toward zero: at ")
            .math("x")
            .text(" = 1 it is 0; at one half, 0; at one third, 0; at one tenth, 0; at one hundredth, 0. Five samples, each closer than the last, every one exactly zero — no rounding anywhere. Is the limit at 0 equal to 0?"))
        .note("The evidence is unanimous and the arithmetic is exact. Decide anyway.")
        .para(|p| p
            .text("No. There is no limit at all, and the table was rigged — by the sampling, not by the function. Every ")
            .math("x")
            .text(" in it has the form one over a whole number, so ")
            .math(r"\pi/x")
            .text(" is a whole multiple of ")
            .math(r"\pi")
            .text(", and the sine of a whole multiple of ")
            .math(r"\pi")
            .text(" is exactly zero by construction. The table sampled the function's zeros and then reported that the function is zero. Step off that lattice and the values are not small: at ")
            .math("x")
            .text(" = 2/5 the sine is exactly 1, again at 2/9 and 2/13, with minus 1 arriving just as often — the oscillating failure from earlier, wearing a disguise the table could not see. A table shows only that a limit has not yet failed at the points you happened to choose. The opening march of 3, 3.5, 3.9, 3.99 was honest evidence, and still only evidence: the reason the answer is 4 is the cancellation, and the reason the cancellation is trustworthy is the tolerance game."))
        .explain(r"\sin(\pi/x)", "The oscillating troublemaker, rescaled",
            "Rescaled so the obvious sample points all return exactly zero: at x = 1, 1/2, 1/10, 1/100 every reading is 0, yet there is no limit — the function also takes the value 1 at 2/5, 2/9, 2/401 and infinitely many smaller inputs.")
        .explain(r"\pi/x", "The angle handed to the sine",
            "At every sampled input it lands on a whole multiple of pi, which is exactly how the table was rigged.")
        .rule()
        .para(|p| p
            .text("The tolerance game, on a new line. Let ")
            .math("f(x) = 5x - 3")
            .text(", with ")
            .math("a")
            .text(" = 1 and a claimed limit of 2. The sceptic demands the output stay within 0.1. What ")
            .math(r"\delta")
            .text(" answers, and what formula answers every possible demand?"))
        .explain(r"f(x) = 5x - 3", "A steeper test line",
            "Slope 5 instead of 3, so it magnifies input error five times over — and demands a correspondingly tighter reply.")
        .note("Answer with a number first, then with a formula. The formula is the part that proves anything.")
        .para(|p| p
            .text("The output is within 0.1 of 2 when ")
            .math("5x")
            .text(" is within 0.1 of 5, which is when ")
            .math("x")
            .text(" is within 0.02 of 1. So 0.02 answers this challenge, and dividing by the slope answers all of them: ")
            .math(r"\delta = \varepsilon / 5")
            .text(". Note it is tighter than the earlier line's ")
            .math(r"\varepsilon / 3")
            .text(" for the same demand, because a steeper function magnifies input error more — the exchange rate is the slope."))
        .explain(r"5x", "Five times the input",
            "The moving part of the output. Holding it within 0.1 of 5 is the same demand as holding x within 0.02 of 1.")
        .explain(r"\varepsilon / 5", "The reply for the steeper line",
            "Divide the demanded tolerance by the slope 5. A steeper line needs a tighter window to keep the same promise.")
        .explain(r"\varepsilon / 3", "The reply for the shallower line",
            "The earlier answer, for slope 3. Comparing the two shows the slope is the exchange rate between the two tolerances.")
        .rule()
        .para(|p| p
            .text("Hold a call struck at $100 to expiry. Its payoff is nothing at all if the stock finishes below 100, and a dollar for every dollar above. Unlike the earnings announcement, this payoff does not jump anywhere — it arrives at zero from both sides of the strike. Does it have a delta at exactly $100?"))
        .note("Continuous, with no gap anywhere — decide whether that is enough. And if you find yourself reaching for one half, ask what would supply it.")
        .para(|p| p
            .text("No. A delta is a derivative, so it is the limit of a difference quotient, and the two sides disagree:"))
        .display(r"\lim_{h \to 0^{-}} \frac{P(100+h) - P(100)}{h} \quad \lim_{h \to 0^{+}} \frac{P(100+h) - P(100)}{h}")
        .explain(r"\lim_{h \to 0^{-}} \frac{P(100+h) - P(100)}{h} \quad \lim_{h \to 0^{+}} \frac{P(100+h) - P(100)}{h}", "The two one-sided deltas, side by side",
            "The payoff's difference quotient approached from below the strike and from above. They read 0 and 1 — and they disagree, so there is no delta at the strike.")
        .explain(r"\lim_{h \to 0^{-}} \frac{P(100+h) - P(100)}{h}", "The delta approached from below",
            "Zero, and zero for every window, because the payoff is flat on that side of the strike. A one-sided derivative.")
        .explain(r"\lim_{h \to 0^{+}} \frac{P(100+h) - P(100)}{h}", "The delta approached from above",
            "One, and one for every window, because the payoff rises a dollar per dollar on that side. It disagrees with the left, so there is no delta at all.")
        .para(|p| p
            .text("Above the strike the payoff gains a dollar per dollar, so the quotient is 1 for every window; below, it gains nothing, so the quotient is 0 for every window. By exactly the rule that killed the limit at the announcement, the derivative does not exist. The tempting answer is one half, on the grounds that the truth ought to sit between the sides — but nothing supplies that average. The secants from the left approach one line, the secants from the right another, and a limit that disagrees with itself is not a limit at a compromise value; it is no limit. And note which item this is: the opening section's list of instantaneous rates named an option's delta alongside a spot yield and a portfolio's exposure, and it is the one on that list a real contract can genuinely fail to have — not quoted from a formula that collapses, but absent."))
        .rule()
        .para(|p| p
            .text("The fund again, later in its life. Using ")
            .math("V'(t) = 8 - 2t")
            .text(", how fast is it growing at six years, and what does the sign mean?"))
        .note("One substitution — but say what the answer means before you read on.")
        .para(|p| p
            .text("8 minus 12 is minus 4: the fund is shrinking by $4 million a year. The rate passed through zero at four years, which was the peak at $116 million, and by six years the drag has fully overtaken the drift. Note that the fund is still worth more than it launched at — 100 plus 48 minus 36 is $112 million — so a positive total return and a negative instantaneous rate coexist happily. They are answers to different questions, which has been the theme throughout."))
        .rule()
        .para(|p| p
            .text("Out at the base of the exponential. Factoring cracked the fund because a polynomial's difference quotient is another polynomial. An exponential has nothing to factor — so take its derivative at zero numerically instead. For the doubling rule, the difference quotient at 0 is ")
            .math(r"\frac{2^h - 1}{h}")
            .text(". Evaluate it at ")
            .math("h")
            .text(" = 0.1, then 0.01, then 0.001; then run the same three divisions with ")
            .math("e")
            .text(" in place of 2. What is base 2 marching on, what is ")
            .math("e")
            .text(" marching on, and what does the difference say about why calculus uses the base it does?"))
        .note("Three divisions for each base. Guess what e will do before you compute it.")
        .para(|p| p
            .text("Base 2 reads 0.717735, then 0.695555, then 0.693387: it is marching on 0.693147, the natural logarithm of 2. Base ")
            .math("e")
            .text(" reads 1.051709, then 1.005017, then 1.000500 — marching on exactly 1:"))
        .display(r"\lim_{h \to 0} \frac{2^h - 1}{h} \quad \lim_{h \to 0} \frac{e^h - 1}{h}")
        .explain(r"\lim_{h \to 0} \frac{2^h - 1}{h} \quad \lim_{h \to 0} \frac{e^h - 1}{h}", "The two base-derivatives, side by side",
            "The same difference quotient at the origin, in base 2 and in base e. The first converges to ln 2, about 0.6931; the second to exactly 1 — and that 1 is what the word natural means.")
        .explain(r"\lim_{h \to 0} \frac{2^h - 1}{h}", "The derivative of the doubling rule, at zero",
            "Rise over run for 2 to the x at the origin. It converges to 0.6931 — ln 2 — so the doubling rule's slope is proportional to itself, with an inconvenient constant attached.")
        .explain(r"\frac{2^h - 1}{h}", "The doubling rule's difference quotient at 0",
            "2 to the h is the value one window after the origin; 1 is the value at the origin. A 0/0 with nothing to factor — the numerical march is the honest first attack.")
        .explain(r"\lim_{h \to 0} \frac{e^h - 1}{h}", "The derivative of e to the x, at zero",
            "The same quotient in base e converges to exactly 1 — the defining property of e, and the claim the exponents lesson recorded as the one thing it could not prove.")
        .para(|p| p
            .text("That single 1 is the whole answer: every exponential's derivative is itself times this limit for its base, so only one base makes the constant exactly 1. ")
            .math("e")
            .text(" was not chosen and then found to have a tidy derivative; the limit was set to 1, and ")
            .math("e")
            .text(" is the base that answers — base 2 pays 0.6931 for its familiarity, base 10 pays 2.3026. And with that, the debt to the exponents lesson is paid: the exponential is its own rate of change, which is the entire content of the word natural."))
        .rule()
        .para(|p| p
            .text("Last, the misconception this lesson was written against. A colleague says: \"the limit as ")
            .math("x")
            .text(" approaches ")
            .math("a")
            .text(" is just ")
            .math("f(a)")
            .text(" — that is what approaching means.\" Where does that go wrong, and why does it so often go right?"))
        .note("Two answers needed: the counterexample, and the reason the belief survives.")
        .para(|p| p
            .text("It goes wrong wherever the function is discontinuous, which is wherever limits are actually needed. The broken formula has limit 6 at 3 and no value there at all. The announcement has a value and no limit. And the difference quotient — the reason calculus exists — is undefined at precisely the point its limit is taken."))
        .para(|p| p
            .text("The belief survives because the equation ")
            .math(r"\lim_{x \to a} f(x) = f(a)")
            .text(" is true so often: it is the definition of continuity, and the everyday functions are continuous almost everywhere. So substitution works nearly always, and the times it fails are exactly the times someone bothered to ask. Getting this backwards — treating continuity as what a limit means rather than as a lucky special case — is the single error that makes derivatives incomprehensible, because a derivative is a limit taken at a point where substitution is guaranteed to fail."))
        .para(|p| p
            .text("The whole lesson in one line: a limit reads where a function is heading, deliberately ignoring where it lands, and that deliberate ignorance is what lets it answer questions the arithmetic cannot."))
}

/// Single-character meanings, set once for the whole lesson so a bare letter
/// hovered anywhere says what this lesson means by it.
fn letter_overrides(b: LessonBuilder) -> LessonBuilder {
    b.explain_char('x', "The moving input",
        "What travels in a limit. It is driven toward the target but never set equal to it.")
        .explain_char('a', "The target input",
            "The place on the input axis that x is driven toward. A location, never a value of the function.")
        .explain_char('L', "The limit value",
            "The single number the outputs crowd around. A value on the output axis — and a number the function need never actually take.")
        .explain_char('h', "The window width",
            "How far apart the two points of a difference quotient sit. Never zero, and driven toward zero.")
        .explain_char('t', "Time",
            "Years since the fund launched. The input to V.")
        .explain_char('V', "The fund's value",
            "Millions of dollars, as a function of years since launch.")
        .explain_char('f', "A function",
            "A rule that turns an input into an output. Its value at a point and its limit at that point are separate questions.")
        .explain_char('g', "A second function",
            "Another rule, kept distinct from f so the limit laws can be stated for two of them at once.")
        .explain_char('m', "An average rate",
            "Change in output divided by change in input: the slope of the line through two points on a curve.")
        .explain_char('n', "A count",
            "How many pieces something is chopped into — compounding periods in a year, or payments in an annuity. Sent to infinity, never to a finite target.")
        .explain_char('r', "A rate",
            "An annual interest or discount rate, written as a decimal: 0.05 for 5%.")
        .explain_char('C', "A cash flow",
            "The fixed amount paid each period by an annuity or a perpetuity.")
        .explain_char('0', "Digit zero",
            "The number zero.")
        .explain_char('1', "Digit one",
            "The number one.")
        .explain_char('P', "A payoff",
            "What an option is worth at expiry, as a function of the final stock price.")
        .explain_char('u', "A small offset",
            "A substitution variable for a shrinking quantity — r/n in the compounding limit, so sending n to infinity is the same journey as sending u to zero.")
        .explain_char('π', "Pi",
            "The circle constant, about 3.14159. The sine of any whole multiple of pi is exactly zero — which is what the rigged table in the practice section exploits.")
}

/// [fig 1] The removable hole at x = 3, as a table of approaches beside the
/// graph. Every tabulated value is x + 3: 2.9 gives 5.9, 2.99 gives 5.99,
/// 3.001 gives 6.001, and so on.
const HOLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 300" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="640" height="300" rx="8" fill="#f8fafc"/>
<!-- table -->
<text x="150" y="26" fill="#64748b" text-anchor="middle">walking up to x = 3</text>
<g fill="#e2e8f0"><rect x="30" y="36" width="110" height="24"/><rect x="140" y="36" width="130" height="24"/></g>
<g fill="#64748b" text-anchor="middle"><text x="85" y="52">x</text><text x="205" y="52">(x&#178; &#8722; 9) / (x &#8722; 3)</text></g>
<g fill="#dcfce7"><rect x="30" y="60" width="240" height="72"/></g>
<g fill="#fef3c7"><rect x="30" y="132" width="240" height="24"/></g>
<g fill="#dbeafe"><rect x="30" y="156" width="240" height="72"/></g>
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M30 36H270M30 60H270M30 84H270M30 108H270M30 132H270M30 156H270M30 180H270M30 204H270M30 228H270"/>
  <path d="M30 36V228M140 36V228M270 36V228"/>
</g>
<g fill="#166534" text-anchor="middle">
  <text x="85" y="76">2.9</text><text x="205" y="76">5.9</text>
  <text x="85" y="100">2.99</text><text x="205" y="100">5.99</text>
  <text x="85" y="124">2.999</text><text x="205" y="124">5.999</text>
</g>
<g fill="#b45309" text-anchor="middle">
  <text x="85" y="148">3</text><text x="205" y="148">0 / 0 &#8212; no value</text>
</g>
<g fill="#1d4ed8" text-anchor="middle">
  <text x="85" y="172">3.001</text><text x="205" y="172">6.001</text>
  <text x="85" y="196">3.01</text><text x="205" y="196">6.01</text>
  <text x="85" y="220">3.1</text><text x="205" y="220">6.1</text>
</g>
<text x="150" y="248" fill="#166534" text-anchor="middle">climbing from below &#8594; 6</text>
<text x="150" y="266" fill="#1d4ed8" text-anchor="middle">falling from above &#8594; 6</text>
<text x="150" y="288" fill="#b45309" text-anchor="middle">both point at a value the function does not have</text>
<!-- graph: x from 1 to 5 across px 360..600, y from 3 to 9 across py 250..40 -->
<text x="480" y="26" fill="#64748b" text-anchor="middle">the same function, drawn</text>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M340 250H612M360 40V266"/></g>
<g fill="#94a3b8" text-anchor="middle" font-size="10">
  <text x="608" y="266">x</text><text x="352" y="46">y</text>
</g>
<!-- dashed guides to the hole at (480, 145) -->
<g stroke="#94a3b8" stroke-width="1" stroke-dasharray="4 3" fill="none">
  <path d="M480 250V145"/><path d="M360 145H480"/>
</g>
<text x="480" y="266" fill="#b45309" text-anchor="middle" font-size="10">3</text>
<text x="352" y="149" fill="#b45309" text-anchor="end" font-size="10">6</text>
<!-- the line y = x + 3, drawn as two segments stopping short of the hole -->
<g stroke="#16a34a" stroke-width="2.5" fill="none"><path d="M360 215L474 148"/></g>
<g stroke="#2563eb" stroke-width="2.5" fill="none"><path d="M486 142L600 75"/></g>
<!-- approach arrows -->
<g fill="#16a34a"><polygon points="474,148 464,148 470,140"/></g>
<g fill="#2563eb"><polygon points="486,142 496,142 490,150"/></g>
<!-- the punched-out point -->
<circle cx="480" cy="145" r="5" fill="#f8fafc" stroke="#b45309" stroke-width="2.5"/>
<text x="500" y="196" fill="#b45309" font-size="10">one point missing</text>
<text x="500" y="210" fill="#b45309" font-size="10">from an otherwise</text>
<text x="500" y="224" fill="#b45309" font-size="10">ordinary line</text>
</svg>"##;

/// [fig 2] An earnings jump: left limit 100, right limit 108, printed value
/// 104 — three different numbers at one instant, so no two-sided limit.
const JUMP_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 300" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="620" height="300" rx="8" fill="#f8fafc"/>
<text x="310" y="24" fill="#64748b" text-anchor="middle">a stock across its earnings announcement</text>
<!-- axes: time -3..3 across px 60..560; price 92..112 across py 250..50 -->
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M60 250H570M60 40V250"/></g>
<g stroke="#e2e8f0" stroke-width="1" stroke-dasharray="3 3" fill="none">
  <path d="M60 170H570"/><path d="M60 90H570"/><path d="M60 130H570"/>
</g>
<g fill="#94a3b8" text-anchor="end" font-size="10">
  <text x="54" y="174">100</text><text x="54" y="134">104</text><text x="54" y="94">108</text>
</g>
<text x="574" y="254" fill="#94a3b8" font-size="10">time</text>
<!-- the announcement instant -->
<g stroke="#b45309" stroke-width="1.5" stroke-dasharray="5 4" fill="none"><path d="M310 40V262"/></g>
<text x="310" y="278" fill="#b45309" text-anchor="middle" font-size="10">the announcement</text>
<!-- before: drifting up to 100 -->
<path d="M60 226 L110 220 L155 228 L200 208 L245 190 L280 178 L310 170" stroke="#16a34a" stroke-width="2.5" fill="none"/>
<!-- after: opening at 108 and drifting on -->
<path d="M310 90 L345 96 L385 82 L430 86 L480 74 L530 78 L560 70" stroke="#2563eb" stroke-width="2.5" fill="none"/>
<!-- the two one-sided limits, both punched out -->
<circle cx="310" cy="170" r="5" fill="#f8fafc" stroke="#16a34a" stroke-width="2.5"/>
<circle cx="310" cy="90" r="5" fill="#f8fafc" stroke="#2563eb" stroke-width="2.5"/>
<!-- the actual print -->
<circle cx="310" cy="130" r="5" fill="#b45309"/>
<text x="296" y="192" fill="#16a34a" text-anchor="end" font-size="10">left limit: 100</text>
<text x="324" y="82" fill="#2563eb" font-size="10">right limit: 108</text>
<text x="324" y="126" fill="#b45309" font-size="10">the auction printed 104</text>
<text x="310" y="296" fill="#64748b" text-anchor="middle">left and right disagree, so there is no limit here &#8212; and moving the print would not change that</text>
</svg>"##;

/// [fig 3] The tolerance game: an epsilon band around L answered by a delta
/// stripe around a, then both tightened.
const EPSILON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 330" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="620" height="330" rx="8" fill="#f8fafc"/>
<text x="310" y="24" fill="#64748b" text-anchor="middle">the sceptic shades a band; you answer with a stripe</text>
<!-- wide epsilon band around L at py 160 -->
<rect x="70" y="128" width="490" height="64" fill="#dcfce7"/>
<!-- delta stripe around a at px 300 -->
<rect x="258" y="46" width="84" height="228" fill="#dbeafe" fill-opacity="0.65"/>
<!-- tightened band and stripe -->
<g stroke="#166534" stroke-width="1.2" stroke-dasharray="5 4" fill="none"><path d="M70 146H560"/><path d="M70 174H560"/></g>
<g stroke="#1d4ed8" stroke-width="1.2" stroke-dasharray="5 4" fill="none"><path d="M282 46V274"/><path d="M318 46V274"/></g>
<!-- axes -->
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M70 274H570M70 40V274"/></g>
<!-- the curve, rising gently through (300,160) -->
<path d="M78 236 C 150 214, 220 186, 300 160 C 380 134, 450 108, 552 84" stroke="#0f172a" stroke-width="2.2" fill="none"/>
<!-- L and a markers -->
<g stroke="#94a3b8" stroke-width="1" stroke-dasharray="4 3" fill="none"><path d="M70 160H300"/><path d="M300 274V160"/></g>
<text x="64" y="164" fill="#166534" text-anchor="end" font-size="10">L</text>
<text x="64" y="132" fill="#166534" text-anchor="end" font-size="10">L + &#949;</text>
<text x="64" y="196" fill="#166534" text-anchor="end" font-size="10">L &#8722; &#949;</text>
<text x="300" y="290" fill="#1d4ed8" text-anchor="middle" font-size="10">a</text>
<text x="258" y="304" fill="#1d4ed8" text-anchor="middle" font-size="10">a &#8722; &#948;</text>
<text x="342" y="304" fill="#1d4ed8" text-anchor="middle" font-size="10">a + &#948;</text>
<!-- the punched-out point above a -->
<circle cx="300" cy="160" r="5" fill="#f8fafc" stroke="#0f172a" stroke-width="2.2"/>
<!-- annotations -->
<text x="576" y="150" fill="#166534" font-size="10">&#949;</text>
<text x="576" y="180" fill="#166534" font-size="10">band</text>
<text x="310" y="322" fill="#64748b" text-anchor="middle">inside the blue stripe the curve never leaves the green band &#8212; and the dashed pair shows the next round, tighter both ways</text>
</svg>"##;

/// [fig 5] The corner: a call payoff at expiry, continuous at the strike,
/// with one-sided difference quotients 0 (left) and 1 (right) — so no delta.
const CORNER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 300" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="640" height="300" rx="8" fill="#f8fafc"/>
<!-- LEFT: payoff P(S) = max(S - 100, 0). S 94..106 across px 50..290 (20 px per dollar);
     P 0..6 across py 240..80 (26.667 px per dollar). -->
<text x="170" y="26" fill="#64748b" text-anchor="middle">a call's payoff at expiry, at the strike</text>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M44 240H300M50 70V254"/></g>
<g stroke="#94a3b8" stroke-width="1" stroke-dasharray="4 3" fill="none"><path d="M170 240V72"/></g>
<g fill="#94a3b8" text-anchor="end" font-size="10"><text x="44" y="244">0</text><text x="44" y="84">6</text></g>
<g fill="#94a3b8" text-anchor="middle" font-size="10"><text x="50" y="256">94</text><text x="290" y="256">106</text></g>
<text x="170" y="270" fill="#b45309" text-anchor="middle" font-size="10">strike, 100</text>
<path d="M50 240H170" stroke="#16a34a" stroke-width="2.6" fill="none"/>
<path d="M170 240L290 80" stroke="#2563eb" stroke-width="2.6" fill="none"/>
<circle cx="170" cy="240" r="5" fill="#b45309"/>
<text x="110" y="232" fill="#16a34a" text-anchor="middle" font-size="10">slope 0</text>
<text x="246" y="150" fill="#2563eb" font-size="10">slope 1</text>
<text x="170" y="290" fill="#64748b" text-anchor="middle" font-size="10">no jump in the value: it is 0 on both sides</text>
<!-- RIGHT: the difference quotient (P(100+h) - P(100))/h.
     h -6..6 across px 360..600, h = 0 at px 480; value 0 at py 200, value 1 at py 120. -->
<text x="480" y="26" fill="#64748b" text-anchor="middle">its difference quotient, either side of the strike</text>
<g stroke="#e2e8f0" stroke-width="1" fill="none"><path d="M354 200H614"/></g>
<g stroke="#e2e8f0" stroke-width="1" stroke-dasharray="3 3" fill="none"><path d="M354 120H614"/></g>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M480 84V236"/></g>
<g fill="#94a3b8" text-anchor="end" font-size="10"><text x="350" y="204">0</text><text x="350" y="124">1</text></g>
<path d="M360 200H474" stroke="#16a34a" stroke-width="2.6" fill="none"/>
<path d="M486 120H600" stroke="#2563eb" stroke-width="2.6" fill="none"/>
<circle cx="480" cy="200" r="5" fill="#f8fafc" stroke="#16a34a" stroke-width="2.5"/>
<circle cx="480" cy="120" r="5" fill="#f8fafc" stroke="#2563eb" stroke-width="2.5"/>
<text x="416" y="222" fill="#16a34a" text-anchor="middle" font-size="10">h &lt; 0: quotient 0</text>
<text x="544" y="112" fill="#2563eb" text-anchor="middle" font-size="10">h &gt; 0: quotient 1</text>
<text x="480" y="256" fill="#b45309" text-anchor="middle" font-size="10">h = 0</text>
<text x="480" y="290" fill="#64748b" text-anchor="middle" font-size="10">the jump has moved here &#8212; so there is no limit, and no delta</text>
</svg>"##;

/// [fig 4] Secants pivoting onto the tangent. V(t) = 100 + 8t - t^2, base
/// point t = 2 where V = 112. Secant slopes are 4 - h: 2 at h = 2, 3 at
/// h = 1, 3.5 at h = 0.5, and the tangent slope is 4.
const SECANT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 340" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="620" height="340" rx="8" fill="#f8fafc"/>
<text x="310" y="24" fill="#64748b" text-anchor="middle">secants closing onto the tangent at t = 2</text>
<!-- axes: t 0..5 across px 70..570; V 96..120 across py 290..70 -->
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M70 290H580M70 44V290"/></g>
<g fill="#94a3b8" text-anchor="middle" font-size="10">
  <text x="70" y="306">0</text><text x="170" y="306">1</text><text x="270" y="306">2</text>
  <text x="370" y="306">3</text><text x="470" y="306">4</text><text x="570" y="306">5</text>
  <text x="584" y="294">t</text>
</g>
<g fill="#94a3b8" text-anchor="end" font-size="10">
  <text x="64" y="247">100</text><text x="64" y="137">112</text><text x="64" y="101">116</text>
</g>
<!-- secant through (2,112) of slope 4-h, drawn px 230..520.
     px slope = -(4-h)*9.1667/100;  h=2: py 140.3->87.2   h=1: 144.0->64.3   h=0.5: 145.8->52.8 -->
<path d="M230 140.3 L520 87.2" stroke="#2563eb" stroke-width="1.6" stroke-dasharray="9 5" fill="none"/>
<path d="M230 144.0 L520 64.3" stroke="#2563eb" stroke-width="1.6" stroke-dasharray="6 4" fill="none"/>
<path d="M230 145.8 L520 52.8" stroke="#2563eb" stroke-width="1.6" stroke-dasharray="2 3" fill="none"/>
<!-- tangent, slope 4: py 169.7 at px 170, py 48.7 at px 500 -->
<path d="M170 169.7 L500 48.7" stroke="#b45309" stroke-width="2.2" fill="none"/>
<!-- V(t) = 100 + 8t - t^2 through (70,243) (170,178.8) (270,133) (370,105.5) (470,96.3) (570,105.5) -->
<path d="M70 243 C 103 221.3, 137 196.8, 170 178.8 C 203 160.9, 237 145.1, 270 133 C 303 120.9, 337 112.4, 370 105.5 C 403 98.6, 437 96.3, 470 96.3 C 503 96.3, 537 100.0, 570 105.5" stroke="#16a34a" stroke-width="2.6" fill="none"/>
<!-- base point t=2 and the second points at t = 4, 3, 2.5 (V = 116, 115, 113.75) -->
<circle cx="270" cy="133" r="5" fill="#16a34a"/>
<circle cx="470" cy="96.3" r="4.5" fill="#f8fafc" stroke="#2563eb" stroke-width="2"/>
<circle cx="370" cy="105.5" r="4.5" fill="#f8fafc" stroke="#2563eb" stroke-width="2"/>
<circle cx="320" cy="117.0" r="4.5" fill="#f8fafc" stroke="#2563eb" stroke-width="2"/>
<text x="270" y="158" fill="#16a34a" text-anchor="middle" font-size="10">t = 2, V = 112</text>
<!-- key -->
<text x="380" y="212" fill="#64748b" font-size="10">slope of the secant = 4 &#8722; h</text>
<g stroke="#2563eb" stroke-width="1.6" fill="none">
  <path d="M380 224H404" stroke-dasharray="9 5"/><path d="M380 240H404" stroke-dasharray="6 4"/><path d="M380 256H404" stroke-dasharray="2 3"/>
</g>
<path d="M380 272H404" stroke="#b45309" stroke-width="2.2" fill="none"/>
<g font-size="10">
  <text x="412" y="227" fill="#2563eb">h = 2, slope 2</text>
  <text x="412" y="243" fill="#2563eb">h = 1, slope 3</text>
  <text x="412" y="259" fill="#2563eb">h = 0.5, slope 3.5</text>
  <text x="412" y="275" fill="#b45309">tangent, slope 4</text>
</g>
<text x="310" y="326" fill="#64748b" text-anchor="middle">every dashed line needs two points, so none of them is the tangent &#8212; the tangent is what they aim at</text>
</svg>"##;
