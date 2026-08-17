//! Prerequisites:
//! - Arithmetic with fractions, decimals and percentages; substituting into a formula.
//! - Averages: what a mean is, and roughly what a standard deviation does.
//! - Logarithms and the exponential, at the level of lesson-exponents.
//! - No probability assumed. Sample spaces, densities and expectation are built here.
//!
//! Probability from counting outcomes to pricing information: the three rules a
//! probability must obey, expectation as a balance point whose one unbreakable
//! property is linearity, conditioning as the whole content of dependence, the
//! jump from counting to measuring that makes a density, the family of
//! distributions a desk actually needs, the three places expected value stops
//! being the number you want, and entropy as an expected value that turns a
//! forecasting edge into a growth rate.
//!
//! Beyond the first course: the Poisson process behind both the count and the
//! waiting time, with superposition, thinning and the two rival repairs for
//! counts that cluster; the compound Poisson that puts a jump inside a price;
//! what a path does on the way rather than where it ends, via reflection and
//! the arcsine law; Chebyshev as the distribution-free floor; and backtesting
//! read backwards, as the transposed conditional it usually is.
//!
//! Cross-links: lesson-exponents (the compound-interest limit inside the
//! Poisson, and logarithms turning multiplication into addition),
//! lesson-limits (a density is a derivative), lesson-algebra-to-linear
//! (regression as the conditional expectation restricted to straight lines)
//! and lesson-kalman-filter (the same conditioning, done on bell curves).
//!
//! Run it: cargo run --release --bin lesson-probability

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
    let b = Lesson::builder("Probability, Expectation and Information");
    let b = counting_the_ways(b);
    let b = the_balance_point(b);
    let b = conditioning_and_dependence(b);
    let b = from_counting_to_measuring(b);
    let b = the_family_of_distributions(b);
    let b = a_price_path_assembled(b);
    let b = expected_value_in_full(b);
    let b = randomness_and_information(b);
    let b = where_the_story_breaks(b);
    let b = practice(b);
    let b = letter_overrides(b);
    b.build()
}

fn counting_the_ways(b: LessonBuilder) -> LessonBuilder {
    b.heading("Counting the ways it could go")
        .note("Hover any term in a formula to see what it means here. Six plots below have sliders — drag them and the curves follow.")
        .para(|p| p
            .text("Probability has a reputation as a subject about the future. It is not. It is a subject about lists, and the bookkeeping is so plain that the first time you see it written out it does not look capable of pricing an option or running a risk system. It is capable of both, and everything in this lesson is built from it."))
        .para(|p| p
            .text("Start with the list. Before anything happens, write down every way things could turn out, arranged so that exactly one entry will end up being true — not two, not none. That list is the sample space. Flip a coin twice and it has four entries: heads-heads, heads-tails, tails-heads, tails-tails. An event is any part of the list you happen to care about: \"at least one head\" is the first three entries out of four."))
        .para(|p| p
            .text("If the entries are equally likely, a probability is a share of the list:"))
        .display(r"P(A) = \frac{|A|}{|S|}")
        .explain(r"P(A)", "The probability of the event A",
            "The share of the ways things could go in which A turns out to be true. It is a number between 0 and 1, and it is a property of the list you wrote down as much as of the world — change what you count as a distinct outcome and this number changes with it. Where the number itself comes from is a separate question with three answers in common use, and the rules do not care which you pick: symmetry, as when a fair die has six faces and no reason to prefer one; frequency, as when this stock closed down on 47 per cent of the last five thousand days; or belief, a considered judgement that this merger closes with probability 0.7, which will never be repeated and has no frequency at all. Purists have fought over these for two centuries. Practitioners use all three before lunch, and the machinery is identical in each case.")
        .explain(r"\frac{|A|}{|S|}", "Ways it happens, over ways in total",
            "How many entries of the list belong to A, divided by how many entries there are. This particular formula needs the entries to be equally likely, which is a strong assumption doing real work; the three rules below survive without it, which is why they, and not this, are the foundation.")
        .para(|p| p
            .text("So \"at least one head\" has probability three quarters. That equally-likely assumption is a crutch and gets retired within the page, but the rules it suggests do not. There are three of them, they are all forced by the words \"share of a whole\", and everything else in probability is a consequence."))
        .para(|p| p
            .text("A probability is never negative, because a share of a list cannot be. The whole list has probability 1, because something happens. And if two events cannot both occur, their probabilities add, because counting two separate piles is the same as counting them together:"))
        .display(r"P(A) + P(B) = P(A \cup B)")
        .explain(r"P(A \cup B)", "The probability of A or B",
            "The share of the list on which at least one of the two events is true. The cup symbol is set-union: all the entries in A, together with all the entries in B. This addition rule holds only when the two events share no entries — if they overlap, adding counts the shared entries twice, and the repair is two paragraphs down. Worth naming while you are here: of the three rules this is the only one carrying real content. Non-negativity and \"the whole list has probability 1\" are definitions in the clothes of axioms — they say what a share is. This one is an assumption, and the paragraphs below are about the form of it that does the work.")
        .explain(r"P(B)", "The probability of the second event",
            "The share of the ways things could go in which B is true, computed exactly as P(A) was. In the conditioning section it becomes the denominator, because B is the news that has arrived and the world has shrunk to it.")
        .para(|p| p
            .text("Kolmogorov wrote those three down in 1933, and his contribution was not discovering them — they had been obvious for centuries — but proving they were enough. Nothing else needs assuming. Every rule you will ever use in this subject is a theorem about shares of a whole, and if you ever forget one you can rebuild it from these three, which is a considerably better position than remembering it."))
        .para(|p| p
            .text("With one enlargement that this lesson leans on twice, so it is worth being honest about now. As written, the addition rule covers two events, and therefore any finite number of them. Later on, probabilities get added infinitely often — the endless doubling prizes of the St Petersburg game, and the infinitely many thin strips that make up an integral — and that does not follow from the finite rule, because a rule holding for every finite collection need not survive the passage to a limit. Kolmogorov's real decision was to assume the infinite version outright. It is the one genuine assumption in the foundations, and everything continuous in this lesson is downstream of it."))
        .para(|p| p
            .text("Two rebuilds, immediately. An event and its opposite cannot both happen and between them cover the list, so their probabilities add to 1:"))
        .display(r"P(A^c) = 1 - P(A)")
        .explain(r"P(A^c)", "The probability A does not happen",
            "The superscript c means complement: every entry on the list that is not in A. Since A and its complement are disjoint and together exhaust the list, rules two and three force this to be one minus the probability of A. It sounds too obvious to name, and it is the single most useful line in elementary probability, because \"at least one\" questions are almost always easier read backwards as \"not none\".")
        .para(|p| p
            .text("And for two events that can overlap, adding double-counts the entries where both happen, so subtract them back once:"))
        .display(r"P(A \cup B) = P(A) + P(B) - P(A, B)")
        .explain(r"P(A, B)", "The probability of A and B together",
            "The share of the list on which both events are true. The comma is read \"and\"; most books write it with an intersection sign instead and mean exactly this. It is subtracted because those entries were counted once inside P(A) and a second time inside P(B), and a share of a list may only be counted once.")
        .para(|p| p
            .text("The complement rule earns its keep straight away, on a question that will convict a great many backtests later in this lesson. Suppose you test a hundred trading strategies, none of which works, and each has a one-in-twenty chance of looking impressive by luck alone. What are the odds that at least one of them does? Asking it forwards means enumerating a hundred overlapping cases. Asking it backwards means one multiplication: none of them impresses with probability 0.95 to the hundredth power, which is 0.0059, so at least one impresses with probability 0.994."))
        .note("Read that number again. A hundred worthless strategies, and you are 99.4 per cent certain to find a winner among them. Nothing in the arithmetic knows or cares that the strategies were worthless. This is why a backtest presented without a count of how many things were tried carries no information whatsoever — not weak information, none.")
        .para(|p| p
            .text("Finally, the misconception that outlives every attempt to teach it. Ten heads in a row have just come up, so tails is now \"due\". It is not: the coin has no record of the last ten flips written on it, and nothing in this lesson has given the eleventh flip any way to know. What people are reaching for is real, but it works by a different mechanism. The proportion of heads really does settle towards a half over enough flips — not because a run of heads is repaid by a run of tails, but because it is diluted. Ten extra heads is a landslide in twenty flips and a rounding error in a million. Nothing compensates. Things get outvoted."))
}

fn the_balance_point(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Expected value: the balance point")
        .para(|p| p
            .text("A list of outcomes is not yet a number you can trade on. The bridge is a random variable, which is badly named twice over: it is neither random nor a variable, but a rule that attaches a number to each entry of the list. Flip a coin twice and count the heads, and you have attached 2, 1, 1, 0 to the four entries. The randomness is in which entry comes up; the rule itself is fixed and known."))
        .para(|p| p
            .text("Now collapse the whole thing into one number by taking a weighted average, with each value weighted by the share of the time it occurs:"))
        .display(r"E[X] = \sum_x x P(X = x)")
        .explain(r"E[X]", "The expected value of X",
            "The weighted average of every value X can take, each weighted by its probability. Also called the mean, or the first moment. It is the single most used number in the whole subject, and the rest of this lesson is either a consequence of it or a warning about where it stops being the number you want.")
        .explain(r"\sum_x", "Added over every value X can take",
            "The instruction to run through the whole list of values, once each, and total what follows. The subscript names the label being run over; it is a dummy and disappears once the sum is done.")
        .explain(r"x P(X = x)", "One value, weighted by how often it occurs",
            "A value X can take, multiplied by the share of the time it takes it. The weights add to 1, which is exactly what makes the total an average rather than merely a sum: it is a weighted mean whose weights arrive pre-normalised.")
        .para(|p| p
            .text("For two coin flips that gives zero heads a quarter of the time, one head half the time and two heads a quarter of the time, so the expected count is 0 times a quarter plus 1 times a half plus 2 times a quarter, which is exactly 1. Flip once instead and the expected number of heads is a half — a value the count can never take. That is the first thing to internalise, and \"expected\" is a poor name for it. Nobody expects half a head."))
        .para(|p| p
            .text("A better name is the balance point. Lay the values out along a ruler and put a weight at each one, of size equal to its probability. The expected value is where the ruler balances. That is not an analogy borrowed from physics; it is the same equation. A lever balances at the point where the weighted distances to either side cancel, and the point where the probability-weighted distances cancel is the value of ")
            .math("c")
            .text(" solving the statement that the average of ")
            .math("X - c")
            .text(" is zero — which rearranges, by nothing more than moving a term, into the sum above."))
        .explain(r"X - c", "How far X lies from a proposed centre",
            "The distance from the candidate balance point c to wherever X actually lands, carrying a sign. Averaging this and setting the result to zero says the pulls to the left exactly cancel the pulls to the right, which is what a balance point means. Solving gives c equal to E[X], so the mean is not merely like a centre of mass — it is defined by the same condition.")
        .figure(Figure::new(BALANCE_SVG,
            "Expectation as a balance point, on a plain bet: a trade that loses 40 seven times in ten and gains 120 the other three, with the weights set to the probabilities and the positions to the payoffs, so the beam balances at plus 8. Notice that the balance point sits at a payoff the trade never actually pays, that it lies far nearer the heavy left weight than the light right one, and that the small weight is held out at a long lever arm — which is exactly how a rare large gain funds a common small loss.")
            .width_percent(78))
        .rule()
        .para(|p| p
            .text("The other reading of expected value is the one that pays the rent: it is the long-run average per trial. Play a game a great many times, take the mean of what you got, and that mean closes on the expected value. This is the law of large numbers, and it is not a vague promise — the average of ")
            .math("n")
            .text(" independent draws has a standard deviation of the individual standard deviation divided by the square root of ")
            .math("n")
            .text(". So the error falls, but slowly: to halve it you must quadruple the sample. Every complaint about needing more data in finance is that square root."))
        .para(|p| p
            .text("Put it on a wheel. A European roulette table has 37 pockets and a single-number bet pays 35 to 1, so the expected return per unit staked is minus one thirty-seventh: minus 2.70 per cent. Bet on red instead, or a dozen, or any combination you can construct, and it is minus 2.70 per cent again, every time — the house edge is the single zero pocket and no arrangement of bets over the other 36 can reach it. The casino is not hoping to win; it is running a business whose margin is known to four decimal places and whose only requirement is volume."))
        .rule()
        .para(|p| p
            .text("Now the property that makes expected value the workhorse of the subject, and the one worth learning to trust before anything else:"))
        .display(r"E[aX + bY] = a E[X] + b E[Y]")
        .explain(r"E[aX + bY]", "The mean of a weighted sum",
            "The expected value of a portfolio made by holding a units of one thing and b units of another, and it holds whatever X and Y are and however they are related. The paragraphs below say why, by totalling over the entries of the sample space rather than over the values. What is worth carrying away from the formula itself is the contrast with the variance, which needs the mean of a product to equal the product of the means — a claim about the joint distribution, and therefore a claim capable of being false. This formula makes no claim about the joint distribution at all, which is exactly why nothing can falsify it.")
        .explain(r"a E[X]", "The first holding's contribution",
            "The mean of X computed entirely on its own, then scaled by how much of X is held. Nothing about Y appears in it, which is precisely the point.")
        .explain(r"b E[Y]", "The second holding's contribution",
            "The mean of Y computed entirely on its own, then scaled by how much of Y is held. Add the two and you have the portfolio's expected return, with the joint behaviour of the holdings nowhere in sight.")
        .para(|p| p
            .text("This is linearity of expectation, and the part everyone underuses is what it does not require. Not independence, not zero correlation, not identical distributions, not any relationship whatsoever between the two quantities. It holds always, because a weighted average is a sum and sums can be regrouped."))
        .para(|p| p
            .text("It is worth seeing what is being regrouped, because that is where the immunity to dependence comes from. Write the expectation as a total over the entries of the sample space rather than over the values — each entry contributing the number the rule attaches to it, weighted by that entry's probability. Collecting the entries that happen to share a value recovers the formula above, so the two are one formula written at two levels of collection. But in the entry-by-entry form the result is immediate: on any single entry the two quantities each just have a number, and the value of their sum is the sum of their values. Totalling that splits in one step. Nothing in the argument ever asks which entries carry which pairs of values, and dependence is precisely a statement about that."))
        .para(|p| p
            .text("Push it until it stops being plausible, which happens well before it stops being true. Two traders are paid on the same year: one receives the number of days the stock rose, the other the number of days it did not. Their payoffs are locked together as tightly as two quantities can be, with a correlation of exactly minus 1. Linearity still says the expected payoffs add, and here you can check it without any probability at all — the two payoffs total 252 every single year, not on average but always. Now ask the same question about risk. The variance of their combined payoff is zero, while each of them separately has a perfectly healthy positive variance. Nothing but the covariance term could have produced that, and it is the term the next formula is about to charge you for."))
        .para(|p| p
            .text("And a portfolio's expected return is the weighted average of its holdings' expected returns, exactly, whatever the correlation matrix says. Which explains the shape of the entire discipline of portfolio construction: the return side is a one-line calculation that dependence cannot touch, and all the difficulty, all the modelling and all the money is on the risk side, where dependence is the only thing that matters."))
        .rule()
        .para(|p| p
            .text("So turn to risk. Take the average squared distance from the mean:"))
        .display(r"\mathrm{Var}(X) = E[(X - \mu)^2]")
        .explain(r"\mathrm{Var}(X)", "The variance of X",
            "The average squared distance from the mean — a measure of spread in the square of X's units. Its square root, the standard deviation, restores the original units and is the number a desk actually quotes as volatility.")
        .explain(r"E[(X - \mu)^2]", "The mean of the squared deviation",
            "Squaring does two jobs. It makes deviations count regardless of sign, which any even function would do. And it makes independent risks add, which is the real reason the square won. Why it adds, driven down: the variance of a sum exceeds the two separate variances by twice the covariance, the covariance is the mean of the product less the product of the means, and under independence the joint probability factorises, so the double sum separates into a product of two single sums and the excess vanishes. That last step is not a theorem about squares but the definition of independence with the fraction cleared. What fails for rival measures is that their combination rule depends on the shape: two independent quantities each equally likely to be minus 1 or plus 1 have an average absolute deviation of 1 apiece, and their sum has an average absolute deviation of 1 — neither 2 nor the square root of 2 — while two independent bell curves combine theirs by the square root of 2. Same independence, two different rules. The variance gives one plus one in both cases and in every case.")
        .para(|p| p
            .text("Now add two of them together and expand the bracket, using linearity on each of the three pieces:"))
        .display(r"\mathrm{Var}(X + Y) = \mathrm{Var}(X) + \mathrm{Var}(Y) + 2\mathrm{Cov}(X, Y)")
        .explain(r"\mathrm{Var}(X + Y)", "The variance of the combined position",
            "The spread of the two holdings taken together — the number a risk system actually reports. Expanding the square of the sum splits it into three pieces, of which only the last knows that there are two holdings rather than one.")
        .explain(r"\mathrm{Var}(Y)", "The second holding's variance on its own",
            "The spread of Y as if it were held alone. Together with the variance of X it is what a naive risk report would add up, and adding those two alone is right only when the covariance is zero.")
        .explain(r"2\mathrm{Cov}(X, Y)", "The cross term, twice",
            "The piece that appears because the square of a sum has a middle term, and it appears twice because there are two identical cross products. This one term is the whole of diversification: negative, it cancels risk; positive, it adds risk the two separate variances never showed.")
        .explain(r"\mathrm{Cov}(X, Y)", "The covariance of X and Y",
            "The average of one variable's deviation multiplied by the other's: the term that appears when the square of a sum is expanded. Divide it by both standard deviations and you have the correlation, the same quantity rescaled to lie between minus 1 and 1. It is zero when the two move independently, and it is the only place in this equation where the relationship between X and Y is allowed to speak. Being an average of a product is also exactly what makes it blind. If X is symmetric about zero and g is any function that treats plus and minus alike, then X times g(X) is odd, and the average of an odd function of a symmetric quantity must equal its own negative and is therefore zero. So the covariance of X with such a g(X) is zero for every one of them — not as a curiosity of one example but always. On a desk that reads directly: any payoff that is even in the underlying, such as a straddle, a variance swap or a gamma position, is completely determined by the underlying and has a correlation of zero with it by construction. A correlation report on that book is not weak evidence of independence. It is no evidence at all.")
        .para(|p| p
            .text("That cross term is the entire content of diversification, and the contrast with the previous formula is the lesson. The mean of a sum ignores dependence completely. The variance of a sum is dependence, plus two pieces that were already known. It is why expected return is cheap and risk is expensive."))
        .para(|p| p
            .text("Numbers make it concrete. Hold half of each of two assets, each with 16 per cent volatility. If they are uncorrelated the portfolio's volatility is 11.31 per cent; at correlation 0.3 it is 12.90; at correlation 1 it is 16, and you have diversified nothing; at correlation minus 1 it is exactly zero and you are holding a perfect hedge. The expected return, meanwhile, is the same in all four cases."))
        .note("A trap sits inside that arithmetic. Variances add; standard deviations do not. Two uncorrelated 16 per cent assets give a variance of two halves-squared times 0.0256, not a volatility of 32 or of 16 — the answer is 11.31 because volatility is the square root of something that added. The same square root is why a 16 per cent annual volatility becomes 16 divided by the square root of 252, or 1.008 per cent a day: time adds variance, so volatility grows as its square root. And that rule is not a law of nature. It is the assumption that days are uncorrelated, wearing a disguise.")
}

fn conditioning_and_dependence(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Conditioning: what dependence actually is")
        .para(|p| p
            .text("Information arrives, and the list has to be redrawn. That redrawing is the whole of conditional probability, and it is a two-step operation you can do with a pencil: cross out every entry of the list on which the news is false, then scale up what survives so that it adds to 1 again."))
        .display(r"P(A \mid B) = \frac{P(A, B)}{P(B)}")
        .explain(r"P(A \mid B)", "The probability of A given B",
            "The share of the ways things could go in which A is true, computed inside the smaller world where B is known to be true. The vertical bar is read \"given\". Nothing about the world changed when B was learned — only which entries of the list are still in play.")
        .explain(r"\frac{P(A, B)}{P(B)}", "Both, renormalised by the survivor",
            "The numerator keeps only the entries where A and B are both true. The denominator is what the surviving entries add up to, and dividing by it is what restores the total to 1 — a probability must be a share of whatever world you are now in. It also explains why conditioning on something of probability zero is undefined: there is no world left to be a share of.")
        .para(|p| p
            .text("Independence is the special case where the smaller world looks exactly like the big one. If learning B tells you nothing about A, then the conditional probability equals the unconditional one, and substituting that into the definition and clearing the fraction gives the form everybody memorises:"))
        .display(r"P(A, B) = P(A) P(B)")
        .explain(r"P(A) P(B)", "The two probabilities multiplied",
            "The product rule for independent events. It is worth seeing that this is not the definition of independence but a consequence of it: independence means the conditional probability equals the unconditional one, and multiplying out is what that statement looks like with the fraction cleared. Which is also why multiplying probabilities together is only ever legitimate after you have argued that independence holds. And why dialling in a correlation is not the same as arguing it. The standard tool for a book of correlated defaults draws correlated bell curves and calls a name defaulted when its draw falls below a threshold — which has a property nobody chose: zero tail dependence. At a correlation of 0.3, the chance a second name defaults given that a first has runs 14.3 per cent when each defaults with probability 5 per cent, 5.6 per cent at 1 per cent, 1.5 per cent at 0.1, and 0.11 per cent at 0.001 — heading to nothing exactly where a senior tranche lives. Turning the dial up does not repair it: at a correlation of 0.5 the same figures are 24.4, 12.9, 5.4 and 1.0 per cent, larger and still heading to nothing. The property holds at every setting below 1, because it belongs to the bell curve's shape rather than to its parameter. Redraw the width each time, as the fat-tail section does, and the same numbers settle near 16 per cent instead of vanishing. So this section's slogan goes one step further: correlated by the right number does not mean dependent in the right shape, and one number was never going to specify a joint distribution.")
        .para(|p| p
            .text("And Bayes' rule is the same definition, read backwards. \"A and B\" does not care about the order of the words, so the fraction can be built either way round, and equating the two gives:"))
        .display(r"P(A \mid B) = \frac{P(B \mid A) P(A)}{P(B)}")
        .explain(r"\frac{P(B \mid A) P(A)}{P(B)}", "Evidence given cause, over evidence from any cause",
            "The numerator is how often the cause A occurs at all, multiplied by how often it produces the evidence B when it does. The denominator is how often B occurs for any reason whatsoever, including reasons that have nothing to do with A. Dividing converts \"how well A explains B\" into \"how likely A is, now that B has been seen\" — and the denominator is where a rare cause loses, because it is dominated by the common causes.")
        .para(|p| p
            .text("Put a signal on it. Two per cent of months contain a crash. A warning indicator fires in 90 per cent of the months that do contain one — genuinely good detection — and also fires in 10 per cent of the calm months. Today it fires. What is the probability of a crash?"))
        .para(|p| p
            .text("The numerator is 0.9 times 0.02, which is 0.018. The denominator is that, plus the false alarms, 0.1 times 0.98, which is 0.098 — a total of 0.116. So the answer is 15.5 per cent. An indicator that catches nine crashes in ten leaves you, when it fires, more than five to one against a crash actually arriving. The reason is that calm months are 49 times more common, so their 10 per cent false-alarm rate produces far more firings than the crashes' 90 per cent hit rate does. The odds form says it in one line: prior odds of 1 to 49, multiplied by a likelihood ratio of 9, gives posterior odds of 9 to 49."))
        .figure(Figure::new(BASERATE_SVG,
            "Conditioning done with a pencil, on a thousand months. The top bar is the whole list before any news arrives: a sliver of 20 crash months at the far left, of which 18 fire, and then the 98 firings that come out of the 980 calm months, gathered next to them so they can be counted. Crossing out every month where the indicator stayed silent leaves the bracketed 116, and stretching those 116 back out to fill the bar is the renormalising step that makes them a share of 1 again. What comes out is the bottom bar, and the answer is read straight off it: 18 of the 116 are real, which is 15.5 per cent. Nothing in the picture disputes that the indicator catches nine crashes in ten. The 98 false alarms are simply drawn from a pool 49 times larger.")
            .width_percent(92))
        .para(|p| p
            .text("The odds form is worth carrying because it is the one that accumulates. Take logarithms of it and multiplication becomes addition: prior odds of 1 to 49 are minus 5.61 bits, a likelihood ratio of 9 is plus 3.17 bits, and their sum, minus 2.44 bits, is odds of 9 to 49 — the same 15.5 per cent. One bit of evidence is exactly a doubling of the odds, so Bayes' rule in this coordinate is a running total, and the base rate stops being a verdict and becomes a budget: you start 5.61 bits behind, each firing buys 3.17, and so one firing leaves you short while two put you ahead. Bits are the unit the information section will build from scratch; they are already here, and they are what makes evidence add up."))
        .rule()
        .para(|p| p
            .text("Now the confusion that costs the most money in practice: uncorrelated does not mean independent."))
        .para(|p| p
            .text("Correlation is built out of covariance, and covariance is an average of products. It can only see straight-line association, and it goes blind to anything symmetric. Take a variable that is equally likely to be minus 1, 0 or plus 1, and let the second variable be its square. The covariance is exactly zero, so the correlation is exactly zero — and yet the second variable is a function of the first, determined completely by it. Zero correlation, total dependence. There is no tension here; correlation was never measuring dependence, only the straight-line part of it."))
        .para(|p| p
            .text("Markets do this in a form you can see on any chart. The autocorrelation of daily returns is close to zero at every lag — which is why the direction of tomorrow is so hard to predict, and roughly what an efficient market should look like. The autocorrelation of the absolute size of those same returns is strongly positive and decays over weeks. Big days follow big days; quiet follows quiet. Returns are near-uncorrelated in sign and profoundly dependent in magnitude, and that single sentence is the reason volatility forecasting works while direction forecasting mostly does not."))
        .para(|p| p
            .text("Dependence also does something to risk that no expected-value calculation can reveal. Take a book of 100 bonds, each with a 5 per cent chance of defaulting over the year. If the defaults are independent, the expected number is 5 and the standard deviation is 2.18 — thirty defaults is off the map. Now suppose instead they all default together or not at all, driven by one common event of probability 5 per cent. The expected number is still exactly 5, unchanged, because linearity does not care. The standard deviation is 21.8, ten times larger, and one year in twenty every single bond in the book defaults at once. Two portfolios, one expected loss, entirely different businesses. That is, in outline, what happened to structured credit in 2007 — though not in the way it is usually told. The models did not assume independence. They modelled the dependence explicitly, with a correlation dialled well away from zero, and failed anyway, because they had the wrong shape of dependence rather than none of it. The danger was never hidden in the expected loss, because it cannot be."))
        .plot(Plot::new(1.0..=60.0)
            .curve("portfolio volatility, per cent", "16 * sqrt(1/x + (1 - 1/x) * correlation)")
            .curve("the floor no amount of diversifying can pass", "16 * sqrt(correlation)")
            .param("correlation", 0.0..=0.8, 0.3)
            .hline(16.0)
            .x_label("number of equally weighted assets in the portfolio")
            .y_label("annualised volatility, per cent")
            .height(300.0)
            .caption("Every asset has 16 per cent volatility and every pair shares the same correlation, which is the slider. Adding names crushes the portfolio's volatility at first and then stops: at the default correlation of 0.3 the first ten holdings take you from 16 per cent to 9.73, and the next fifty take you only to 8.93, because 8.76 is the floor. The floor is 16 times the square root of the correlation, and it is drawn as the second curve so you can watch the first settle onto it. Drag the correlation to zero and the floor drops to nothing, so the curve keeps falling forever — that is the textbook picture, and it exists only at a correlation no market has. Drag it up towards 0.8 and diversification is almost worthless before you start. The whole of what diversification can achieve is the gap between the two curves, and the correlation, not the number of holdings, decides how large that gap is."))
        .rule()
        .para(|p| p
            .text("One last piece of conditioning, because it turns out to be what regression is. Condition an expectation instead of a probability, and you get the best forecast of one quantity given another:"))
        .display(r"E[Y \mid X]")
        .explain(r"E[Y \mid X]", "The conditional expectation of Y given X",
            "The average of Y computed inside the smaller world where X is known. It is a function of X rather than a single number: read off a value of X and it returns the corresponding average of Y. Among all possible functions of X, this is the one that minimises the average squared forecast error — which is exactly the quantity a least-squares regression is built to minimise, so a regression is an attempt to estimate this object, restricted to straight lines. That restriction is what lesson-algebra-to-linear draws as perpendicularity: the fitted line is the shadow the truth casts on the space of straight lines, and the error is what sticks out at right angles. Condition one bell curve on another rather than one event on another and the same object becomes the update rule that lesson-kalman-filter is built from, where it goes by the name precisions add.")
        .para(|p| p
            .text("It comes with a property worth carrying. Average the conditional expectation back over all the values X can take and you recover the plain expectation of Y — the tower rule. Practically: a forecaster whose predictions average out to the right unconditional mean has demonstrated nothing at all, because that is automatic for any honest forecast, including the constant one that ignores X entirely. Skill lives in the variation of the forecast, not its average."))
        .para(|p| p
            .text("One more property, and it is the one the whole apparatus of forecasting rests on: among all possible functions of X, the conditional expectation is the one that minimises the average squared forecast error. That is a theorem rather than a convention — and it quietly settles a choice most people never notice they are making. Squared error rewards predicting the mean, which for a skewed distribution is not the most likely outcome and may be an outcome that never occurs. If what you need is the median, or the most likely value, or the fifth percentile, then squared error is scoring the wrong thing: absolute error hands you the median, and an asymmetric penalty hands you a quantile. The scoring rule chooses the summary. Pick the rule first."))
}

fn from_counting_to_measuring(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("From counting to measuring")
        .para(|p| p
            .text("Everything so far has rested on a list you could in principle write out. Tomorrow's return on a share is not on such a list. It could be minus 0.7 per cent, or minus 0.71, or minus 0.70413 — there is no finite catalogue of possibilities and therefore nothing to count. This is not a technicality to be waved away; the entire second half of probability exists to handle it, and the fix is a single substitution."))
        .para(|p| p
            .text("Stop counting outcomes. Start measuring them. Spread one unit of probability along the number line the way you would spread a kilogram of clay, and let the probability of a range be the amount of clay lying over it. Nothing about the three rules changes — the clay is never negative, there is exactly one unit of it, and clay over two separate ranges adds. Only the way you total it up changes."))
        .para(|p| p
            .text("Push the analogy and it breaks in two places. Real clay is made of atoms: halve a lump enough times and you reach a piece that cannot be halved again, sitting at a definite spot with a definite positive mass. Probability's clay has no smallest piece, which is precisely why a single point comes out weighing nothing here. And a kilogram of clay has to be spread over somewhere finite, while a distribution can be spread along the whole infinite line and still total exactly one."))
        .para(|p| p
            .text("Two consequences follow at once, and both are famous for unsettling people."))
        .para(|p| p
            .text("The first: every single point has probability zero. There is no clay sitting over a point, because a point has no width. And this is forced rather than chosen — if every value between 0 and 1 carried the same positive probability, then taking enough of them would push the total past 1, which rule two forbids. So probability zero cannot mean impossible in the continuous world. Whatever number tomorrow's return turns out to be, that exact number had probability zero this morning, and it happened anyway."))
        .para(|p| p
            .text("The second: since points carry nothing, the quantity you can actually quote is a probability per unit of width — a density. To get a probability back out of it you must multiply by a width, which for a curved density means adding up infinitely many thin strips:"))
        .display(r"P(a \le X \le b) = \int_a^b f(x) dx")
        .explain(r"P(a \le X \le b)", "The chance of landing between a and b",
            "The only kind of question a continuous quantity can answer. Ask instead for the chance of landing exactly on a single value and the answer is always zero, so every continuous probability is a probability of a range.")
        .explain(r"\int_a^b", "Total the strips from a to b",
            "The integral sign is a stretched S for sum: add up everything that follows, over the range written on it, in the limit where the pieces being added become infinitesimally thin.")
        .explain(r"f(x) dx", "One strip: height times width",
            "The density at x, multiplied by an infinitesimal width. Height times width is the whole idea — the density is a rate of probability per unit of x, so it becomes a probability only once a width has been supplied. This is also why a density may exceed 1 while a probability may not.")
        .para(|p| p
            .text("Which means a density is not a probability, and the units are the tell. A density has units of one over whatever is on the horizontal axis, so its value can exceed 1 without anything being wrong. A quantity spread evenly over the range from 0 to 0.1 has a density of 10 everywhere on that range, because 10 times 0.1 is 1. A bell curve with a standard deviation of 0.01 peaks at a density of 39.9. Both are perfectly ordinary; both would look like errors to anyone who thought the number on the vertical axis was a probability."))
        .figure(Figure::new(DENSITY_SVG,
            "Counting on the left, measuring on the right — and what changes is what you are allowed to read off the chart. On the left a probability is a height. On the right a height is only a rate, and the probability is the shaded area underneath it, which is why the strip has to be given a width before it means anything. Squeeze that strip to a line and its area goes to zero: the whole reason single points carry nothing.")
            .width_percent(88))
        .para(|p| p
            .text("The object that behaves itself in both worlds is the running total. Define the cumulative distribution function as the probability of coming in at or below a given value:"))
        .display(r"F(x) = P(X \le x)")
        .explain(r"P(X \le x)", "The chance of coming in at or below x",
            "Everything to the left of a cut, totalled. It is a probability rather than a density, so it is honestly bounded by 0 and 1, and it is defined the same way whether the quantity is discrete or continuous.")
        .explain(r"F(x)", "The cumulative distribution function",
            "How much of the clay lies at or to the left of x. It always runs from 0 to 1, never decreases, and — unlike a density — exists for discrete and continuous quantities alike, which is why it is the more fundamental of the two. The density is its slope: how fast probability is accumulating as you sweep rightwards, which makes the density a derivative and therefore a limit, in exactly the sense lesson-limits builds from zero. It also does a job no density can. Run a uniform draw between 0 and 1 backwards through this function and out comes a draw from the distribution itself — which is how a path gets simulated, and for the waiting time of the next section it is one line: minus the logarithm of a uniform draw, divided by the rate.")
        .para(|p| p
            .text("Read the cumulative function backwards and you have a quantile: the value below which a given share of the outcomes fall. That is not an abstraction — a value-at-risk number is precisely the 1 per cent quantile of a profit-and-loss distribution, and a median is the 50 per cent quantile. Quantiles are what desks actually quote, because they are the questions clients actually ask."))
        .para(|p| p
            .text("Put it on a book. A portfolio of $10 million with a daily volatility of 1 per cent has a daily standard deviation of $100,000. The 1 per cent quantile of a bell curve sits 2.326 standard deviations below the mean, so the one-day 99 per cent value-at-risk is $232,600. That is the whole computation, and what matters is reading it for what it is: a cut point, not a worst case. On 1 per cent of days the loss should be worse than it — which over a 252-day year is 2.52 days. And the count of those breaches is a binomial, so its standard deviation is the square root of 252 times 0.01 times 0.99, which is 1.58. Three breaches in a year is unremarkable; eight is three and a half standard deviations above expectation, which is not a bad year but a refuted model. Counting exceptions is how a value-at-risk model is actually tested, and the test is a distribution from the next section, run on the model's own claim."))
        .para(|p| p
            .text("A quantile is also silent about everything beyond it: value-at-risk says where the cliff edge is and nothing about the drop. Ask instead for the average loss given that the cut point is breached — the expected shortfall, $266,500 under a bell curve — and the tail assumption that barely moved the first number moves this one hard. Swap in a shape that fits returns tolerably and the value-at-risk rises 14 per cent while the expected shortfall rises 39. The assumption barely shows in the number that gets quoted and dominates the number that gets paid."))
        .para(|p| p
            .text("Finally, expected value. Nothing conceptual changes at all: it is still each value weighted by how much probability sits at it, with the sum replaced by an integral because the weights are now spread rather than piled."))
        .display(r"E[X] = \int_{-\infty}^{\infty} x f(x) dx")
        .explain(r"\int_{-\infty}^{\infty}", "Total over the whole line",
            "The same instruction as before, with the range opened out to everything: from arbitrarily far negative to arbitrarily far positive. If the total fails to settle on a finite number, the quantity has no mean at all — a possibility the fat-tailed section takes seriously.")
        .explain(r"x f(x) dx", "One value, weighted by the probability at it",
            "A value multiplied by the probability sitting in a thin strip around it — the continuous version of value times probability. It is the centre of mass of the clay, exactly as the discrete sum was the balance point of a set of weights on a beam. Linearity, the variance formula and the covariance term all survive this move unchanged — the discrete and continuous halves of the subject differ in how the total is computed and in nothing else.")
        .note("Which is the honest summary of this whole section. Discrete and continuous are not two subjects. They are one subject with two ways of adding up, and the only genuinely new idea is that a density has to be multiplied by a width before it means anything.")
}

fn the_family_of_distributions(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("The distributions that earn their place")
        .para(|p| p
            .text("A distribution is not a shape somebody found attractive. Each of the ones worth knowing is the unique answer to a specific question about a mechanism, and if your situation matches the mechanism then the distribution is not a choice you are making but a consequence you are stuck with. That is the only way to hold them in your head — not as a zoo of formulas, but as a family with a small number of parents."))
        .figure(Figure::new(FAMILY_SVG,
            "Eight distributions, in which the moves are the content — so read the arrows rather than the boxes. Blue counts and green measures. The tinted panel holds one object seen twice: ask a stream of arrivals how many came and you get the Poisson, ask it how long until the next and you get the exponential. Two arrows lead down through the central limit door, because a Poisson count over a long period is itself the sum of the counts over its pieces and goes bell-shaped for a large rate, exactly as a binomial does for large n. And the two violet arrows are the same move made twice: hold the shape but let one parameter be redrawn rather than held fixed, and the mean survives untouched while the tails fatten — redraw a Poisson's rate and the count becomes the negative binomial, redraw a bell curve's width and the return becomes the Student t.")
            .width_percent(92))
        .rule()
        .para(|p| p
            .text("Start with the smallest thing that can be uncertain at all: one yes-or-no. Call the chance of a yes ")
            .math("p")
            .text(". Score a yes as 1 and a no as 0, and the expected value and variance drop straight out of the definitions:"))
        .display(r"E[X] = p")
        .display(r"\mathrm{Var}(X) = p(1-p)")
        .explain(r"p(1-p)", "The variance of a single yes-or-no",
            "Largest at p equal to a half, where it is 0.25, and falling away to zero at both ends. A near-certain event has almost no variance because it almost always does the same thing — you are not uncertain about it. That the uncertainty peaks at even money is a fact this lesson meets again, in a different currency, when entropy arrives.")
        .para(|p| p
            .text("Add up ")
            .math("n")
            .text(" independent copies of that and count the yeses, and you have the binomial distribution — up days in a year, defaults in a book of identical names, wins in a run of trades:"))
        .display(r"P(K = k) = \binom{n}{k} p^k (1-p)^{n-k}")
        .explain(r"P(K = k)", "The chance the count comes out exactly k",
            "K is the count of yeses and k is a particular value it might take. Listing this probability for every k is what it means to give a distribution.")
        .explain(r"\binom{n}{k} p^k (1-p)^{n-k}", "Sequences, times the chance of one of them",
            "The right-hand piece prices one particular sequence: each of the k yeses contributes a factor of p and each of the n minus k noes a factor of one minus p, multiplied because the trials are independent — and that multiplication is the only step where independence is actually needed. The binomial coefficient in front counts how many distinct orderings produce the same total, since all of them are equally likely. Counting, exactly as in the first section.")
        .para(|p| p
            .text("Its mean is ")
            .math("np")
            .text(", which follows from linearity alone and therefore holds even if the trials are dependent. Its variance is ")
            .math("np(1-p)")
            .text(", and that one genuinely needs independence, because it needs every covariance term to vanish. The same split as before: means survive dependence, risks do not."))
        .explain(r"np", "The expected count",
            "The number of trials multiplied by the chance of each. It is the sum of n expected values, so linearity delivers it with no assumption about how the trials relate to one another.")
        .explain(r"np(1-p)", "The variance of the count",
            "The n individual variances added, which is legitimate only when the covariances are all zero. Dependence between trials leaves this number unchanged on paper and badly wrong in practice.")
        .para(|p| p
            .text("A track record is where both get used. A trader wins 55 of their first 100 trades. With no edge whatever, that count has mean 50 and variance 100 times a half times a half, which is 25, so a standard deviation of exactly 5. Fifty-five wins is one standard deviation above chance, and the probability of doing at least that well by luck alone is 18 per cent. It is not evidence of anything. Sixty wins is two standard deviations and starts to be interesting; seventy is four and would be remarkable. Now ask what it takes to establish a genuine 55 per cent hit rate rather than to notice one: the edge accumulates like the number of trades and the noise like its square root, so the edge of 0.05 per trade must outrun the noise, and it does so at 400 trades — mean 200, standard deviation 10, and 220 wins is the two-standard-error result. Four hundred trades to demonstrate an edge you already have. And the first section's warning comes with it: review twenty traders with no skill between them and 18 per cent of them, between three and four, will show 55 wins or better in their first hundred."))
        .rule()
        .para(|p| p
            .text("Now push the binomial to a limit that matters. Let the number of trials become enormous and the chance of each become tiny, holding their product fixed at ")
            .math("\\lambda")
            .text(". The individual trials stop being visible and what survives is a count of rare events in continuous time, with no upper limit:"))
        .display(r"P(K = k) = \frac{\lambda^k e^{-\lambda}}{k!}")
        .explain(r"\frac{\lambda^k e^{-\lambda}}{k!}", "The Poisson probability of exactly k events",
            "The binomial formula after the limit has been taken. Where the exponential comes from, since it is the one symbol here that looks like it wandered in from another subject: substitute p equal to lambda over n into the binomial and it splits into exactly four factors. One is lambda to the k over k factorial, already the front half of this formula. Two of them are a fixed number of terms each heading to 1, so they vanish. The only survivor is one minus lambda over n, all raised to the n — and that is the compound-interest limit from lesson-exponents run at a negative rate, which is the definition of the exponential. So e to the minus lambda is not a new object: it is literally the probability that nothing happens at all, and the formula reads as the chance of nothing, times a correction for the k that did. Check it at lambda equal to 0.61: the binomial gives 0.5329 at ten trials, 0.5423 at a hundred, 0.54325 at a thousand, against the exponential's 0.54335. And why the mean and the variance are forced equal: the binomial's are np and np(1-p), and the Poisson limit is by construction the one where p vanishes while np is held at lambda. The mean stays lambda; the (1-p) that is the only thing separating the variance from the mean goes to 1. Not a coincidence to be admired but the visible residue of each trial almost never firing — which is why a variance above the mean is evidence the opportunities were not independent.")
        .para(|p| p
            .text("That the mean and variance are both ")
            .math("\\lambda")
            .text(" is the most useful thing about it — and it is the binomial's one-minus-p vanishing along with p, not a coincidence — because it makes the model checkable in one line. Count events per period, take their mean and their variance, and compare."))
        .para(|p| p
            .text("The classic demonstration is grim and perfect. Bortkiewicz, in 1898, tabulated deaths by horse kick in the Prussian army: 200 corps-years, 122 deaths, so lambda is 0.61. From that single average the formula predicts 108.7 corps-years with no deaths, 66.3 with one, 20.2 with two, 4.1 with three and 0.6 with four; the observed counts were 109, 65, 22, 3 and 1. Nothing was fitted beyond the mean, and there is nothing about horses in the formula — only that the events were rare, numerous in opportunity, and did not influence one another."))
        .para(|p| p
            .text("On a desk the same distribution counts trades arriving in a minute, jumps in a year, and defaults in a diversified book. And it fails in the same place every time: real defaults and real jumps are contagious, so their variance outruns their mean. Assuming Poisson is assuming no contagion, stated in a way that hides what is being assumed."))
        .para(|p| p
            .text("Which is worth stating as a repair rather than a complaint, because the repair is a move you have already met. The Student t was a normal whose width is redrawn each morning; do the same to a Poisson and redraw its rate each period. If the rate is drawn from a gamma, the count that comes out is exactly the negative binomial, and its variance is inflated while its mean is not:"))
        .display(r"\mathrm{Var}(K) = \lambda + \frac{\lambda^2}{r}")
        .explain(r"\mathrm{Var}(K)", "The variance of the count",
            "The spread of how many events land in a period, once the rate is no longer held fixed. Compare it with the plain Poisson, where this same quantity is exactly lambda — the gap between the two is the whole of what mixing bought.")
        .explain(r"\frac{\lambda^2}{r}", "The inflation from letting the rate wander",
            "The extra variance a Poisson acquires once its rate is no longer a constant. The mean is untouched at lambda, and this term vanishes as r grows, recovering the Poisson exactly as infinite degrees of freedom recovered the normal. One move, both halves of the family tree: mixing a parameter leaves the mean alone and fattens everything past it. Fitted to a desk with a mean of 0.5 and a variance of 2.1, r comes out at 0.156, and that number is its own diagnosis — the rate's own coefficient of variation is one over the square root of r, which is 2.5, so the week-to-week rate varies with a standard deviation two and a half times its own average. It was never a constant.")
        .para(|p| p
            .text("But that is one of two stories and they are not the same business. Mixing is unobserved heterogeneity: the rate was always wandering and you could not see it. The alternative is contagion, where each event raises the chance of the next — a self-exciting process, whose rate is a base level plus a decaying kick from every event so far:"))
        .display(r"\lambda(t) = \lambda_0 + \sum_{t_i < t} \eta\, e^{-\beta(t - t_i)}")
        .explain(r"\lambda(t)", "The rate, now a function of time",
            "The arrival rate at this instant. In the plain Poisson process this was a constant, and making it depend on t — and specifically on what has already happened — is the entire difference between a process with no memory and one that feeds on itself.")
        .explain(r"\lambda_0", "The background rate",
            "The rate that would prevail if nothing had happened yet: arrivals that are genuinely their own cause. Everything above this level is the process reacting to itself.")
        .explain(r"\sum_{t_i < t}", "Added over every event so far",
            "The instruction to run through the times of all events that have already occurred and total their contributions. Only the past appears, which is what keeps the process causal — no arrival is influenced by one that has not happened.")
        .explain(r"\eta\, e^{-\beta(t - t_i)}", "One past event's push on the present",
            "The kick each earlier event contributes to the current rate, decaying at speed beta as it recedes. The interpretable number is the kick divided by the decay: the expected count of further events each event triggers directly, called the branching ratio, so an average cluster holds one over one-minus-that many events. There is an estimator that reads it off the two numbers already in hand — one minus the square root of the mean over the variance — which on a mean of 0.5 and a variance of 2.1 gives 0.512, so roughly half of that desk's events were triggered by earlier ones and the average burst holds about two. The identity is asymptotic in the window length, so on weekly counts it is an indication and not a measurement. Two rooms can produce identical overdispersed counts and not be the same world: admissions spike in a heatwave because the rate was drawn high for everybody, and spike in an epidemic because each case makes more cases. Two numbers cannot tell those apart. Only the timestamps can — and only the second story says a quiet week is evidence the next week will be quiet.")
        .para(|p| p
            .text("This also settles a debt. Volatility clustering has been asserted three times in this lesson with no mechanism behind it, in a lesson whose whole method is that a distribution is the unique answer to a question about a mechanism. Self-excitation is the mechanism, and it is the same one."))
        .para(|p| p
            .text("One more thing a large rate buys, and it puts the Poisson back through a door this lesson is about to open. Split the period into many small pieces: the count over the whole is the sum of the counts over the pieces, independent and of comparable size, so a Poisson count with a large mean is itself a bell curve — good in the middle and never in the tails, which is the same caveat that approximation carries everywhere else."))
        .para(|p| p
            .text("Before turning it round, name the thing being turned. The Poisson distribution and the waiting time that follows are not two family members joined by a resemblance; they are two questions asked of one object, and the object is the Poisson process. It is fixed by three assumptions and nothing else: events arrive at a constant rate per unit of exposure, what happens in one stretch of time says nothing about any disjoint stretch, and two events never arrive at the same instant. Ask that object how many arrived in a window and the Poisson formula is forced. Ask it how long until the next one and the exponential is forced. They were never separate."))
        .para(|p| p
            .text("Naming it buys structure a desk uses daily. Lambda is not a property of the world but a rate times an exposure, which is why counts over disjoint windows simply add — a year is the sum of its 252 days and is itself Poisson — and why the same model reads as 0.5 events a week or 26 a year interchangeably. It also fixes how well a rate can ever be known: the count's standard deviation is the square root of lambda, so relative noise is one over that square root, and a desk seeing four gap events a year knows its rate only to 50 per cent. Streams merge by adding their rates, and the deep version is the one that matters: superpose many independent sparse streams of any shape whatsoever and the total approaches Poisson. Sit with one trader and nothing is memoryless — they trade the open, do nothing through lunch, fire six orders in ninety seconds when the number prints. Add a thousand such clocks and what comes out has no clock at all. That is the argument for Poisson order flow, and it names its own failure: it holds because no single stream is a large share of the total, so it breaks exactly when every clock strikes together."))
        .para(|p| p
            .text("Turn the same mechanism round and ask not how many events but how long until the next one, and you get the exponential distribution:"))
        .display(r"P(T > t) = e^{-\lambda t}")
        .explain(r"P(T > t)", "The chance the wait is longer than t",
            "T is the time until the next event. This is a survival probability: the chance that nothing has happened yet by time t.")
        .explain(r"e^{-\lambda t}", "The chance of still waiting after time t",
            "The survival probability of a waiting time whose hazard — the chance of the event arriving in the next instant, given it has not yet — is the constant lambda. A constant hazard is what makes the survival curve exponential, and the average wait is one over lambda. Why the exponential and nothing else: memorylessness says the chance of surviving a further stretch does not depend on how long you have already waited, so surviving two stretches back to back has the same probability as surviving each separately, multiplied. That is an additive input producing a multiplicative output, and among functions that never increase there is exactly one family that does it. The information section meets the same equation running the other way — a multiplicative input producing an additive output — and is forced to the logarithm. Same functional equation, two directions, and the two answers are inverses of each other, which is not a coincidence but the reason both appear in one lesson.")
        .para(|p| p
            .text("Its defining property is memorylessness: given that you have already waited an hour, the distribution of the remaining wait is exactly what it was at the start. It is the only continuous distribution that does this, and the reason is the constant hazard — the process has no clock and no accumulated state. Which is also its warning label in finance. Modelling the time to the next crash as exponential is asserting that a crash is no more likely after five calm years than after one, and that a crash which is \"overdue\" is nothing of the kind. Sometimes that is right. It is rarely argued for."))
        .para(|p| p
            .text("Numbers make the warning bite. Suppose a market averages one day worse than minus 3 per cent a year, so the rate is one per year. The counting side says 37 per cent of years contain no such day at all, another 37 per cent contain exactly one, and 8 per cent contain three or more. The waiting side says the average gap is a year and the chance of going more than three years without one is 5 per cent. Memorylessness then says that after two entirely calm years the expected wait is still a year, and the chance of three more calm years is still that same 5 per cent. Nothing accumulates. For large market moves the variance of the yearly counts comfortably outruns their mean, so the model fails its own one-line test — and it is worth naming precisely what it failed at: the model that says a crash cannot be overdue is the same model that says crashes do not cluster, and it is the second half that the data refuses."))
        .rule()
        .para(|p| p
            .text("Now the door between the counting half of this lesson and the measuring half, and the single most consequential theorem in the subject. Add up many independent contributions of comparable size into a running total, subtract that total's mean and divide by its standard deviation, and the result approaches one fixed shape:"))
        .display(r"\frac{S_n - n\mu}{\sigma n^{1/2}} \to N(0, 1)")
        .explain(r"\frac{S_n - n\mu}{\sigma n^{1/2}} \to N(0, 1)", "The central limit theorem",
            "On the left, the total of n contributions, shifted so its mean is zero and scaled so its standard deviation is 1 — the scaling is by the square root of n because variances add and standard deviations are their square root, which is the same square root as the annualisation rule. On the right, the bell curve with mean 0 and standard deviation 1. The arrow says the left-hand shape settles onto the right-hand one as n grows, whatever the individual contributions looked like. Why this shape and no other: the limit has to be a shape that survives the operation that produced it, since a sum of twice as many terms is a sum of two sums, so adding two settled copies and rescaling must hand the settled shape back. The rescaling then divides away every feature except two — the lopsidedness of a sum of n copies is the single lopsidedness divided by the square root of n, the excess peakedness is divided by n, and every higher feature dies faster still, while the mean and the variance are exactly what the shifting and scaling hold fixed. So the surviving shape can carry no information beyond a mean and a variance, and there is precisely one distribution with nothing else in it. The normal is not a shape that happens to fit; it is the only shape with nothing left to divide away. Which also says why the conditions are the conditions: a finite variance is the thing you divide by, comparable sizes are what stop one term surviving the division, and independence is what makes those features add in the first place. Returns do have a finite variance, so the theorem does apply to them — what fails is the rate, since the higher features it needs are themselves infinite, which is why aggregated returns grow bell-shaped in the middle while the extremes stay fat longest. There is a second route to the same place, and for quantities that have a density it is the more satisfying one. Adding independent contributions and rescaling to hold the variance fixed can only destroy information about which piece was which, never create it, so the average surprise the total carries climbs as terms are added. At a fixed variance there is a largest that average surprise can be, and a quantity that increases and is bounded converges to its ceiling. The ceiling is the bell curve, by the one derivative in the entropy section. So the central limit theorem and the maximum-entropy argument are not two reasons the normal is everywhere: adding pushes a distribution uphill in entropy until it reaches the most non-committal shape available, and a finite variance is exactly what makes that ceiling exist.")
        .para(|p| p
            .text("The normal distribution is not a fact about nature. It is a fact about adding. Whatever the individual pieces look like — coin flips, dice, order sizes, anything with a finite variance — their sum forgets the shape and remembers only the mean and the variance. Two dice already show it: the sum of two flat distributions is a triangle, peaked at 7, and six dice are visually indistinguishable from a bell. The bell is what is left when the details cancel — not the winner of a competition, but the only entrant."))
        .para(|p| p
            .text("Which makes the conditions the important part, because they are precisely what markets violate: the contributions must be independent, of comparable size, and many. Where those hold the bell is inevitable. Where they do not, invoking it is a decision rather than a derivation — and the last section of this lesson takes the conditions apart one at a time."))
        .note("Worth memorising the three landmarks, because half of risk conversation is conducted in them: a normal quantity lands within one standard deviation of its mean 68.3 per cent of the time, within two 95.4 per cent, and within three 99.7 per cent. Every one of those numbers is an overstatement of calm for daily equity returns, and the final section measures by how much.")
        .rule()
        .para(|p| p
            .text("Prices, though, do not add. A price is the previous price multiplied by a factor, and a year is those factors multiplied together. So take logarithms — which turn multiplication into addition, the whole reason the logarithm exists — and the central limit theorem applies to the sum of the log returns instead. The log price is normal, so the price itself is the exponential of a normal, which is called lognormal."))
        .para(|p| p
            .text("This is why log returns are the natural quantity on a desk rather than percentage changes: log returns add across time, so a year is the sum of its days. It is also why prices modelled this way can never go negative, which is the right behaviour for a limited-liability share. And it produces a gap that the next section spends its time on:"))
        .display(r"E[S] = S_0 e^{\mu + \sigma^2/2}")
        .explain(r"E[S]", "The expected price",
            "The probability-weighted average of every price the share might be trading at on the horizon date. Note that this is an average of prices, not of returns, and averaging things that multiply is where the trouble in the next section begins.")
        .explain(r"S_0 e^{\mu + \sigma^2/2}", "The mean of a lognormal price",
            "The starting price grown by the average log return, and then multiplied once more by the exponential of half the variance. That extra half-variance is not drift anyone earns: it is the arithmetic of averaging a set of multiplicative outcomes whose upside is unbounded and whose downside stops at zero. Drop it and you have the median instead, which is the outcome that actually splits the paths in half. Where the half comes from: approximate any curved function near the mean by a straight piece plus a curvature piece. Averaging kills the straight piece outright, because the average distance from the mean is zero — that is what the word mean means, and it is the balance-point condition from the first section of this lesson. So the first thing that survives averaging is the curvature term, which carries a one-half from the expansion and multiplies the variance. Curvature times spread, halved: that is the entire content of the correction, and for the exponential the curvature equals the function itself, which is why the price is multiplied by it rather than shifted.")
        .para(|p| p
            .text("Take a share at 100 whose log return over the coming year has a mean of 6 per cent and a standard deviation of 20 per cent. Half the variance is 2 per cent, so the mean of next year's price is 100 times the exponential of 0.08, which is 108.33, while the median is 100 times the exponential of 0.06, which is 106.18. The 2.02 per cent between them is the whole of the half-variance term, and it is the reason more than half of all paths finish below the average path. The average is being held up by a thin tail of very large outcomes, and the median is what actually splits the paths down the middle."))
        .rule()
        .para(|p| p
            .text("Which leaves the last member of the family: what returns actually look like, as opposed to what the central limit theorem would like them to look like. Empirically, the chance of a move larger than ")
            .math("k")
            .text(" standard deviations does not fall off like a bell curve's exponential. It falls off like a power:"))
        .display(r"P(|R| > k) \approx C k^{-\alpha}")
        .explain(r"P(|R| > k)", "The chance of a move bigger than k",
            "How often the return, sign discarded, exceeds k standard deviations. Plotted against k, this is the single most informative picture of a return series, and the one that decides whether a risk model is honest.")
        .explain(r"C k^{-\alpha}", "A power-law tail",
            "The probability of a move bigger than k standard deviations, falling as a fixed power of k rather than as the bell curve's exponential of minus k squared over two. For daily equity returns, measured across many markets and many time scales, alpha comes out close to 3 — which is why it is called the inverse cubic law.")
        .explain(r"\alpha", "The tail exponent",
            "How fast the tail thins. It also decides which averages exist at all: moments up to but not including alpha are finite, and the rest are infinite. At alpha near 3 the mean and the variance exist, the fourth moment does not, and so a sample kurtosis is a fact about your sample rather than a property of the thing you sampled — it keeps climbing as you add data.")
        .para(|p| p
            .text("The Student t distribution says the same thing with a mechanism attached, and it is the mechanism worth picturing rather than the formula. You are not drawing tomorrow's return from one bell curve. You are drawing it from a bell chosen fresh each morning — a narrow one on most days, a much wider one on the days the market is frightened — and nobody tells you which one you were handed. Every single draw is normal. The year's histogram is not, and cannot be, because a stack of bells of different widths is not a bell. So the tail is not built out of extraordinary days but out of ordinary days drawn from an unusually wide bell, and the shape that comes out is normal-looking in the middle and power-tailed in the extremes, with the tail exponent equal to the degrees of freedom. Push those to infinity, the width stops varying, and the normal comes back exactly. Daily equity returns are fitted decently by three to five."))
}

fn a_price_path_assembled(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("A price path, assembled")
        .para(|p| p
            .text("There is now enough on the table to build the model a trading desk actually runs on, and the valuable part of doing it is watching exactly where each assumption enters — because when the model misbehaves later, you want to know which brick to pull out."))
        .para(|p| p
            .text("A price path is a product of daily factors. Take logarithms and it is a sum of daily log returns. Suppose each day's log return has some mean and some standard deviation, the same every day. Over ")
            .math("n")
            .text(" days the mean of the total is ")
            .math(r"n\mu")
            .text(" by linearity, with no assumption of any kind. The variance of the total is ")
            .math(r"n\sigma^2")
            .text(" only if the covariance terms all vanish, so:"))
        .explain(r"n\mu", "The drift accumulated over n days",
            "One day's average log return, multiplied by the number of days. Linearity of expectation delivers it whatever the days do to one another, which is why this half of the scaling story is never in question.")
        .explain(r"n\sigma^2", "The variance accumulated over n days",
            "One day's variance multiplied by the number of days — legitimate only when every covariance between days is zero. This is the assumption the whole annualisation convention rests on, and volatility clustering is the observation that it is not quite true.")
        .display(r"\sigma_n = \sigma n^{1/2}")
        .explain(r"\sigma_n", "The volatility over n days",
            "The standard deviation of the total log return accumulated over a stretch of n days, as opposed to the standard deviation of one day.")
        .explain(r"\sigma n^{1/2}", "One day's volatility, times the square root of the days",
            "The square-root-of-time rule, which is what every annualisation on every risk report is doing. The square root is there because variances add and the standard deviation is their square root — so it is not a property of markets but the arithmetic consequence of one assumption, that the days do not covary.")
        .para(|p| p
            .text("Which is the point of deriving it rather than quoting it. The return half scales straight with time no matter how dependent the days are; the risk half scales with the square root of time only because the covariances were assumed away. One of those two halves is a theorem and the other is a modelling choice wearing the same clothes."))
        .para(|p| p
            .text("With numbers: a 1 per cent daily standard deviation over 252 trading days gives 15.87 per cent a year, because the square root of 252 is 15.87. That root is near enough 16, which is where the desk shorthand of dividing an annual volatility by 16 to get a daily one comes from."))
        .para(|p| p
            .text("Apply the central limit theorem to the sum and the total log return is normal, so the price itself is lognormal. That is the geometric random walk: Bachelier reached its additive version in his 1900 thesis on the Paris bourse, and it sits underneath the Black-Scholes formula and underneath very nearly every value-at-risk number computed today. Every ingredient in it has now been built in this lesson."))
        .para(|p| p
            .text("Except one, and the distribution that supplies it was built one section ago. Let events arrive as a Poisson process, and let each event carry a random size drawn independently of the rest. Their total is a compound Poisson, and it needs only two numbers: the mean is the rate times the average size, and the variance is the rate times the average of the size squared. Note which one appears — the mean of the square, not the variance of the size. The two differ by the square of the mean, and the consequence is worth stopping on: a jump whose average size is exactly zero still adds risk, because a squared quantity has nothing to cancel against."))
        .para(|p| p
            .text("Bolt it onto the path just built. Take the 16 per cent annual volatility from above and add one jump a year, averaging minus 3 per cent with a standard deviation of 5 per cent about that. The average square is 0.0009 plus 0.0025, or 0.0034, so the annual variance goes from 0.0256 to 0.0290 and the volatility from 16 per cent to 17.03. One percentage point. No risk report would blink at it — and yet that 0.0034 is 11.7 per cent of the year's variance, delivered on one day in 252. Four tenths of one per cent of the days carrying nearly an eighth of the risk. Set the jump's average to zero and the volatility is still 16.76, because the risk was in the square all along."))
        .para(|p| p
            .text("Look at what that one point of volatility bought. On the drift the jump is enormous — minus 3 per cent a year comes straight off the expected return, halving a 6 per cent drift with one day. On the volatility it is nearly invisible. And on the tail it is the entire story, because the diffusion contributes no excess peakedness whatsoever and the jump contributes all of it: at a daily horizon this path's excess kurtosis is 9.9, up from exactly zero. Jumps hide in the second moment and live in the fourth — this section's own asymmetry pushed one moment further. Measured annually that same kurtosis is 0.039, smaller by a factor of exactly 252, so aggregation does not remove jumps but hides them, which is why a model calibrated on annual data reports that daily jumps do not exist. This is Merton's 1976 repair to the formula named above, and the exponential's one-bad-day-a-year example was never an aside. It was this jump, at this rate."))
        .rule()
        .para(|p| p
            .text("Now the consequence that decides what quantitative finance can and cannot do, and it falls straight out of the two scalings just derived. The signal-to-noise ratio of an estimated average return improves as the square root of the sample and no faster:"))
        .display(r"t = \frac{n\mu}{\sigma n^{1/2}}")
        .explain(r"\frac{n\mu}{\sigma n^{1/2}}", "Accumulated return over accumulated risk",
            "The drift you have collected after n periods, divided by the standard deviation of what you have collected. The n on top and the square root of n below leave the square root of n as the growth rate of the whole ratio, which is the same square root as the law of large numbers — and it is exactly the ratio a t-statistic reports.")
        .para(|p| p
            .text("Put a fund in it. A Sharpe ratio of 0.5 — an 8 per cent return against 16 per cent volatility, which is a respectable long-run equity-like figure — gives a t-statistic of 0.5 after one year, 1.0 after four years and 2.0 after sixteen. Sixteen years of data to establish, at two standard errors, that the return is not zero at all. That is not a criticism of anybody's fund. It is the arithmetic of the square root, and it applies to every fund there has ever been."))
        .para(|p| p
            .text("The second moment is a completely different story. The relative error of a volatility estimate made from ")
            .math("n")
            .text(" observations is about one over the square root of twice ")
            .math("n")
            .text(", so a single year of daily data pins a 16 per cent volatility to within about 0.7 percentage points, and even a single quarter gets you to 1.4. Weeks, not decades."))
        .note("That asymmetry — risk measurable in weeks, return not measurable in a career — is not a cultural preference for caution. It is why the entire apparatus of the industry, the risk models and the factor models and the volatility targeting and the position limits, is built around the second moment. The profession concentrates on variance because variance is the part you can actually measure.")
        .para(|p| p
            .text("One last consequence, because it settles an argument people have without noticing they are having two of them. Since return accumulates like time and risk like its square root, holding for longer genuinely does make a loss less likely — at 8 per cent against 16 per cent volatility the chance of being down falls from about 34 per cent after one year to about 9 after ten — while the size of a plausible shortfall grows, because that scales with the square root of time. Time makes a loss rarer and, when it does arrive, larger. Both camps are quoting a true fact, and each is quoting the one that suits."))
        .rule()
        .para(|p| p
            .text("Every question so far has asked where the path finishes. Nobody experiences an endpoint. Traders are stopped out mid-year, funds are redeemed at the drawdown, and a strategy is judged on the shape of its equity curve — and what a path does on the way is the least intuitive thing in this lesson. Start with the cleanest result. For a path with no drift, the chance of ever touching a level is exactly twice the chance of finishing beyond it. The proof is a picture: reflect the path about the level from the moment it first touches, and the reflected paths pair off one-to-one with the paths that finish beyond, so touching is worth double. At 16 per cent annual volatility a 10 per cent fall is closed below 25.5 per cent of the time and touched at some point during the year 51.0 per cent of the time. Half the years visit a level only a quarter of them close beyond, which is to say your stop-loss is hit about twice as often as the year-end distribution suggests it should be, and nothing has gone wrong. It is also why a value-at-risk number computed on endpoints is silent about the year it was computed over."))
        .plot(Plot::new(0.01..=0.99)
            .curve("the tally: fraction of the flips that came up heads",
                "sqrt(2 * flips / 3.14159265) * exp(-2 * flips * pow(x - 0.5, 2))")
            .curve("the lead: fraction of the time spent on the winning side",
                "1 / (3.14159265 * sqrt(x * (1 - x)))")
            .param("flips", 10.0..=150.0, 30.0)
            .vline(0.5)
            .x_label("fraction of the run")
            .y_label("probability density")
            .height(300.0)
            .caption("Two questions asked of one fair coin, and one slider answers both. The first curve is the tally: what fraction of the flips came up heads, which is the bell curve the central limit theorem hands you. The second is the lead: what fraction of the time the running total spent on the winning side. Drag the number of flips and watch only one of them respond. The tally does exactly what this lesson promised — at 30 flips it lands between 45 and 55 per cent 41.6 per cent of the time, at 150 flips 77.9 per cent of the time, and it goes on narrowing forever. The lead does not narrow at all. It is the same shape at ten flips as at ten thousand, and the shape is a bowl: its lowest point sits at exactly a half, so the least likely thing a fair coin does is split its time evenly, and the most likely thing it does is spend nearly the whole run on one side. Take two equal tenths of the axis and compare their areas under it: one side holds the lead for more than 95 per cent of the run 28.7 per cent of the time, against 6.4 per cent for a 45-to-55 split. Check it by hand on four flips — of the sixteen equally likely paths, twelve spend either all the time ahead or none of it, and only four split it down the middle. This is the arcsine law, and what it says is that the shape of an equity curve is not evidence of anything."))
        .para(|p| p
            .text("Sit with what that means in front of a screen. A manager sends in a year's equity curve: down through February and March, a turn in April, then eight months quietly above water, finishing up. It has a shape. It has a story — a bad start, a lesson learned, a regime found. The manager was flipping a coin, and this is simply what coin flipping looks like. Everything in this lesson so far has been armour against over-claiming from numbers. This is the over-claiming that happens before anybody reaches for a number."))
}

fn expected_value_in_full(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Expected value, in full")
        .para(|p| p
            .text("Everything so far has treated the expected value as obviously the right summary of an uncertain quantity. It very often is, and it is the most reliable tool in the box. But there are three distinct ways it can be the wrong number, they catch different people for different reasons, and every one of them has cost real money at institutional scale. Each one has the same shape: the expectation is computed correctly and then read as something it never claimed to be."))
        .rule()
        .para(|p| p
            .text("The first is the one that generalises all the others. Averaging the inputs to a function is not the same as averaging its outputs:"))
        .display(r"E[g(X)] \ge g(E[X])")
        .explain(r"E[g(X)]", "The average of the transformed quantity",
            "Push each possible value of X through the function g first, then average the results. This is what actually happens to you: the world picks an outcome, the function acts on that outcome, and the average is taken over many such episodes.")
        .explain(r"g(E[X])", "The transform of the average",
            "Average X first, then push the single resulting number through g. This is what a spreadsheet does when it applies a formula to a forecast, and outside of straight lines it answers a different question from the one above.")
        .para(|p| p
            .text("This is Jensen's inequality, and the direction of the sign is settled by the shape of the function. A function that curves upwards — convex, like a square, an exponential, or an option payoff — always sits below the straight chord joining two of its points. Averaging the outputs slides along the chord; averaging the inputs slides along the curve underneath it. So for a convex function the average output is the larger of the two. Flip to a curve that bends downwards, like a logarithm or a square root, and the inequality flips with it. The two agree only when the function is a straight line, which is exactly the case linearity of expectation covers."))
        .para(|p| p
            .text("The cleanest illustration is an option, and it explains the entire existence of the instrument. A share will be worth either 90 or 110, equally likely, so its expected price is 100. A call struck at 100 pays nothing at 90 and 10 at 110, so its expected payoff is 5. But the payoff evaluated at the expected price is zero. The whole value of an at-the-money option is the Jensen gap of a kinked, convex payoff — and because the gap widens as the two outcomes spread apart, an option is worth more when volatility is higher. That is not a modelling result. It is a statement about convex functions that would be true if nobody had ever written a pricing model."))
        .figure(Figure::new(JENSEN_SVG,
            "Jensen's inequality with the option numbers in it. Averaging the payoffs slides along the dashed chord and lands at 5, directly above a price of 100; averaging the prices first lands on the kink underneath, at 0. The amber bar between them is the gap, and for an at-the-money option the gap is the entire value — there is nothing else in it. Push the two outcomes further apart, to 80 and 120, and both dots slide outward along the same payoff line while the chord lifts to 10: the gap grows with the spread, which is the whole of why an option is worth more when volatility is higher.")
            .width_percent(78))
        .para(|p| p
            .text("Run it the other way with a concave function and the loss is just as real. A quantity that is 1 or 9 with equal chance has a mean of 5, whose square root is 2.236, while the average of the two square roots is 2. Anywhere you take a square root, a logarithm, or an average of ratios, the same gap is opening beneath you."))
        .rule()
        .para(|p| p
            .text("The second failure is Jensen's inequality wearing a suit, and it is the one that ruins people. Consider a coin that gains you 50 per cent of everything you have when it lands heads, and costs you 40 per cent when it lands tails. Fair coin, and you reinvest each time."))
        .para(|p| p
            .text("The expected value per round is unarguable: half of 1.5 plus half of 0.6 is 1.05, a 5 per cent expected gain every single round. Now ask what actually happens to you. Over any long run, close to half the flips come up heads and close to half tails, and the order does not matter because multiplication commutes — so your wealth is multiplied by 1.5 and 0.6 in roughly equal numbers, which is a factor of 0.9 for every pair, or 0.9487 per round. You lose 5.13 per cent per round, essentially certainly, in a game with a 5 per cent expected gain."))
        .para(|p| p
            .text("After fifty rounds the average across all possible paths is 11.5 times the stake, and the typical path is 0.072 times the stake — down 93 per cent. Both numbers are correct. The expectation is being carried by a vanishing minority of enormous paths, and the minority is where all the money is."))
        .plot(Plot::new(0.0..=25.0)
            .curve("the average across all paths", "pow(((1 + gain) + 0.6) / 2, x)")
            .curve("the path you are actually on", "pow(sqrt((1 + gain) * 0.6), x)")
            .param("gain", 0.20..=0.70, 0.50)
            .hline(1.0)
            .x_label("rounds played, reinvesting everything each time")
            .y_label("multiple of the original stake")
            .height(300.0)
            .caption("Heads multiplies your wealth by one plus the slider; tails multiplies it by 0.6. At the starting gain of 0.5 the two curves separate immediately and never meet again: by round 25 the average is 3.39 times the stake and the path you are on is 0.27. The dashed line at 1 is breaking even. Now drag the slider and find the two crossings, which are not in the same place. The average only turns upward at a gain of 0.4, where 1.4 and 0.6 average to exactly 1 and the green curve flattens onto the dashed line. The typical path only stops losing at a gain of 0.667, where 1.667 times 0.6 is exactly 1 — and there the average is still climbing at 13.3 per cent a round. Every setting between those two is a bet with a positive expected value that impoverishes almost everyone who takes it. The ensemble is not the individual, and no amount of correct arithmetic about the ensemble will tell you what happens to the individual.")
            )
        .note("The name for this is non-ergodicity: the average over many parallel players and the average over one player's long run are different numbers. A process that adds is ergodic in this sense and a process that multiplies is not, which matters because almost everything in finance multiplies. Returns compound. Bankrolls compound. Only a fixed-stake bet on the side, funded from outside, genuinely adds.")
        .rule()
        .para(|p| p
            .text("The same fact arrives on a desk under a plainer name. If a return compounds, the rate at which wealth actually grows is not the average return but the average return less half the variance:"))
        .display(r"g \approx \mu - \frac{\sigma^2}{2}")
        .explain(r"g", "The compound growth rate",
            "What the account actually grows at over a long run — the geometric mean return. It is what determines where you end up, whereas the arithmetic mean determines the average of where everyone ends up, which is a different and much less personal question.")
        .explain(r"\frac{\sigma^2}{2}", "The volatility drag",
            "Half the variance of the return, subtracted from the average return to get the growth rate. It is exactly the half-variance term inside the lognormal mean, and exactly the Jensen gap for the logarithm, which is concave — three descriptions of one piece of arithmetic. And it is subtracted whatever the returns are doing: volatility costs growth even when nothing is going wrong. The reason all three coincide is that there is only one calculation underneath. Approximate a curved function near the mean, average it, and the straight part vanishes because deviations from the mean average to zero, leaving half the curvature times the variance. Feed that one expression three functions and it produces the whole family: the exponential curves upwards and its curvature is itself, giving the lognormal's multiply-by-the-exponential-of-half-the-variance; the logarithm curves downwards, so the same size of term arrives with a minus sign, and that is the volatility drag; and Jensen's inequality is the same expression read for its sign alone, with a straight line giving equality — which is exactly the case linearity of expectation covers. Four results, one Taylor expansion, looked at from four sides.")
        .para(|p| p
            .text("Put three funds side by side, each with an 8 per cent arithmetic average annual return — the number a factsheet prints. At 20 per cent volatility the compound growth is 6 per cent. At 40 per cent it is exactly zero. At 60 per cent it is minus 10 per cent a year, and the fund goes to nothing while its average return stays resolutely positive. Same expected return, three completely different fates, and the only difference is the width of the distribution."))
        .para(|p| p
            .text("It also explains, with no reference to fees or tracking error, why a leveraged fund reliably lags a multiple of its index over time. Leverage multiplies the average return above cash by the leverage factor but multiplies the variance by its square, so the drag grows faster than the return does. Double leverage doubles the 8 to 16 and quadruples the drag from 2 to 8, so the fund compounds at 8 per cent while twice the index compounds at 12 — and the gap widens with the square of the leverage. Push to four times and the drag has eaten the whole 32: the expected return is at its most impressive and the growth rate is exactly zero."))
        .rule()
        .para(|p| p
            .text("The third failure is the oldest, and it is the case where the expected value is not merely misleading but infinite. Nicolaus Bernoulli posed it in 1713. A coin is flipped until it comes up heads; if that happens on the first flip you are paid 1, on the second 2, on the third 4, doubling each time. The chance of reaching round ")
            .math("k")
            .text(" halves each round while the prize doubles, so every round contributes exactly one half to the expectation, and there are infinitely many rounds. The expected payoff is infinite."))
        .para(|p| p
            .text("Nobody will pay 100 to play it, let alone infinity. Daniel Bernoulli's 1738 resolution was that people value the logarithm of wealth, which is concave, so Jensen bites and the expected utility is finite and small. The blunter one needs no psychology at all: no counterparty can pay an unbounded prize. Cap the bank at about a million and the sum has only twenty terms left in it, so the fair price falls to 10. The infinity was living entirely in outcomes the counterparty could never honour."))
        .note("The transferable lesson is worth more than the puzzle. An expectation is a sum, and a sum can be dominated by terms whose probability is negligible. Whenever the mean of a distribution is carried by its far tail — as it is for insurance losses, venture returns, and anything with a power-law tail — the mean is a real number that describes nothing anybody will experience, and quoting it alone is a way of not answering the question.")
        .rule()
        .para(|p| p
            .text("Which brings the repair, and it is a single move: when outcomes multiply, take the expectation of the logarithm instead. The logarithm is the function that turns multiplication into addition, so it converts a compounding process into an adding one, and the law of large numbers — which only ever worked on sums — applies again. Maximising the expected log of wealth is therefore maximising the long-run growth rate, and it produces a definite answer to the question of how much to bet:"))
        .display(r"f^* = p - \frac{1-p}{b}")
        .explain(r"f^*", "The Kelly fraction",
            "The share of the bankroll to stake on each bet in order to maximise the long-run growth rate. It is a fraction of current wealth, not a fixed sum, so a losing run shrinks the stakes automatically and the bankroll can never reach zero. Kelly derived it at Bell Labs in 1956, working on the capacity of a noisy communication channel rather than on gambling. Where it comes from: maximise the expected logarithm of wealth, which means setting the probability-weighted marginal gain from the last unit staked equal to the probability-weighted marginal loss — the same balance-point condition that defined the mean in the second section of this lesson, now applied to growth instead of position. Solving gives the formula above. Re-express it against a price implying probability q and the fraction becomes your edge over one minus the price, and then look at what you are left holding: a win multiplies your wealth by your probability divided by theirs, and so does a loss. Kelly divides the bankroll across the outcomes in proportion to your beliefs, which is why the growth rate turns out to be the distance between two distributions and nothing else. And the shape of the curve is one fact, not three: at the peak the slope is zero, so near its top the growth curve is the parabola that passes through zero at a zero stake and turns over at the Kelly fraction — which reads off directly as three quarters of the maximum at half of Kelly and exactly zero at twice it. The asymmetry the plot warns about is the next term: the logarithm of one minus the stake runs away as the stake approaches the whole bankroll, which flattens the left side and steepens the right, and is why the true zero sits at 0.389 rather than 0.4.")
        .explain(r"\frac{1-p}{b}", "The chance of losing, divided by the odds",
            "What the bet has to overcome. Subtracting it from the chance of winning leaves the size of your edge measured in units of the odds on offer. When the two are equal the fraction is zero: with no advantage the growth-maximising stake is nothing at all, whatever odds are being waved at you. Generous odds b shrink this term, which is why a long shot at the right price can still be worth a stake.")
        .para(|p| p
            .text("At even money with a 60 per cent chance of winning, the fraction is 0.6 minus 0.4, which is 0.2 — stake a fifth of the bankroll. The resulting growth is 2.01 per cent per bet. Bet more and it falls, and the way it falls is the whole point of the plot below."))
        .plot(Plot::new(0.0..=0.6)
            .curve("growth rate per bet, in per cent", "100 * (win * ln(1 + x) + (1 - win) * ln(1 - x))")
            .curve("expected profit per bet, in per cent", "100 * (2 * win - 1) * x").secondary()
            .param("win", 0.50..=0.75, 0.60)
            .hline(0.0)
            .vline(0.2)
            .x_label("fraction of the bankroll staked on each bet")
            .y_label("long-run growth per bet, per cent")
            .y2_label("expected profit per bet, per cent")
            .height(300.0)
            .caption("Two ways to score the same even-money bet, on a chance of winning you can drag. The expected profit is a straight line rising forever, so maximising it says stake everything — which loses everything the first time the bet goes against you. The growth rate is the curve, and at the default 60 per cent it peaks at 2.01 per cent per bet at exactly the marked fifth of the bankroll, then falls, crossing zero at 0.389. Betting twice Kelly earns nothing at all while carrying every ounce of the risk. Notice how flat the peak is on its left: half of Kelly, at 0.1, still returns 1.50 per cent, three quarters of the maximum for half the exposure, which is why experienced people bet fractions of Kelly and nobody sensible bets over it. Drag the chance of winning down towards 0.5 and the whole curve sinks below zero except at the origin — with no edge, the only growth-maximising stake is nothing.")
            )
        .rule()
        .para(|p| p
            .text("So when is the plain expected value exactly the right number? When the bets are many, independent, and small relative to the bankroll, and when the stakes add rather than compound. Under those conditions the law of large numbers does its work, the arithmetic and geometric means converge, and ruin is off the table."))
        .para(|p| p
            .text("That is precisely the insurer's position, and the market maker's, and the casino's. All three run on expected value and all three are right to: thousands of small independent exposures, none of which can take the firm down. It is not the position of anyone using leverage, concentrating a position, or facing exposures that all fail together — which is to say it is not the position of most people who invoke it. The condition that makes expected value correct is not that the calculation is right. The calculation is always right. The condition is that you get to play often enough for the average to arrive, and that you are still solvent when it does."))
}

fn randomness_and_information(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Randomness, and what information costs")
        .para(|p| p
            .text("A shuffled deck of cards is completely determined. Every card is already in a fixed position, and nothing about the deck will change between now and the moment you turn one over. Yet the top card is as random as anything in this lesson. Randomness, at least the kind anybody actually deals with, is not a property of the world. It is a property of the gap between the world and what you know about it."))
        .para(|p| p
            .text("Which invites an obvious question, and answering it turns out to unify everything above. How much is that gap worth? Start with something smaller: how much is a single surprise worth? Any sensible answer has to satisfy three things. An outcome you were certain of is worth nothing when it arrives. A rarer outcome is worth more than a common one. And two independent surprises should be worth the sum of their separate values, because information from unrelated sources ought to accumulate."))
        .para(|p| p
            .text("That third requirement is the binding one. The probabilities of independent events multiply, and the values are required to add, so the function converting one into the other must turn multiplication into addition. Exactly one family of functions does that, and it is the logarithm. So the value of a surprise is not a modelling choice; it is forced:"))
        .display(r"s(x) = \log_2 \frac{1}{P(x)}")
        .explain(r"s(x)", "The surprisal of the outcome x",
            "How surprising it is that x happened, measured in bits. Base 2 makes the unit a yes-or-no question's worth of information, which is why bits are the convention; base e gives nats and changes nothing but the scale.")
        .explain(r"\log_2 \frac{1}{P(x)}", "The logarithm of one over the probability",
            "One over the probability is how many equally likely alternatives the outcome was picked out of, and taking the logarithm base 2 counts how many halvings that is — how many yes-or-no questions it would have taken to single it out. A certainty gives the logarithm of 1, which is 0; a one-in-a-hundred event gives 6.64 bits. Why the logarithm is forced, and what each requirement actually does: the demand that independent surprises add gives an equation, since squaring a probability must double the value, cubing it must treble it, and running that backwards fixes the value on every fractional power too — so on powers of a fixed base the value is exactly proportional to the exponent. The demand that rarer be more surprising is what upgrades that from a dense scattering of points to the whole function, and it is load-bearing rather than decorative: without it the equation admits monstrous solutions. The demand that a certainty be worth nothing turns out not to be an assumption at all, since the equation already forces it. What is left free is a single constant, and setting one fair coin flip to be worth exactly one unit is the convention that names that unit the bit.")
        .explain(r"P(x) \log_2 \frac{1}{P(x)}", "One outcome's contribution to the average surprise",
            "The surprisal of an outcome, weighted by how often that outcome actually occurs. Rare outcomes are very surprising but seldom seen, and common ones are seen constantly but tell you little, so each term is a product of a large number and a small one — which is why the total peaks in the middle rather than at either extreme.")
        .para(|p| p
            .text("Now average the surprisal over everything that might happen. This is the expected value of the first half of this lesson, applied to surprise, and it is called entropy:"))
        .display(r"H(X) = \sum_x P(x) \log_2 \frac{1}{P(x)}")
        .explain(r"H(X)", "The entropy of X",
            "The average surprise you should expect from an observation, in bits — equivalently, how many yes-or-no questions it takes on average to pin down what happened. Shannon introduced it in 1948 for the capacity of a communication channel, and it is nothing more exotic than an expected value whose random quantity happens to be surprisal. It is also what picks a distribution out of thin air, and the mechanism is one derivative. Maximise it subject to whatever you claim to know, attaching a multiplier to each claim, and the condition that falls out is that the logarithm of the probability must be linear in the quantities you constrained. Exponentiate that and the answer is forced: constrain nothing but a range and the probability is constant, giving the flat distribution; constrain the mean of a positive quantity and it is an exponential; constrain the mean square and it is a bell curve. Three distributions, one derivative — which also means the exponential now has three separate derivations in this lesson, as the waiting time of a Poisson process, as the only memoryless survival curve, and as the least committed distribution with a known mean. They agree because all three are the same functional equation.")
        .para(|p| p
            .text("A fair coin has entropy 1 bit: one question, and you know. A fair die has 2.585 bits, which is the logarithm of 6 — and the fact that it is not a whole number is honest rather than awkward, since it says a die cannot be resolved by any fixed number of yes-or-no questions but can be resolved by 2.585 of them on average if you ask about many dice at once. A coin biased to 60 per cent has entropy 0.971 bits. At 90 per cent it is 0.469, and at 99 per cent it is 0.081: you already know what will happen, so watching it happen tells you almost nothing."))
        .para(|p| p
            .text("Entropy is largest for the flat distribution, for the same reason the variance of a yes-or-no is largest at even money: uncertainty peaks when nothing is favoured. Read that as a rule for choosing distributions rather than merely describing them and it says: among all distributions consistent with what you actually know, pick the one with the most entropy, because any other choice smuggles in an assumption you cannot justify. Know nothing but the range, and that rule hands you the flat distribution. Know only the average of a positive quantity, and it hands you the exponential. Know only the variance, and it hands you the normal."))
        .para(|p| p
            .text("That last one is worth sitting with, because it is a second reason the bell curve is everywhere — and, looked at properly, the same reason. It is not merely what sums converge to. It is the least committed thing you can say about a quantity whose variance you know and whose shape you do not. Which also explains precisely why it fails on returns: with returns you do know more than the variance — you know from a century of data that the tails are far heavier than any bell curve's — and a maximum-entropy argument only licenses ignoring what you do not know."))
        .rule()
        .para(|p| p
            .text("Next, what a wrong model costs. Suppose the truth is one distribution and you are working from another. Your surprisal is computed with your probabilities, but the outcomes arrive according to the truth, so you pay more surprise on average than you needed to. The excess has a name and a formula:"))
        .display(r"D = \sum_x p(x) \log_2 \frac{p(x)}{q(x)}")
        .explain(r"D", "The divergence of q from p",
            "The extra bits per observation you pay for believing q when the truth is p — usually written with a double bar between the two distributions and named after Kullback and Leibler. It is never negative, and it is zero only when the two distributions agree everywhere. It is not symmetric, and the asymmetry is meaningful rather than a defect: assigning a near-zero probability to something that then happens is catastrophic, while assigning a fat probability to something that never happens is merely wasteful.")
        .explain(r"p(x) \log_2 \frac{p(x)}{q(x)}", "One outcome's share of the penalty",
            "For each outcome, how many bits your belief was off by, weighted by how often the truth actually delivers that outcome. Weighting by the truth rather than by your belief is the whole reason this measures your cost rather than your comfort.")
        .para(|p| p
            .text("The asymmetry is not a technicality, and it carries a bill you can compute. A bell curve prices a 20-standard-deviation day at 5.5 times ten to the minus 89, and the last section of this lesson says where such a day came from. Suppose it happens. Your surprisal for that one day is the logarithm base 2 of one over that number, which is 293 bits — two hundred and ninety-three yes-or-no questions' worth of being wrong, out of a single Monday. A model carrying the cubic tail from the distributions section calls the same day a once-in-436-years event and pays about 17 bits when it arrives, and the caution that bought that reduction costs it fractions of a bit spread across all the ordinary days in between. That is the whole of why the divergence runs the way it does. Over-confidence about a tail is not a slightly worse model; it is a bill nobody can settle, presented on precisely the day you needed the model to work."))
        .rule()
        .para(|p| p
            .text("And now the identity that ties the entire lesson together. Take an even-money bet — a market priced as a coin flip — where you believe the true probability is ")
            .math("p")
            .text(". Stake the Kelly fraction from the previous section, and the rate at which your bankroll compounds, measured in bits per bet, is exactly:"))
        .display(r"g = 1 - H(p)")
        .explain(r"H(p)", "The entropy of your own belief",
            "The market has priced the outcome as a fair coin, which is 1 bit of uncertainty. Your own remaining uncertainty is the entropy of your belief. The difference is what you know that the price does not, and it is also — identically, not approximately — the divergence of the market's distribution from yours. Kelly proved in 1956 that this quantity is the growth rate of an optimally staked bankroll, which is why a result about noisy telephone lines is also a result about betting. The derivation is three lines and exact. At even money the Kelly fraction is twice your probability less one, so a win multiplies the bankroll by twice your probability and a loss by twice one-minus-it. Take the logarithm base 2 of each, weight them by how often each actually happens, and the twos contribute exactly one — because the logarithm of 2 is 1 and the two weights add to 1 — while the rest is the negative of the entropy. One whole bit, less whatever uncertainty you have left. Repeat it against a market implying probabilities other than a half and the same substitution gives the divergence instead, with no approximation anywhere.")
        .para(|p| p
            .text("Check it. At a 60 per cent belief the entropy is 0.9710 bits, so the edge is 0.0290 bits per bet. The Kelly stake is a fifth of the bankroll, and the growth rate computed from the logarithms directly is 0.020136 in natural units, which is 0.029049 bits. The same number, arrived at from two directions that appear to have nothing to do with each other."))
        .para(|p| p
            .text("They only appear to. Look at what the Kelly stake actually leaves you holding, because this is the sentence that dissolves the coincidence. Staking a fifth of the bankroll at even money means that after the bet your wealth is multiplied by 1.2 if the thing you gave 60 per cent to happened, and by 0.8 if it did not — which is to say, multiplied by twice your own probability of whatever occurred. Kelly betting is nothing more than dividing your bankroll across the outcomes in proportion to your beliefs. So the logarithm of your multiplier is the logarithm of your own belief, plus a constant. And the average of the logarithm of your belief, taken over what actually happens, is the definition of entropy. The identity is not two directions meeting in the middle. It is one definition seen twice."))
        .para(|p| p
            .text("Read the identity and several things that are usually asserted become derivable. A forecasting edge is literally an amount of information, and it is priced in the same units as everything else in this section. Small edges are small in exactly the way information is small: a 55 per cent belief against a fairly priced coin is worth 0.0072 bits a bet, which is a growth rate of 0.501 per cent — unimpressive alone and a factor of 3.5 over 250 such bets. And the efficient-market hypothesis acquires a sharp statement it usually lacks. A market is efficient with respect to you when the price already encodes everything you know, so the divergence is zero and your growth rate is zero. Not \"the price is correct\" — nobody can check that. \"You have no bits the price does not already have.\""))
        .plot(Plot::new(0.01..=0.99)
            .curve("your remaining uncertainty, in bits",
                "(x * ln(1 / x) + (1 - x) * ln(1 / (1 - x))) / 0.693147")
            .curve("your growth rate against the price, in bits per bet",
                "(x * ln(x / q) + (1 - x) * ln((1 - x) / (1 - q))) / 0.693147")
            .param("q", 0.2..=0.8, 0.5)
            .hline(1.0)
            .vline(0.6)
            .x_label("the probability you believe the outcome has")
            .y_label("bits")
            .height(300.0)
            .caption("Two curves that are the same arithmetic read twice. The first is the entropy of your own belief, transcribed straight from the formula above it. The second is the divergence of the market's distribution from yours, which is your growth rate. The slider is the price: the probability the market is charging you for. Park it at a fair coin, at 0.5, and the two curves are exact mirrors that add to the dashed line at 1 everywhere along the axis — that addition is the identity, visible as a gap rather than asserted. Read the marked belief of 0.6 off the lower curve and it is 0.0290 bits, which is the 2.01 per cent per bet of the Kelly section in different units; a fifth of a bit is not on offer to anybody. Now drag the price. The growth curve's zero does not stay at 0.5 — it slides across and parks itself exactly under the price, wherever you put it, because your edge is zero precisely when your belief and the price agree, and that is the whole content of an efficient market stated as a picture. Notice also that the curve never once dips below the axis: a belief on the wrong side of the price earns just as well, because Kelly simply takes the other side."))
        .note("The same logarithm gives the only honest scoreboard for probabilistic forecasts. Charge a forecaster the surprisal of whatever actually happened — minus the log of the probability they gave it — and total it up. Its expected value is minimised uniquely by stating your true beliefs, so there is no way to game it by shading forecasts towards the middle or towards the extremes. Any scoring rule that can be gamed will be, which is why option-implied distributions and weather forecasts are both judged this way.")
}

fn where_the_story_breaks(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Where the clean story breaks")
        .para(|p| p
            .text("Nothing in the machinery of this lesson is in doubt. The three rules are what a probability is, expectation is linear as a matter of arithmetic, and the central limit theorem is proved. What breaks in markets is never the machinery. It is the three assumptions the convenient results are conditional on, and it is worth naming them one at a time, because \"the model failed\" is a much less useful sentence than \"this assumption failed\"."))
        .para(|p| p
            .text("The first is a finite, stable variance. Returns carry a power-law tail with an exponent near 3, so the mean and the variance exist but the fourth moment does not, and a sample kurtosis is therefore not an estimate of anything — it grows as you add data and lurches on single large days. The second is independence: volatility clusters, so the size of moves is dependent even where the sign is not, and the square-root-of-time rule is exactly that assumption in disguise, while correlations rise towards 1 in a crisis, precisely when the floor they set is what you are relying on. The third is stationarity — that there is one fixed distribution being sampled from. There is not. Regimes change, participants learn, regulations arrive, and the thing you estimated last year has been partly dismantled by everyone else estimating it too. A coin does not read your notes on it. A market does."))
        .rule()
        .para(|p| p
            .text("Now the arithmetic that makes the first of those concrete, because it is more extreme than almost anyone's intuition. On the 19th of October 1987 the S&P 500 fell 20.5 per cent in a day. Against a 16 per cent annualised volatility — a daily standard deviation of 1.008 per cent — that was a 20-standard-deviation move, which a bell curve prices at 5.5 times ten to the minus 89, or one day in 1.8 times ten to the 88. The universe has existed for roughly 3.5 times ten to the 12 trading days, so you would need about ten to the 76 universes running markets end to end to expect one such day. It happened on a Monday, in living memory, and it was not even the last time."))
        .para(|p| p
            .text("Now run the same day through a cubic tail instead. Calibrate the power law so it agrees with the normal at 3 standard deviations — a deliberately conservative starting point, since it concedes the bell curve the whole middle of the distribution — and the same 20-sigma day comes out as a once-in-436-years event. Still rare. Entirely possible. That gap, between impossible and merely rare, is the whole practical difference between the two models, and it is the difference between a risk system that is wrong and one that is unlucky."))
        .para(|p| p
            .text("Which invites the question the cubic tail alone cannot settle, since it is only one rival model: how much of that absurdity was probability, and how much was the bell curve? There is a distribution-free answer. For any quantity with a finite variance whatsoever, whatever its shape, the chance of landing k standard deviations or more from the mean is at most one over k squared — Chebyshev's bound, which knows nothing about markets and nothing about tails. At twenty standard deviations it permits one such day in 400, about once every eighteen months. So probability itself was never the obstacle. A factor of roughly ten to the 86 of that impossibility belonged to one assumption about shape, and the machinery never failed because it was never even asked."))
        .plot(Plot::new(1.0..=100.0)
            .line("what the bell curve allows", vec![
                [1.0, 2.881], [1.47, 3.000], [2.0, 3.093], [3.0, 3.211],
                [4.0, 3.293], [5.0, 3.355], [7.0, 3.447], [10.0, 3.542],
                [14.0, 3.630], [20.0, 3.721], [30.0, 3.822], [40.0, 3.893],
                [55.0, 3.969], [70.0, 4.026], [85.0, 4.072], [100.0, 4.109]])
            .curve("what a power-law tail allows", "3 * pow(0.680348 * x, 1 / alpha)")
            .param("alpha", 2.0..=6.0, 3.0)
            .hline(5.0)
            .vline(6.8)
            .x_label("return period: a day this bad or worse arrives once in this many years")
            .y_label("size of that day, in standard deviations")
            .height(300.0)
            .caption("The same question asked of both models: plan for a horizon, and how large a day must you be ready for? The power law is pinned to the bell curve at three standard deviations, so both lines pass through the same point at a return period of about a year and a half, and to the left of that crossing the fat-tailed model is the calmer of the two — fat tails buy their extremes by being thinner in the near tail. To the right they part company and never rejoin. The two reference lines meet on the curve, and that crossing is worth reading off: a 5-sigma day arrives once in about seven years under the cubic law, while the bell curve's line does not reach 5 anywhere on this chart and would not reach it until the seven-thousandth year. That flatness is the point worth carrying away, and it is the second limit theorem of this lesson rather than the one you have met. Sums have exactly one limit shape; maxima have three, and the tail alone decides which of them you get — an exponential-ish tail gives one, a power tail gives another with the same exponent, and a quantity with a hard ceiling gives the third. Under a power tail the largest of n days grows like n raised to one over alpha, so ten times the data buys a worst day 2.15 times larger at an alpha of 3: read it off the curve, which runs 5.68 at ten years to 12.25 at a hundred. Under a bell curve it grows like the square root of the logarithm of n, so ten times the data buys 1.16 times larger and a century of markets buys barely one standard deviation over a single year, from 2.88 to 4.11. For a bell curve the worst day you will ever see hardly depends on how long you watch; for a power law it is decided by nothing else — which is why extreme quantiles are estimated by fitting a tail to the observations that exceeded a high threshold, rather than by extrapolating a fitted bell curve. Now drag the exponent. At 3 the century move is 12.2 standard deviations; push down towards 2 and it reaches 24.7, and because the vertical axis stretches to hold it the bell curve's line — unmoved, still topping out at 4.11 — is squashed into the bottom of the frame. Every setting passes through the same three-sigma pivot, so what the slider changes is not where the model starts but how fast it leaves."))
        .note("In August 2007 the chief financial officer of Goldman Sachs explained a week of heavy losses in the firm's quantitative funds by saying they had seen 25-standard-deviation moves several days running. Under a normal distribution a single such day is a one-in-ten-to-the-137 event, so several in a row is not a run of bad luck by any stretch of language. The sentence is a correct description of what the model said and a complete misreading of what it meant. When a model reports that what just happened was impossible, the thing that has been refuted is the model.")
        .rule()
        .para(|p| p
            .text("One more failure, and it needs no fat tails at all. The hundred-strategies argument has now appeared twice in this lesson and both times it ran forwards: given that nothing works, what are the odds something looks like it does? That is not the question a researcher has. They are holding a strategy that passed and want to know whether it is real — and this lesson built the machinery to turn one into the other five sections ago without ever pointing it at this. A p-value is the probability of the evidence given no edge. What is wanted is the probability of no edge given the evidence. Reading the first as the second is the transposed conditional, the identical error as reading the crash indicator's \"right nine times out of ten\" as the probability of a crash."))
        .para(|p| p
            .text("So run it in the odds form. Suppose one strategy in ten that gets tried has a genuine edge, which is generous; suppose the test finds a real edge half the time when there is one; and keep the conventional one-in-twenty false-positive rate. Prior odds are 1 to 9, the likelihood ratio is 0.5 over 0.05, which is 10, so posterior odds are 10 to 9 and a strategy that passes is real with probability 52.6 per cent. Half of all discoveries are false at the conventional threshold, with a generous prior and not one instance of p-hacking. Move the prior to 1 in 100, nearer the truth for a desk that screens systematically, and it falls to 9.2 per cent. Which is why it has been argued that a newly proposed factor should have to clear a t-statistic of about 3 rather than the conventional 2, given how many have been tried. Feed that back into the arithmetic three sections above: at a Sharpe ratio of 0.5 a t of 2 needed sixteen years, and a t of 3 needs the square root of the years to reach 6, so thirty-six. This lesson computed sixteen years and called it the arithmetic of the square root. Once the count of hypotheses is honest, sixteen was not even the right bar."))
        .para(|p| p
            .text("The honest posture at the end of all this is not scepticism about probability. It is precision about which assumption is carrying the weight. Every failure above is a distributional or independence assumption breaking, not the calculus of probability failing, and by now the repairs all have names: the negative binomial for counts whose variance outruns their mean, a self-exciting rate when the clustering has an arrow, the compound Poisson for the jump that hides in the second moment and lives in the fourth, a mixed width for a tail the bell curve cannot reach, Chebyshev for what survives when you refuse to name a shape at all, and the arcsine law for what a path looks like before anybody has tested it. The repair is never a different subject. It is this one, with the assumption you were leaning on written down."))
}

fn practice(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Practice")
        .para(|p| p
            .text("Eight questions, all built from this lesson's own material. The first is worked in full; after that each one stops and asks you to commit to an answer before the working appears. Take the invitation — a number you have written down is what makes the answer stick, and a number you skipped past leaves nothing behind."))
        .para(|p| p
            .text("First, worked. A fund closes up on 53 per cent of days. Over a 252-day year, how many up days should you expect? And in a given week of five trading days, what is the chance of at least one down day, assuming days are independent?"))
        .para(|p| p
            .text("The count is 252 separate yes-or-no events, so write the year's total as a sum of 252 indicators and apply linearity: 252 times 0.53 is 133.56 up days. Nothing about independence was used, so the answer survives even though real days are dependent. The week is different, because \"at least one\" is the signature of the complement rule. All five up has probability 0.53 to the fifth, which is 0.0418, so at least one down day has probability 0.958. And here independence was used, in raising 0.53 to a power — which is exactly the step where independence is always used, and the step to be suspicious of: nothing in the 53 per cent said the days were independent. The same move, run backwards, answers the question this lesson opened with. How many independent tests at the one-in-twenty level does it take before finding a false winner is more likely than not? Solve 0.95 to the power of the count equals a half: the count is 13.5, so fourteen tests. It does not take a hundred. It takes fourteen."))
        .rule()
        .para(|p| p
            .text("Now one with the first half done for you. A book of 200 corporate loans, each with a 4 per cent chance of defaulting over the coming year. If the defaults are independent, the expected number is 200 times 0.04, which is 8, and the variance is 200 times 0.04 times 0.96, which is 7.68 — a standard deviation of 2.77 defaults. Now change one thing: suppose the loans are all exposed to a single common event, which occurs with probability 4 per cent and takes every one of them down together. Find the expected number and the standard deviation of that second book, and then say which of the two you would rather hold. Write both numbers down before you read on."))
        .para(|p| p
            .text("The tempting answer is that the two books are worth the same, since their expected losses are identical — and the first half of that is exactly right, which is what makes it dangerous. The expected number of defaults in the second book is 8, precisely, because writing the book as a sum of 200 indicators and applying linearity gives 200 times 0.04 whatever the loans do to one another. The standard deviation is where they part company. Every pair in the second book is perfectly correlated, so expanding the variance of the sum gives 200 individual variances of 0.0384, which is 7.68, plus 200 times 199 covariance terms of 0.0384, which is 1,528.32 — a total of 1,536, and a standard deviation of 39.19. Fourteen times the first book's, and that factor is the square root of 200 rather than a coincidence. The first book reaches 30 defaults only on an eight-standard-deviation day; the second loses all 200 one year in 25. Same expected loss, two entirely different businesses, and the expected loss was never capable of telling you which one you were in."))
        .rule()
        .para(|p| p
            .text("No help with this one, and read the three inputs carefully, because one of them is not the input the earlier example gave you. A risk system flags 5 per cent of all months. Two per cent of months contain a crash, and the system flags 90 per cent of those. What fraction of its flags are real? Then suppose a second, separate system of the same quality also fires. Does a crash become more likely than not, and what did you have to assume to say so?"))
        .para(|p| p
            .text("Thirty-six per cent. What changed is that the false-alarm rate is not handed to you: 5 per cent is how often the system fires for any reason at all, which is exactly the denominator Bayes' rule asks for, so the answer is 0.9 times 0.02, which is 0.018, over 0.05. Back the false-alarm rate out to see why this system beats the earlier one — 0.05 less 0.018 is 0.032 spread across the 0.98 calm months, which is 3.27 per cent rather than 10 — and note that the crash rate and the detection rate never moved. The whole journey from 15.5 per cent to 36 was bought by that one number. Now the second firing. Its likelihood ratio is 0.9 over 0.0327, which is 27.6, so the odds go from 0.5625 to 15.5 to 1: 94 per cent. The assumption is conditional independence — that the second system's firing is news rather than the first one restated. Two systems built on the same desk from the same data very rarely are, and if the errors are driven by something persistent the second firing carries almost nothing and the honest answer stays near 36."))
        .rule()
        .para(|p| p
            .text("Another tempting one, from the continuous half. A risk model reports a probability density of 4.2 at today's return. A colleague objects that this is obviously a bug, since no probability can exceed 1. Who is right? Decide before reading on — and if you say the model is fine, produce the number that would settle the argument."))
        .para(|p| p
            .text("The model is fine. A density is a probability per unit of return, not a probability, and it only becomes one when multiplied by a width. With returns measured as decimal fractions, a density of 4.2 over a one-percentage-point band — a width of 0.01 — gives a probability of 0.042, which is entirely ordinary. The settling number is the units: re-express the same returns in percentage points instead of fractions and the identical distribution now reports a density of 0.042 at the same place. A quantity whose value depends on the units of the horizontal axis cannot be a probability, and that is the tell. The colleague is applying a rule from the discrete world, where heights really are probabilities, in the one place it does not hold."))
        .rule()
        .para(|p| p
            .text("A diagnostic. A desk models the number of gap events per week as Poisson. Over 200 weeks it observes 100 events, so the mean is 0.5 per week, matching the model's parameter exactly. The variance of the weekly counts is 2.1. Should the model be accepted? Accept or reject before reading on, and name the single number that decides it."))
        .para(|p| p
            .text("Reject it. The Poisson distribution has no freedom to fit a variance separately from a mean — they are forced to be the same number, which is what makes it a testable model rather than a flexible one. The mean matched because linearity does not care about dependence, exactly as in the first practice question; only the second moment could ever have revealed the problem, and the practical consequence is that the desk's expected number of gap events is right while its worst week is badly underestimated, which is the more expensive of the two errors. Name the number and it becomes a test rather than an impression: the dispersion index — variance over mean — is 4.2 where Poisson demands 1, and its standard error is the square root of two over n minus 1, which over 200 weeks is 0.100. So 4.2 sits 32 standard errors above 1, and no sample this size produces that by accident, while a dispersion of 1.1 would be noise. Two things follow. Read the excess as self-excitation and the branching ratio is one minus the square root of the mean over the variance, which is 0.512 — so about half the events were triggered by earlier ones. Or read it as a wandering rate, and matching a variance of 2.1 needs a negative binomial with its shape at 0.156."))
        .rule()
        .para(|p| p
            .text("No help with this one either. A manager reports a 10 per cent annual return above cash against 25 per cent annual volatility, over eight years, and argues that eight years of daily data — more than two thousand observations — settles whether the return is real. How many years would it actually take before the estimated return sat two standard errors from zero? And if the same eight years were re-measured minute by minute, giving nearly eight hundred thousand observations, what would happen to that figure? Commit to both before reading on."))
        .para(|p| p
            .text("The Sharpe ratio is 10 over 25, which is 0.4, and the t-statistic is that multiplied by the square root of the number of years. Eight years gives 0.4 times 2.83, which is 1.13 — not close to significant on anybody's threshold. Two standard errors needs the square root of the years to reach 5, so 25 years. And the minute data changes nothing at all: the t-statistic is still 1.13. Sampling 390 times more finely divides the drift per observation by 390 and the noise per observation by the square root of 390, and the two cancel exactly in the ratio. Two thousand observations and eight hundred thousand observations carry the same information about a mean, which is why \"we have a great deal of data\" is so often a statement about storage rather than about evidence. What the finer data would settle, and settle fast, is the volatility: its relative error is about one over the square root of twice the number of observations, so a single year of daily data already pins that 25 per cent to within about 1.1 percentage points. Twenty-five years for the first moment and one year for the second, out of the same series, by the same square root."))
        .rule()
        .para(|p| p
            .text("Now one that uses the jump model, with the volatility handed to you because this lesson already computed it. A desk runs 16 per cent annualised on ordinary daily moves and adds one jump a year, averaging minus 3 per cent with a standard deviation of 5 per cent about that: the annualised volatility becomes 17.03 per cent, and 16.76 even if the jumps average exactly nothing. One point of volatility, and no risk report would blink. Now commit to a number before reading on: what does that same jump do to the frequency of a minus 10 per cent day?"))
        .para(|p| p
            .text("This is where the guess should have hurt. On the diffusion alone a day has a 1.008 per cent standard deviation, so a minus 10 per cent day is a 9.9-sigma move, which a bell curve prices at once in two times ten to the 20 years. With the jumps in, such a day needs a jump 1.4 standard deviations into its own left tail, which is 8 per cent of jumps — once in 12.4 years. One point of volatility; twenty orders of magnitude of frequency. For contrast, Chebyshev, which assumes nothing whatever about shape, permits a 9.9-sigma day on 1 per cent of days. The bell curve is absurdly tight, the distribution-free bound absurdly loose, and the only number you would trade on sits between them and came from naming a mechanism."))
        .rule()
        .para(|p| p
            .text("Last one, and it joins the two halves of the lesson. A binary market is priced at 0.35. You have no view whatever on the outcome — you think it is a fair coin. Do you have an edge at all? If so, how large is it in bits, and what fraction of the bankroll does Kelly stake? Commit first of all to whether an edge is even possible when you are as uncertain as it is possible to be."))
        .para(|p| p
            .text("You have an edge of 0.0680 bits a bet — nearly ten times what a 55 per cent view against a fairly priced coin is worth — while your own entropy is a full bit, the maximum. There is no contradiction, because a growth rate was never your distance from certainty. It is the price's distance from you, and here the price is the thing that is wrong. Half times the base-two logarithm of 0.5 over 0.35, plus half times the base-two logarithm of 0.5 over 0.65, is 0.2573 less 0.1893, which is 0.0680. Check it through Kelly: a price of 0.35 is odds of 1.857 to 1, so the fraction is 0.5 minus 0.5 over 1.857, which is 0.2308 — just under a quarter of the bankroll. A win multiplies your wealth by 1.4286 and a loss by 0.7692, and weighting those two logarithms equally gives 4.72 per cent a bet, which divided by the logarithm of 2 is 0.0680 bits again. Then look at those two multipliers once more. 1.4286 is 0.5 over 0.35, and 0.7692 is 0.5 over 0.65. Kelly has divided the bankroll across the outcomes in proportion to your beliefs, and the growth rate is the price's distance from yours because a multiplier of your-belief-over-theirs cannot measure anything else."))
}

fn letter_overrides(b: LessonBuilder) -> LessonBuilder {
    b.explain_char('P', "A probability",
        "The share of the ways things could go on which whatever stands in the brackets is true — a number between 0 and 1 inclusive. Written as a lower-case p in the information section, where it names a whole distribution rather than one event.")
        .explain_char('E', "An expected value",
            "The probability-weighted average of whatever stands in the brackets: the balance point of its distribution, and its long-run average per trial. Linear in its argument, always, whatever the dependence between the pieces.")
        .explain_char('X', "A random variable",
            "A rule attaching a number to each entry of the sample space — a return, a payoff, a count. Neither random nor a variable: the randomness lies in which entry occurs, and the rule itself is fixed and known.")
        .explain_char('Y', "A second random variable",
            "Another quantity attached to the same sample space, so that the two can be added, correlated, or conditioned on one another.")
        .explain_char('K', "A count",
            "How many of something happened: up days in a year, defaults in a book, events in a week. The quantity the binomial and Poisson distributions describe.")
        .explain_char('T', "A waiting time",
            "How long until the next event arrives. The quantity the exponential distribution describes.")
        .explain_char('R', "A return",
            "A market return, measured in standard deviations in the fat-tail section, where the question is how often a move of a given size occurs.")
        .explain_char('S', "The sample space, or a price",
            "In the first section, the full list of ways things could turn out, written so that exactly one entry will be true. In the lognormal and central-limit passages it is instead a price, or a running total of contributions.")
        .explain_char('A', "An event",
            "Any part of the sample space: a set of outcomes you care about, which either happens or does not.")
        .explain_char('B', "A second event",
            "The other event, and in the conditioning section the one that has been observed — the news that shrinks the world.")
        .explain_char('F', "The cumulative distribution function",
            "How much probability lies at or below a given value. Always between 0 and 1, never decreasing, and defined identically for discrete and continuous quantities.")
        .explain_char('D', "The divergence between two distributions",
            "How many extra bits per observation a wrong model costs. Zero only when the two distributions agree everywhere, and never negative.")
        .explain_char('H', "The entropy",
            "The average surprise an observation carries, in bits. Largest for a flat distribution and zero for a certainty.")
        .explain_char('C', "A scale constant",
            "The multiplier in front of the power-law tail. It sets how common large moves are; the exponent alpha sets how fast they thin out, and the exponent is the part that matters.")
        .explain_char('N', "The normal distribution",
            "Named with its mean and its variance in brackets, so N(0, 1) is the bell curve centred at zero with a standard deviation of 1.")
        .explain_char('c', "A candidate centre, or a complement",
            "In the balance-point argument, a proposed centre for a distribution. As a superscript in the first section it means complement: everything not in the event.")
        .explain_char('n', "How many",
            "The number of trials, draws, holdings or days, depending on the section. In the central limit theorem it is the number of contributions being added, and the thing being driven large.")
        .explain_char('k', "A particular count, or a number of standard deviations",
            "In the counting distributions, the specific value a count might take. In the fat-tail section it is instead how many standard deviations a move is, which is the natural scale for asking how often something that size occurs.")
        .explain_char('p', "A probability of success",
            "The chance of a yes on one trial — an up day, a default, a winning bet. In the information section it stands for a whole distribution, namely the true one.")
        .explain_char('q', "The believed distribution",
            "The distribution you are working from, as opposed to the true one. In the divergence formula it is the model being charged for its errors; for a market bet it is the distribution the price implies.")
        .explain_char('a', "A lower limit, or the first weight",
            "The left-hand end of a range in the integrals, and how many units of X are held in the linearity formula.")
        .explain_char('b', "An upper limit, a weight, or odds",
            "The right-hand end of a range in the integrals, how many units of Y are held in the linearity formula, and in the Kelly formula the odds received on a winning bet — b to 1, so even money is b equal to 1.")
        .explain_char('f', "A density, or a stake",
            "The probability density in the continuous section: probability per unit of x, which must be multiplied by a width before it means anything. With a star it is instead the Kelly fraction, the share of a bankroll to stake.")
        .explain_char('g', "A growth rate, or a function",
            "The compound rate at which wealth actually grows — the geometric mean return, which is what determines where you end up. In Jensen's inequality it is instead a general function being applied to a random quantity.")
        .explain_char('s', "The surprisal",
            "How surprising a particular outcome is, in bits. Averaging it over all outcomes gives the entropy.")
        .explain_char('t', "A time, or a t-statistic",
            "How long has elapsed, in the waiting-time formulas. In the price-path section it is instead the ratio of an accumulated return to the standard deviation of that accumulation — how many standard errors from zero an estimated average return sits.")
        .explain_char('x', "A value the quantity can take",
            "The label run over by a sum or an integral, and the horizontal position at which a density is read.")
        .explain_char('λ', "The rate",
            "The average number of events per period, in the Poisson distribution — and, in the exponential, the constant hazard, so that the average wait is one over it. It is both the mean and the variance of a Poisson count, which is what makes that model checkable.")
        .explain_char('μ', "The mean",
            "The expected value of the quantity under discussion, used as the centre that deviations are measured from. In the growth-rate formula it is the average return before the volatility drag is taken off.")
        .explain_char('σ', "The standard deviation",
            "The square root of the variance, in the same units as the quantity itself — what a desk quotes as volatility. Its square is what adds across independent pieces, which is why volatilities combine through a square root and not by addition.")
        .explain_char('r', "The shape of a wandering rate",
            "How steady a Poisson rate is when it is allowed to vary from period to period. Large r means barely varying, and recovers the plain Poisson; small r means a rate that swings widely, and fattens the count's tail. Its reciprocal square root is the rate's own coefficient of variation.")
        .explain_char('η', "The kick",
            "How much one event lifts the arrival rate for the events that follow it, in a self-exciting process. Divided by the decay speed it gives the branching ratio: the number of further events each event triggers directly.")
        .explain_char('β', "The decay speed",
            "How quickly one event's influence on the arrival rate fades away. A large beta means the excitement is over in minutes; a small one means a burst that takes weeks to settle.")
        .explain_char('α', "The tail exponent",
            "How fast the tail of a power-law distribution thins. Close to 3 for daily equity returns, which is fat enough that the fourth moment does not exist.")
}

/// [fig 1] Expectation as a lever. Beam from x=60 to x=560 at y=210,
/// fulcrum under the balance point. Payoffs minus 40 and plus 120 map to
/// pixels 110 and 510, so a payoff of x sits at 110 plus 2.5 times x plus 40;
/// the balance point at plus 8 therefore maps to pixel 230. The two weights
/// share a width, so their heights and hence their areas are in the ratio of
/// the probabilities: 75 to 32.1 is 0.70 to 0.30.
const BALANCE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 300" font-family="sans-serif" font-size="12">
<rect x="0" y="0" width="620" height="300" rx="8" fill="#f8fafc"/>
<text x="310" y="26" fill="#64748b" text-anchor="middle">the mean is where the beam balances, not where the payoff lands</text>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M60 210H560"/></g>
<path d="M60 205H560v10H60z" fill="#94a3b8"/>
<path d="M230 216 L208 268 L252 268 Z" fill="#b45309"/>
<rect x="82" y="130" width="56" height="75" rx="4" fill="#2563eb"/>
<rect x="482" y="172.9" width="56" height="32.1" rx="4" fill="#16a34a"/>
<text x="110" y="122" fill="#2563eb" text-anchor="middle">weight 0.70</text>
<text x="110" y="172" fill="#f8fafc" text-anchor="middle">loses</text>
<text x="110" y="190" fill="#f8fafc" text-anchor="middle">40</text>
<text x="510" y="164" fill="#16a34a" text-anchor="middle">weight 0.30</text>
<text x="510" y="194" fill="#f8fafc" text-anchor="middle">+120</text>
<g stroke="#94a3b8" stroke-width="1" stroke-dasharray="4 4" fill="none"><path d="M110 215v45M510 215v45"/></g>
<text x="110" y="284" fill="#64748b" text-anchor="middle">payoff &#8722;40</text>
<text x="510" y="284" fill="#64748b" text-anchor="middle">payoff +120</text>
<text x="230" y="288" fill="#b45309" text-anchor="middle">E = +8</text>
<path d="M230 100 L230 200" stroke="#b45309" stroke-width="1.4" stroke-dasharray="5 4" fill="none"/>
<text x="238" y="96" fill="#b45309">balance point</text>
<text x="160" y="248" fill="#64748b" text-anchor="middle">short arm, heavy</text>
<text x="400" y="248" fill="#64748b" text-anchor="middle">long arm, light</text>
</svg>"##;

/// [fig 2] Counting versus measuring. Left panel: five bars whose heights are
/// the probabilities 0.1, 0.2, 0.4, 0.2, 0.1, scaled by 300 px. Right panel: a
/// bell curve centred at px 495 on a baseline of 250, peak height 120 px, with
/// three standard deviations to each edge, and one shaded strip from 520 to 545.
const DENSITY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 660 320" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="660" height="320" rx="8" fill="#f8fafc"/>
<text x="170" y="26" fill="#64748b" text-anchor="middle">counting: the bar is the probability</text>
<text x="495" y="26" fill="#64748b" text-anchor="middle">measuring: the area is the probability</text>
<g fill="#2563eb">
<rect x="50" y="220" width="36" height="30" rx="2"/>
<rect x="100" y="190" width="36" height="60" rx="2"/>
<rect x="150" y="130" width="36" height="120" rx="2"/>
<rect x="200" y="190" width="36" height="60" rx="2"/>
<rect x="250" y="220" width="36" height="30" rx="2"/>
</g>
<g fill="#334155" text-anchor="middle">
<text x="68" y="214">0.1</text><text x="118" y="184">0.2</text><text x="168" y="124">0.4</text>
<text x="218" y="184">0.2</text><text x="268" y="214">0.1</text>
</g>
<g stroke="#94a3b8" stroke-width="1.2" fill="none"><path d="M36 250H300"/></g>
<g fill="#64748b" text-anchor="middle">
<text x="68" y="266">0</text><text x="118" y="266">1</text><text x="168" y="266">2</text>
<text x="218" y="266">3</text><text x="268" y="266">4</text>
</g>
<text x="170" y="292" fill="#64748b" text-anchor="middle">five heights, adding to exactly 1</text>
<path d="M520.0 250.0L520.0 149.8L522.5 153.5L525.0 157.4L527.5 161.5L530.0 165.7L532.5 170.0L535.0 174.3L537.5 178.7L540.0 183.0L542.5 187.3L545.0 191.6L545.0 250.0Z" fill="#bfdbfe" stroke="none"/>
<path d="M370.0 248.7L375.0 248.1L380.0 247.3L385.0 246.3L390.0 245.0L395.0 243.3L400.0 241.1L405.0 238.4L410.0 235.0L415.0 231.0L420.0 226.3L425.0 220.7L430.0 214.5L435.0 207.4L440.0 199.8L445.0 191.6L450.0 183.0L455.0 174.3L460.0 165.7L465.0 157.4L470.0 149.8L475.0 143.1L480.0 137.5L485.0 133.4L490.0 130.9L495.0 130.0L500.0 130.9L505.0 133.4L510.0 137.5L515.0 143.1L520.0 149.8L525.0 157.4L530.0 165.7L535.0 174.3L540.0 183.0L545.0 191.6L550.0 199.8L555.0 207.4L560.0 214.5L565.0 220.7L570.0 226.3L575.0 231.0L580.0 235.0L585.0 238.4L590.0 241.1L595.0 243.3L600.0 245.0L605.0 246.3L610.0 247.3L615.0 248.1L620.0 248.7" fill="none" stroke="#2563eb" stroke-width="2.4"/>
<g stroke="#94a3b8" stroke-width="1.2" fill="none"><path d="M356 250H636"/></g>
<g stroke="#b45309" stroke-width="1.2" stroke-dasharray="4 3" fill="none"><path d="M520 250v-100.2M545 250v-58.4"/></g>
<path d="M520 268H545" stroke="#b45309" stroke-width="1.4" fill="none"/>
<text x="532" y="284" fill="#b45309" text-anchor="middle">width 0.25</text>
<text x="576" y="168" fill="#b45309">height about 0.64</text>
<text x="576" y="184" fill="#b45309">so area about 0.16</text>
<text x="410" y="292" fill="#64748b" text-anchor="middle">a height here is a rate, not a probability</text>
</svg>"##;

/// [fig 3] The family tree. Top row of four boxes 140 wide at y = 56, at
/// x = 20, 193, 366 and 539; the normal at x = 280, y = 180; the lognormal and
/// the Student t at y = 300, at x = 100 and x = 440.
const FAMILY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 720 524" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="720" height="524" rx="8" fill="#f8fafc"/>
<text x="360" y="24" fill="#64748b" text-anchor="middle">every arrow is a mechanism, not a resemblance</text>
<rect x="316" y="44" width="306" height="70" rx="10" fill="#e7eef7" stroke="#94a3b8" stroke-width="1" stroke-dasharray="5 4"/>
<text x="469" y="57" fill="#475569" text-anchor="middle" font-size="9">the Poisson process &#8212; one object, two questions</text>
<g fill="#dbeafe" stroke="#2563eb" stroke-width="1.6">
<rect x="18" y="64" width="128" height="44" rx="6"/>
<rect x="166" y="64" width="128" height="44" rx="6"/>
<rect x="326" y="64" width="128" height="44" rx="6"/>
<rect x="484" y="64" width="128" height="44" rx="6"/>
<rect x="520" y="180" width="180" height="44" rx="6"/>
</g>
<g fill="#dcfce7" stroke="#16a34a" stroke-width="1.6">
<rect x="286" y="320" width="148" height="44" rx="6"/>
<rect x="110" y="420" width="170" height="44" rx="6"/>
<rect x="440" y="420" width="170" height="44" rx="6"/>
</g>
<g fill="#1e3a5f" text-anchor="middle" font-size="12">
<text x="82" y="86">Bernoulli</text><text x="82" y="102" font-size="10">one yes-or-no</text>
<text x="230" y="86">binomial</text><text x="230" y="102" font-size="10">count n of them</text>
<text x="390" y="86">Poisson</text><text x="390" y="102" font-size="10">how many arrived</text>
<text x="548" y="86">exponential</text><text x="548" y="102" font-size="10">how long until one</text>
<text x="610" y="202">negative binomial</text><text x="610" y="218" font-size="10">a count that clusters</text>
</g>
<g fill="#14532d" text-anchor="middle" font-size="12">
<text x="360" y="342">normal</text><text x="360" y="358" font-size="10">the bell curve</text>
<text x="195" y="442">lognormal</text><text x="195" y="458" font-size="10">what a price does</text>
<text x="525" y="442">Student t</text><text x="525" y="458" font-size="10">what a return does</text>
</g>
<g stroke="#64748b" stroke-width="1.6" fill="none">
<path d="M146 86H159"/><path d="M166 86l-7-4v8z" fill="#64748b" stroke="none"/>
<path d="M294 86H319"/><path d="M326 86l-7-4v8z" fill="#64748b" stroke="none"/>
<path d="M461 86H477"/><path d="M484 86l-7-4v8z" fill="#64748b" stroke="none"/><path d="M454 86l7-4v8z" fill="#64748b" stroke="none"/>
<path d="M230 108L302.6 310.6"/><path d="M306 320L298.4 312.1L306.9 309.1Z" fill="#64748b" stroke="none"/>
<path d="M390 108L409.1 310.0"/><path d="M410 320L404.6 310.5L413.5 309.6Z" fill="#64748b" stroke="none"/>
<path d="M320 364L246.3 414.4"/><path d="M238 420L243.7 410.7L248.8 418.1Z" fill="#64748b" stroke="none"/>
</g>
<g stroke="#7c3aed" stroke-width="2" fill="none">
<path d="M450 108L547.7 174.4"/><path d="M556 180L545.2 178.1L550.2 170.7Z" fill="#7c3aed" stroke="none"/>
<path d="M400 364L473.7 414.4"/><path d="M482 420L471.2 418.1L476.3 410.7Z" fill="#7c3aed" stroke="none"/>
</g>
<g fill="#b45309" font-size="10" text-anchor="middle">
<text x="156" y="128">add n of them</text>
<text x="310" y="128">n huge, p tiny</text>
<text x="150" y="205">add many of anything,</text>
<text x="150" y="219">of comparable size:</text>
<text x="150" y="233">the central limit theorem</text>
<text x="500" y="252">a large rate: a Poisson count is</text>
<text x="500" y="266">itself a sum of many small ones,</text>
<text x="500" y="280">so the same door applies</text>
<text x="175" y="392">exponentiate,</text>
<text x="175" y="406">because prices multiply</text>
</g>
<g fill="#7c3aed" font-size="10" text-anchor="middle">
<text x="610" y="150">let the rate itself vary,</text>
<text x="610" y="164">because arrivals cluster</text>
<text x="555" y="392">let the width itself vary,</text>
<text x="555" y="406">because volatility does</text>
</g>
<text x="360" y="492" fill="#64748b" text-anchor="middle">the top row counts; the bottom row measures; the central limit theorem is the door between them</text>
<text x="360" y="510" fill="#7c3aed" text-anchor="middle" font-size="10">the two violet arrows are one move &#8212; mix a parameter and the mean is untouched while everything else fattens</text>
</svg>"##;

/// [fig 4] Base rates as two bars. Both bars run x = 60 to 620, 560 px wide.
/// The top bar is 1,000 months at 0.56 px each, split 18 / 2 / 98 / 882 into
/// widths 10.08, 1.12, 54.88 and 493.92, so the 116 firings occupy the first
/// 64.96 px. The bottom bar is those same 116 rescaled to the full 560, at
/// 4.8276 px each, so the 18 true alarms take 86.9 px — 15.517 per cent of the
/// width, which is the posterior.
const BASERATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 680 278" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="680" height="278" rx="8" fill="#f8fafc"/>
<text x="340" y="22" fill="#64748b" text-anchor="middle">what &#8220;given the indicator fired&#8221; does to a list of 1,000 months</text>
<g font-size="10" fill="#334155">
<rect x="60" y="34" width="10" height="10" fill="#16a34a"/><text x="76" y="43">crash, fires: 18</text>
<rect x="200" y="34" width="10" height="10" fill="#86efac"/><text x="216" y="43">crash, silent: 2</text>
<rect x="330" y="34" width="10" height="10" fill="#b45309"/><text x="346" y="43">calm, fires: 98</text>
<rect x="460" y="34" width="10" height="10" fill="#cbd5e1"/><text x="476" y="43">calm, silent: 882</text>
</g>
<rect x="60" y="58" width="10.08" height="36" fill="#16a34a"/>
<rect x="70.08" y="58" width="1.12" height="36" fill="#86efac"/>
<rect x="71.2" y="58" width="54.88" height="36" fill="#b45309"/>
<rect x="126.08" y="58" width="493.92" height="36" fill="#cbd5e1"/>
<text x="373" y="80" fill="#475569" text-anchor="middle">882 calm months, and the indicator says nothing</text>
<g stroke="#334155" stroke-width="1.2" fill="none"><path d="M60 98v6h66.08v-6"/></g>
<text x="134" y="108" fill="#334155" font-size="10">116 months in 1,000 where it fires &#8212; cross out all the rest</text>
<path d="M60 112L126.08 112L620 152L60 152Z" fill="#e6ecf3"/>
<path d="M126.08 112L620 152" stroke="#94a3b8" stroke-width="1" stroke-dasharray="3 3" fill="none"/>
<rect x="60" y="152" width="86.9" height="36" fill="#16a34a"/>
<rect x="146.9" y="152" width="473.1" height="36" fill="#b45309"/>
<text x="103" y="174" fill="#ffffff" text-anchor="middle" font-size="10">18 crashes</text>
<text x="383" y="174" fill="#ffffff" text-anchor="middle" font-size="10">98 false alarms</text>
<text x="103" y="206" fill="#16a34a" text-anchor="middle">15.5 per cent</text>
<text x="383" y="206" fill="#b45309" text-anchor="middle">84.5 per cent</text>
<text x="340" y="236" fill="#334155" text-anchor="middle">the indicator catches nine crashes in ten, and still five of every six firings are false</text>
<text x="340" y="258" fill="#64748b" text-anchor="middle" font-size="10">calm months are 49 times more common, so their 10 per cent false-alarm rate outruns the crashes' 90 per cent hit rate</text>
</svg>"##;

/// [fig 5] The Jensen gap on a call payoff. x: share price 80 at px 70, 12.5 px
/// per unit, so 90, 100, 110 and 120 sit at 195, 320, 445 and 570. y: payoff 0
/// at py 250, 9 px per unit, so a payoff of 10 is at 160. The chord runs from
/// (90, 0) at (195,250) to (110, 10) at (445,160); its midpoint is (320,205),
/// which reads as a payoff of 5 — the average payoff — directly above the kink
/// at (320,250), where the payoff of the average price is 0.
const JENSEN_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 340" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="620" height="340" rx="8" fill="#f8fafc"/>
<text x="310" y="26" fill="#64748b" text-anchor="middle">averaging the payoffs, against the payoff of the average</text>
<g stroke="#94a3b8" stroke-width="1.2" fill="none"><path d="M50 250H590"/></g>
<path d="M70 250H320L570 70" fill="none" stroke="#2563eb" stroke-width="2.4"/>
<path d="M195 250L445 160" fill="none" stroke="#16a34a" stroke-width="2" stroke-dasharray="6 4"/>
<circle cx="195" cy="250" r="4.5" fill="#16a34a"/>
<circle cx="445" cy="160" r="4.5" fill="#16a34a"/>
<g stroke="#b45309" stroke-width="2" fill="none"><path d="M320 205V250M312 205h16M312 250h16"/></g>
<circle cx="320" cy="205" r="5" fill="#b45309"/>
<circle cx="320" cy="250" r="4" fill="#64748b"/>
<text x="185" y="240" fill="#16a34a" text-anchor="end">pays 0</text>
<text x="455" y="180" fill="#16a34a">pays 10</text>
<text x="306" y="200" fill="#b45309" text-anchor="end">average payoff 5</text>
<text x="570" y="56" fill="#2563eb" text-anchor="end">the call's payoff</text>
<text x="398" y="242" fill="#b45309" font-size="10">the gap is the option's value</text>
<g fill="#64748b" text-anchor="middle"><text x="70" y="268">80</text><text x="195" y="268">90</text><text x="320" y="268">100</text><text x="445" y="268">110</text><text x="570" y="268">120</text></g>
<text x="310" y="292" fill="#64748b" text-anchor="middle" font-size="10">share price at expiry &#8212; 90 or 110, equally likely, so the average price is 100</text>
<text x="310" y="322" fill="#334155" text-anchor="middle">the average of the payoffs is 5; the payoff at the average price is 0</text>
</svg>"##;
