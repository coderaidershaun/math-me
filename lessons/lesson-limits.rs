//! Limits from zero to hero: what a limit actually asks, why it deliberately
//! refuses to look at the point it names, the moves that answer a 0/0
//! question, and the payoff — the derivative, which is a limit and nothing
//! else. Finance-flavoured throughout: instantaneous growth rates,
//! announcement jumps, continuous compounding. Each section function is named
//! after the heading it renders and chained in document order.
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
        .explain(r"t^(2)", "The drag",
            "A subtraction that grows with the square of time: small at first, then dominant. It is what eventually turns the fund over.")
        .para(|p| p
            .text("At two years it is worth 100 plus 16 minus 4 — $112 million. Now the question every investor in it actually wants answered: how fast is it growing ")
            .math("right")
            .text(" now, at the two-year mark? Not on average since launch. Now."))
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
            "Not a value of f. It names the single number the outputs f(x) crowd around as the inputs x are driven toward a — a number f may never actually take.")
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
            .text("The notation has been assembled over two centuries. Simon l'Huilier wrote the abbreviation \"lim\" in a prize essay of 1786, borrowing the first three letters of the Latin. The arrow tucked underneath it is far younger: G. H. Hardy put it there in A Course of Pure Mathematics in 1908, and every calculus book since has copied him. Before Hardy people wrote things like \"lim, x = a\", with an equals sign where the arrow now sits — and that equals sign was exactly the confusion the arrow was invented to kill, because ")
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
        .explain(r"(x-3)", "The shared factor",
            "Zero exactly when x is 3 — which is why cancelling it is illegal at 3 and legal at every other input, including every input the limit visits.")
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
            .text("This is the primary idea of the lesson, and everything after it is machinery. ")
            .math("A")
            .text(" limit reports where a function is heading, not where it lands. Which is precisely why it can answer the fund's question: a formula can be undefined at the one input you care about and still point unambiguously at a number, and the limit is how you read the pointing."))
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
        .para(|p| p
            .text("These are the one-sided limits, and the rule joining them to the plain two-sided limit is a definition rather than a theorem: the limit exists exactly when both one-sided limits exist and are equal, and then it equals their common value."))
        .display(r"\lim_{x \to a^{-}} f(x) = \lim_{x \to a^{+}} f(x) = L")
        .explain(r"\lim_{x \to a^{+}} f(x) = L", "Both sides agreeing on L",
            "When the approach from above and the approach from below arrive at the same L, that shared number is the limit. When they do not, there is no limit — not a disputed one, none.")
        .para(|p| p
            .text("Finance supplies the disagreement for free. A company reports earnings at nine in the morning. Right before the announcement the stock is trading around $100 and drifting quietly. Right after, it is $108. Between those two states there is no path, only a discontinuity in the tape."))
        .figure(Figure::new(JUMP_SVG,
            "An earnings announcement, drawn as a function of time. Approaching the announcement from before, the price heads for 100; approaching from after, it heads for 108. The auction that reopened the stock printed 104. Three different numbers at one instant — the left limit, the right limit, and the actual value — and because the first two disagree, the price has no limit at that instant, whatever the third one says."))
        .para(|p| p
            .text("Three separate numbers live at that one moment: the left limit of 100, the right limit of 108, and the actual print of 104. Because the first two disagree, there is no two-sided limit. Notice how little the print of 104 has to do with it: even if the auction had printed exactly 100, or exactly 108, the two-sided limit would still not exist. The value at the point can neither create a limit nor destroy one."))
        .rule()
        .para(|p| p
            .text("There are exactly three ways a limit can fail to exist, and it is worth being able to name which one you are looking at."))
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
            "Bounded between minus one and one, but oscillating infinitely fast as x nears zero. It never settles, so it has no limit at 0 — bounded and convergent are different things.")
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
            "The tolerance the sceptic demands: how close the outputs must come to L. The sceptic picks it, and may pick it as small as they like.")
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
            .text(". Slope is the exchange rate between the two tolerances — which is a hint about where this lesson is going."))
        .para(|p| p
            .text("The formulation is not decoration; it is what rescued calculus. Bernard Bolzano wrote it down in 1817 and was ignored. Augustin-Louis Cauchy used the letters in the 1820s but never quite tied ")
            .math(r"\delta")
            .text(" to ")
            .math(r"\varepsilon")
            .text(". Karl Weierstrass, lecturing in Berlin in the 1860s, stated it in the form above, and with that the arguments about who was reasoning correctly ended, because there was finally something to check. In practice you will almost never run the game by hand — you will use the laws of the next section. But when a limit is contested, this is the court of appeal, and there is no higher one."))
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
            .text("Chain those laws over a polynomial — built from constants and copies of ")
            .math("x")
            .text(" by adding and multiplying — and every polynomial has limits you get by substitution. The same holds for sines, cosines, exponentials and logarithms on their domains. Functions with that property have a name:"))
        .display(r"\lim_{x \to a} f(x) = f(a)")
        .explain(r"\lim_{x \to a} f(x) = f(a)", "The definition of continuity at a",
            "The journey and the destination agree. Everything the approach predicts, the value at a delivers. This is a property some functions have at some points — never something a limit guarantees.")
        .para(|p| p
            .text("A function is continuous at ")
            .math("a")
            .text(" when the limit there exists, the value there exists, and the two are equal. Continuity is not what a limit is; continuity is the special case where the journey and the destination happen to agree, and \"just plug in\" is a licence you get only from it. Every worked example in this lesson that could not be plugged in was a point of discontinuity."))
        .para(|p| p
            .text("Which makes the failures easy to catalogue, since there are only three, one for each clause. The broken formula ")
            .math(r"(x^2-9)/(x-3)")
            .text(" has a limit but no value: a removable discontinuity, so called because defining the value to be 6 removes it. The earnings announcement has both one-sided limits but no two-sided one: a jump. And ")
            .math(r"1/x^2")
            .text(" has no finite limit at all: an infinite discontinuity."))
        .explain(r"(x^2-9)/(x-3)", "The broken formula, written inline",
            "Limit 6 at x = 3, no value at x = 3. Filling in the missing value repairs it completely, which is what makes the discontinuity removable.")
        .note("Removable is the interesting case, and the one to remember: the function is not merely fixable, it is fixable in exactly one way, because the limit names the only value that would make it continuous. Every derivative in the next section is a removable discontinuity, repaired.")
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
        .explain(r"x^2", "The dominant term",
            "The fastest-growing piece anywhere in the fraction. Dividing through by it is a change of units that leaves the ratio unchanged and the answer visible.")
        .explain(r"3 + 5/x", "The top, rescaled",
            "After dividing by x squared. The 5/x fades to nothing as x runs off, leaving 3.")
        .explain(r"2 - 7/x^2", "The bottom, rescaled",
            "After dividing by x squared. The 7 over x squared fades even faster, leaving 2.")
        .rule()
        .para(|p| p
            .text("Move four: squeeze it. When a function is too wild to evaluate, trap it between two functions that are not, and if the trap closes on a single number the function has nowhere else to go. The oscillating troublemaker from earlier makes the point. On its own ")
            .math(r"\sin(1/x)")
            .text(" has no limit at zero. But multiply it by ")
            .math("x")
            .text(" and, for positive ")
            .math("x")
            .text(":"))
        .display(r"-x \le x \sin(1/x) \le x")
        .explain(r"x \sin(1/x)", "The oscillation, damped",
            "Still swinging infinitely fast, but between walls that are closing. The oscillation never stops; the room it has to oscillate in runs out.")
        .explain(r"-x", "The lower wall",
            "Because the sine never drops below minus one, multiplying by a positive x can never push the product below minus x.")
        .para(|p| p
            .text("because the sine never leaves the range from minus one to one. Both walls head for zero, so the function trapped between them has no choice but to head for zero too — even though it never stops oscillating on the way. A hedged position behaves like this: the underlying keeps thrashing, but the position size is being cut on a schedule, and the profit and loss goes to zero regardless of what the market does, because it is trapped."))
        .note("Try each move in order when a limit returns 0/0: can it be factored, conjugated, rescaled, or trapped? One of the four almost always opens it.")
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
        .explain(r"\frac{C}{r}", "The perpetuity price",
            "The annuity price with n sent to infinity. At 5% a payment of $5 a year forever is worth $100 — the far-off payments contribute so little that the total stops moving.")
        .para(|p| p
            .text("The reason it converges is visible in the formula: with ")
            .math("r")
            .text(" positive, ")
            .math(r"(1+r)^{-n}")
            .text(" is a discount factor shrinking geometrically, so it goes to zero and takes the whole subtracted term with it. Only ")
            .math("C")
            .text(" over ")
            .math("r")
            .text(" survives. That an infinite stream of payments has a finite price is not an accounting convention; it is a limit, and it converges because distant money is worth almost nothing."))
        .explain(r"(1+r)^{-n}", "The discount factor n years out",
            "What a dollar n years away is worth today. With r positive it shrinks geometrically toward zero, which is exactly why the infinite sum stays finite.")
        .rule()
        .para(|p| p
            .text("The most famous limit in finance is also the most easily missed, because it hides inside a piece of everyday arithmetic. Invest $1 at an annual rate ")
            .math("r")
            .text(", compounded ")
            .math("n")
            .text(" times a year. After a year you hold ")
            .math(r"(1 + r/n)^n")
            .text(". Each compounding chops the rate into smaller slices and applies more of them, and the two effects nearly cancel — but not quite, and the residue is worth money."))
        .explain(r"(1 + r/n)^n", "One dollar, compounded n times a year",
            "Split the annual rate into n slices and apply each one to whatever the previous slices produced. More frequent compounding always beats less, but by an ever-shrinking margin.")
        .para(|p| p
            .text("At 5%, compounding once gives 1.05. Twelve times gives 1.051162. Every day gives 1.051267. Every hour gives 1.0512709. The gains are collapsing, and they collapse onto a number:"))
        .display(r"\lim_{n \to \infty} (1 + r/n)^n = e^r")
        .explain(r"\lim_{n \to \infty} (1 + r/n)^n", "Compounding, taken to the limit",
            "What a dollar grows to in a year if the interest is applied continuously rather than at intervals. The ceiling that ever-finer compounding approaches and never passes.")
        .explain(r"e^r", "The continuous growth factor",
            "Euler's number raised to the rate. At r = 0.05 it is 1.05127110 — the value daily compounding is already within a millionth of.")
        .para(|p| p
            .text("This is where continuous compounding comes from, and it deserves a moment. Continuous compounding is not an approximation to daily compounding, and it is not a modelling convenience. It is a limit — the exact ceiling that finer and finer compounding approaches. And the number ")
            .math("e")
            .text(" is not chosen; it falls out. Set ")
            .math("r")
            .text(" to 1 and the same limit defines ")
            .math("e")
            .text(" itself, at 2.71828 and onward. Every continuously-compounded rate on a screen, every log return in a risk model, traces back to this limit."))
        .explain_char('e', "Euler's number",
            "About 2.71828. Not chosen but produced: it is the value of the compounding limit at a rate of 1, and it is why continuous compounding wears exponentials.")
        .plot(Plot::new(1.0..=52.0)
            .curve("value of $1 after one year", "pow(1 + r/x, x)")
            .curve("the limit, e to the r", "exp(r)")
            .param("r", 0.01..=0.30, 0.05)
            .x_label("compounding periods per year")
            .y_label("value of one dollar after one year")
            .height(280.0)
            .caption("The rising curve is n compoundings a year; the flat one is the continuous limit it is climbing toward. Most of the gain is won in the first few periods and the curve is nearly flat well before weekly. Drag r up and the gap widens — the higher the rate, the more compounding frequency is worth — but the ceiling never moves in the wrong direction, and the curve never crosses it."))
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
        .explain(r"t^2", "The drag term",
            "The squared term. Unlike the drift it does not cancel cleanly — it leaves behind the minus 2th that becomes the fading part of the growth rate.")
        .explain(r"100 + 8t + 8h - t^2 - 2th - h^2", "V(t+h), expanded",
            "The fund's rule with t + h substituted and the square multiplied out, ready for the subtraction to knock out everything that does not carry an h.")
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
            "Eight million a year at launch, falling by two million for every year that passes. At t = 2 it is 4, which is the number the shrinking windows were marching on.")
        .para(|p| p
            .text("At ")
            .math("t")
            .text(" = 2 that is 8 minus 4: exactly the 4 the shrinking windows marched on in the opening section, now derived rather than guessed. And the formula says more than the guess could. It goes to zero at ")
            .math("t")
            .text(" = 4 — the fund peaks at $116 million in its fourth year — and turns negative after, the drag having overtaken the drift. One limit, and the fund's whole future is legible."))
        .rule()
        .para(|p| p
            .text("The picture makes the same argument with no algebra. Pick the point on the curve at ")
            .math("t")
            .text(" = 2, pick a second point a window away, and draw the straight line through both. That line is a secant, and its slope is precisely the difference quotient — rise over run, value change over time change. Now slide the second point in."))
        .figure(Figure::new(SECANT_SVG,
            "The fund's value curve with the point at t = 2 fixed and the second point sliding in. The secant through t = 4 has slope 2; through t = 3, slope 3; through t = 2.5, slope 3.5. Each is the difference quotient for that window. As the second point closes on the first the secants pivot, and the line they pivot toward — slope 4 — is the tangent. The tangent is the one line the secants never quite become, which is exactly the relationship a limit describes."))
        .para(|p| p
            .text("Every secant needs two distinct points; a line through one point is not defined. So the tangent is a line the construction can never actually produce — and yet the secants aim at it unmistakably. That is the limit's whole job, restated geometrically. The derivative is the slope of the tangent, and the tangent is the limit of the secants."))
        .plot(Plot::new(0.0..=5.0)
            .curve("fund value V(t)", "100 + 8*x - x^2")
            .curve("tangent at t = 2", "112 + 4*(x - 2)")
            .curve("secant from t = 2 across a window h", "112 + (4 - h)*(x - 2)")
            .param("h", 0.05..=3.0, 2.5)
            .vline(2.0)
            .x_label("years since launch")
            .y_label("fund value, millions of dollars")
            .height(300.0)
            .caption("Drag h down and watch the secant pivot onto the tangent it can never reach. Its slope is 4 minus h, so the gap to the tangent's slope of 4 is exactly h — the error is the window, which is why closing the window is the same act as taking the limit. Drag h up instead and the secant sags below the curve: a wide window reports the average, and the average understates a fund that is still growing."))
        .rule()
        .para(|p| p
            .text("It is worth knowing that this was contested, bitterly, and that the objection was a good one. Newton and Leibniz got calculus working in the 1600s using quantities that were treated as non-zero when it came time to divide by them and as zero when it came time to discard them. In 1734 Bishop George Berkeley attacked exactly that step: quantities that are neither finite, nor infinitely small, nor nothing — \"may we not call them the ghosts of departed quantities?\""))
        .para(|p| p
            .text("He was right, and for a century nobody had a clean answer. The answer, when it came, was the limit. Nothing is ever divided by zero and nothing is ever discarded: ")
            .math("h")
            .text(" stays honestly non-zero throughout the cancellation, and then the limit — with the tolerance game underneath it — asks a different question entirely, about where the surviving expression heads. Berkeley's ghost was exorcised not by better arithmetic but by a better question. Which is why this lesson spent its first half on what a limit asks, and not on how to compute one."))
}

fn practice(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Practice")
        .para(|p| p
            .text("Six questions, all built from this lesson's own examples. Commit to an answer before reading past each rule — the guess is where the learning is."))
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
            .text(" = 3.99 the original formula gives 7.99, and at 4.01 it gives 8.01."))
        .explain(r"(x-4)(x+4)", "The top, factored",
            "A difference of squares. One factor matches the bottom and cancels; the other survives to give the answer.")
        .explain(r"(x-4)", "The vanishing factor",
            "Zero exactly at 4 — never zero anywhere the limit actually looks.")
        .explain(r"x + 4", "What the cancellation leaves",
            "A line, continuous, so its limit at 4 is its value there: 8.")
        .rule()
        .para(|p| p
            .text("Now you. A stock drifts up to $100 before an earnings announcement and trades at $108 after it, and the reopening auction prints exactly $100 — the same number the price was heading for from the left. Does the price have a limit at the announcement?"))
        .note("Decide before reading on. The auction print is bait.")
        .para(|p| p
            .text("No. The left limit is 100 and the right limit is 108; they disagree, so no two-sided limit exists. That the print happens to equal the left limit is irrelevant — the value at a point never participates in whether a limit exists. What the coincidence does buy is one-sided continuity from the left, which matters for a stop order resting below the market and for nothing else here."))
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
        .explain(r"\delta = \varepsilon / 5", "The reply for the steeper line",
            "Divide the demanded tolerance by the slope 5. A steeper line needs a tighter window to keep the same promise.")
        .explain(r"\varepsilon / 3", "The reply for the shallower line",
            "The earlier answer, for slope 3. Comparing the two shows the slope is the exchange rate between the two tolerances.")
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
            .text("Out at infinity. A rate of 8% is compounded ")
            .math("n")
            .text(" times a year. What does a dollar approach as ")
            .math("n")
            .text(" runs off — and roughly how much does going from daily to continuous compounding actually earn?"))
        .note("The second half is the real question. Guess the magnitude before computing.")
        .para(|p| p
            .text("It approaches ")
            .math(r"e^{0.08}")
            .text(", about 1.083287. Daily compounding already gives about 1.083278, so the whole of the rest of the journey to infinity is worth roughly nine millionths of a dollar — under a dollar on a hundred thousand. This is the practical shape of most limits at infinity: nearly all the convergence happens early, and the tail is a rounding error. Which is exactly why continuous compounding is used in practice — not because it is more accurate, but because ")
            .math(r"e^{0.08}")
            .text(" is easier to differentiate than a power of n, and costs nothing to adopt."))
        .explain(r"e^{0.08}", "The continuous growth factor at 8%",
            "About 1.083287. Daily compounding is already within a hundred-thousandth of it, which is why the idealisation is free.")
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
<!-- the tangent, slope 4 value-units per year = 36.7 px per 100 px -->
<path d="M170 170 L490 53" stroke="#b45309" stroke-width="2.2" fill="none"/>
<!-- secants from (270,133) -->
<path d="M230 148 L510 45" stroke="#2563eb" stroke-width="1.6" stroke-dasharray="6 4" fill="none"/>
<path d="M230 144 L470 56" stroke="#2563eb" stroke-width="1.6" stroke-dasharray="6 4" fill="none"/>
<path d="M230 140 L490 45" stroke="#94a3b8" stroke-width="0" fill="none"/>
<path d="M230 140 L510 37" stroke="#0f172a" stroke-width="0" fill="none"/>
<path d="M230 141 L490 82" stroke="#2563eb" stroke-width="1.6" stroke-dasharray="6 4" fill="none"/>
<!-- the value curve, through (70,243) (170,179) (270,133) (370,106) (470,97) (570,106) -->
<path d="M70 243 C 103 221, 137 197, 170 179 C 203 161, 237 145, 270 133 C 303 121, 337 112, 370 106 C 403 100, 437 97, 470 97 C 503 97, 537 101, 570 106" stroke="#16a34a" stroke-width="2.6" fill="none"/>
<!-- base point and the sliding second points -->
<circle cx="270" cy="133" r="5" fill="#16a34a"/>
<circle cx="470" cy="97" r="4.5" fill="#f8fafc" stroke="#2563eb" stroke-width="2"/>
<circle cx="370" cy="106" r="4.5" fill="#f8fafc" stroke="#2563eb" stroke-width="2"/>
<circle cx="320" cy="117" r="4.5" fill="#f8fafc" stroke="#2563eb" stroke-width="2"/>
<text x="264" y="153" fill="#16a34a" text-anchor="end" font-size="10">t = 2, V = 112</text>
<!-- slope key -->
<g fill="#2563eb" font-size="10">
  <text x="486" y="112">h = 2, slope 2</text>
  <text x="386" y="121">h = 1, slope 3</text>
  <text x="330" y="134">h = 0.5, slope 3.5</text>
</g>
<text x="496" y="49" fill="#b45309" font-size="10">tangent, slope 4</text>
<text x="310" y="326" fill="#64748b" text-anchor="middle">each dashed line needs two points, so none of them is the tangent &#8212; the tangent is what they aim at</text>
</svg>"##;
