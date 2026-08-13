//! The Kalman filter, taught from nothing to the research edge. Each section
//! function is named after the heading it renders, and they are chained in
//! document order, so a section can be added without touching its neighbours.

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
    let b = Lesson::builder("The Kalman Filter");
    let b = the_paper_nobody_would_publish(b);
    let b = one_question_asked_over_and_over(b);
    let b = how_to_read_this_lesson(b);
    let b = part_one(b);
    let b = part_two(b);
    let b = idea_one(b);
    let b = why_a_belief_needs_a_second_number(b);
    let b = why_the_second_number_is_recomputed(b);
    let b = how_it_updates_what_it_cannot_observe(b);
    let b = why_a_variance(b);
    let b = why_square_the_error(b);
    let b = idea_one_at_work(b);
    let b = idea_two(b);
    let b = why_one_over_sigma_squared(b);
    let b = the_mechanism_of_multiplying_gaussians(b);
    let b = why_two_derivations_agree(b);
    let b = idea_two_at_work(b);
    let b = interlude_from_two_numbers_to_a_matrix(b);
    let b = idea_three(b);
    let b = why_the_gaussian_family_is_closed(b);
    let b = what_breaks_when_linearity_or_gaussianity_fails(b);
    let b = is_the_gaussian_special_or_merely_convenient(b);
    let b = idea_three_at_work(b);
    b.build()
}

fn the_paper_nobody_would_publish(b: LessonBuilder) -> LessonBuilder {
    b.heading("The paper nobody would publish")
        .para(|p| p
            .text("In 1960 an engineer called Rudolf Kálmán published a paper titled \"A New Approach to Linear Filtering and Prediction Problems\". It came out in the Transactions of the ASME — Journal of Basic Engineering: a mechanical engineers' journal."))
        .para(|p| p
            .text("Kálmán was not a mechanical engineer, and the paper has nothing to do with mechanical engineering. He published there because his ideas \"were initially met with vast skepticism, so much so that he was forced to do the first publication of his results in mechanical engineering, rather than in electrical engineering or systems engineering.\" The most influential paper in the history of estimation went into the wrong field's journal because the right field's reviewers did not believe it."))
        .para(|p| p
            .text("It is worth understanding why neither camp wanted it, because the reason is also the reason this subject feels harder than it is. To a statistician the paper looked like an engineer's recursion — a loop, an algorithm, something you run rather than something you prove. To an engineer it looked like a statistician's covariance matrix — a probability object with no obvious wiring diagram. It belonged to neither, which is precisely why it took an outsider to publish it, and why you will find the same equations taught today in six mutually unintelligible vocabularies."))
        .para(|p| p
            .text("Within months, Kálmán visited NASA Ames, where Stanley Schmidt saw the method, built the first working implementation, and — faced with orbital mechanics, which is not linear — bolted on an approximation under deadline that is now deployed more widely than the exact method it approximates. That code flew to the Moon in about 2K words of erasable core memory. Since then the same algorithm has gone into GPS, submarine navigation, air-traffic control, weather assimilation, econometrics, robotics, self-driving cars, drones and the phone in your pocket. It runs on a weather supercomputer tracking ")
            .math(r"10^8")
            .text(" state variables, and it runs on a $1 microcontroller. In 2009, aged 79, Kálmán received the National Medal of Science from President Obama — roughly half a century after the paper nobody wanted."))
        .explain(r"10^8", "One hundred million",
            "The number of state variables the filter tracks on a weather supercomputer.")
        .para(|p| p
            .text("Here is the strangest true thing about it, and it is the first sign that this is not an ordinary algorithm: you can compute the filter's entire uncertainty schedule before you have any data at all. How confident this thing will be at 3pm on Thursday is knowable before Thursday happens. We will come back to that, and to the dark side of it."))
}

fn one_question_asked_over_and_over(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("One question, asked over and over")
        .para(|p| p
            .text("The whole subject is one question wearing different clothes, and it is worth meeting the question before any machinery."))
        .para(|p| p
            .text("You carry a mark of $100 on an illiquid corporate bond. It is a real position on your book, and you are not certain of it: call your uncertainty a variance of ")
            .math(r"P^-=4")
            .text(", which is a one-sigma of ")
            .math(r"\pm\$2")
            .text(". This morning a dealer quotes the bond at $104."))
        .explain(r"P^-", "The prior variance",
            "The variance of the belief carried in before a measurement arrives — the prior. Here it is 4, the uncertainty on your own mark.")
        // An accent's generated glyphs borrow a neighbour's span, so in later
        // fragments such as `\ddot\theta = -(g/L)\theta` the `=` ends up holding
        // more than one atom, and only a term entry — never `explain_char` —
        // can name it. This one entry covers every such fragment in the lesson.
        .explain(r"=", "Equals",
            "The two sides are the same quantity, written two ways.")
        .explain(r"\pm\$2", "Give or take two dollars",
            "The one-sigma spread that a variance of 4 squared dollars comes to.")
        .para(|p| p
            .text("How far do you move your mark?"))
        .para(|p| p
            .text("Not \"should you believe the dealer\" — how far, in cents. That number is the Kalman gain, it is the only thing the filter ever computes, and by the end of this lesson you will read three answers to that identical question without effort:"))
        .para(|p| p
            .text("the quote is as good as your own mark: you move half the way, to $102.00;"))
        .para(|p| p
            .text("it is a thin day and the quote is wide: you move one tenth of the way, to $100.40;"))
        .para(|p| p
            .text("the quote is near-useless junk: you move 3.8% of the way, to $100.15 — and, astonishingly, you end up more certain than before, not less."))
        .para(|p| p
            .text("Same bond, same $100, same $4 of disagreement. Three answers. Keep this scene: it comes back throughout, deliberately with the same numbers every time, so that a change in the answer is always caused by exactly one thing you can point at."))
}

fn how_to_read_this_lesson(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("How to read this lesson")
        .para(|p| p
            .text("This lesson is built to run from knowing nothing at all to genuinely advanced, in one document. That means two things."))
        .para(|p| p
            .text("Nothing is assumed. Not linear algebra, not probability, not calculus. Everything the filter leans on is built here, in the order it is needed. If you already know what a covariance matrix is, Part 1 is a five-minute skim; if you do not, it is the reason the rest of the lesson will make sense."))
        .para(|p| p
            .text("It is long, and it is layered. Read it in three passes if you like:"))
        .para(|p| p
            .text("The spine — Parts 1 to 4, plus Layer 4 of Part 5. Everything a person needs to write down a filter for a new problem, run it, read its output honestly, and let a regression coefficient drift. This alone is a complete lesson, and it is built around one idea: precisions add. Roughly 90 to 120 minutes — that figure is an estimate rather than a measurement, so treat it as a shape, not a promise."))
        .para(|p| p
            .text("The depth — the Going deeper blocks scattered through the spine, which answer the why behind each rule at the moment you would ask it, and the rest of Parts 5 and 6, which run out to the research edge: nonlinear extensions, optimality theory, the Riccati equation, duality. Another two hours or so, on the same kind of estimate."))
        .para(|p| p
            .text("The reference — Further notes at the end, which keeps material that is real and worth having but would clog the path."))
        .para(|p| p
            .text("Two warnings before you start."))
        .para(|p| p
            .text("The word \"filter\" is a historical accident, and it misleads. It was inherited from signal processing, where a filter separates signal from noise by frequency. Here it means something narrower and much more precise: computing the distribution of a hidden quantity at the current time, given all measurements up to and including now. Nothing is being blocked or attenuated. If you hold the signal-processing meaning in your head, several things later will look wrong."))
        .para(|p| p
            .text("The same equations are taught under at least seven names. Control engineers say Kalman filter, linear quadratic estimation, optimal observer; signal processing says recursive linear MMSE estimator or the innovations filter; Bayesian statistics says dynamic linear model; econometrics says state-space model or unobserved components model; machine learning says linear dynamical system or inference in a Gaussian HMM; geoscience says data assimilation; robotics says Bayes filter or the estimation half of SLAM. A student who learned one of these will not recognise the others, and that is a large and underrated source of confusion. There is a full cross-walk in Part 3."))
}

fn part_one(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Part 1 — Uncertainty, written down as a number")
        .para(|p| p
            .text("The filter's whole premise is that measurement error cannot be removed, only reasoned about. So before anything else, we need a way of writing \"I am not sure\" as arithmetic."))
        .para(|p| p
            .text("Measurement error is the difference between what an instrument reports and the truth. Every real measurement has one. We will write it ")
            .math(r"v")
            .text(" when it belongs to an observation."))
        .para(|p| p
            .text("Noise is error modelled as random rather than as a mistake. That is a decision, not a discovery: calling something noise asserts that you know its statistical behaviour but not its value on any given occasion. You are claiming to know the shape of the cloud, not where the next dart lands."))
        .para(|p| p
            .text("Accuracy is not precision. Accuracy is closeness to the truth — the absence of bias. Precision is repeatability — small spread. A sensor can be precise and inaccurate: a tight cluster in the wrong place. This distinction matters more here than in most subjects, because the Kalman filter assumes your sensors are accurate and estimates their precision. Zero-mean noise is an assumption it makes about you. A sensor that reads half a degree high forever violates it, and the repair is not a better filter but an extra number in the model — we will meet that repair in Idea 4."))
        .para(|p| p
            .text("A random variable is a quantity whose value is uncertain, described by a distribution rather than a number. Statistics writes them as capitals (")
            .math(r"X")
            .text("); the filtering literature routinely writes ")
            .math(r"x")
            .text(" for both the random variable and its value, and so will we."))
        .para(|p| p
            .text("For a continuous quantity, a probability density function ")
            .math(r"p(x)")
            .text(" is a function whose area over an interval gives the probability of landing in that interval."))
        .explain(r"p(x)", "Probability density function",
            "A function whose area over an interval gives the probability of landing in that interval.")
        .para(|p| p
            .text("Careful here. A density is not a probability, and it may exceed 1. A uniform density on ")
            .math(r"[0,\ 0.5]")
            .text(" has height 2. Only the integral is a probability. This trips people up later, because a Gaussian with a small variance is very tall, and a tall curve looks like it is claiming a probability greater than one. It is not."))
        .explain(r"[0,\ 0.5]", "The interval from 0 to 0.5",
            "Half a unit of width. A uniform density spread over it has height 2, which is how a density comes to exceed 1.")
        .para(|p| p
            .text("The discrete analogue is a probability mass function, where the values genuinely are probabilities and sum to 1."))
        .para(|p| p
            .text("Two summaries of a distribution do almost all the work in this subject."))
        .para(|p| p
            .text("The expectation, or mean, is the probability-weighted average:"))
        .display(r"\mathbb{E}[X] = \int x\,p(x)\,dx")
        .explain(r"\mathbb{E}[X]", "The expectation of X",
            "The expectation, or mean: the probability-weighted average of X.")
        .explain(r"x\,p(x)\,dx", "Each value weighted by its density",
            "A value x, weighted by how much density sits there, across a slice dx of the line. Summing these is what makes the expectation a weighted average.")
        .para(|p| p
            .text("Read it as the centre of mass of the distribution — the point it would balance on."))
        .para(|p| p
            .text("The variance is the mean squared distance from that centre:"))
        .display(r"\mathrm{Var}(X) = \mathbb{E}[(X-\mu)^2]")
        .explain(r"\mathrm{Var}(X)", "The variance of X",
            "The mean squared distance of X from the centre of its distribution.")
        .explain(r"\mathbb{E}[(X-\mu)^2]", "The mean squared distance from the centre",
            "The probability-weighted average of the squared gap between X and its mean.")
        .para(|p| p
            .text("and the standard deviation ")
            .math(r"\sigma=\surd(\mathrm{Var}(X))")
            .text(" puts it back into the original units, so that a variance of ")
            .math(r"4")
            .text(" squared dollars is a spread of ")
            .math(r"\pm\$2")
            .text("."))
        .explain(r"\surd(\mathrm{Var}(X))", "The standard deviation",
            "The square root of the variance, which puts the spread back into the original units.")
        .para(|p| p
            .text("Everything from here on is those two numbers, and what happens to them."))
        .para(|p| p
            .text("And one distribution carries all of it. The normal, or Gaussian, distribution ")
            .math(r"\mathcal{N}(\mu,\sigma^2)")
            .text(" is the shape this whole subject lives inside, and it is built from nothing but the two numbers you have just met:"))
        .explain(r"\mathcal{N}(\mu,\sigma^2)", "The normal, or Gaussian, distribution",
            "The shape this whole subject lives inside, built from nothing but a mean μ and a variance σ².")
        .display(r"p(x) = \frac{1}{\surd(2\pi\sigma^2)}\exp\left(-\frac{(x-\mu)^2}{2\sigma^2}\right)")
        .explain(r"\frac{1}{\surd(2\pi\sigma^2)}\exp\left(-\frac{(x-\mu)^2}{2\sigma^2}\right)", "The Gaussian density",
            "A constant out front that exists only to make the area equal 1, times the exponent, which is where the shape lives.")
        .para(|p| p
            .text("The constant out front exists only to make the area equal ")
            .math(r"1")
            .text(" — it follows from the Gaussian integral ")
            .math(r"\int_{-\infty}^{\infty} e^{-x^2}\,dx = \surd\pi")
            .text(" — so nothing about the shape depends on it. What matters is the exponent, and one quantity inside it earns its own name: ")
            .math(r"1/\sigma^2")
            .text(" is the precision, \"how much you know\" rather than \"how unsure you are\". Take it as given for now. Idea 2 is the discovery that precision is the coordinate in which this entire algorithm becomes addition, and the multivariate version arrives in the Interlude before Idea 3."))
        .explain(r"\int_{-\infty}^{\infty}", "Integrate over the whole line",
            "Add the curve up across every value there is, from minus infinity to plus infinity.")
        .explain(r"e^{-x^2}\,dx", "The bare bell shape, across a slice of the line",
            "The Gaussian curve stripped of its mean, its variance and its front constant, taken across a slice dx.")
        .explain(r"\surd\pi", "The square root of pi",
            "What the Gaussian integral comes to. The density's front constant exists only to divide it out, so that the area equals 1.")
        .explain(r"1/\sigma^2", "The precision",
            "One over the variance: how much you know, rather than how unsure you are.")
        .para(|p| p
            .text("Three pieces of notation, met now because they are used from here to the last page. Nothing else in this lesson is left undefined, and these should not be either."))
        .para(|p| p
            .text("The hat always means estimate of, never the true value. ")
            .math(r"x_k")
            .text(" is the truth, and is unknowable; ")
            .math(r"\hat x")
            .text(" is your best guess at it."))
        .explain(r"x_k", "The true state at time k",
            "The truth at step k, and unknowable.")
        .para(|p| p
            .text("Minus and plus mark before and after looking. ")
            .math(r"\hat x^-")
            .text(" and ")
            .math(r"P^-")
            .text(" are the belief carried in before a measurement arrives — the prior. ")
            .math(r"\hat x^+")
            .text(" and ")
            .math(r"P^+")
            .text(" are the belief after it has been folded in — the posterior."))
        .explain(r"\hat x^-", "The prior estimate",
            "The best guess carried in before a measurement arrives.")
        .explain(r"\hat x^+", "The posterior estimate",
            "The best guess after the measurement has been folded in.")
        .explain(r"P^+", "The posterior variance",
            "The variance of the belief after the measurement has been folded in.")
        .para(|p| p
            .text("The bar says the same thing with the bookkeeping made explicit, and is the form Idea 7 and Part 3 onward use. ")
            .math(r"\hat x_{k|j}")
            .text(" means the estimate of ")
            .math(r"x")
            .text(" at time ")
            .math(r"k")
            .text(", given every measurement up to and including time ")
            .math(r"j")
            .text(". So ")
            .math(r"\hat x_{k|k-1}")
            .text(" is exactly the prior and ")
            .math(r"\hat x_{k|k}")
            .text(" exactly the posterior. A third case exists — ")
            .math(r"\hat x_{k|N}")
            .text(", with ")
            .math(r"N")
            .text(" later than ")
            .math(r"k")
            .text(" — and it is a different question with a different answer, which Part 4 gives a section of its own."))
        .explain(r"\hat x_{k|j}", "The estimate at k, given data up to j",
            "The estimate of x at time k, given every measurement up to and including time j.")
        .explain(r"\hat x_{k|k-1}", "The prior, in bar form",
            "The estimate of x at time k given every measurement up to time k-1: exactly the prior.")
        .explain(r"\hat x_{k|k}", "The posterior, in bar form",
            "The estimate of x at time k given every measurement up to and including time k: exactly the posterior.")
        .explain(r"\hat x_{k|N}", "The estimate at k, given data up to a later N",
            "The third case, with N later than k. It is a different question with a different answer, and Part 4 gives it a section of its own.")
}

fn part_two(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Part 2 — The seven ideas the filter is made of")
        .para(|p| p
            .text("The Kalman filter is usually taught as five equations. That is a mistake, because five equations can only be memorised. Underneath them are seven ideas, and from any one of them you can regenerate the equations it produces. One of the seven — the second — generates almost the whole algorithm on its own. If you keep exactly one thing from this lesson, keep that one."))
}

fn idea_one(b: LessonBuilder) -> LessonBuilder {
    b.heading("Idea 1 — A belief is a number and an uncertainty")
        .para(|p| p
            .text("Ordinary thinking carries estimates as single numbers: \"the price is 100\", \"the position is 3.2 metres\". The filter refuses to do that. Every quantity it holds is a pair — a best guess, and a statement of how wrong that guess could be. The second number is not decoration. It is the half that does the work."))
        .para(|p| p
            .text("Imagine. Put your hand out in the dark to find a light switch. In your own hallway your fingertip goes straight to it. In a hotel corridor the same hand goes out as a flat, spread palm — and sweeps. Notice that your aim was identical in both: same wall, same chest height. The only thing that changed was the spread of your hand. Your body has been carrying a best guess and a spread as two separate things your whole life, and it is the spread, not the aim, that decided what your arm actually did."))
        .para(|p| p
            .text("Engineers already refuse single numbers, and have for centuries. A machinist's drawing never says \"10 mm\"; it says ")
            .math(r"10.00 \pm 0.02")
            .text(" mm. A lab result is never \"")
            .math(r"g = 9.79")
            .text("\"; it is ")
            .math(r"g = 9.79 \pm 0.03\ \mathrm{m/s^2}")
            .text(". Strip the tolerance and the number becomes unusable — you cannot tell whether the shaft fits the hole."))
        .explain(r"10.00", "Ten millimetres, written to two decimal places",
            "The nominal dimension on a machinist's drawing, which is never written without the tolerance beside it.")
        .explain(r"0.02", "The machinist's tolerance, in millimetres",
            "How far off the nominal ten millimetres the finished part is allowed to be. Strip it and the number becomes unusable.")
        .explain(r"9.79", "The lab's measured value for g",
            "A measured acceleration due to gravity. A lab result is never this number alone.")
        .explain(r"0.03\ \mathrm{m/s^2}", "The tolerance on the lab's g",
            "How far the measured value of g could be from the truth, in metres per second squared.")
        .para(|p| p
            .text("Why this one idea generates rules. Once a belief is a pair, every operation must say what it does to both halves. That single requirement forces the filter's entire shape: two equations per step instead of one, one for the mean and one for the spread. It is also why the covariance matrix ")
            .math(r"P")
            .text(" exists at all and is the central object; why ")
            .math(r"P")
            .text(", ")
            .math(r"Q")
            .text(" and ")
            .math(r"R")
            .text(" are three different things rather than one; why the filter's output is a distribution rather than a value; why an accurate-but-overconfident filter counts as a failure rather than a quibble; and why the honest answer to \"what is the state?\" is always ")
            .math(r"\mathcal{N}(\hat x, P)")
            .text(" and never a number."))
        .explain(r"\mathcal{N}(\hat x, P)", "The state as a distribution",
            "The honest answer to \"what is the state?\": a Gaussian centred on the estimate, with the covariance P as its spread. Never a number.")
}

fn why_a_belief_needs_a_second_number(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — why a belief needs a second number at all")
        .para(|p| p
            .text("Everything the filter does is read a difference, and a difference on its own carries no meaning until you know the scale it should be read against."))
        .para(|p| p
            .text("Your model says the price is ")
            .math(r"100")
            .text("; the sensor says ")
            .math(r"103")
            .text(". The raw disagreement is ")
            .math(r"3")
            .text(" — and ")
            .math(r"3")
            .text(" is not yet information. Three units of disagreement is a catastrophe for an instrument normally right to within ")
            .math(r"0.1")
            .text(", and utterly unremarkable for one normally wrong by ")
            .math(r"10")
            .text(". The second number is that scale."))
        .explain(r"100", "The model's price",
            "What your own model says the price is, before any sensor is consulted.")
        .explain(r"103", "The sensor's price",
            "What the sensor says the price is.")
        .explain(r"0.1", "A tight instrument's typical error",
            "How far wrong an instrument normally is when three units of disagreement would be a catastrophe.")
        .explain(r"10", "A loose instrument's typical error",
            "How far wrong an instrument normally is when three units of disagreement is utterly unremarkable.")
        .para(|p| p
            .text("Withhold it and three things become impossible at once — and they are really the same impossibility three times over. You cannot combine the two sources, because nothing tells you how far to move from ")
            .math(r"100")
            .text(" toward ")
            .math(r"103")
            .text(". You cannot act, because a position size, a landing clearance and a stop distance each need \"how wrong could this be\", not \"what is it\". And you cannot check yourself, because with no scale no measurement can ever be surprising, so a filter that has silently gone wrong has no way to notice."))
        .para(|p| p
            .text("Supplying the scale does more than inform a judgement call — it determines a unique answer. Take two unbiased estimates ")
            .math(r"\mu_1,\mu_2")
            .text(" of the same unknown with error variances ")
            .math(r"\sigma_1^2,\sigma_2^2")
            .text(", and combine them as ")
            .math(r"\hat\mu = w\mu_1 + (1-w)\mu_2")
            .text(". The weights are forced to sum to ")
            .math(r"1")
            .text(": if they summed to ")
            .math(r"1+\delta")
            .text(", the combination would be wrong by ")
            .math(r"\delta x")
            .text(" on average, a systematic error no amount of data removes. The combined error variance is then"))
        .explain(r"\mu_1", "The first estimate",
            "One of two unbiased estimates of the same unknown quantity.")
        .explain(r"\mu_2", "The second estimate",
            "The other of two unbiased estimates of the same unknown quantity.")
        .explain(r"\sigma_1^2", "The first estimate's error variance",
            "How wrong the first estimate typically is, written as a variance.")
        .explain(r"\sigma_2^2", "The second estimate's error variance",
            "How wrong the second estimate typically is, written as a variance.")
        .explain(r"\hat\mu", "The combined estimate",
            "The blend of the two estimates: a weighted average of them.")
        .explain(r"w\mu_1", "The first estimate, weighted",
            "The first estimate carrying a share w of the blend.")
        .explain(r"(1-w)", "One minus the weight",
            "Whatever share of the blend the first estimate did not take.")
        .explain(r"1+\delta", "Weights that do not sum to one",
            "What the weights would sum to if they were allowed to miss: one plus a slip of δ.")
        .explain(r"\delta x", "The systematic error a bad weight sum causes",
            "How wrong the combination would be on average if the weights summed to 1 + δ instead of 1. No amount of data removes it.")
        .display(r"V(w) = w^2\sigma_1^2 + (1-w)^2\sigma_2^2")
        .explain(r"V(w)", "The combined error variance",
            "How wrong the blend is, as a function of the weight w given to the first estimate.")
        .explain(r"w^2\sigma_1^2", "The first estimate's contribution to the combined variance",
            "The first estimate's variance, scaled by the square of the weight it was given.")
        .explain(r"(1-w)^2", "One minus the weight, squared",
            "The share the second estimate carries, squared, which is how its variance enters the total.")
        .para(|p| p
            .text("an upward parabola in ")
            .math(r"w")
            .text(", so it has one lowest point and no other. Setting ")
            .math(r"V'(w)=0")
            .text(" gives"))
        .explain(r"V'(w)", "The slope of the combined variance",
            "How the combined error variance changes as the weight moves. It is zero at the parabola's one lowest point.")
        .display(r"w^\star = \frac{\sigma_2^2}{\sigma_1^2+\sigma_2^2}")
        .explain(r"w^\star", "The best weight",
            "The weight on the first estimate that makes the combined error variance as small as it can be.")
        .explain(r"\frac{\sigma_2^2}{\sigma_1^2+\sigma_2^2}", "The other source's share of the total variance",
            "The second estimate's variance over the two variances added together: the weight the first estimate earns.")
        .para(|p| p
            .text("Notice where the two second numbers sit: they are the arguments of the function that produces the answer. A rule that ignores them is not a simpler rule — it is this same formula evaluated at the wrong ")
            .math(r"w")
            .text(", and by the shape of the parabola it is strictly worse than the right one except by coincidence."))
        .para(|p| p
            .text("Two checks you can do in your head. If ")
            .math(r"\sigma_1^2=\sigma_2^2")
            .text(", then ")
            .math(r"w^\star=\frac12")
            .text(" — the familiar average. If ")
            .math(r"\sigma_2^2=0")
            .text(", a perfect sensor, then ")
            .math(r"w^\star=0")
            .text(" — take the sensor and discard the model. And the minimum value is"))
        .explain(r"\frac12", "One half",
            "The weight each source earns when the two are equally uncertain: the familiar average.")
        .display(r"V(w^\star) = \frac{\sigma_1^2\sigma_2^2}{\sigma_1^2+\sigma_2^2}, \qquad \mathrm{equivalently} \qquad \frac{1}{\sigma^2}=\frac{1}{\sigma_1^2}+\frac{1}{\sigma_2^2}")
        .explain(r"V(w^\star)", "The smallest attainable combined variance",
            "How wrong the blend is once the best weight has been used.")
        .explain(r"\frac{\sigma_1^2\sigma_2^2}{\sigma_1^2+\sigma_2^2}", "The two variances, combined",
            "The product of the two error variances over their sum: the minimum value of the combined error variance.")
        .explain(r"\qquad \mathrm{equivalently} \qquad \frac{1}{\sigma^2}", "Equivalently, the combined precision",
            "The same statement written the other way round: one over the combined variance.")
        .explain(r"\frac{1}{\sigma^2}", "The combined precision",
            "One over the combined variance: how much you know after the two sources are put together.")
        .explain(r"\frac{1}{\sigma_1^2}", "The first source's precision",
            "One over the first estimate's variance: how much the first source knows.")
        .explain(r"\frac{1}{\sigma_2^2}", "The second source's precision",
            "One over the second estimate's variance: how much the second source knows.")
        .para(|p| p
            .text("That last line is the whole of Idea 2 arriving early. (Checked numerically at ")
            .math(r"\sigma_1^2=2,\ \sigma_2^2=3")
            .text(": a grid search returns ")
            .math(r"w^\star=0.600")
            .text(" and ")
            .math(r"V=1.2")
            .text(" exactly, matching both closed forms.)"))
        .explain(r"0.600", "The best weight in the checked case",
            "What a grid search returns for the best weight when the two variances are 2 and 3.")
        .explain(r"1.2", "The combined variance in the checked case",
            "What the combined error variance comes to when the two variances are 2 and 3 and the best weight is used.")
        .explain(r"\ \sigma_2^2", "The second source's error variance",
            "How wrong the second estimate typically is, written as a variance.")
}

fn why_the_second_number_is_recomputed(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — why the second number must be recomputed every step")
        .para(|p| p
            .text("Could you not just measure your uncertainty once and hard-code it? No, because the correct weight provably changes with history, by orders of magnitude, and it changes in a way the best guess does not record."))
        .para(|p| p
            .text("Two beliefs can have an identical mean and yet deserve completely different responses to an identical next measurement — one having seen a thousand confirmations, the other having just been switched on. The cleanest demonstration is a case we will solve exactly later: filtering a constant with ")
            .math(r"F=H=1")
            .text(", ")
            .math(r"Q=0")
            .text(" and a vague starting prior gives exactly ")
            .math(r"P_k = R/k")
            .text(" and ")
            .math(r"K_k = 1/k")
            .text(", and the estimate is exactly the running arithmetic mean. The tenth measurement deserves weight ")
            .math(r"1/10")
            .text("; the hundredth deserves ")
            .math(r"1/100")
            .text(". One hard-coded constant cannot be both, and choosing one freezes the filter into permanently over-reacting or permanently under-reacting."))
        .explain(r"P_k", "The error covariance at step k",
            "How wrong the filter's estimate is at step k. Filtering a constant, it comes to R over k.")
        .explain(r"R/k", "The noise divided by the count",
            "The measurement noise shared out over the k measurements seen so far.")
        .explain(r"K_k", "The Kalman gain at step k",
            "The fraction of the disagreement the filter moves on at step k. Filtering a constant, it is one over k.")
        .explain(r"1/k", "One over the count",
            "The weight the k-th measurement earns when the filter is averaging a constant.")
        .explain(r"1/10", "One tenth",
            "The weight the tenth measurement deserves.")
        .explain(r"1/100", "One hundredth",
            "The weight the hundredth measurement deserves.")
        .para(|p| p
            .text("On top of that, the uncertainty also grows whenever time passes without evidence and shrinks whenever evidence arrives — the sawtooth of Idea 5. So the uncertainty is not a setting of the algorithm. It is part of the state of the belief, with its own equation of motion. That is the concrete content of \"two equations per step instead of one\"."))
}

fn how_it_updates_what_it_cannot_observe(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — how can it update a number it can never observe?")
        .para(|p| p
            .text("To measure how wrong you are, you would need the truth; and if you had the truth you would not be filtering. So how does the filter ever update ")
            .math(r"P")
            .text("?"))
        .para(|p| p
            .text("It never measures its uncertainty. It derives it, from arithmetic that requires no data whatsoever. Spread obeys exact composition rules under precisely the two operations the filter performs:"))
        .display(r"\mathrm{Var}(aX) = a^2\mathrm{Var}(X), \qquad \mathrm{Var}(X+Y) = \mathrm{Var}(X)+\mathrm{Var}(Y)")
        .explain(r"\mathrm{Var}(aX)", "The variance of a scaled quantity",
            "How spread out X becomes once it has been multiplied by a. It is a squared times the variance of X.")
        .explain(r"a^2\mathrm{Var}(X)", "The variance of X, scaled by a squared",
            "What scaling a quantity by a does to its spread: the variance moves by the square of the scale.")
        .explain(r"\qquad \mathrm{Var}(X+Y)", "The variance of a sum",
            "How spread out X and Y are once added together. For independent X and Y it is the two variances added.")
        .explain(r"\mathrm{Var}(Y)", "The variance of Y",
            "The mean squared distance of Y from the centre of its distribution.")
        .para(|p| p
            .text("for independent ")
            .math(r"X,Y"))
        .explain(r"X,Y", "X and Y taken together",
            "The two quantities whose variances add — and they add only when the two are independent.")
        .para(|p| p
            .text("In matrix dress these are exactly ")
            .math(r"P\mapsto FPF^\top")
            .text(" and ")
            .math(r"P\mapsto P+Q")
            .text(" — which is the entire predict step for the second half of the belief. Once you have declared how the world moves (")
            .math(r"F")
            .text("), how it reveals itself (")
            .math(r"H")
            .text("), and how noisy each is (")
            .math(r"Q")
            .text(", ")
            .math(r"R")
            .text("), the entire future trajectory of ")
            .math(r"P")
            .text(" is already fixed. In the linear-Gaussian case ")
            .math(r"P_k")
            .text(" and ")
            .math(r"K_k")
            .text(" do not depend on the measurements at all and can be computed before a single datum exists. That is the \"know your Thursday confidence before Thursday\" fact from the opening, and it is not a curiosity — it has four consequences we will collect in Part 3."))
        .explain(r"P\mapsto FPF^\top", "The covariance, pushed through the motion",
            "The predict step's first half in matrix dress: P is replaced by F P F transpose, which is the scaling rule for a linear map.")
        .explain(r"FPF^\top", "The state covariance, sandwiched by the motion",
            "The covariance P mapped forward by F, which acts on both sides because spread scales by the square of a scale.")
        .explain(r"P\mapsto P", "The covariance, replaced by itself plus something",
            "The predict step's second half: P is replaced by P plus the process noise Q, which is the addition rule for independent quantities.")
        .para(|p| p
            .text("Two more things fall out, and the second one is a debugging rule you will use for the rest of your life."))
        .para(|p| p
            .text("First, the propagation is incapable of producing an illegitimate uncertainty. ")
            .math(r"A\Sigma A^\top")
            .text(" is automatically symmetric, since ")
            .math(r"(A\Sigma A^\top)^\top = A\Sigma^\top A^\top = A\Sigma A^\top")
            .text("; and it is automatically non-negative in every direction, since ")
            .math(r"u^\top A\Sigma A^\top u = (A^\top u)^\top \Sigma (A^\top u) = w^\top\Sigma w \ge 0")
            .text(". Adding ")
            .math(r"Q\succeq 0")
            .text(" preserves both."))
        .explain(r"A\Sigma A^\top", "A covariance pushed through a linear map",
            "The spread Σ after the map A has been applied to it. The map appears on both sides because spread scales by the square of a scale.")
        .explain(r"(A\Sigma A^\top)^\top", "The transpose of the sandwich",
            "The sandwich turned about its diagonal. It comes back to the sandwich itself, which is what being symmetric means.")
        .explain(r"A\Sigma^\top A^\top", "The transpose, expanded",
            "What transposing the sandwich gives once the order is reversed term by term.")
        .explain(r"u^\top A\Sigma A^\top u", "The sandwich, read in one direction",
            "How much spread the sandwich claims along the direction u. It can never come out negative.")
        .explain(r"(A^\top u)^\top \Sigma (A^\top u)", "The same quantity, regrouped",
            "The direction u carried back through A first, so that the whole thing is Σ read along a single direction.")
        .explain(r"w^\top\Sigma w", "The spread of Σ along a direction",
            "How much spread Σ claims along the direction w. Never negative, which is what makes the result a legitimate uncertainty.")
        .explain(r"Q\succeq 0", "The process noise is non-negative",
            "Q claims no negative spread in any direction, so adding it keeps the result a legitimate uncertainty.")
        .para(|p| p
            .text("Second, and therefore: if a running filter's ")
            .math(r"P")
            .text(" ever loses symmetry or acquires a negative eigenvalue, that is never mathematics. It is arithmetic. Which is exactly why \"true divergence\" is a numerical diagnosis with numerical fixes, and why the Joseph form — a sum of two such sandwiches — is safe where ")
            .math(r"(I-KH)P^-")
            .text(", a difference, is not."))
        .explain(r"(I-KH)", "The shrinkage factor",
            "What survives of the prior covariance once the measurement has been folded in. Because it is a difference, arithmetic error can push it out of shape, which the Joseph form's sum cannot.")
        .para(|p| p
            .text("One honest question remains. Why is a number computed entirely from assumptions entitled to be called \"how wrong I could be\"?"))
        .para(|p| p
            .text("Because it is not a claim about the world; it is a conditional, and its condition is separately testable. ")
            .math(r"P")
            .text(" is defined as ")
            .math(r"\mathbb{E}[(x-\hat x)(x-\hat x)^\top]")
            .text(" under the model you stated — \"if the world really is ")
            .math(r"F,H,Q,R")
            .text(", then my error has this spread.\" It is exactly as true as those four assumptions, no more and no less. Which is precisely why \"")
            .math(r"P")
            .text(" is small\" only ever means \"the filter claims to be doing well\"."))
        .explain(r"\mathbb{E}[(x-\hat x)(x-\hat x)^\top]", "The error covariance, by definition",
            "The average of the estimate's error multiplied by itself, under the model you stated. This is what P is defined to be.")
        .explain(r"F,H,Q,R", "The four assumptions",
            "How the world moves, how it reveals itself, and how noisy each of those is. P is exactly as true as these four and no more.")
        .para(|p| p
            .text("What rescues this from being unfalsifiable is that the same assumptions make a prediction about something you can see without ever knowing the truth: the surprise must have covariance ")
            .math(r"S_k = H_kP_{k|k-1}H_k^\top + R_k")
            .text(", so the normalised squared surprise ")
            .math(r"\tilde y_k^\top S_k^{-1}\tilde y_k")
            .text(" must average ")
            .math(r"m")
            .text(". Ground truth is never required. ")
            .math(r"P")
            .text(" is a definition that arrives with an experiment attached — which is why consistency testing (Idea 7) is not an optional extra. It is the only thing standing between \"an uncertainty\" and \"a number someone made up\"."))
        .explain(r"S_k", "The surprise covariance at step k",
            "How spread out the disagreement between measurement and forecast ought to be, if the model is right.")
        .explain(r"H_kP_{k|k-1}H_k^\top", "Your own doubt, seen in measurement space",
            "The prior covariance carried through the measurement map, so that it can be compared with the sensor's own noise.")
        .explain(r"R_k", "The measurement noise covariance at step k",
            "How noisy the sensor is at step k.")
        .explain(r"\tilde y_k^\top S_k^{-1}\tilde y_k", "The normalised squared surprise",
            "The disagreement divided by the yardstick the filter itself supplies, squared. If the model is right it averages m.")
        .para(|p| p
            .text("Bedrock: this chain stops at a definition. ")
            .math(r"P")
            .text(" is the error covariance under the assumed model, and there is nothing beneath a definition to ask \"why\" of. The one question that remains — whether the assumed model is the right one — is not settled by further reasoning but empirically, by that test, on data the filter already has."))
}

fn why_a_variance(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — why a variance, of all things?")
        .para(|p| p
            .text("\"How wrong I could be\" is written as a variance: a squared quantity, in the wrong units, which nobody's intuition speaks. Why not a ")
            .math(r"\pm")
            .text(" interval, or an average absolute error?"))
        .para(|p| p
            .text("Because the filter only ever does two things to an uncertain quantity — multiplies it by a linear map, and adds an independent one to it — and variance is the measure of spread that survives both exactly, by arithmetic, with no approximation and no bookkeeping. The alternatives do not."))
        .para(|p| p
            .text("A hard ")
            .math(r"\pm")
            .text(" interval does combine: ")
            .math(r"\pm3")
            .text(" and ")
            .math(r"\pm4")
            .text(" give ")
            .math(r"\pm7")
            .text(". But it combines worst case, so after ")
            .math(r"k")
            .text(" steps of typical error ")
            .math(r"b")
            .text(" it has grown to ")
            .math(r"kb")
            .text(" while the error that actually occurs has grown like ")
            .math(r"b\surd k")
            .text(". A filter built on intervals becomes uselessly vague within a few dozen steps, its willingness to listen collapses toward zero, and it stops listening altogether."))
        .explain(r"\pm3", "Give or take three",
            "One hard interval, three units wide either side.")
        .explain(r"\pm4", "Give or take four",
            "The other hard interval, four units wide either side.")
        .explain(r"\pm7", "Give or take seven",
            "What the two hard intervals come to when combined worst case: three and four added.")
        .explain(r"kb", "The worst-case growth",
            "How far a hard interval has grown after k steps of typical error b: the two multiplied.")
        .explain(r"b\surd k", "The growth that actually happens",
            "How far the error that really occurs has grown after k steps of typical error b: the square root of k, not k.")
        .para(|p| p
            .text("Average absolute error does not combine at all. For independent ")
            .math(r"X,Y\sim\mathcal{N}(0,1)")
            .text(" we have ")
            .math(r"\mathbb{E}|X| = \mathbb{E}|Y| = \surd(2/\pi) \approx 0.7979")
            .text(", and yet ")
            .math(r"\mathbb{E}|X+Y| \approx 1.1291")
            .text(", not ")
            .math(r"1.5960")
            .text(" (verified on ")
            .math(r"2\times10^6")
            .text(" draws). And notice which number ")
            .math(r"1.1291")
            .text(" is: it is ")
            .math(r"\surd 2 \times 0.7979")
            .text(". Even when you insist on writing your uncertainty in absolute units, it combines in quadrature. The square is doing the work whether or not you write it down."))
        .explain(r"\mathcal{N}(0,1)", "The standard normal distribution",
            "A Gaussian centred on zero with a variance of one.")
        .explain(r"\mathbb{E}|X|", "The average absolute size of X",
            "How far X lands from zero on average, sign discarded.")
        .explain(r"\mathbb{E}|Y|", "The average absolute size of Y",
            "How far Y lands from zero on average, sign discarded.")
        .explain(r"\surd(2/\pi)", "The square root of two over pi",
            "What the average absolute size of a standard normal comes to: about 0.7979.")
        .explain(r"0.7979", "The average absolute size of a standard normal",
            "How far a standard normal lands from zero on average.")
        .explain(r"\mathbb{E}|X", "The average absolute size of X plus Y",
            "How far X plus Y lands from zero on average, sign discarded.")
        .explain(r"Y|", "The size of X plus Y",
            "How far X plus Y lands from zero, sign discarded.")
        .explain(r"1.1291", "What the sum's average absolute size actually is",
            "The measured answer for the sum of two standard normals, and not the two averages added.")
        .explain(r"1.5960", "What adding the two would have given",
            "The two average absolute sizes added together — which is not the answer, because absolute error does not combine.")
        .explain(r"10^6", "A million",
            "Ten to the sixth. The check was run on two million draws.")
        .explain(r"\surd 2 \times 0.7979", "The square root of two, times the single average",
            "What 1.1291 turns out to be: absolute error combines in quadrature even when written in absolute units.")
        .para(|p| p
            .text("Why does variance add when other measures do not? Because it is the expectation of a square, and the square of a sum expands with exactly one cross-term. Writing ")
            .math(r"X'=X-\mu_X")
            .text(" and ")
            .math(r"Y'=Y-\mu_Y")
            .text(","))
        .explain(r"X'", "X, centred",
            "X with its own mean taken off, so that it averages zero.")
        .explain(r"\mu_X", "The mean of X",
            "The centre of X's own distribution.")
        .explain(r"Y'", "Y, centred",
            "Y with its own mean taken off, so that it averages zero.")
        .explain(r"\mu_Y", "The mean of Y",
            "The centre of Y's own distribution.")
        .display(r"\mathbb{E}[(X'+Y')^2] = \mathbb{E}[X'^2] + 2\,\mathbb{E}[X'Y'] + \mathbb{E}[Y'^2]")
        .explain(r"\mathbb{E}[(X'+Y')^2]", "The variance of the sum",
            "The average squared size of the two centred quantities added together.")
        .explain(r"\mathbb{E}[X'^2]", "The variance of X",
            "The average squared distance of X from its own mean.")
        .explain(r"2\,\mathbb{E}[X'Y']", "Twice the covariance",
            "The one cross-term the expansion produces. It vanishes when the two are uncorrelated.")
        .explain(r"\mathbb{E}[Y'^2]", "The variance of Y",
            "The average squared distance of Y from its own mean.")
        .para(|p| p
            .text("The outer two terms are the variances; the middle one is the covariance, and it vanishes when the two are uncorrelated. Additivity is not a mysterious property discovered about variance — it is the expansion of a bracket, plus one term going to zero. Absolute value admits no such expansion: ")
            .math(r"|X+Y|")
            .text(" simply cannot be written in terms of ")
            .math(r"|X|")
            .text(" and ")
            .math(r"|Y|")
            .text(", and the nearest available statement, ")
            .math(r"|X+Y|\le|X|+|Y|")
            .text(", is an inequality — it yields a bound rather than a value, and bounds compound, which is the interval problem returning by another door."))
        .explain(r"|X", "The size of X plus Y",
            "How far X plus Y lands from zero, sign discarded.")
        .explain(r"|X|", "The absolute size of X",
            "How far X lands from zero, sign discarded.")
        .explain(r"|Y|", "The absolute size of Y",
            "How far Y lands from zero, sign discarded.")
        .para(|p| p
            .text("The same expansion performed on vectors, with outer products in place of squares, is the identity ")
            .math(r"\mathrm{Cov}(Ax)=A\Sigma A^\top")
            .text(". Every sandwich in this subject — ")
            .math(r"FPF^\top")
            .text(", ")
            .math(r"HPH^\top")
            .text(", ")
            .math(r"KRK^\top")
            .text(" — is that one bracket-expansion wearing matrices. It is the single most reused expression in the whole field, and you have just derived it."))
        .explain(r"\mathrm{Cov}(Ax)", "The covariance of a linearly mapped quantity",
            "How spread out x becomes once the linear map A has been applied to it.")
        .explain(r"HPH^\top", "Your own doubt, seen through the measurement map",
            "The state covariance carried into measurement space by H, so that it can be compared with the sensor's noise.")
        .explain(r"KRK^\top", "The sensor's noise, carried into the state",
            "The measurement noise R brought back into state space by the gain K.")
}

fn why_square_the_error(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — why square the error rather than take its size?")
        .para(|p| p
            .text("Go one level down again: why should badness be measured by a square at all? Because the loss you choose decides which summary of the belief is the correct answer, and only one of the candidates survives the filter's own operations."))
        .para(|p| p
            .text("Minimising ")
            .math(r"\mathbb{E}[(x-a)^2]")
            .text(" over ")
            .math(r"a")
            .text(" lands on the mean. Minimising ")
            .math(r"\mathbb{E}|x-a|")
            .text(" lands on the median. (Verified on a skewed exponential sample where the two genuinely differ: the squared-loss minimiser came out at ")
            .math(r"1.0015")
            .text(", matching the sample mean ")
            .math(r"1.0016")
            .text("; the absolute-loss minimiser at ")
            .math(r"0.6975")
            .text(", matching the sample median ")
            .math(r"0.6946")
            .text(".)"))
        .explain(r"\mathbb{E}[(x-a)^2]", "The expected squared error of a guess",
            "The average squared distance between the uncertain quantity x and a single guess a. Minimising it lands on the mean.")
        .explain(r"\mathbb{E}|x", "The expected absolute error of a guess",
            "The average distance between x and a single guess a, sign discarded. Minimising it lands on the median.")
        .explain(r"a|", "The distance from the guess",
            "How far x sits from the single guess a, sign discarded.")
        .explain(r"1.0015", "The squared-loss minimiser",
            "Where minimising the expected squared error landed on the checked sample.")
        .explain(r"1.0016", "The sample mean",
            "The average of the checked sample, which the squared-loss minimiser matches.")
        .explain(r"0.6975", "The absolute-loss minimiser",
            "Where minimising the expected absolute error landed on the checked sample.")
        .explain(r"0.6946", "The sample median",
            "The middle of the checked sample, which the absolute-loss minimiser matches.")
        .para(|p| p
            .text("Now ask which of the two can be propagated forward in time. Means add unconditionally — ")
            .math(r"\mathbb{E}[X+Y]=\mathbb{E}[X]+\mathbb{E}[Y]")
            .text(" holds even when ")
            .math(r"X")
            .text(" and ")
            .math(r"Y")
            .text(" are dependent — so the mean of a predicted state is computable from the means of its ingredients, and ")
            .math(r"\hat x^- = F\hat x")
            .text(" is legitimate. Medians do not add. Let ")
            .math(r"X,Y")
            .text(" be independent, each taking ")
            .math(r"0")
            .text(" with probability ")
            .math(r"0.4")
            .text(" and ")
            .math(r"1")
            .text(" with probability ")
            .math(r"0.6")
            .text(". Both medians equal ")
            .math(r"1")
            .text(". But ")
            .math(r"X+Y")
            .text(" has law ")
            .math(r"(0.16,\ 0.48,\ 0.36)")
            .text(" on ")
            .math(r"\{0,1,2\}")
            .text(", with median ")
            .math(r"1")
            .text(" — not ")
            .math(r"2")
            .text(" (verified)."))
        .explain(r"\mathbb{E}[X+Y]", "The mean of a sum",
            "The average of X and Y added together. It is the two means added, whether or not X and Y are dependent.")
        .explain(r"\mathbb{E}[Y]", "The mean of Y",
            "The probability-weighted average of Y.")
        .explain(r"F\hat x", "The estimate, pushed forward",
            "The current best guess carried through the motion model. This is the predict step for the first half of the belief.")
        .explain(r"0.4", "The probability of landing on zero",
            "How often each of X and Y takes the value 0 in the median counter-example.")
        .explain(r"0.6", "The probability of landing on one",
            "How often each of X and Y takes the value 1 in the median counter-example.")
        .explain(r"X+Y", "The sum of the two",
            "X and Y added together.")
        .explain(r"(0.16,\ 0.48,\ 0.36)", "The law of the sum",
            "How the sum's probability is spread over its three possible values.")
        .explain(r"\{0,1,2\}", "The three values the sum can take",
            "Zero, one and two: everything X plus Y can come to.")
        .para(|p| p
            .text("Choose absolute loss and your answer is a median; and a median cannot be pushed through ")
            .math(r"x_k = Fx_{k-1}+w_k")
            .text(", so there is nothing for the recursion to recurse on. A second, independent reason points the same way: a square is smooth, so minimising it means setting a derivative to zero, which gives a linear equation and hence a closed-form gain. Absolute loss gives a kinked objective with no closed form and no recursion."))
        .explain(r"Fx_{k-1}", "The previous state, moved forward",
            "Where the motion model says the state goes, applied to the previous step's true state.")
        .explain(r"w_k", "The process noise at step k",
            "How much the world moved on its own between steps, for reasons the model does not carry.")
        .para(|p| p
            .text("And there is one line of algebra that is the deepest thing available at this level. Insert and remove ")
            .math(r"\mu=\mathbb{E}[x]")
            .text(":"))
        .explain(r"\mathbb{E}[x]", "The mean of x",
            "The probability-weighted average of x: the point its distribution would balance on.")
        .display(r"\mathbb{E}[(x-a)^2] = \mathbb{E}[((x-\mu)+(\mu-a))^2] = \mathrm{Var}(x) + 2(\mu-a)\mathbb{E}[x-\mu] + (\mu-a)^2 = \mathrm{Var}(x)+(\mu-a)^2")
        .explain(r"\mathbb{E}[((x-\mu)+(\mu-a))^2]", "The same cost, with the mean inserted and removed",
            "The expected squared error rewritten as the gap from x to the mean plus the gap from the mean to the guess, squared.")
        .explain(r"\mathrm{Var}(x)", "The variance of x",
            "The mean squared distance of x from the centre of its distribution. It does not contain the guess at all.")
        .explain(r"(\mu-a)", "The gap between the mean and the guess",
            "How far the guess a sits from the mean.")
        .explain(r"(\mu-a)^2", "The squared gap between the mean and the guess",
            "How far the guess sits from the mean, squared: a square, hence never negative, and zero only when the guess is the mean.")
        .para(|p| p
            .text("In the middle term, ")
            .math(r"\mathbb{E}[x-\mu] = 0")
            .text("."))
        .explain(r"\mathbb{E}[x-\mu]", "The average gap between x and its own mean",
            "How far x sits from its own mean, on average. It is zero, which is what kills the cross term.")
        .para(|p| p
            .text("(Verified numerically at four values of ")
            .math(r"a")
            .text("; exact agreement to six decimals.)"))
        .para(|p| p
            .text("Read what it says. The first term does not contain ")
            .math(r"a")
            .text(" at all — it is the cost you cannot escape no matter what you guess. The second is a square, hence ")
            .math(r"\ge 0")
            .text(", hence zero only when ")
            .math(r"a=\mu")
            .text(". So the best single number is the mean, and the irreducible cost of even the best single number is exactly the variance."))
        .para(|p| p
            .text("That is why the pair is a pair. The mean is the argmin and the variance is the min of one and the same objective. They are not two summaries someone decided to carry side by side; the second is the residue left behind by the first. Reporting a best guess without its variance is reporting where a minimum sits while suppressing how deep it is — which is the precise sense in which the uncertainty is the half that does the work."))
        .para(|p| p
            .text("Is minimum expected squared error the right definition of \"best\"? It is not derived from anything. It is chosen, and honesty requires saying so."))
        .para(|p| p
            .text("Bedrock: this chain stops at a convention chosen for consistency. It is adopted for reasons you can check. First, closure: it is the choice under which the filter's two operations stay exact — pick another and there is no recursion to write down. Second, in the Gaussian case it costs nothing, because a Gaussian is symmetric and unimodal, so its mean, median and mode coincide and minimum-squared-error, minimum-absolute-error and maximum-density all name the identical point. Where the filter actually lives, there is nothing to choose between the criteria."))
        .para(|p| p
            .text("And third, against it — because a convention you cannot argue against is a convention you have not understood. A square punishes one ")
            .math(r"10\sigma")
            .text(" error exactly as much as a hundred ")
            .math(r"1\sigma")
            .text(" errors, since ")
            .math(r"10^2 = 100\times 1^2")
            .text(". That is why a single wild measurement wrecks a Gaussian filter, and why robust alternatives exist at all: Huber gains, Student-")
            .math(r"t")
            .text(" measurement models, innovation gating. If your real cost is asymmetric or fat-tailed, squared error is not \"best\" in your sense of the word, and you should say so rather than inherit it silently."))
        .explain(r"10\sigma", "A ten-sigma error",
            "An error ten standard deviations wide: one wild measurement.")
        .explain(r"1\sigma", "A one-sigma error",
            "An error one standard deviation wide: an ordinary one.")
        .explain(r"10^2", "Ten squared",
            "One hundred: what squaring does to a ten-sigma error.")
        .explain(r"1^2", "One, squared",
            "What a single one-sigma error costs under a square. A hundred of them come to the same as one ten-sigma error.")
}

fn idea_one_at_work(b: LessonBuilder) -> LessonBuilder {
    b.heading("Idea 1 at work — a drop of ink, and a position size")
        .para(|p| p
            .text("In the physical world. The picture to hold for a pair over time is a drop of ink in still water. Diffusion says the spread grows as"))
        .display(r"\sigma^2 = 2Dt")
        .explain(r"\sigma^2", "The variance",
            "How smeared the drop has become: the spread, written as a squared quantity.")
        .explain(r"2Dt", "Twice the diffusion constant, times time",
            "What diffusion says the spread of the ink drop grows to after time t.")
        .para(|p| p
            .text("with ")
            .math(r"D")
            .text(" the diffusion constant. The centre of the blob never moves; the blob spreads. The mean is where the dye is; the variance is how smeared it has become."))
        .para(|p| p
            .text("Now the question. You release a drop, and after one second the cloud is roughly ")
            .math(r"\pm1")
            .text(" mm across. Ten seconds later, how wide is it?"))
        .explain(r"\pm1", "Give or take one",
            "How wide the ink cloud is after one second, in millimetres.")
        .para(|p| p
            .text("The tempting answer is ten times wider, ")
            .math(r"\pm10")
            .text(" mm — ten times the time, ten times the mess. It is about ")
            .math(r"\pm3.2")
            .text(" mm. What grows linearly with time is the variance: ")
            .math(r"\sigma^2")
            .text(" goes from ")
            .math(r"1")
            .text(" to ")
            .math(r"10\ \mathrm{mm^2}")
            .text(", and ")
            .math(r"\surd 10 = 3.16")
            .text(". Width grows as ")
            .math(r"\surd t")
            .text("."))
        .explain(r"\pm10", "Give or take ten",
            "The tempting answer for the ink cloud's width after ten seconds, in millimetres. It is wrong.")
        .explain(r"\pm3.2", "Give or take three point two",
            "How wide the ink cloud actually is after ten seconds, in millimetres.")
        .explain(r"10\ \mathrm{mm^2}", "Ten square millimetres",
            "What the ink drop's variance has grown to after ten seconds.")
        .explain(r"\surd 10", "The square root of ten",
            "About 3.16: what the width comes to once the variance has reached ten.")
        .explain(r"3.16", "Three point one six",
            "The square root of ten, in millimetres.")
        .explain(r"\surd t", "The square root of time",
            "How width grows with time, since it is the variance and not the width that grows linearly.")
        .para(|p| p
            .text("That is the prediction step in physical form: no new information arrives, the first half of the belief is untouched, and the second half inflates. In two dimensions the same ink dropped into a slow current spreads further downstream than across it — an elongated blob, tilted along the flow. That tilted blob is the covariance matrix ")
            .math(r"P")
            .text(", and its tilt is the correlation between the two coordinates. Since no direction can have negative width, ")
            .math(r"P")
            .text(" is always positive semi-definite."))
        .para(|p| p
            .text("In finance. The filter's answer to \"what is this worth?\" is never a number. It is the pair ")
            .math(r"(\hat x_t, P_t)")
            .text(" — in distribution form, ")
            .math(r"\mathcal{N}(\hat x_t, P_t)")
            .text(". Say your filtered fair value for a stock is ")
            .math(r"\hat x_t = 100")
            .text(" with ")
            .math(r"P_t = 4")
            .text(". Read that as: best guess $100; one-sigma spread ")
            .math(r"\surd 4 = \$2")
            .text("; a 95% band of ")
            .math(r"100 \pm 1.96\surd 4")
            .text(", that is $96.08 to $103.92."))
        .explain(r"(\hat x_t, P_t)", "The pair the filter carries",
            "The best guess and the uncertainty on it, held together as one belief.")
        .explain(r"\mathcal{N}(\hat x_t, P_t)", "The belief, written as a distribution",
            "The same pair in distribution form: a Gaussian centred on the estimate with the covariance as its spread.")
        .explain(r"\hat x_t", "The filtered fair value",
            "Your best guess at what the stock is worth at time t.")
        .explain(r"P_t", "The variance on the filtered fair value",
            "How unsure you are about that best guess at time t.")
        .explain(r"\surd 4", "The square root of four",
            "Two: the one-sigma spread that a variance of 4 comes to.")
        .explain(r"\$2", "Two dollars",
            "The one-sigma spread on the mark.")
        .explain(r"1.96\surd 4", "Just under two standard deviations",
            "The half-width of a 95% band around the best guess.")
        .para(|p| p
            .text("The prediction step for a random-walk fair value is"))
        .display(r"\hat x^-_{t+1} = \hat x_t, \qquad P^-_{t+1} = P_t + Q")
        .explain(r"\hat x^-_{t+1}", "Tomorrow's prior estimate",
            "The best guess carried into tomorrow before any quote arrives. For a random walk it is simply today's guess.")
        .explain(r"\qquad P^-_{t+1}", "Tomorrow's prior variance",
            "How unsure you are about tomorrow's mark before any quote arrives.")
        .para(|p| p
            .text("Symbol by symbol: ")
            .math(r"\hat x")
            .text(" is your mark; ")
            .math(r"P")
            .text(" is your own confusion about it; and ")
            .math(r"Q")
            .text(" is how much the fair value genuinely moves on its own each day, for reasons you never observe."))
        .para(|p| p
            .text("Toy numbers: park that mark for four days without looking, with ")
            .math(r"Q=1")
            .text(" per day. Then ")
            .math(r"P: 4\to 8")
            .text(", and the band widens from ")
            .math(r"\pm\$2")
            .text(" to ")
            .math(r"\pm\$2.83")
            .text(". Your best guess is still exactly $100 — nothing about the first half of the belief changed — and yet the belief itself changed completely."))
        .explain(r"P: 4\to 8", "The variance, over four parked days",
            "How the mark's variance grows while nobody looks: four, then eight.")
        .explain(r"\pm\$2.83", "Give or take two dollars eighty-three",
            "The one-sigma spread once the variance has doubled from four to eight.")
        .para(|p| p
            .text("Now the reason the second half is the half that does the work: it sets the position size. Maximising ")
            .math(r"w\mu - \frac{\gamma}{2}w^2\sigma^2")
            .text(" over the position ")
            .math(r"w")
            .text(" gives the standard mean–variance (and Kelly-shaped) rule"))
        .explain(r"w\mu", "The expected gain on the position",
            "The position size multiplied by the edge it is taken for.")
        .explain(r"\frac{\gamma}{2}w^2\sigma^2", "The penalty for carrying risk",
            "The risk the position takes on, weighted by how much the holder dislikes it.")
        .display(r"w^\star = \frac{\mu}{\gamma\sigma^2}")
        .explain(r"\frac{\mu}{\gamma\sigma^2}", "The edge over the risk",
            "The expected edge divided by risk aversion times variance: the position size the rule gives.")
        .para(|p| p
            .text("Toy numbers at ")
            .math(r"\gamma=1")
            .text(": an expected edge of ")
            .math(r"\mu=4\%")
            .text(" with ")
            .math(r"\sigma=20\%")
            .text(" (variance ")
            .math(r"0.04")
            .text(") gives ")
            .math(r"w^\star = 0.04/0.04 = 1")
            .text(", a full-size position. Now suppose you are twice as unsure — same ")
            .math(r"4\%")
            .text(" edge, but ")
            .math(r"\sigma=40\%")
            .text(". How much smaller should the position be?"))
        .explain(r"4\%", "Four per cent",
            "The expected edge on the trade.")
        .explain(r"20\%", "Twenty per cent",
            "How much the first analyst's process swings in a year.")
        .explain(r"0.04", "Four hundredths",
            "The variance a twenty per cent standard deviation comes to — and, in the same numbers, the edge itself.")
        .explain(r"0.04/0.04", "Four hundredths over four hundredths",
            "The edge divided by the variance, when the two happen to be equal: a full-size position.")
        .explain(r"40\%", "Forty per cent",
            "How much the second, twice-as-loose process swings in a year.")
        .para(|p| p
            .text("Two analysts pitch the same trade on the same morning. Both mark the stock at $104 against a $100 price, so both claim an edge of ")
            .math(r"\mu = 4\%")
            .text(" — identical to the decimal. The first analyst's process swings about ")
            .math(r"\sigma = 20\%")
            .text(" a year; the second's is twice as loose, ")
            .math(r"\sigma = 40\%")
            .text(". With ")
            .math(r"\gamma = 1")
            .text(" and ")
            .math(r"w^\star = \mu/(\gamma\sigma^2)")
            .text(", size both positions. What fraction of the first position should the second one be?"))
        .explain(r"\mu/(\gamma\sigma^2)", "The position-sizing rule",
            "The edge divided by risk aversion times variance.")
        .rule()
        .note("Work both sizes out and commit to the fraction before reading on — say it out loud: is the second position half the first, or something else?")
        .para(|p| p
            .text("The first gets ")
            .math(r"w^\star = 0.04/(1 \times 0.20^2) = 0.04/0.04 = 1")
            .text(" — a full-size position. The second gets ")
            .math(r"w^\star = 0.04/(1 \times 0.40^2) = 0.04/0.16 = 0.25")
            .text(". The second position is a quarter of the first, not half. Doubling the doubt does not halve the size, because ")
            .math(r"\sigma")
            .text(" enters the formula squared: ")
            .math(r"\sigma^2")
            .text(" goes from ")
            .math(r"0.04")
            .text(" to ")
            .math(r"0.16")
            .text(", a factor of four. Notice what did the work. The forecast was identical in both cases and chose only the direction; the second half of the belief chose the size, entirely on its own. That is the sense in which the uncertainty is the half that does the work — a point estimate cannot size anything."))
        .explain(r"0.04/", "Four hundredths, divided by what follows",
            "The four per cent edge, about to be divided by risk aversion times the variance.")
        .explain(r"(1 \times 0.20^2)", "Risk aversion times the first analyst's variance",
            "One times a twenty per cent standard deviation, squared.")
        .explain(r"(1 \times 0.40^2)", "Risk aversion times the second analyst's variance",
            "One times a forty per cent standard deviation, squared.")
        .explain(r"0.04/0.16", "Four hundredths over sixteen hundredths",
            "The edge divided by the doubled analyst's variance.")
        .explain(r"0.25", "A quarter",
            "The second analyst's position size: a quarter of the first, not half.")
        .explain(r"0.16", "Sixteen hundredths",
            "The variance a forty per cent standard deviation comes to: four times the twenty per cent one.")
        .para(|p| p
            .text("Most people say half. It is a quarter: ")
            .math(r"0.04/0.16 = 0.25")
            .text(". The mean chooses the direction; the variance alone chooses the size, and it does so through its square."))
        .para(|p| p
            .text("A story to fix it. Two analysts pitch the same trade on the same morning. Both say the stock is worth $104 against a $100 price — a ")
            .math(r"4\%")
            .text(" edge, identical to the decimal. The committee is about to fund them equally when someone asks the only question that separates them: and how wrong could you be? The first analyst's process swings about ")
            .math(r"20\%")
            .text(" a year; the second's is twice as loose, ")
            .math(r"40\%")
            .text(". At ")
            .math(r"\gamma=1")
            .text(", ")
            .math(r"w^\star=\mu/(\gamma\sigma^2)")
            .text(" gives the first ")
            .math(r"0.04/0.04 = 1")
            .text(", a full position, and the second ")
            .math(r"0.04/0.16 = 0.25")
            .text(" — a quarter, because the doubt enters squared. Identical forecast, four times the capital. The forecast chose the direction and decided nothing else; the second number chose the size."))
}

fn idea_two(b: LessonBuilder) -> LessonBuilder {
    b.heading("Idea 2 — Precisions add. This is the one to keep")
        .para(|p| p
            .text("If two independent sources each estimate the same quantity, the best combination weights each by how certain it is, and the combined result is more certain than either. In symbols:"))
        .display(r"\frac{1}{\sigma^2} = \frac{1}{\sigma_1^2} + \frac{1}{\sigma_2^2}, \qquad \hat\mu = \mu_1 + \frac{\sigma_1^2}{\sigma_1^2+\sigma_2^2}\,(\mu_2-\mu_1)")
        .explain(r"\frac{\sigma_1^2}{\sigma_1^2+\sigma_2^2}\,(\mu_2-\mu_1)", "A fraction of the disagreement",
            "The gap between the two estimates, taken only as far as the first source's share of the total confusion allows.")
        .para(|p| p
            .text("Read the second one out loud, because it is the sentence the entire algorithm is built from: old belief, plus a fraction of the disagreement."))
        .figure(Figure::new(ILL_1_SVG, "Multiplying N(3, 1.7²) by N(4.4, 0.6²). Precisions add: 1/2.89 + 1/0.36 = 3.1238, so the posterior variance is 0.3201 and σ = 0.566 — narrower than the sharper of the two parents. Its mean, 4.2449, sits only σ₁²/(σ₁² + σ₂²) = 88.9% of the way from 3 to 4.4, not halfway: the vague belief keeps a vote, but only 11.1% of one.")
            .print_svg(ILL_1_SVG_PRINT)
            .width_percent(80))
        .para(|p| p
            .text("Why this generates the rules. That fraction is the Kalman gain. Written this way, the whole update step is already visible before a single matrix appears:"))
        .display(r"\hat x^+ = \hat x^- + K(z - H\hat x^-), \qquad K = \frac{P^-}{P^- + R}")
        .explain(r"K(z - H\hat x^-)", "A fraction of the disagreement, in matrix dress",
            "The gap between what the sensor said and what the prior predicted it would say, taken as far as the gain allows.")
        .explain(r"z - H\hat x^-", "The disagreement, in measurement space",
            "What the sensor reported, less what the prior belief predicted the sensor would report.")
        .explain(r"H\hat x^-", "The measurement the prior predicts",
            "The prior estimate carried into measurement space, so that it can be compared with what the sensor actually said.")
        .explain(r"\frac{P^-}{P^- + R}", "The share of the total confusion that is mine",
            "Your own doubt over your doubt plus the sensor's: the Kalman gain.")
        .explain(r"P^- + R", "The total confusion",
            "How unsure you were before the measurement, plus how noisy the measurement is.")
        .para(|p| p
            .text("is \"old belief, plus a fraction of the disagreement\", and ")
            .math(r"K")
            .text(" is the share of the total confusion that is mine rather than the sensor's. A student who has this cannot forget the gain formula, because it is no longer arbitrary — it is the only weighting that could be right."))
        .para(|p| p
            .text("What falls out of it: the gain formula ")
            .math(r"K = P^-H^\top S^{-1}")
            .text(" and its scalar form; every limiting case (")
            .math(r"R\to0")
            .text(", ")
            .math(r"R\to\infty")
            .text(", ")
            .math(r"P^-\to0")
            .text(", ")
            .math(r"P^-\to\infty")
            .text(", and the counter-intuitive ")
            .math(r"Q")
            .text(" direction); the fact that measuring never increases uncertainty; the shrinkage ")
            .math(r"P^+ = (I-KH)P^-")
            .text("; multi-sensor fusion; why the information filter simply adds information; the whole update step; and why an exponential moving average is a Kalman filter in disguise."))
        .explain(r"P^-H^\top S^{-1}", "The gain in matrix form",
            "Your own doubt, carried into measurement space and divided by the total spread of plausible measurements.")
        .explain(r"R\to0", "A perfect sensor",
            "The measurement noise shrinking to nothing.")
        .explain(r"R\to\infty", "A useless sensor",
            "The measurement noise growing without limit.")
        .explain(r"P^-\to0", "A certain prior",
            "Your own doubt shrinking to nothing.")
        .explain(r"P^-\to\infty", "A prior that knows nothing",
            "Your own doubt growing without limit.")
        .para(|p| p
            .text("Why this is the primary idea. It is the idea the algorithm is named for. It is graspable with two numbers and no matrices. It generates the update step in full. And every later sophistication — matrices, time-variation, nonlinearity — is this same weighted average wearing more clothes. If a student kept only one idea, this is the one from which they could rebuild the most."))
}

fn why_one_over_sigma_squared(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — why 1/σ² and nothing else")
        .para(|p| p
            .text("Assume no distribution at all. Demand only two things of a combined estimate ")
            .math(r"\hat\mu = w_1\mu_1 + w_2\mu_2")
            .text(": that it be unbiased whatever the true value is, and that its error variance be as small as possible."))
        .explain(r"w_1\mu_1", "The first estimate, weighted",
            "The first estimate carrying the share w₁ of the blend.")
        .explain(r"w_2\mu_2", "The second estimate, weighted",
            "The second estimate carrying the share w₂ of the blend.")
        .explain(r"w_2", "The weight on the second source",
            "The share of the blend the second source is given.")
        .para(|p| p
            .text("Unbiasedness forces ")
            .math(r"w_1+w_2=1")
            .text(", since ")
            .math(r"\mathbb{E}[\hat\mu] = (w_1+w_2)\theta")
            .text(" must equal ")
            .math(r"\theta")
            .text(" for every ")
            .math(r"\theta")
            .text(". With uncorrelated errors, ")
            .math(r"\mathrm{Var}(\hat\mu) = w_1^2\sigma_1^2 + w_2^2\sigma_2^2")
            .text(". Cauchy–Schwarz then settles the whole question in one line:"))
        .explain(r"\mathbb{E}[\hat\mu]", "The average of the combined estimate",
            "Where the blend lands on average. Unbiasedness demands that it be the true value itself.")
        .explain(r"(w_1+w_2)", "The weights added together",
            "The sum of the two weights, which unbiasedness forces to be one.")
        .explain(r"\mathrm{Var}(\hat\mu)", "The variance of the combined estimate",
            "How wrong the blend is, written as a variance.")
        .explain(r"w_1^2\sigma_1^2", "The first source's contribution to the blend's variance",
            "The first source's variance, scaled by the square of the weight it was given.")
        .explain(r"w_2^2\sigma_2^2", "The second source's contribution to the blend's variance",
            "The second source's variance, scaled by the square of the weight it was given.")
        .display(r"1 = \left(\sum_i w_i\right)^2 = \left(\sum_i (w_i\sigma_i)\cdot\frac{1}{\sigma_i}\right)^2 \le \left(\sum_i w_i^2\sigma_i^2\right)\left(\sum_i \frac{1}{\sigma_i^2}\right)")
        .explain(r"w_i", "The weight on source i",
            "The share of the blend the i-th source is given.")
        .explain(r"\left(\sum_i w_i\right)^2", "The weights, added and squared",
            "The total weight, squared. It is one, because the weights sum to one.")
        .explain(r"\left(\sum_i (w_i\sigma_i)\cdot\frac{1}{\sigma_i}\right)^2", "The same sum, split into two factors and squared",
            "Each weight written as its weighted spread times the reciprocal of that spread, which is the form Cauchy-Schwarz needs.")
        .explain(r"\left(\sum_i w_i^2\sigma_i^2\right)", "The combined error variance",
            "Each source's variance scaled by the square of its weight, added over the sources.")
        .explain(r"\left(\sum_i \frac{1}{\sigma_i^2}\right)", "The total precision",
            "Every source's precision added together.")
        .explain(r"\sum_i", "Summed over the sources",
            "Add what follows over every source i.")
        .explain(r"\propto", "Proportional to",
            "The two sides agree up to a constant factor that is the same for every source.")
        .explain(r"w_i\sigma_i", "The i-th source's weighted spread",
            "The weight on source i multiplied by its standard deviation.")
        .explain(r"\frac{1}{\sigma_i}", "One over the i-th source's spread",
            "The reciprocal of source i's standard deviation — the inverse error size, not the precision.")
        .explain(r"w_i^2\sigma_i^2", "The i-th source's contribution to the total error",
            "Source i's variance, scaled by the square of its weight.")
        .explain(r"\frac{1}{\sigma_i^2}", "The i-th source's precision",
            "One over source i's variance: how much that source knows.")
        .para(|p| p
            .text("so ")
            .math(r"\mathrm{Var}(\hat\mu) \ge \left(\sum_i \sigma_i^{-2}\right)^{-1}")
            .text(", with equality if and only if ")
            .math(r"w_i\sigma_i \propto 1/\sigma_i")
            .text(", that is, ")
            .math(r"w_i \propto 1/\sigma_i^2")
            .text("."))
        .explain(r"\left(\sum_i \sigma_i^{-2}\right)^{-1}", "The reciprocal of the summed precisions",
            "The best variance any blend can attain: add the precisions, then invert.")
        .explain(r"\sigma_i^{-2}", "The i-th source's precision",
            "One over source i's variance, written as a negative power.")
        .explain(r"1/\sigma_i", "One over the i-th source's spread",
            "The inverse of the standard deviation: the wrong quantity to weight by.")
        .explain(r"1/\sigma_i^2", "The i-th source's precision",
            "One over source i's variance: the quantity the weights are actually proportional to.")
        .para(|p| p
            .text("Both halves of the primary idea fall out of a single inequality: the weights must be the precisions, and the best attainable variance is exactly the reciprocal of the summed precisions. No Gaussian, no Bayes, no likelihood — only \"uncorrelated\" and \"finite second moments\". Inverse-variance weighting is not a modelling choice one could have made differently. It is what the word best forces."))
        .para(|p| p
            .text("Why ")
            .math(r"1/\sigma_i^2")
            .text(" and not ")
            .math(r"1/\sigma_i")
            .text(" — precision rather than \"inverse error size\"? Because the quantity being traded off is ")
            .math(r"\sum_i w_i^2\sigma_i^2")
            .text(", in which each weight enters squared: halving a source's weight quarters its contribution to the total error. Setting the derivative of ")
            .math(r"w^2\sigma_1^2 + (1-w)^2\sigma_2^2")
            .text(" to zero gives the balance condition ")
            .math(r"w_1\sigma_1^2 = w_2\sigma_2^2")
            .text(" — an equality of marginal squared error, not of error magnitude — whose solution is the ratio of precisions. Substituting back gives the cleanest statement of the whole idea: at the optimum ")
            .math(r"w_i\sigma_i^2 = \sigma^2")
            .text(" for every ")
            .math(r"i")
            .text(", that is,"))
        .explain(r"w_1\sigma_1^2", "The first source's marginal squared error",
            "Its weight times its variance. At the optimum the two sources' values of this are equal.")
        .explain(r"w_2\sigma_2^2", "The second source's marginal squared error",
            "Its weight times its variance. At the optimum the two sources' values of this are equal.")
        .explain(r"w_i\sigma_i^2", "The i-th source's marginal squared error",
            "Its weight times its variance, which at the optimum equals the combined variance for every source.")
        .display(r"w_i = \frac{\sigma^2}{\sigma_i^2} = \frac{1/\sigma_i^2}{\sum_j 1/\sigma_j^2}")
        .explain(r"\frac{\sigma^2}{\sigma_i^2}", "The combined variance over the source's own",
            "How much of the total precision source i holds, written as a ratio of variances.")
        .explain(r"\frac{1/\sigma_i^2}{\sum_j 1/\sigma_j^2}", "The source's share of the total precision",
            "Source i's precision divided by all the precisions added together.")
        .explain(r"1/\sigma_j^2", "The j-th source's precision",
            "One over source j's variance, summed over every source to give the total precision.")
        .para(|p| p
            .text("Each source's weight is literally its share of the total precision. (Checked numerically: ")
            .math(r"\sigma_1 = 1.7")
            .text(", ")
            .math(r"\sigma_2 = 0.6")
            .text(" gives ")
            .math(r"w_1 = 0.11077")
            .text(", brute-force minimiser ")
            .math(r"0.11077")
            .text(", and ")
            .math(r"w_1\sigma_1^2 = w_2\sigma_2^2 = \sigma^2 = 0.320123")
            .text(".) An inverse-standard-deviation weighting would be balancing the wrong thing — the sizes of the errors rather than their contributions."))
        .explain(r"\sigma_1", "The first source's spread",
            "The first source's standard deviation.")
        .explain(r"1.7", "One point seven",
            "The first source's standard deviation in the checked case.")
        .explain(r"\sigma_2", "The second source's spread",
            "The second source's standard deviation.")
        .explain(r"0.6", "Nought point six",
            "The second source's standard deviation in the checked case.")
        .explain(r"w_1", "The weight on the first source",
            "The share of the blend the first source is given.")
        .explain(r"0.11077", "The first source's weight in the checked case",
            "What the vague source's weight comes to when the spreads are 1.7 and 0.6.")
        .explain(r"0.320123", "The combined variance in the checked case",
            "What the two variances come to once combined, and what each source's weight times its own variance equals.")
        .para(|p| p
            .text("One more level. Why does the trade-off run on squares in the first place? Because variance is the spread measure with the two properties the problem needs, and they are the same two properties that make the objective a parabola with a unique minimum: it adds across uncorrelated sources, and it scales as ")
            .math(r"a^2")
            .text(" under ")
            .math(r"X\mapsto aX")
            .text(". Additivity makes the two sources' contributions separable, so that there is a trade-off at all rather than one tangled expression; quadratic scaling makes the objective convex in ")
            .math(r"w")
            .text(" with a single interior optimum. A measure lacking additivity — say a quantile range — leaves the combined spread depending on the sources jointly, and there is then no weight-by-weight balance to strike."))
        .explain(r"a^2", "The square of the scale",
            "What multiplying a quantity by a does to its variance.")
        .explain(r"X\mapsto aX", "Scaling X by a",
            "The linear map that multiplies a quantity by a.")
        .para(|p| p
            .text("And this is not an artefact of having chosen squared-error loss. When the two errors are Gaussian, the blend ")
            .math(r"we_1 + (1-w)e_2")
            .text(" is itself Gaussian with mean ")
            .math(r"0")
            .text(" and scale ")
            .math(r"s(w)")
            .text(", so its shape is fixed and only its scale moves with ")
            .math(r"w")
            .text(". Hence every loss that increases with ")
            .math(r"|\mathrm{error}|")
            .text(" — absolute error, any quantile width, any tail probability — is minimised at the very same ")
            .math(r"w")
            .text(". The inverse-variance weight survives changing the definition of \"best\"."))
        .explain(r"we_1", "The first error, weighted",
            "The first source's error carrying the share w of the blend.")
        .explain(r"e_2", "The second source's error",
            "How wrong the second source is on a given occasion.")
        .explain(r"s(w)", "The blend's scale",
            "How wide the blended error is, as a function of the weight. Only this moves with w; the shape does not.")
        .explain(r"|\mathrm{error}|", "The size of the error",
            "How far the blend lands from the truth, sign discarded.")
        .para(|p| p
            .text("Why do variances add for uncorrelated sources? Expand the square and use linearity of expectation: for zero-mean errors, ")
            .math(r"\mathbb{E}[(ae_1+be_2)^2] = a^2\mathbb{E}[e_1^2] + b^2\mathbb{E}[e_2^2] + 2ab\,\mathbb{E}[e_1e_2]")
            .text(", and the cross-term dies because ")
            .math(r"\mathbb{E}[e_1e_2] = \mathbb{E}[e_1]\mathbb{E}[e_2] = 0")
            .text(" for independent — indeed merely uncorrelated — zero-mean errors. Everything here is the identity ")
            .math(r"(u+v)^2 = u^2+2uv+v^2")
            .text(" plus the fact that expectation is an integral and integration is linear."))
        .explain(r"\mathbb{E}[(ae_1+be_2)^2]", "The variance of a weighted sum of errors",
            "The average squared size of the two errors, each scaled, then added.")
        .explain(r"a^2\mathbb{E}[e_1^2]", "The first error's contribution",
            "The first error's variance, scaled by the square of its coefficient.")
        .explain(r"b^2\mathbb{E}[e_2^2]", "The second error's contribution",
            "The second error's variance, scaled by the square of its coefficient.")
        .explain(r"2ab\,\mathbb{E}[e_1e_2]", "The cross-term",
            "The term that carries how the two errors move together. It dies when they are uncorrelated.")
        .explain(r"\mathbb{E}[e_1e_2]", "The average product of the two errors",
            "How the two errors move together. It is zero for uncorrelated zero-mean errors.")
        .explain(r"\mathbb{E}[e_1]\mathbb{E}[e_2]", "The two averages, multiplied",
            "What the average product becomes when the errors are independent: each averages zero, so the product does too.")
        .explain(r"(u+v)^2", "The square of a sum",
            "The bracket the whole argument rests on.")
        .explain(r"u^2", "u squared",
            "The first of the two squares the expansion produces.")
        .explain(r"2uv", "Twice the cross-term",
            "The one cross-term the expansion of a squared sum produces.")
        .explain(r"v^2", "v squared",
            "The second of the two squares the expansion produces.")
        .para(|p| p
            .text("Note where this fails, because it is the single most expensive mistake in applied filtering: correlated sources keep the cross term. That is exactly why fusing dependent sensors is not \"add the precisions\", and why double-counting the same evidence twice makes a filter overconfident."))
        .para(|p| p
            .text("Finally, the punchline — the version of \"precisions add\" a beginner could have known before any of this. Suppose each source is itself the average of ")
            .math(r"n_i")
            .text(" raw readings, each reading with variance ")
            .math(r"\tau^2")
            .text(". Then ")
            .math(r"\sigma_i^2 = \tau^2/n_i")
            .text(", so the precision is"))
        .explain(r"n_i", "The number of readings behind source i",
            "How many raw readings source i is the average of.")
        .explain(r"\tau^2", "The variance of one raw reading",
            "How noisy a single reading is.")
        .explain(r"\sigma_i^2", "Source i's variance",
            "How wrong source i is: the variance of one reading, shared out over the readings it averaged.")
        .explain(r"\tau^2/n_i", "One reading's variance, divided by the count",
            "What averaging n readings does to their noise.")
        .display(r"\frac{1}{\sigma_i^2} = \frac{n_i}{\tau^2}")
        .explain(r"\frac{n_i}{\tau^2}", "The count, in units of one over the reading variance",
            "Source i's precision, which is literally the number of readings it holds.")
        .para(|p| p
            .text("— literally the number of readings, measured in units of ")
            .math(r"1/\tau^2")
            .text(". Feed those into the inverse-variance formula and it collapses to the plain average of all the readings: weights ")
            .math(r"n_i/(n_1+n_2)")
            .text(", combined precision ")
            .math(r"(n_1+n_2)/\tau^2")
            .text(" (verified: ")
            .math(r"n_1=3")
            .text(", ")
            .math(r"n_2=5")
            .text(" gives weight ")
            .math(r"0.375 = 3/8")
            .text(" both ways)."))
        .explain(r"1/\tau^2", "One over the reading variance",
            "The precision of a single raw reading: the unit the counts are measured in.")
        .explain(r"n_i/", "Source i's readings, divided by what follows",
            "How many readings source i holds, about to be divided by how many there are altogether.")
        .explain(r"(n_1+n_2)", "All the readings",
            "The two sources' reading counts added together.")
        .explain(r"/\tau^2", "Divided by the reading variance",
            "The pooled count measured in units of one reading's precision.")
        .explain(r"n_1", "The first source's reading count",
            "How many raw readings the first source averaged.")
        .explain(r"n_2", "The second source's reading count",
            "How many raw readings the second source averaged.")
        .explain(r"0.375", "Three eighths, in decimals",
            "The first source's weight when it holds three readings and the second holds five.")
        .para(|p| p
            .text("So precisions add because counts add: three readings pooled with five is eight readings, and no other answer is conceivable. This also shows immediately why variance could never be the additive quantity — averaging more data must make you more certain, and only the reciprocal grows with ")
            .math(r"n")
            .text("."))
        .para(|p| p
            .text("An analogy, at the moment it lands. Two friends both tell you the film starts at 8. One has checked the listing three times; the other five. Do you weight them 50/50? You weight the sightings: ")
            .math(r"3/8")
            .text(" and ")
            .math(r"5/8")
            .text(" — and you now hold eight sightings' worth of certainty, more than either friend had alone. That is the whole of \"precisions add\", and notice that no probability was used anywhere."))
        .explain(r"3/8", "Three eighths",
            "The weight the friend with three sightings earns.")
        .explain(r"5/8", "Five eighths",
            "The weight the friend with five sightings earns.")
        .para(|p| p
            .text("Where it breaks down: both friends read the same listing. Then the sightings are not independent, the counts do not add, and pooling them makes you overconfident — which is the same cross term coming back the moment a filter is fed a re-derived version of its own output."))
        .para(|p| p
            .text("Bedrock: this chain stops at a definition, reached by counting. Pooling ")
            .math(r"n_1")
            .text(" observations with ")
            .math(r"n_2")
            .text(" observations is having ")
            .math(r"n_1+n_2")
            .text(" observations — that is what pooling means, not a fact about it — and the additivity of counts is not a further claim requiring justification."))
        .explain(r"n_1+n_2", "The pooled count",
            "The two sources' readings added together, which is what pooling them means.")
}

fn the_mechanism_of_multiplying_gaussians(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — the mechanism: what a Gaussian is, and why multiplying two of them adds precisions")
        .para(|p| p
            .text("Part 1 handed you the bell curve as a formula. Here is where you need it as a mechanism, so keep it in front of you:"))
        .display(r"p(x) = \frac{1}{\surd(2\pi\sigma^2)}\exp\left(-\frac{(x-\mu)^2}{2\sigma^2}\right)")
        .para(|p| p
            .text("Now: how, exactly, does that shape turn a product of densities into an addition of the precisions ")
            .math(r"1/\sigma^2")
            .text("? Three moves, none of them clever."))
        .para(|p| p
            .text("1. ")
            .math(r"e^u e^v = e^{u+v}")
            .text(", so multiplying densities adds exponents."))
        .explain(r"e^u e^v", "Two exponentials, multiplied",
            "The exponential of u times the exponential of v.")
        .explain(r"e^{u+v}", "The exponential of the sum",
            "What that product comes to: multiplying exponentials adds their exponents.")
        .para(|p| p
            .text("2. A Gaussian's exponent is a quadratic polynomial in ")
            .math(r"x")
            .text(": ")
            .math(r"-\frac{(x-\mu_i)^2}{2\sigma_i^2} = -\frac{1}{2\sigma_i^2}x^2 + \frac{\mu_i}{\sigma_i^2}x - \frac{\mu_i^2}{2\sigma_i^2}")
            .text("."))
        .explain(r"-\frac{(x-\mu_i)^2}{2\sigma_i^2}", "The i-th Gaussian's exponent",
            "Minus how far x sits from that source's mean, squared and divided by twice its variance.")
        .explain(r"-\frac{1}{2\sigma_i^2}x^2", "The exponent's x-squared term",
            "The quadratic part of the exponent, whose coefficient is half the source's precision.")
        .explain(r"\frac{\mu_i}{\sigma_i^2}x", "The exponent's x term",
            "The linear coefficient of the exponent: the source's mean, weighted by its precision.")
        .explain(r"\frac{\mu_i^2}{2\sigma_i^2}", "The exponent's constant term",
            "The part of the exponent that does not depend on x at all.")
        .para(|p| p
            .text("3. Adding two polynomials adds their coefficients term by term."))
        .para(|p| p
            .text("Hence the product's ")
            .math(r"x^2")
            .text(" coefficient is ")
            .math(r"-\frac12\left(\frac{1}{\sigma_1^2}+\frac{1}{\sigma_2^2}\right)")
            .text(" and its ")
            .math(r"x")
            .text(" coefficient is ")
            .math(r"\frac{\mu_1}{\sigma_1^2}+\frac{\mu_2}{\sigma_2^2}")
            .text(". Precisions add because they are the ")
            .math(r"x^2")
            .text(" coefficients, and collecting like terms is addition."))
        .explain(r"x^2", "x squared",
            "The quadratic term of the exponent, whose coefficient is where the precision lives.")
        .explain(r"-\frac12", "Minus a half",
            "The one half that sits in front of every Gaussian exponent.")
        .explain(r"\left(\frac{1}{\sigma_1^2}+\frac{1}{\sigma_2^2}\right)", "The two precisions, added",
            "The first source's precision plus the second's: the combined precision.")
        .explain(r"\frac{\mu_1}{\sigma_1^2}", "The first mean, weighted by its precision",
            "The first source's mean divided by its variance.")
        .explain(r"\frac{\mu_2}{\sigma_2^2}", "The second mean, weighted by its precision",
            "The second source's mean divided by its variance.")
        .para(|p| p
            .text("Completing the square is then not a derivation but a reading-off: any ")
            .math(r"-\frac12(ax^2 - 2bx)")
            .text(" equals ")
            .math(r"-\frac{a}{2}\left(x-\frac{b}{a}\right)^2 + \mathrm{const}")
            .text(", so the product is ")
            .math(r"\mathcal{N}(b/a,\ 1/a)")
            .text(" — mean ")
            .math(r"\frac{\mu_1/\sigma_1^2 + \mu_2/\sigma_2^2}{1/\sigma_1^2 + 1/\sigma_2^2}")
            .text(", variance ")
            .math(r"\left(\frac{1}{\sigma_1^2}+\frac{1}{\sigma_2^2}\right)^{-1}")
            .text(". Nothing is optimised anywhere. The entire Bayesian answer is bookkeeping on two coefficients. (Verified by numerical integration of the actual product of two densities: mean ")
            .math(r"4.244923")
            .text(", variance ")
            .math(r"0.3201231")
            .text(", matching both closed forms and matching the \"old belief plus a fraction of the disagreement\" form to twelve digits.)"))
        .explain(r"(ax^2 - 2bx)", "A quadratic in its raw form",
            "Any exponent written as a quadratic with coefficients a and b, before the square is completed.")
        .explain(r"-ax^2", "The quadratic term of the log-density",
            "The x-squared part, whose coefficient is the precision in disguise.")
        .explain(r"-\frac{a}{2}\left(x-\frac{b}{a}\right)^2", "The same quadratic, square completed",
            "The exponent rewritten around its own peak, which is at b over a.")
        .explain(r"\frac{b}{a}", "The peak of the quadratic",
            "Where the exponent is largest: the mean of the resulting Gaussian.")
        .explain(r"\mathrm{const}", "A constant",
            "The leftover that does not depend on x, and so is absorbed by normalisation.")
        .explain(r"\mathcal{N}(b/a,\ 1/a)", "The Gaussian the product comes to",
            "A Gaussian with mean b over a and variance one over a: read straight off the two coefficients.")
        .explain(r"b/a", "The product's mean",
            "The linear coefficient over the quadratic one.")
        .explain(r"1/a", "The product's variance",
            "One over the quadratic coefficient.")
        .explain(r"\frac{\mu_1/\sigma_1^2 + \mu_2/\sigma_2^2}{1/\sigma_1^2 + 1/\sigma_2^2}", "The precision-weighted mean",
            "Each mean weighted by its own precision, divided by the precisions added together.")
        .explain(r"\mu_1/\sigma_1^2", "The first mean, weighted by its precision",
            "The first source's mean divided by its variance.")
        .explain(r"\mu_2/\sigma_2^2", "The second mean, weighted by its precision",
            "The second source's mean divided by its variance.")
        .explain(r"1/\sigma_1^2", "The first source's precision",
            "One over the first source's variance.")
        .explain(r"1/\sigma_2^2", "The second source's precision",
            "One over the second source's variance.")
        .explain(r"\left(\frac{1}{\sigma_1^2}+\frac{1}{\sigma_2^2}\right)^{-1}", "The reciprocal of the summed precisions",
            "The combined variance: add the two precisions, then invert.")
        .explain(r"4.244923", "The verified posterior mean",
            "What numerical integration of the actual product of the two densities gives for the mean.")
        .explain(r"0.3201231", "The verified posterior variance",
            "What numerical integration of the actual product of the two densities gives for the variance.")
        .para(|p| p
            .text("Turning that into the correction form is a one-line identity, valid for any weights summing to one: ")
            .math(r"w\mu_1 + (1-w)\mu_2 \equiv \mu_1 + (1-w)(\mu_2-\mu_1)")
            .text(". What it buys is threefold, and the third is the one that matters most."))
        .explain(r"\equiv", "Is identically equal to",
            "The two sides are the same expression, for every value of the weights.")
        .explain(r"(\mu_2-\mu_1)", "The disagreement",
            "How far the second estimate sits from the first.")
        .para(|p| p
            .text("It makes the estimator recursive: you never need to hold the weights and both estimates, only your current belief and the surprise."))
        .para(|p| p
            .text("It isolates a single object, ")
            .math(r"1-w = \frac{\sigma_1^2}{\sigma_1^2+\sigma_2^2}")
            .text(", which reads as the share of the total confusion that is mine rather than the sensor's — and that object is the Kalman gain."))
        .explain(r"1-w", "The leftover weight",
            "Whatever share of the blend the first estimate did not take: the Kalman gain.")
        .explain(r"\frac{\sigma_1^2}{\sigma_1^2+\sigma_2^2}", "My share of the total confusion",
            "The first source's variance over the two variances added: the share of the doubt that is mine rather than the sensor's.")
        .para(|p| p
            .text("The correction form generalises and the average form does not. When ")
            .math(r"H\ne I")
            .text(" the measurement ")
            .math(r"z")
            .text(" and the state ")
            .math(r"x")
            .text(" do not even live in the same space or carry the same units, so there is no such thing as \"averaging the estimate and the measurement\". But ")
            .math(r"\hat x^- + K(z - H\hat x^-)")
            .text(" stays meaningful, because the disagreement is formed in measurement space and ")
            .math(r"K")
            .text(" carries the conversion back to state space. The weighted-average reading is the special case; the correction reading is the real one."))
        .para(|p| p
            .text("Now the question underneath. Is \"evidence combines by adding two numbers\" a property the Gaussian happens to have, or is it what a Gaussian is?"))
        .para(|p| p
            .text("It is what a Gaussian is, and it is checkable in two lines. Suppose a density satisfies ")
            .math(r"\log p(x) = -ax^2+bx+c")
            .text(". Normalisability forces ")
            .math(r"a>0")
            .text(", since otherwise the tails do not decay and the integral diverges; completing the square gives ")
            .math(r"p(x)\propto\exp\left(-a\left(x-\frac{b}{2a}\right)^2\right)")
            .text(", which after normalising is exactly ")
            .math(r"\mathcal{N}\left(\frac{b}{2a},\ \frac{1}{2a}\right)")
            .text("."))
        .explain(r"\log p(x)", "The log-density",
            "The logarithm of the density. For a Gaussian it is a quadratic, and nothing else is.")
        .explain(r"bx", "The linear term of the log-density",
            "The x part, which places the peak.")
        .explain(r"c", "The constant term of the log-density",
            "The part with no x in it, which is spent on making the area equal one.")
        .explain(r"\exp\left(-a\left(x-\frac{b}{2a}\right)^2\right)", "The density, square completed",
            "The bell shape rewritten around its own peak at b over 2a.")
        .explain(r"\mathcal{N}\left(\frac{b}{2a},\ \frac{1}{2a}\right)", "The Gaussian those coefficients name",
            "Mean b over 2a and variance one over 2a: read straight off the quadratic.")
        .explain(r"\frac{b}{2a}", "The peak, from the coefficients",
            "Where a quadratic log-density is largest.")
        .explain(r"\frac{1}{2a}", "The variance, from the coefficients",
            "One over twice the quadratic coefficient.")
        .para(|p| p
            .text("So the Gaussians are precisely the densities whose logarithm is a quadratic — and a quadratic is precisely three coefficients, one of which is spent on normalisation. A Gaussian is two free numbers. Remember that sentence; five later results are it in disguise."))
        .para(|p| p
            .text("There is a general version worth naming even at this stage. A density written as ")
            .math(r"\exp\left(\eta_1T_1(x)+\eta_2T_2(x)-A(\eta)\right)")
            .text(" has its parameters ")
            .math(r"\eta")
            .text(" add when densities multiply, and for the Gaussian the sufficient statistic is ")
            .math(r"T(x)=(x,x^2)")
            .text(" with natural parameters ")
            .math(r"\eta_1 = \mu/\sigma^2")
            .text(" and ")
            .math(r"\eta_2 = -1/(2\sigma^2)")
            .text(". Those two coordinates are exactly ")
            .math(r"P^{-1}\hat x")
            .text(" and ")
            .math(r"P^{-1}")
            .text(". The information filter is not a clever rearrangement of the Kalman filter; it is the Gaussian written in the coordinates in which its own update is literally vector addition."))
        .explain(r"\exp\left(\eta_1T_1(x)+\eta_2T_2(x)-A(\eta)\right)", "An exponential-family density",
            "The general shape whose parameters add when two densities are multiplied.")
        .explain(r"\eta_1T_1(x)", "The first natural parameter, against its statistic",
            "One coordinate of the density's exponent: a parameter multiplying a function of x.")
        .explain(r"\eta_2T_2(x)", "The second natural parameter, against its statistic",
            "The other coordinate of the density's exponent.")
        .explain(r"A(\eta)", "The normaliser",
            "The term that makes the density integrate to one.")
        .explain(r"T(x)=(x,x^2)", "The Gaussian's sufficient statistic",
            "x and x squared: everything about a sample a Gaussian ever reads.")
        .explain(r"T(x)", "The sufficient statistic",
            "The function of x that the density reads.")
        .explain(r"(x,x^2)", "x and x squared",
            "The two functions of x that a Gaussian reads, and nothing else.")
        .explain(r"\eta_1", "The first natural parameter",
            "For a Gaussian, the mean divided by the variance.")
        .explain(r"\mu/\sigma^2", "The mean, weighted by the precision",
            "The Gaussian's first natural coordinate.")
        .explain(r"\eta_2", "The second natural parameter",
            "For a Gaussian, minus one over twice the variance.")
        .explain(r"-1/(2\sigma^2)", "Minus half the precision",
            "Minus one over twice the variance: the Gaussian's second natural coordinate.")
        .explain(r"P^{-1}\hat x", "The information-form estimate",
            "The estimate weighted by the precision matrix: the first of the two coordinates in which the update is vector addition.")
        .explain(r"P^{-1}", "The information matrix",
            "The inverse of the covariance: precision in matrix form.")
        .para(|p| p
            .text("Why should the ")
            .math(r"x^2")
            .text(" coefficient be what \"confidence\" means? Because it is the curvature of the log-density: ")
            .math(r"-\frac{d^2}{dx^2}\log p(x) = 1/\sigma^2")
            .text(", exactly, at every ")
            .math(r"x")
            .text(". Curvature of the log measures how fast the density falls away from its peak, so it is inverse squared width: for every Gaussian, whatever its ")
            .math(r"\sigma")
            .text(", the log-density has fallen by exactly ")
            .math(r"\frac12")
            .text(" at ")
            .math(r"x = \mu\pm\sigma")
            .text(". \"Curvature\" and \"")
            .math(r"1/\mathrm{width}^2")
            .text("\" are one number in two costumes."))
        .explain(r"-\frac{d^2}{dx^2}\log p(x)", "The curvature of the log-density",
            "How fast the density falls away from its peak. For a Gaussian it is the precision, at every x.")
        .explain(r"\mu\pm\sigma", "One standard deviation either side of the mean",
            "The two points at which the log-density has fallen by exactly a half.")
        .explain(r"1/\mathrm{width}^2", "One over the width, squared",
            "The other costume curvature wears: inverse squared width.")
        .para(|p| p
            .text("And this is not a Gaussian quirk. The quantity ")
            .math(r"-\mathbb{E}[∂_\theta^2\log p]")
            .text(" is the Fisher information ")
            .math(r"I(\theta)")
            .text(", which for the mean of ")
            .math(r"\mathcal{N}(\theta,\sigma^2)")
            .text(" is exactly ")
            .math(r"1/\sigma^2")
            .text("; which adds over independent observations (")
            .math(r"I_n = nI_1")
            .text("); and which the Cramér–Rao bound shows to be the exact reciprocal of the best variance any unbiased estimator can achieve, ")
            .math(r"\mathrm{Var}(\hat\theta)\ge 1/I(\theta)")
            .text(". That triple is the real answer to \"why that quantity and no other\": ")
            .math(r"1/\sigma^2")
            .text(" is the summary of a source that is additive when evidence is pooled and whose reciprocal is the attainable variance. The Cauchy–Schwarz floor above and Cramér–Rao's ")
            .math(r"1/I")
            .text(" are, for the Gaussian, the same number — which is the structural reason the two derivations cannot disagree."))
        .explain(r"-\mathbb{E}[∂_\theta^2\log p]", "The Fisher information, written out",
            "Minus the average curvature of the log-density in the parameter: how sharply it bends as the parameter moves.")
        .explain(r"I(\theta)", "The Fisher information",
            "How much a source tells you about the parameter θ. For the mean of a Gaussian it is exactly one over the variance.")
        .explain(r"\mathcal{N}(\theta,\sigma^2)", "A Gaussian with unknown mean",
            "A normal distribution whose mean is the parameter being estimated.")
        .explain(r"I_n", "The information in n observations",
            "How much n independent observations tell you: n times what one tells you.")
        .explain(r"nI_1", "n times one observation's information",
            "Information adds over independent observations, so n of them carry n times as much.")
        .explain(r"\mathrm{Var}(\hat\theta)", "The variance of an estimator",
            "How wrong an estimator of θ is, written as a variance.")
        .explain(r"1/I(\theta)", "The Cramér–Rao floor",
            "The reciprocal of the Fisher information: the best variance any unbiased estimator can achieve.")
        .explain(r"1/I", "One over the information",
            "The attainable variance, written short.")
        .para(|p| p
            .text("Note also the generalisation and its price. Curvatures of log-densities always add, because differentiation is linear. What is special about the Gaussian is that the curvature does not depend on ")
            .math(r"x")
            .text(", so one number suffices instead of a function. Every nonlinear method downstream — the extended Kalman filter, the Laplace approximation, Gauss–Newton — is the act of pretending the curvature is constant near the current estimate."))
        .para(|p| p
            .text("Last question in this chain: why do the logs add in the first place — why is stacking evidence a multiplication? Bayes gives ")
            .math(r"p(x\mid z_1,z_2) \propto p(z_1\mid x)\,p(z_2\mid x)\,p(x)")
            .text(", and the sole reason the two likelihoods appear as a product is that conditional independence is defined as the joint density factorising. The logarithm then converts that product into a sum. Two things are worth noticing. Prior and likelihood enter completely symmetrically — a prior is only previously digested evidence — which is why one formula serves both \"fuse two sensors\" and \"fuse belief with sensor\", and is what licenses the recursion. And nothing in this step knows anything about Gaussians: evidence always combines by adding log-densities. It is the Gaussian's quadratic log that collapses \"add two functions\" into \"add two numbers\"."))
        .explain(r"p(x\mid z_1,z_2)", "The posterior density",
            "How the unknown x is distributed once both measurements are in hand.")
        .explain(r"p(z_1\mid x)\,p(z_2\mid x)\,p(x)", "The two likelihoods and the prior, multiplied",
            "What Bayes says the posterior is proportional to. The two likelihoods appear as a product because conditional independence is defined as the joint density factorising.")
        .explain(r"z_1,z_2", "The two measurements",
            "The first and second observations being folded in.")
        .para(|p| p
            .text("Bedrock: one definition and one identity. Independence is the factorisation of the joint density — not a claim that could be checked and found false — and ")
            .math(r"e^{u+v}=e^ue^v")
            .text(" is the defining functional equation of the exponential rather than a theorem about it. There is nothing beneath either to ask \"why\" of."))
        .explain(r"e^ue^v", "The two exponentials, multiplied",
            "The exponential of u times the exponential of v: the other side of the exponential's defining equation.")
}

fn why_two_derivations_agree(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — why two completely different derivations give the same answer")
        .para(|p| p
            .text("One derivation assumed Gaussian densities. The other assumed no distribution whatsoever. They land on identical weights. Why?"))
        .para(|p| p
            .text("The shallow answer is that both minimise the same quadratic. Bayes minimises the negative log posterior ")
            .math(r"\frac{(x-\mu_1)^2}{2\sigma_1^2}+\frac{(x-\mu_2)^2}{2\sigma_2^2}")
            .text(" over ")
            .math(r"x")
            .text("; minimum-variance minimises ")
            .math(r"w^2\sigma_1^2 + (1-w)^2\sigma_2^2")
            .text(" over ")
            .math(r"w")
            .text(". Different variables, but each is a convex quadratic with the two precisions as its only inputs, so each yields a linear stationarity condition balancing ")
            .math(r"\sigma_1^2")
            .text(" against ")
            .math(r"\sigma_2^2")
            .text(". True — but that notes a coincidence of shapes rather than explaining it."))
        .explain(r"\frac{(x-\mu_1)^2}{2\sigma_1^2}", "The first source's penalty",
            "How far a candidate x sits from the first estimate, squared and divided by twice its variance.")
        .explain(r"\frac{(x-\mu_2)^2}{2\sigma_2^2}", "The second source's penalty",
            "How far a candidate x sits from the second estimate, squared and divided by twice its variance.")
        .para(|p| p
            .text("The real reason is that under Gaussianity the best estimator of all is already a linear one. Two facts meet:"))
        .para(|p| p
            .text("1. The conditional mean ")
            .math(r"\mathbb{E}[x\mid z]")
            .text(" minimises mean squared error among all functions of ")
            .math(r"z")
            .text(" — the orthogonality principle, which holds for any distribution whatever."))
        .explain(r"\mathbb{E}[x\mid z]", "The conditional mean",
            "The average of x once z is known. It minimises mean squared error among all functions of z, whatever the distribution.")
        .para(|p| p
            .text("2. For jointly Gaussian ")
            .math(r"(x,z)")
            .text(", that conditional mean is affine in ")
            .math(r"z")
            .text(": ")
            .math(r"\mu_x + \Sigma_{xz}\Sigma_{zz}^{-1}(z-\mu_z)")
            .text("."))
        .explain(r"(x,z)", "The state and the measurement, taken together",
            "The pair whose joint distribution is being assumed Gaussian.")
        .explain(r"\mu_x", "The mean of the state",
            "Where the state sits on average, before the measurement is read.")
        .explain(r"(z-\mu_z)", "The measurement's surprise",
            "How far the measurement landed from where it was expected to land.")
        .explain(r"\Sigma_{xz}\Sigma_{zz}^{-1}", "The affine coefficient",
            "How the state and measurement move together, divided by how the measurement moves on its own.")
        .explain(r"\mu_z", "The mean of the measurement",
            "Where the measurement was expected to land.")
        .para(|p| p
            .text("Put together: Gaussianity places the unrestricted optimum inside the linear class. The minimum-variance route, searching only linear blends, is not settling for second best — the best was already in the room. This also marks exactly where the two routes part company: with non-Gaussian noise the linear route still returns the best linear unbiased estimator, while some nonlinear function of ")
            .math(r"z")
            .text(" may beat it, and the Kalman filter is then optimal only within its class."))
        .para(|p| p
            .text("Where does that affine coefficient come from? Build the residual ")
            .math(r"r = (x-\mu_x) - \Sigma_{xz}\Sigma_{zz}^{-1}(z-\mu_z)")
            .text(" and compute its covariance with the data:"))
        .explain(r"(x-\mu_x)", "The state's own departure from its mean",
            "How far the state sits from where it was expected to sit.")
        .display(r"\mathrm{Cov}(r,z) = \Sigma_{xz} - \Sigma_{xz}\Sigma_{zz}^{-1}\Sigma_{zz} = 0")
        .explain(r"\mathrm{Cov}(r,z)", "The covariance between residual and data",
            "How the leftover moves with the data that was used. Making it zero is what defines the coefficient.")
        .explain(r"\Sigma_{xz}", "The cross-covariance",
            "How the state and the measurement move together.")
        .explain(r"\Sigma_{xz}\Sigma_{zz}^{-1}\Sigma_{zz}", "The coefficient, undone by the data's own spread",
            "The affine coefficient multiplied back by the measurement covariance, which returns the cross-covariance and cancels it.")
        .para(|p| p
            .text("One line, using only second moments and no distributional assumption at all. The coefficient ")
            .math(r"\Sigma_{xz}\Sigma_{zz}^{-1}")
            .text(" is defined by the demand \"make whatever is left over uncorrelated with the data you used\" — and with ")
            .math(r"\Sigma_{xz} = P^-H^\top")
            .text(" and ")
            .math(r"\Sigma_{zz} = S")
            .text(" it is the Kalman gain ")
            .math(r"K = P^-H^\top S^{-1}")
            .text("."))
        .explain(r"\Sigma_{zz}", "The measurement's own covariance",
            "How the measurement moves on its own. In filter notation it is S.")
        .explain(r"P^-H^\top", "The state-measurement cross-covariance, in filter notation",
            "How the state and the measurement move together, built from the prior covariance and the measurement map.")
        .para(|p| p
            .text("Only now is Gaussianity invoked, and for exactly one purpose: for jointly Gaussian variables, uncorrelated implies independent, so ")
            .math(r"\mathbb{E}[r\mid z] = \mathbb{E}[r] = 0")
            .text(" and the affine expression is the true conditional mean rather than merely the best linear approximation to it. The entire relationship between the two derivations is that sentence: the second-moment route computes the decorrelating coefficient, and Gaussianity upgrades \"uncorrelated\" to \"independent\", which upgrades \"best linear\" to \"best\"."))
        .explain(r"\mathbb{E}[r\mid z]", "The average leftover, given the data",
            "What the residual averages to once the measurement is known. Being zero is what makes the affine expression the true conditional mean.")
        .explain(r"\mathbb{E}[r]", "The average leftover",
            "What the residual averages to. It is zero by construction.")
        .para(|p| p
            .text("Why does uncorrelated imply independent for joint Gaussians, when it fails for essentially every other family? Because the joint density touches its variables only through ")
            .math(r"\exp\left(-\frac12 u^\top\Sigma^{-1}u\right)")
            .text(". \"Uncorrelated\" means ")
            .math(r"\Sigma")
            .text(" is block diagonal; the inverse of a block-diagonal matrix is block diagonal (multiply the blocks and check); a block-diagonal quadratic form is a sum over blocks with no cross-terms; and the exponential of a sum is a product. The density factorises, which is the definition of independence. For any other family, zero covariance kills exactly one number — the second cross-moment — and leaves every higher-order dependence untouched."))
        .explain(r"\exp\left(-\frac12 u^\top\Sigma^{-1}u\right)", "The multivariate Gaussian's kernel",
            "The only place the joint density touches its variables: a quadratic form built from the inverse covariance.")
        .explain(r"\frac12 u^\top\Sigma^{-1}u", "The quadratic form in the exponent",
            "How far the vector u sits from the centre, measured in units the covariance sets.")
        .explain(r"\Sigma^{-1}", "The inverse covariance",
            "Precision in matrix form: what the Gaussian's exponent is actually built from.")
        .para(|p| p
            .text("And why is a method that reads only two moments entitled to lose nothing? Because for a Gaussian there is nothing else to read. A Gaussian is precisely the exponential of a quadratic, and a quadratic is precisely three coefficients with one spent on normalisation — so a Gaussian is two numbers. Mean and variance are not a summary of a Gaussian; they are a complete description of it. The agreement is forced, not lucky. (The sophisticated restatement, for later: the Gaussian is the maximum-entropy distribution with a given mean and variance — the unique law that commits to its two moments and to nothing beyond them.)"))
        .para(|p| p
            .text("Bedrock: a definition. The chain stops at what a Gaussian is — the exponential of a quadratic, hence exactly two free numbers — and the coincidence of the two routes is a restatement of that definition rather than a separate fact needing its own justification."))
}

fn idea_two_at_work(b: LessonBuilder) -> LessonBuilder {
    b.heading("Idea 2 at work — two springs, and a bond nobody wants to price")
        .para(|p| p
            .text("In the physical world. Picture two springs tied to the same knot, anchored at the two estimates. Each spring pulls the knot toward its own anchor with force ")
            .math(r"k(x-\mu)")
            .text(", and stiffness is precision: ")
            .math(r"k = 1/\sigma^2")
            .text(". Balancing the two forces, ")
            .math(r"k_1(x-\mu_1) + k_2(x-\mu_2)=0")
            .text(", gives"))
        .explain(r"k(x-\mu)", "The spring's pull",
            "How hard a spring pulls the knot toward its own anchor: its stiffness times how far the knot has been dragged away.")
        .explain(r"1/\sigma^2", "The precision, as a stiffness",
            "One over the variance. In the spring picture it is literally how stiff that spring is.")
        .explain(r"(x-\mu_1)", "The knot's distance from the first anchor",
            "How far the resting knot sits from the first estimate.")
        .explain(r"(x-\mu_2)", "The knot's distance from the second anchor",
            "How far the resting knot sits from the second estimate.")
        .display(r"x = \frac{k_1\mu_1 + k_2\mu_2}{k_1+k_2}, \qquad k_{\mathrm{combined}} = k_1 + k_2")
        .explain(r"\frac{k_1\mu_1 + k_2\mu_2}{k_1+k_2}", "The stiffness-weighted mean",
            "Each anchor weighted by its own spring's stiffness, divided by the two stiffnesses added: the precision-weighted mean.")
        .explain(r"k_1\mu_1", "The first anchor, weighted by stiffness",
            "The first anchor's position multiplied by its spring's stiffness.")
        .explain(r"k_2\mu_2", "The second anchor, weighted by stiffness",
            "The second anchor's position multiplied by its spring's stiffness.")
        .explain(r"k_1+k_2", "The two stiffnesses added",
            "The joint's stiffness: stiffness adds exactly as precision does.")
        .explain(r"\qquad k_{\mathrm{combined}}", "The stiffness of the joint",
            "How stiff the knot is once both springs are attached.")
        .explain(r"k_1", "The first spring's stiffness",
            "The first source's precision, in the spring picture.")
        .explain(r"k_2", "The second spring's stiffness",
            "The second source's precision, in the spring picture.")
        .para(|p| p
            .text("which is the precision-weighted mean and \"precisions add\", with no probability in sight. The stored energy ")
            .math(r"\frac12 k_1(x-\mu_1)^2 + \frac12 k_2(x-\mu_2)^2")
            .text(" is exactly the sum of squared errors the filter minimises — the knot settles where the belief is most likely. The same algebra is on every circuit diagram: two 4 Ω resistors in parallel make 2 Ω, just as two variance-4 estimates make variance 2."))
        .explain(r"\frac12 k_1", "Half the first spring's stiffness",
            "The half-stiffness that multiplies the first spring's stretch in the stored energy.")
        .explain(r"(x-\mu_1)^2", "The first spring's stretch, squared",
            "How far the knot sits from the first anchor, squared.")
        .explain(r"\frac12 k_2", "Half the second spring's stiffness",
            "The half-stiffness that multiplies the second spring's stretch in the stored energy.")
        .explain(r"(x-\mu_2)^2", "The second spring's stretch, squared",
            "How far the knot sits from the second anchor, squared.")
        .para(|p| p
            .text("Toy numbers. Anchors 4 cm apart, at 100 cm and 104 cm, equal stiffness: the knot sits at 102 cm, and the joint is twice as stiff as either spring — compliance ")
            .math(r"4\to2")
            .text("."))
        .explain(r"4\to2", "Compliance halved",
            "The joint's compliance against a single spring's: bolting on a second spring of equal stiffness halves it.")
        .para(|p| p
            .text("Now make the second spring three times floppier in the ")
            .math(r"\sigma")
            .text(" sense: ")
            .math(r"\sigma_2 = 6")
            .text(" against ")
            .math(r"\sigma_1 = 2")
            .text(". Where does the knot sit? The tempting answer is a quarter of the way across, at 101 cm — three-to-one, by the ratio of the ")
            .math(r"\pm")
            .text("s. It sits one tenth of the way, at 100.4 cm. Three times floppier in spread means ")
            .math(r"3^2 = 9")
            .text(" times floppier in stiffness, so the weights are 9:1, not 3:1."))
        .explain(r"3^2", "Three, squared",
            "Nine: what three times floppier in spread comes to in stiffness.")
        .para(|p| p
            .text("And the picture that matters most: bolting on an extra spring, however floppy, can only ever stiffen the joint. You can never make an estimate worse by fusing in another independent one."))
        .para(|p| p
            .text("In finance — the running example. You mark an illiquid corporate bond every day. Your carried mark is ")
            .math(r"\hat x^- = 100")
            .text(" with ")
            .math(r"P^- = 4")
            .text(", a one-sigma of ")
            .math(r"\pm\$2")
            .text(". A dealer quote arrives at ")
            .math(r"z = 104")
            .text(". The update is"))
        .display(r"K = \frac{P^-}{P^-+R}, \qquad \hat x^+ = \hat x^- + K(z-\hat x^-), \qquad P^+ = (1-K)P^-")
        .explain(r"K(z-\hat x^-)", "A fraction of the disagreement, in the scalar case",
            "The gap between the quote and your mark, taken as far as the gain allows.")
        .explain(r"\qquad \hat x^+", "The posterior estimate",
            "The best guess after the measurement has been folded in.")
        .explain(r"\qquad P^+", "The posterior variance",
            "The variance of the belief after the measurement has been folded in.")
        .explain(r"104", "A hundred and four dollars",
            "The dealer's quote on the bond.")
        .explain(r"z-\hat x^-", "The disagreement",
            "How far the quote sits from your own mark.")
        .explain(r"(1-K)", "One minus the gain",
            "The share of your prior variance that survives the update.")
        .para(|p| p
            .text("Reading it: ")
            .math(r"z - \hat x^-")
            .text(" is the disagreement — here $4 — between the market and you; ")
            .math(r"R")
            .text(" is how noisy a single quote is; ")
            .math(r"P^-")
            .text(" is how unsure you were before it arrived; and ")
            .math(r"K")
            .text(" is the share of the total confusion that is yours rather than the quote's. Equivalently, in precision form,"))
        .display(r"\frac{1}{P^+} = \frac{1}{P^-}+\frac{1}{R}, \qquad \frac{\hat x^+}{P^+} = \frac{\hat x^-}{P^-} + \frac{z}{R}")
        .explain(r"\frac{1}{P^+}", "The posterior precision",
            "One over the variance you are left with: how much you know after the quote.")
        .explain(r"\frac{1}{P^-}", "The prior precision",
            "One over the variance you carried in: how much you knew before the quote.")
        .explain(r"\frac{1}{R}", "The quote's precision",
            "One over the quote's noise: how much a single quote knows.")
        .explain(r"\qquad \frac{\hat x^+}{P^+}", "The new mark, weighted by its precision",
            "The posterior estimate divided by the posterior variance.")
        .explain(r"\frac{\hat x^-}{P^-}", "The old mark, weighted by its precision",
            "The prior estimate divided by the prior variance.")
        .explain(r"\frac{z}{R}", "The quote, weighted by its precision",
            "The dealer quote divided by how noisy a single quote is.")
        .para(|p| p
            .text("Case 1 — the quote is as good as your mark, ")
            .math(r"R = 4")
            .text(". Then ")
            .math(r"K = 4/8 = 0.5")
            .text(", so ")
            .math(r"\hat x^+ = \$102")
            .text(". And the new uncertainty?"))
        .explain(r"4/8", "Four over eight",
            "The gain when your own doubt and the quote's noise are equal.")
        .explain(r"0.5", "Half",
            "The gain when you and the quote are equally unsure: you move exactly half the way.")
        .explain(r"\$102", "A hundred and two dollars",
            "Where the mark lands when you move half the way from 100 toward the 104 quote.")
        .para(|p| p
            .text("You mark an illiquid corporate bond at ")
            .math(r"\hat x^- = 100")
            .text(" with ")
            .math(r"P^- = 4")
            .text(", i.e. one-sigma ")
            .math(r"\pm\$2")
            .text(". A dealer quote arrives at ")
            .math(r"z = 104")
            .text(", and a single quote on this bond is worth about ")
            .math(r"\pm\$2")
            .text(", so ")
            .math(r"R = 4")
            .text(". Work through ")
            .math(r"K")
            .text(", the new mark, and the new uncertainty."))
        .para(|p| p
            .text("Follow the three lines in order."))
        .display(r"K = \frac{P^-}{P^- + R} = \frac{4}{4+4} = \frac12, \qquad \hat x^+ = 100 + \frac12(104 - 100) = \$102, \qquad P^+ = (1-K)P^- = \frac12 \times 4 = 2.")
        .explain(r"\frac{4}{4+4}", "Four over eight",
            "Your own doubt over your doubt plus the quote's, when the two are equal.")
        .explain(r"(104-100)", "The disagreement, in dollars",
            "The four dollars between your mark and the quote.")
        .explain(r"2.", "Two",
            "The variance left after folding in a quote as good as your own mark.")
        .para(|p| p
            .text("Read each one. ")
            .math(r"z - \hat x^- = \$4")
            .text(" is the disagreement between the market and you. ")
            .math(r"K")
            .text(" is the share of the total confusion that is yours rather than the quote's — here exactly half, because you and the quote are equally unsure, so you move exactly half way. And the new uncertainty is ")
            .math(r"P^+ = 2")
            .text(", one-sigma ")
            .math(r"\surd 2 = \pm\$1.41")
            .text(" — not the ")
            .math(r"\pm\$2")
            .text(" you might expect from averaging two ")
            .math(r"\pm\$2")
            .text(" numbers. The precision form says why: ")
            .math(r"\frac{1}{P^+} = \frac{1}{P^-} + \frac{1}{R} = \frac14 + \frac14 = \frac12")
            .text(". Precisions add, so two agreeing sources leave you more certain than either one alone."))
        .explain(r"\$4", "Four dollars",
            "The disagreement between the market and you.")
        .explain(r"\surd 2", "The square root of two",
            "About 1.41: the one-sigma spread a variance of 2 comes to.")
        .explain(r"\pm\$1.41", "Give or take a dollar forty-one",
            "The one-sigma spread left after folding in a quote as good as your own mark.")
        .explain(r"\frac14", "A quarter",
            "The precision of a variance-4 belief: one over four.")
        .para(|p| p
            .text("Most people say ")
            .math(r"\pm\$2")
            .text(": you averaged two ")
            .math(r"\pm\$2")
            .text(" numbers, so surely you are as unsure as before. It is ")
            .math(r"\pm\$1.41")
            .text(", because ")
            .math(r"1/4 + 1/4 = 1/2")
            .text(" and so ")
            .math(r"P^+ = 2")
            .text(". Agreeing sources make you more certain than either alone."))
        .explain(r"1/4", "A quarter",
            "One over a variance of four: the precision each of the two equally uncertain sources carries.")
        .explain(r"1/2", "A half",
            "The two quarter-precisions added together.")
        .para(|p| p
            .text("Case 2 — a thin day, one wide quote, ")
            .math(r"\pm\$6")
            .text(" so ")
            .math(r"R = 36")
            .text(". How far do you move your mark toward $104?"))
        .explain(r"\pm\$6", "Give or take six dollars",
            "How good the only quote you can get on a thin day is.")
        .para(|p| p
            .text("Same bond, same carried mark ")
            .math(r"\hat x^- = 100")
            .text(" with ")
            .math(r"P^- = 4")
            .text(" (")
            .math(r"\pm\$2")
            .text("), same quote ")
            .math(r"z = 104")
            .text(". But today is thin and the only quote you can get is worth about ")
            .math(r"\pm\$6")
            .text(", so ")
            .math(r"R = 36")
            .text(". The gain is still ")
            .math(r"K = P^-/(P^- + R)")
            .text(", and ")
            .math(r"K = 4/40 = 0.1")
            .text(". Finish it: where does the mark end up, and what is ")
            .math(r"P^+")
            .text("?"))
        .explain(r"P^-/", "Your own doubt, divided by what follows",
            "The prior variance, about to be divided by the total confusion.")
        .explain(r"(P^- + R)", "The total confusion",
            "How unsure you were before the quote, plus how noisy the quote is.")
        .explain(r"4/40", "Four over forty",
            "The gain on a thin day: one tenth.")
        .rule()
        .note("Do the two lines and commit to both numbers before you read on — say the new mark, in cents, and the new variance out loud.")
        .para(|p| p
            .math(r"\hat x^+ = 100 + 0.1 \times (104-100) = \$100.40")
            .text(", and ")
            .math(r"P^+ = (1 - 0.1) \times 4 = 3.6")
            .text(", one-sigma ")
            .math(r"\surd 3.6 = \pm\$1.90")
            .text(". You moved one tenth of the way to the quote. If you expected roughly a quarter of the way — $101, weighting your ")
            .math(r"\pm\$2")
            .text(" against its ")
            .math(r"\pm\$6")
            .text(" in the ratio ")
            .math(r"6:2")
            .text(" — you weighted by standard deviations. The arithmetic runs on variances: ")
            .math(r"4")
            .text(" against ")
            .math(r"36")
            .text(" is ")
            .math(r"1:9")
            .text(", so a quote three times noisier in ")
            .math(r"\pm")
            .text(" terms gets one ninth of the say, not one third. Note also that the band still tightened, from ")
            .math(r"\pm\$2.00")
            .text(" to ")
            .math(r"\pm\$1.90")
            .text(": even a wide quote is information."))
        .explain(r"\$100.40", "A hundred dollars forty",
            "Where the mark lands after moving one tenth of the way toward the quote.")
        .explain(r"(1-0.1)", "Nine tenths",
            "What survives of the prior variance when the gain is a tenth.")
        .explain(r"3.6", "Three point six",
            "The variance left after a wide quote has been folded in.")
        .explain(r"\surd 3.6", "The square root of three point six",
            "About 1.90: the one-sigma spread a variance of 3.6 comes to.")
        .explain(r"\pm\$1.90", "Give or take a dollar ninety",
            "The one-sigma spread left after the thin-day quote.")
        .explain(r"6:2", "Six to two",
            "The ratio of the two spreads, which is the tempting but wrong basis for the weights.")
        .explain(r"36", "Thirty-six",
            "The variance of a quote good to plus or minus six dollars.")
        .explain(r"1:9", "One to nine",
            "The ratio of the two variances, which is what the weights actually run on.")
        .explain(r"\pm\$2.00", "Give or take two dollars",
            "The one-sigma spread carried in before the quote arrived.")
        .para(|p| p
            .text("The tempting answer weights by the ")
            .math(r"\pm")
            .text("s, ")
            .math(r"6:2")
            .text(", moving a quarter of the way to $101. The truth is ")
            .math(r"K = 4/40 = 0.1")
            .text(": you move one tenth of the way, to $100.40, and ")
            .math(r"P^+ = 0.9\times 4 = 3.6")
            .text(", a spread of ")
            .math(r"\pm\$1.90")
            .text(". Weights go as ")
            .math(r"1/\sigma^2")
            .text(", so a quote three times noisier gets one ninth the say."))
        .explain(r"0.9", "Nine tenths",
            "The share of the prior variance that survives a gain of one tenth.")
        .para(|p| p
            .text("Case 3 — a near-useless quote, ")
            .math(r"\pm\$10")
            .text(", so ")
            .math(r"R = 100")
            .text(". Does letting junk into the blend pollute your mark?"))
        .explain(r"\pm\$10", "Give or take ten dollars",
            "How good a near-useless quote is.")
        .para(|p| p
            .text("Carried mark ")
            .math(r"\hat x^- = 100")
            .text(", ")
            .math(r"P^- = 4")
            .text(". This morning two independent dealer quotes arrive, both worth about ")
            .math(r"\pm\$2")
            .text(" (")
            .math(r"R = 4")
            .text(" each): first ")
            .math(r"z_1 = 104")
            .text(", then ")
            .math(r"z_2 = 102")
            .text(". Fold them in one at a time. Where does the mark finish, what is ")
            .math(r"P")
            .text(", and what would the answer have been if you had only had the second quote?"))
        .explain(r"z_1", "The first quote",
            "The first of the two independent dealer quotes to arrive.")
        .explain(r"z_2", "The second quote",
            "The second of the two independent dealer quotes to arrive.")
        .explain(r"102", "A hundred and two dollars",
            "The second dealer quote, which lands exactly on the mark.")
        .rule()
        .note("Fold the quotes in one at a time and commit to all three numbers first — the finishing mark, the variance, and what the second quote alone would have given.")
        .para(|p| p
            .text("First quote, exactly as in the worked case: ")
            .math(r"K = 4/8 = \frac12")
            .text(", mark ")
            .math(r"\to \$102")
            .text(", ")
            .math(r"P \to 2")
            .text(". Second quote, now against ")
            .math(r"P^- = 2")
            .text(": ")
            .math(r"K = 2/(2+4) = \frac13")
            .text(", and the innovation is ")
            .math(r"102 - 102 = 0")
            .text(", so the mark does not move — it stays at $102 — while ")
            .math(r"P \to (1-\frac13)\times 2 = \frac43 \approx 1.33")
            .text(", one-sigma ")
            .math(r"\pm\$1.15")
            .text(". The second quote moved the estimate not at all and yet made you meaningfully more certain, which is the cleanest possible demonstration that the mean and the variance are updated by different things."))
        .explain(r"\to \$102", "The mark moves to a hundred and two",
            "Where the first quote leaves the mark.")
        .explain(r"P \to 2", "The variance falls to two",
            "Where the first quote leaves the uncertainty.")
        .explain(r"2/", "Your own doubt, divided by what follows",
            "The variance you now carry, about to be divided by the total confusion.")
        .explain(r"(2+4)", "The total confusion, second time round",
            "Your remaining doubt of two, plus the quote's noise of four.")
        .explain(r"\frac13", "A third",
            "The gain on the second quote, now that your own doubt has already been cut to two.")
        .explain(r"102 - 102 = 0", "No disagreement at all",
            "The second quote lands exactly on the mark, so there is nothing for the estimate to move on.")
        .explain(r"P\to", "The variance becomes",
            "The variance is replaced by what follows.")
        .explain(r"(1-\frac13)", "Two thirds",
            "What survives of the variance when the gain is a third.")
        .explain(r"\frac43", "Four thirds",
            "The variance left after both quotes: about 1.33.")
        .explain(r"1.33", "About one and a third",
            "The variance left after both quotes, in decimals.")
        .explain(r"\pm\$1.15", "Give or take a dollar fifteen",
            "The one-sigma spread left after both quotes.")
        .para(|p| p
            .text("Check it in precision form: ")
            .math(r"\frac14 + \frac14 + \frac14 = \frac34")
            .text(", so ")
            .math(r"P = \frac43")
            .text(", and the precision-weighted mean is ")
            .math(r"(100/4 + 104/4 + 102/4)/(3/4) = 76.5/0.75 = \$102")
            .text(" — order of arrival irrelevant. With only the second quote you would have held ")
            .math(r"K = 4/8 = \frac12")
            .text(", mark ")
            .math(r"\$101")
            .text(", ")
            .math(r"P = 2")
            .text(": a different mark and twice the variance."))
        .explain(r"\frac34", "Three quarters",
            "The three quarter-precisions added: your own and the two quotes'.")
        .explain(r"(100/4 + 104/4 + 102/4)", "Your mark and both quotes, each weighted by its precision",
            "Every source divided by its own variance, added together.")
        .explain(r"/(3/4)", "Divided by the total precision",
            "The weighted total shared out over the three quarter-precisions added together.")
        .explain(r"76.5/0.75", "The same division, worked out",
            "The weighted total over the total precision, which comes to 102.")
        .explain(r"\$101", "A hundred and one dollars",
            "Where the mark would have finished on the second quote alone.")
        .para(|p| p
            .text("It cannot. ")
            .math(r"K = 4/104 = 0.038")
            .text(", the mark creeps $0.15 to $100.15, and the band tightens from $2.00 to $1.96 (")
            .math(r"P^+ = 3.85")
            .text("). Bad data can only move you a little; it can never move you backwards. This is why a filter never discards an observation, and why the same precision-weighted blend reappears wherever finance merges a prior with evidence — the Black–Litterman posterior is an equilibrium prior and a set of views, each weighted by its own precision, in exactly this shape."))
        .explain(r"4/104", "Four over a hundred and four",
            "The gain on a near-useless quote.")
        .explain(r"0.038", "Under four per cent",
            "The gain on a near-useless quote: the mark creeps, and no more.")
        .explain(r"3.85", "Three point eight five",
            "The variance left after a near-useless quote has been folded in — smaller than the four you started with.")
        .para(|p| p
            .text("A story to fix it. A stale dealer mark arrives on that same bond, carried at $100 with ")
            .math(r"P^- = 4")
            .text(". The quote is good to about ")
            .math(r"\pm\$10")
            .text(" — plainly junk — so the junior deletes it before it can contaminate the book. The senior makes him put it back. Predict the damage before reading on. ")
            .math(r"K = 4/104 = 0.038")
            .text(": the mark creeps fifteen cents to $100.15, and the band tightens, from ")
            .math(r"\pm\$2.00")
            .text(" to ")
            .math(r"\pm\$1.96")
            .text(". There is no such thing as an independent observation that leaves you less certain — only ones that barely move you, which is why the filter never throws a print away."))
        .explain(r"\pm\$1.96", "Give or take a dollar ninety-six",
            "The one-sigma spread left after even a near-useless quote: tighter than the two dollars you carried in.")
        .para(|p| p
            .text("Then the twist the junior should have worried about instead: the one thing that genuinely poisons the blend is not bad data, it is the same data twice, because \"precisions add\" needs the sources to be uncorrelated, and double-counted evidence keeps the cross term the derivation dropped."))
}

fn interlude_from_two_numbers_to_a_matrix(b: LessonBuilder) -> LessonBuilder {
    b.heading("Interlude — from two numbers to a matrix")
        .para(|p| p
            .text("Everything so far has used one uncertain quantity. Real problems have several at once, and the moment there are two, a new thing exists that was not there before: they can be wrong together. Idea 6 will show that this is where most of the filter's power comes from, so we need the vocabulary now."))
        .para(|p| p
            .text("Covariance measures how two quantities vary together:"))
        .display(r"\mathrm{Cov}(X,Y) = \mathbb{E}[(X-\mu_X)(Y-\mu_Y)]")
        .explain(r"\mathrm{Cov}(X,Y)", "The covariance of X and Y",
            "How two quantities vary together. Positive means they move together; zero means no linear relationship.")
        .explain(r"\mathbb{E}[(X-\mu_X)(Y-\mu_Y)]", "The average product of the two departures from the mean",
            "Each quantity's distance from its own mean, multiplied together and averaged: that is what covariance is.")
        .para(|p| p
            .text("Positive means they move together; zero means no linear relationship. Correlation ")
            .math(r"\rho = \mathrm{Cov}(X,Y)/(\sigma_X\sigma_Y)")
            .text(" is the unit-free version, always in ")
            .math(r"[-1,1]")
            .text("."))
        .explain(r"\mathrm{Cov}(X,Y)/(\sigma_X\sigma_Y)", "Correlation",
            "The unit-free version of covariance: the covariance divided by the two standard deviations.")
        .explain(r"[-1,1]", "From minus one to one",
            "The range a correlation always lies in.")
        .para(|p| p
            .text("Stack all of these into a grid and you get the covariance matrix ")
            .math(r"\Sigma")
            .text(" — written ")
            .math(r"P")
            .text(" throughout this subject — whose ")
            .math(r"(i,j)")
            .text(" entry is ")
            .math(r"\mathrm{Cov}(x_i,x_j)")
            .text(", with the variances down the diagonal. This is the single most important object in the subject. It is the filter's representation of everything it does not know."))
        .explain(r"(i,j)", "Row i, column j",
            "A position in the grid: the entry that holds the covariance of state i with state j.")
        .explain(r"\mathrm{Cov}(x_i,x_j)", "The covariance of two states",
            "What the covariance matrix keeps at row i and column j. Down the diagonal, where the two agree, it is a variance.")
        .para(|p| p
            .text("The multivariate Gaussian is the scalar one with the same parts in matrix dress:"))
        .display(r"p(x) = \frac{1}{(2\pi)^{n/2}|\Sigma|^{1/2}}\exp\left(-\frac12 (x-\mu)^\top\Sigma^{-1}(x-\mu)\right)")
        .explain(r"\frac{1}{(2\pi)^{n/2}|\Sigma|^{1/2}}\exp\left(-\frac12 (x-\mu)^\top\Sigma^{-1}(x-\mu)\right)", "The multivariate Gaussian density",
            "The scalar Gaussian with the same parts in matrix dress: a constant out front that makes the area 1, and an exponent built from how far x sits from the mean.")
        .para(|p| p
            .text("Three readings of that exponent, each of which you will use."))
        .para(|p| p
            .math(r"\Lambda = \Sigma^{-1}")
            .text(" is the precision matrix — \"how much you know\" rather than \"how unsure you are\". The information filter of Part 5 works entirely in this coordinate, and the reason will be familiar: in these coordinates, updating is addition."))
        .para(|p| p
            .math(r"d^2 = (x-\mu)^\top\Sigma^{-1}(x-\mu)")
            .text(" is the Mahalanobis distance — distance measured in standard deviations, accounting for correlation. The exponent is ")
            .math(r"-\frac12 d^2")
            .text(", so contours of constant density are contours of constant Mahalanobis distance: ellipses. The covariance ellipse picture is not an illustration of the density; it is the density."))
        .explain(r"d^2", "The squared Mahalanobis distance",
            "Distance measured in standard deviations, accounting for correlation.")
        .explain(r"(x-\mu)^\top", "The departure from the mean, laid on its side",
            "How far x sits from the mean, written as a row so it can multiply the precision matrix.")
        .explain(r"(x-\mu)", "The departure from the mean",
            "How far x sits from the mean.")
        .explain(r"-\frac12 d^2", "Minus half the squared Mahalanobis distance",
            "The exponent of the multivariate Gaussian. Contours of constant density are contours of constant Mahalanobis distance: ellipses.")
        .para(|p| p
            .text("The eigenvectors of ")
            .math(r"\Sigma")
            .text(" are the principal axes of that ellipse and the eigenvalues are the squared semi-axis lengths. A covariance matrix is a shape."))
        .para(|p| p
            .text("And here is the property the whole subject leans on. In general, independent implies uncorrelated but not conversely. For jointly Gaussian variables, uncorrelated does imply independent — and you can check it in a line. When ")
            .math(r"\Sigma")
            .text(" is block-diagonal, so is ")
            .math(r"\Sigma^{-1}")
            .text(" (multiply and see), and ")
            .math(r"|\Sigma| = |\Sigma_{aa}||\Sigma_{bb}|")
            .text(", so the Mahalanobis exponent splits into a plain sum; ")
            .math(r"\mathrm{exp}")
            .text(" of a sum is a product; the density factorises as ")
            .math(r"p(a)p(b)")
            .text(", which is the definition of independence. That is why, later, \"no correlation left\" will mean \"no information left\" rather than merely \"no linear information left\"."))
        .explain(r"|\Sigma|", "The determinant of the covariance matrix",
            "The volume scaling factor of the covariance, so it measures the volume of the uncertainty ellipsoid.")
        .explain(r"|\Sigma_{aa}||\Sigma_{bb}|", "The two block determinants multiplied",
            "What the determinant of a block-diagonal covariance comes to — one of the steps that makes the density factorise.")
        .explain(r"\mathrm{exp}", "The exponential function",
            "Raising e to the power of what follows. Of a sum, it is a product.")
        .explain(r"p(a)p(b)", "One density times the other",
            "The density split into a part for a and a part for b, which is the definition of independence.")
        .para(|p| p
            .text("Just enough linear algebra. A vector is an ordered list of numbers — here the state ")
            .math(r"x\in\mathbb{R}^n")
            .text(". A matrix is a rectangular array representing a linear map; ")
            .math(r"A\in\mathbb{R}^{m\times n}")
            .text(" maps ")
            .math(r"\mathbb{R}^n\to\mathbb{R}^m")
            .text(". The transpose ")
            .math(r"A^\top")
            .text(" reflects across the diagonal, the identity ")
            .math(r"I")
            .text(" is the do-nothing map, and the inverse ")
            .math(r"A^{-1}")
            .text(" is the undo map, which exists only when ")
            .math(r"A")
            .text(" is square and non-singular (")
            .math(r"\det A\ne0")
            .text(")."))
        .explain(r"\mathbb{R}^n", "n-dimensional real space",
            "Where the state lives: an ordered list of n numbers.")
        .explain(r"\mathbb{R}^{m\times n}", "The real matrices with m rows and n columns",
            "Rectangular arrays of that shape, each one representing a linear map.")
        .explain(r"\mathbb{R}^n\to\mathbb{R}^m", "Maps n-dimensional space into m-dimensional space",
            "What the matrix does: it takes a list of n numbers and returns a list of m.")
        .explain(r"A^\top", "The transpose of A",
            "A reflected across its diagonal.")
        .explain(r"A^{-1}", "The inverse of A",
            "The undo map, which exists only when A is square and non-singular.")
        .explain(r"\det A", "The determinant of A",
            "The number whose being non-zero is what makes A invertible.")
        .para(|p| p
            .text("Three algebraic facts get used constantly: matrix multiplication does not commute (")
            .math(r"AB\ne BA")
            .text(" in general); ")
            .math(r"(AB)^\top = B^\top A^\top")
            .text("; and ")
            .math(r"(AB)^{-1} = B^{-1}A^{-1}")
            .text(". Note that both of the last two reverse the order."))
        .explain(r"AB", "A times B",
            "The two linear maps applied one after the other.")
        .explain(r"BA", "B times A",
            "The same two maps in the other order, which in general is a different map: matrix multiplication does not commute.")
        .explain(r"(AB)^\top", "The transpose of a product",
            "The transpose of A times B.")
        .explain(r"B^\top A^\top", "The transposes, in reversed order",
            "What the transpose of a product comes to. The order reverses.")
        .explain(r"(AB)^{-1}", "The inverse of a product",
            "The undo map of A times B.")
        .explain(r"B^{-1}A^{-1}", "The inverses, in reversed order",
            "What the inverse of a product comes to. The order reverses here too.")
        .para(|p| p
            .text("A matrix is symmetric if ")
            .math(r"A=A^\top")
            .text(", positive semi-definite if ")
            .math(r"x^\top Ax\ge0")
            .text(" for all ")
            .math(r"x")
            .text(", and positive definite if ")
            .math(r">0")
            .text(" for ")
            .math(r"x\ne0")
            .text(". Every covariance matrix is symmetric positive semi-definite, and the filter's health depends on ")
            .math(r"P")
            .text(" staying that way."))
        .explain(r"x^\top Ax", "The matrix seen along a direction",
            "A single number built from the matrix and a direction x. The matrix is positive semi-definite when this is never negative, whatever x is.")
        .para(|p| p
            .text("What positive semi-definiteness actually asserts. ")
            .math(r"u^\top Pu")
            .text(" is the variance of the belief viewed along direction ")
            .math(r"u")
            .text(" — that is, the variance of the single number ")
            .math(r"u^\top x")
            .text(". So \"")
            .math(r"P")
            .text(" is positive semi-definite\" says: every one-dimensional view of the belief has non-negative spread. This is why one negative eigenvalue is fatal rather than cosmetic. It asserts that some linear combination of the states has negative variance, and no random quantity can have that."))
        .explain(r"u^\top Pu", "The belief's spread along a direction",
            "The variance of the belief viewed along direction u — that is, the variance of the single number u-transpose x.")
        .explain(r"u^\top x", "The state seen along a direction",
            "The single number you get by looking at the state along the direction u.")
        .para(|p| p
            .text("Three more objects, each of which turns up at a specific place later. The Cholesky decomposition ")
            .math(r"P = LL^\top")
            .text(" with ")
            .math(r"L")
            .text(" lower triangular is a \"matrix square root\", existing exactly when ")
            .math(r"P")
            .text(" is positive definite; it is used for sampling, for square-root filters, and for sigma points. The trace ")
            .math(r"\mathrm{tr}(A)=\sum_i A_{ii}")
            .text(" is, for a covariance, the total variance summed over coordinates — so minimising ")
            .math(r"\mathrm{tr}(P)")
            .text(" is minimising total expected squared error. The determinant ")
            .math(r"|A|")
            .text(" is the volume scaling factor, so ")
            .math(r"|P|")
            .text(" measures the volume of the uncertainty ellipsoid, and ")
            .math(r"\log|S_k|")
            .text(" will appear in the log-likelihood as the \"how surprised should I be\" normaliser."))
        .explain(r"LL^\top", "The Cholesky factor times its own transpose",
            "A matrix square root of P, with L lower triangular. It exists exactly when P is positive definite.")
        .explain(r"\mathrm{tr}(A)", "The trace of A",
            "The sum of the entries down the diagonal.")
        .explain(r"A_{ii}", "A diagonal entry",
            "The entry sitting on row i and column i, which is what the trace sums over.")
        .explain(r"\mathrm{tr}(P)", "The trace of P",
            "For a covariance, the total variance summed over coordinates — so minimising it is minimising total expected squared error.")
        .explain(r"|A|", "The determinant of A",
            "The volume scaling factor of the map A.")
        .explain(r"|P|", "The determinant of P",
            "The volume of the uncertainty ellipsoid.")
        .explain(r"\log|S_k|", "The log determinant of the innovation covariance",
            "The term that appears in the log-likelihood as the \"how surprised should I be\" normaliser.")
        .para(|p| p
            .text("Finally, the identity that appears more often than any other expression in this subject — thirteen of the seventeen deep chains in the source material lean on it:"))
        .display(r"\mathrm{Cov}(Ax) = A\Sigma A^\top")
        .para(|p| p
            .text("You already derived it. It is ")
            .math(r"\mathbb{E}[(X'+Y')^2] = \mathbb{E}[X'^2]+2\mathbb{E}[X'Y']+\mathbb{E}[Y'^2]")
            .text(" done with vectors and outer products instead of scalars and squares. Every \"sandwich\" in the filter — ")
            .math(r"FPF^\top")
            .text(", ")
            .math(r"HPH^\top")
            .text(", ")
            .math(r"KRK^\top")
            .text(" — is that one bracket-expansion wearing matrices."))
        .explain(r"2\mathbb{E}[X'Y']", "Twice the average of the product",
            "The cross term of the expansion: twice the expected product of the two centred quantities.")
}

fn idea_three(b: LessonBuilder) -> LessonBuilder {
    b.heading("Idea 3 — A Gaussian pushed through a linear map is still a Gaussian")
        .para(|p| p
            .text("This is the closure property, and it is the reason the filter exists as a finite object at all."))
        .para(|p| p
            .text("Linear maps preserve Gaussianity. Sums of independent Gaussians are Gaussian. Products of Gaussian densities are Gaussian. Conditionals of joint Gaussians are Gaussian. So no matter how long the filter runs, the belief is still describable by exactly two things: its mean and its covariance. The belief never changes shape. Only its mean and covariance move."))
        .para(|p| p
            .text("Why this generates the rules. Because the shape never changes, you never have to carry the distribution itself — only its two parameters. That is what makes the recursion finite, constant-memory and real-time. It is also exactly why the theory stops where it does: the moment linearity or Gaussianity fails, the belief changes shape and two numbers no longer suffice."))
        .para(|p| p
            .text("What falls out of it: the predict equations ")
            .math(r"\hat x^- = F\hat x")
            .text(" and ")
            .math(r"P^- = FPF^\top + Q")
            .text(", which are just the mean and covariance of a linear map plus independent noise; the identity ")
            .math(r"\mathrm{Cov}(Ax) = A\Sigma A^\top")
            .text(" and every sandwich in the filter; why the filter is finite-dimensional at all; why the extended, unscented and particle filters must exist and what each gives up; and the exact scope of the optimality theorems."))
}

fn why_the_gaussian_family_is_closed(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — why the Gaussian family is closed when almost no other is")
        .para(|p| p
            .text("A Gaussian density is not really a function the filter has to carry. It is a quadratic polynomial wearing an exponential as a coat. Write ")
            .math(r"p(x)\propto\exp(-\frac12 q(x))")
            .text(" with ")
            .math(r"q(x) = (x-\mu)^\top\Sigma^{-1}(x-\mu)")
            .text(", a polynomial of degree at most 2. Every one of the four operations acts on ")
            .math(r"q")
            .text(", not on ")
            .math(r"p")
            .text(", and each maps a degree-")
            .math(r"\le2")
            .text(" polynomial to another degree-")
            .math(r"\le2")
            .text(" polynomial."))
        .explain(r"\exp(-\frac12 q(x))", "The exponential of minus half the quadratic",
            "A Gaussian density written out as what it is: a quadratic polynomial wearing an exponential as a coat.")
        .explain(r"q(x)", "The quadratic in the exponent",
            "A polynomial of degree at most 2. Every one of the four operations acts on this rather than on the density.")
        .para(|p| p
            .text("So the real closure statement is not about bell curves at all: the set of quadratics is closed under the filter's operations, and a quadratic is pinned down by exactly two things — where its vertex sits and how it curves. Those two things are ")
            .math(r"\mu")
            .text(" and ")
            .math(r"\Sigma^{-1}")
            .text(". \"The belief never changes shape\" and \"a quadratic stays a quadratic\" are the same sentence."))
        .para(|p| p
            .text("The four operations reduce to two atomic facts about polynomials plus one integral."))
        .para(|p| p
            .text("1. Affine map. Substituting ")
            .math(r"x = Ay+b")
            .text(" into ")
            .math(r"q")
            .text(" composes a degree-2 polynomial with a degree-1 one, and degrees multiply under composition: ")
            .math(r"2\times1=2")
            .text(". Expanding ")
            .math(r"(Ay+b-\mu)^\top\Sigma^{-1}(Ay+b-\mu)")
            .text(" gives a quadratic in ")
            .math(r"y")
            .text(" whose curvature matrix is ")
            .math(r"(A\Sigma A^\top)^{-1}")
            .text(". That expansion is literally where ")
            .math(r"\hat x^- = F\hat x")
            .text(" and ")
            .math(r"\mathrm{Cov}(Ax) = A\Sigma A^\top")
            .text(" come from — the mean is the new vertex, the covariance is the new curvature, and there is no third thing to update because a quadratic has no third parameter."))
        .explain(r"Ay", "A times y",
            "The linear part of the substitution x = Ay + b.")
        .explain(r"(Ay+b-\mu)^\top", "The mapped departure from the mean, laid on its side",
            "How far the substituted variable sits from the mean, written as a row.")
        .explain(r"(Ay+b-\mu)", "The mapped departure from the mean",
            "How far the substituted variable sits from the mean.")
        .explain(r"(A\Sigma A^\top)^{-1}", "The new curvature matrix",
            "How the quadratic in y curves after the affine substitution: the inverse of the mapped covariance.")
        .para(|p| p
            .text("2. Product of densities. ")
            .math(r"e^{-q_1/2}e^{-q_2/2} = e^{-(q_1+q_2)/2}")
            .text(". Two things do the work: the exponential's functional equation, which converts multiplying functions into adding exponents; and the fact that polynomials of degree ")
            .math(r"\le2")
            .text(" form a vector space, so adding two of them gives a third. This is why precisions add — you read the new curvature straight off the sum."))
        .explain(r"e^{-q_1/2}e^{-q_2/2}", "Two Gaussian densities multiplied",
            "Two exponentiated quadratics, one carried by each belief.")
        .explain(r"e^{-(q_1+q_2)/2}", "One density with the two exponents added",
            "Multiplying the functions adds the exponents, and two quadratics add to a third quadratic.")
        .para(|p| p
            .text("3. Conditioning. Freezing some arguments is just restriction, and a quadratic with some arguments held constant is still a quadratic in the rest. It is case 1 again. Conditioning is the cheapest of the four; it needs no integral, only renormalising."))
        .para(|p| p
            .text("4. Independent sum. This is the only one that needs real work, and it is not atomic — it is derived from the other three. Writing the convolution as an integral, the integrand is quadratic in the pair by (1), the sum is quadratic by (2), and all that remains is to integrate one block out."))
        .para(|p| p
            .text("So the entire question collapses to one thing: does integrating a block of variables out of ")
            .math(r"e^{-\mathrm{quadratic}}")
            .text(" leave ")
            .math(r"e^{-\mathrm{quadratic}}")
            .text("?"))
        .explain(r"e^{-\mathrm{quadratic}}", "The exponential of minus a quadratic",
            "The shape the whole closure question is about: does integrating a block of variables out of one leave another of the same kind?")
        .para(|p| p
            .text("Yes — because of completing the square, and completing the square is not a trick. It is Gaussian elimination. Split"))
        .display(r"q(a,b) = (b-m(a))^\top\Lambda_{bb}(b-m(a)) + r(a), \qquad m(a) = \mu_b - \Lambda_{bb}^{-1}\Lambda_{ba}(a-\mu_a)")
        .explain(r"q(a,b)", "The quadratic in both blocks",
            "The exponent before the b-block has been integrated out.")
        .explain(r"(b-m(a))^\top", "The shifted b-block, laid on its side",
            "How far b sits from the shift m(a), written as a row.")
        .explain(r"\Lambda_{bb}(b-m(a))", "The pivot block applied to the shifted b-block",
            "The b-block of the precision matrix acting on b's departure from m(a).")
        .explain(r"r(a)", "The Schur complement",
            "What survives in the a-block once the off-diagonal has been killed.")
        .explain(r"\qquad m(a)", "The shift",
            "The amount b must be moved by so that all the b-dependence collects into one bracket.")
        .explain(r"\mu_b", "The mean of the b-block",
            "Where the b-block sits before the cross term shifts it.")
        .explain(r"\Lambda_{bb}^{-1}\Lambda_{ba}(a-\mu_a)", "The cross term divided by the pivot block",
            "The correction that a's departure from its own mean makes to b: the off-diagonal being eliminated.")
        .para(|p| p
            .text("In that split the first bracket is all the ")
            .math(r"b")
            .text("-dependence, and ")
            .math(r"r(a)")
            .text(" is none of it."))
        .para(|p| p
            .text("which is exactly one block row-operation killing the off-diagonal of the symmetric precision matrix ")
            .math(r"\Lambda")
            .text(". What survives in the ")
            .math(r"a")
            .text("-block is the Schur complement ")
            .math(r"r")
            .text(". Then"))
        .display(r"\int e^{-\frac12 q(a,b)}\,db = e^{-\frac12 r(a)}\int e^{-\frac12 (b-m(a))^\top\Lambda_{bb}(b-m(a))}\,db")
        .explain(r"e^{-\frac12 q(a,b)}\,db", "The joint density over a slice of b",
            "The integrand of the marginalisation: the exponentiated quadratic in both blocks, across a slice db.")
        .explain(r"e^{-\frac12 r(a)}", "The Schur complement's exponential",
            "The factor that carries all the a-dependence out of the integral.")
        .explain(r"e^{-\frac12 (b-m(a))^\top\Lambda_{bb}(b-m(a))}\,db", "The shifted b-block's density over a slice of b",
            "The surviving integral, which is a constant that does not depend on a.")
        .para(|p| p
            .text("and the surviving integral is a constant that does not depend on ")
            .math(r"a")
            .text(", because sliding the integration variable sideways by ")
            .math(r"m(a)")
            .text(" does not change the area under a curve. The only ")
            .math(r"a")
            .text("-dependence left is ")
            .math(r"e^{-r(a)/2}")
            .text(": a Gaussian."))
        .explain(r"m(a)", "The shift",
            "The amount the integration variable is slid sideways by, which does not change the area under a curve.")
        .explain(r"e^{-r(a)/2}", "The exponential of minus half the Schur complement",
            "The only a-dependence left once b has been integrated out: a Gaussian.")
        .para(|p| p
            .text("Two things are load-bearing there and both can be told to a beginner: translation invariance — moving a shape sideways does not change the area under it — and the fact that the Gaussian integral ")
            .math(r"\int e^{-x^2}dx = \surd\pi")
            .text(" is finite at all, so that constant is a number rather than ")
            .math(r"\infty")
            .text(" or ")
            .math(r"0")
            .text("."))
        .explain(r"e^{-x^2}dx", "The Gaussian bump over a slice of the line",
            "The integrand of the Gaussian integral, whose being finite is what makes that constant a number.")
        .para(|p| p
            .text("And note the payoff. The Kalman covariance update ")
            .math(r"\Sigma_{aa} - \Sigma_{ab}\Sigma_{bb}^{-1}\Sigma_{ba}")
            .text(" is a Schur complement because completing the square is Schur complementation. That is not an analogy; it is the same algebra. Learn this one act properly and you have paid in advance for six later results."))
        .explain(r"\Sigma_{aa}", "The a-block of the covariance",
            "The covariance of the a-block on its own, before the correction is taken off it.")
        .explain(r"\Sigma_{ab}\Sigma_{bb}^{-1}\Sigma_{ba}", "The correction the update subtracts",
            "The cross-covariance, divided by the b-block, times the cross-covariance back: the other half of the Schur complement.")
        .para(|p| p
            .text("Why is the cross term always eliminable? Because solving for the shift ")
            .math(r"m(a)")
            .text(" requires dividing by the pivot block ")
            .math(r"\Lambda_{bb}")
            .text(", and ")
            .math(r"\Lambda_{bb}")
            .text(" is always invertible because it is positive definite. Positive definiteness is doing double duty: it makes the elimination legal, and it makes ")
            .math(r"e^{-q/2}")
            .text(" an integrable bump rather than a trough. If ")
            .math(r"q")
            .text(" had even one negative direction, ")
            .math(r"e^{-q/2}")
            .text(" would grow without bound along it and there would be no probability distribution to integrate in the first place."))
        .explain(r"\Lambda_{bb}", "The pivot block",
            "The b-block of the precision matrix, always invertible because it is positive definite.")
        .explain(r"e^{-q/2}", "The exponentiated quadratic",
            "An integrable bump while q is positive definite; along a negative direction it would instead grow without bound.")
        .para(|p| p
            .text("And why is a covariance matrix always positive semi-definite? Not by decree, and not by convention. For any direction ")
            .math(r"u")
            .text(", ")
            .math(r"u^\top\Sigma u = \mathrm{Var}(u^\top x)")
            .text(" — the variance of the single number you get by looking at ")
            .math(r"x")
            .text(" along ")
            .math(r"u")
            .text(". And a variance is defined as the average of a squared quantity. An average of things that are never negative is never negative. \"Positive definite covariance\" is nothing more exotic than \"you cannot have a negative amount of spread\"."))
        .explain(r"u^\top\Sigma u", "The covariance seen along a direction",
            "The variance of the single number you get by looking at x along u.")
        .explain(r"\mathrm{Var}(u^\top x)", "The variance of the state seen along u",
            "An average of a squared quantity, so it can never be negative.")
        .para(|p| p
            .text("Bedrock: a definition plus an order axiom of the real numbers. Variance is defined as the expectation of a square, and ")
            .math(r"t^2\ge0")
            .text(" for every real ")
            .math(r"t")
            .text(" is an axiom of ")
            .math(r"\mathbb{R}")
            .text(" as an ordered field. There is nothing underneath \"a squared number is not negative\" left to explain."))
        .explain(r"t^2", "t squared",
            "A real number multiplied by itself, which is never negative.")
        .para(|p| p
            .text("Why exactly degree 2? It is worth knowing that this is not arbitrary. Odd degree is out immediately: if ")
            .math(r"q")
            .text(" has odd degree then ")
            .math(r"-q")
            .text(" runs to ")
            .math(r"+\infty")
            .text(" in one direction, ")
            .math(r"e^{-q/2}")
            .text(" explodes, and there is no distribution. Degrees 0 and 1 are out because constant or linear exponents are not integrable over all of ")
            .math(r"\mathbb{R}^n")
            .text(". So 2 is the smallest exponent degree that yields a genuine, non-degenerate probability distribution — the Gaussian is the cheapest bump there is."))
        .explain(r"-q", "Minus the quadratic",
            "The exponent with its sign flipped. If the quadratic had odd degree this would run to plus infinity in one direction.")
        .explain(r"+\infty", "Plus infinity",
            "Growing without bound, which is where an odd-degree exponent sends the density.")
        .para(|p| p
            .text("And degree 4, though integrable, fails at the marginalisation step for a structural reason: a quadratic form is a symmetric matrix. The correspondence ")
            .math(r"q(x)=x^\top Ax\leftrightarrow A")
            .text(" is a bijection, which hands quadratics the whole of linear algebra — elimination, inversion, and Sylvester's law of inertia, which says every real quadratic form can be brought to a plain sum of ")
            .math(r"\pm")
            .text(" squares by a change of coordinates. That normal form is what \"completing the square\" is. A cubic or quartic form is not a matrix; it is a 3- or 4-index tensor, and no comparable classification exists. In a quadratic the coupling between two blocks is bilinear, so it can be absorbed entirely by shifting one block by an amount affine in the other; in a quartic the coupling includes terms like ")
            .math(r"a^2b^2")
            .text(" and ")
            .math(r"a^3b")
            .text(", which no shift removes, so the leftover dependence is in general not the exponential of a polynomial and the family leaks."))
        .explain(r"x^\top Ax\leftrightarrow A", "A quadratic form and its matrix, in one-to-one correspondence",
            "The bijection that hands quadratics the whole of linear algebra: elimination, inversion, and Sylvester's law of inertia.")
        .explain(r"a^2b^2", "a squared times b squared",
            "One of the quartic coupling terms that no shift removes.")
        .explain(r"a^3b", "a cubed times b",
            "Another quartic coupling term that no shift removes.")
        .para(|p| p
            .text("Bedrock: a definition. \"A quadratic form is a symmetric matrix\" is not a theorem to be justified but the definition of what a quadratic form is; every closure property in this lesson is a consequence of the fact that degree 2, and only degree 2, has matrices behind it."))
}

fn what_breaks_when_linearity_or_gaussianity_fails(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — what actually breaks when linearity or Gaussianity fails")
        .para(|p| p
            .text("Once the exponent is no longer quadratic, there is no finite list of parameters left to update. Push a Gaussian through a nonlinear ")
            .math(r"f")
            .text(" and the exponent becomes ")
            .math(r"q(f^{-1}(y))")
            .text(" plus a Jacobian term — no longer degree 2 — so the belief acquires skew, then multiple modes, then features with no name."))
        .explain(r"q(f^{-1}(y))", "The quadratic composed with the inverse of the nonlinear map",
            "What the exponent becomes once a Gaussian is pushed through a nonlinear f: no longer of degree 2.")
        .para(|p| p
            .text("Exactly filtering such a system means evolving an entire density in an infinite-dimensional function space (the Kushner–Stratonovich and Zakai equations): the solution is infinite-dimensional in the general case, with the linear-Gaussian model called out as the exception where the density is Gaussian and can be characterised by its mean and covariance. Whether any exact finite-dimensional filter exists for a given nonlinear model is a classification problem in its own right — the theory of finite-dimensional estimation algebras — and the catalogue after forty years of searching is essentially the Kalman filter, the Beneš filter, and a few Daum-type generalisations."))
        .para(|p| p
            .text("This is precisely why the extended, unscented and particle filters must exist, and it names what each surrenders. The EKF keeps \"finite\" and \"exact-looking\" by linearising, so the closure now applies to a lie — hence its known inconsistency. The UKF keeps two moments but pushes sample points through the true nonlinearity, giving up the pretence that the belief really is Gaussian. The particle filter gives up the finite summary entirely and carries samples, paying memory that grows with the accuracy you want."))
        .para(|p| p
            .text("You may have exact, finite, and nonlinear — pick two."))
        .para(|p| p
            .text("Bedrock: an empirical fact, about the mathematical literature rather than about the world. That the list of models admitting an exact finite-dimensional filter is this short is something that was discovered by decades of searching, not deduced from a simpler principle. There is no theorem saying \"no others exist\" — only an enumeration that has stayed almost empty."))
}

fn is_the_gaussian_special_or_merely_convenient(b: LessonBuilder) -> LessonBuilder {
    b.heading("Going deeper — is the Gaussian special, or merely convenient?")
        .para(|p| p
            .text("Name it plainly, because the lesson's whole treatment turns on it: it is a convention — but a forced one. Two halves, and they must be kept apart."))
        .para(|p| p
            .text("The filter's reason for using Gaussians is closure, not evidence. Nobody inspected data, found bell curves, and then invented the filter. Kálmán needed a belief representable by finitely many numbers forever, and the exponentiated-quadratic family is the family that delivers it. The decisive evidence that this is a modelling choice rather than a fact about the world is that the minimum-variance derivation of the very same gain uses only linearity and second moments — no Gaussianity anywhere. Strip Gaussianity out and the algorithm does not change by a single character; only the claim weakens, from \"best of all estimators\" to \"best of all linear estimators\". A genuine empirical premise, removed, would break the machine. This one does not."))
        .para(|p| p
            .text("But it is not an arbitrary convention. Once you have decided to carry a mean and a covariance and nothing else, four independent results say the Gaussian is the only consistent completion."))
        .para(|p| p
            .text("1. Maximum entropy. Among all distributions on ")
            .math(r"\mathbb{R}^n")
            .text(" with a given mean and covariance, the Gaussian has the largest differential entropy, ")
            .math(r"h = \frac12\log[(2\pi e)^n\det\Sigma]")
            .text(". In words a beginner can hold: of all the beliefs consistent with the two numbers you claim to know, the Gaussian is the vaguest. So \"assume Gaussian\" is not an extra assumption bolted onto \"I know a mean and a covariance\". It is the absence of any further assumption."))
        .explain(r"\frac12\log[(2\pi e)^n\det\Sigma]", "The differential entropy of a Gaussian",
            "The largest differential entropy any distribution with that mean and covariance can have.")
        .para(|p| p
            .text("2. Stability — the sharpest one for this filter. A location-scale family closed under both linear maps and independent sums is, by definition, a stable family. Stable laws are classified by an index ")
            .math(r"\alpha\in(0,2]")
            .text(", and only ")
            .math(r"\alpha=2")
            .text(" — the Gaussian — has finite variance. Every other stable law has infinite variance, so it has no covariance matrix at all and the filter would have nothing to propagate. Note what this identifies as the discriminating property: the multivariate Student-")
            .math(r"t")
            .text(" is closed under affine maps, marginalisation and conditioning — three of the four — but is not closed under independent sums. It is the convolution closure, the ")
            .math(r"+Q")
            .text(" in ")
            .math(r"P^- = FPF^\top+Q")
            .text(", that singles out the Gaussian. The predict step, not the update step."))
        .explain(r"(0,2]", "From zero up to and including two",
            "The range of the index that classifies the stable laws.")
        .explain(r"+Q", "Plus Q",
            "The independent process noise added at the predict step. It is this convolution closure that singles out the Gaussian.")
        .para(|p| p
            .text("3. Cramér's decomposition theorem (Cramér 1936, anticipated by Lévy): if a normal random variable is written as a sum of two independent pieces, both pieces must themselves be normal. So Gaussianity cannot be manufactured out of non-Gaussian parts. With (2), this upgrades the closure from available to exclusive."))
        .para(|p| p
            .text("4. Herschel–Maxwell — the one to actually show a beginner, because it is pure symmetry. It characterises the normal distribution as the distribution of the components of a spherically symmetric random vector, provided those components are independent. If your uncertainty looks the same from every direction and its coordinates are independent, it is Gaussian and nothing else. This is Maxwell's 1860 argument for molecular velocities, and it is the reason behind the most-leaned-on fact in this lesson: for jointly Gaussian variables, uncorrelated implies independent."))
        .para(|p| p
            .text("What makes maximum entropy a principled reason rather than a nicety? Because entropy is a count, not a mood. Entropy is the log of the number of ways a large sample could be arranged and still display the summary statistics you specified. The maximum-entropy distribution is the one realised by overwhelmingly the most arrangements consistent with your constraints; picking anything narrower asserts that the world landed in a vastly smaller set of arrangements for a reason you have not stated and cannot point to. Concretely: you measured a centre and a spread. If you now choose a bimodal belief, or a skewed one, you have claimed to know where the second bump is, or which side is fatter — and you did not measure that. The Gaussian is the shape that refuses to claim it."))
        .para(|p| p
            .text("This also makes the filter's uncertainty legible as information: ")
            .math(r"\frac12\log\det(2\pi eP)")
            .text(" is ")
            .math(r"P")
            .text(" measured in bits, so shrinking ")
            .math(r"P")
            .text(" in the update step is literally the acquisition of information, and ")
            .math(r"Q")
            .text(" in the predict step is literally its loss."))
        .explain(r"\frac12\log\det(2\pi eP)", "The filter's uncertainty measured in bits",
            "P read as information, so shrinking it in the update step is literally the acquisition of information.")
        .para(|p| p
            .text("Why not just say the central limit theorem hands us Gaussians? Because for a finance lesson the CLT is the weakest of the four arguments, and the one the reader's own data will falsify."))
        .para(|p| p
            .text("First, it is circular here. The CLT converges to the Gaussian precisely because the Gaussian is the fixed point of the operation the CLT performs. Add two independent copies and rescale to restore the variance: for independent ")
            .math(r"X_1,X_2\sim\mathcal{N}(0,\sigma^2)")
            .text(", closure properties (1) and (2) give ")
            .math(r"\frac{X_1+X_2}{\surd 2}\sim\mathcal{N}(0,\sigma^2)")
            .text(" — identically the same distribution, not approximately. The ")
            .math(r"\surd 2")
            .text(" is forced, not chosen: independent variances add, so the sum has variance ")
            .math(r"2\sigma^2")
            .text(", and dividing by ")
            .math(r"\surd 2")
            .text(" divides the variance back by 2. The CLT is the further statement that this fixed point attracts. So \"the CLT gives us Gaussians\" and \"Gaussians are closed under sums\" are one fact read in two directions; the CLT cannot be offered as independent support for the closure that generates it."))
        .explain(r"X_1", "The first copy",
            "One of the two independent normal variables being added.")
        .explain(r"X_2", "The second copy",
            "The other of the two independent normal variables being added.")
        .explain(r"\mathcal{N}(0,\sigma^2)", "The normal distribution with mean zero and variance sigma squared",
            "The distribution both copies are drawn from — and, after rescaling, the distribution their sum has again.")
        .explain(r"\frac{X_1+X_2}{\surd 2}", "The sum of two copies, rescaled",
            "Two independent copies added and divided by the square root of two, which restores the variance.")
        .explain(r"2\sigma^2", "Twice the variance",
            "The variance of the sum, because independent variances add.")
        .para(|p| p
            .text("Second, it empirically fails where this lesson is aimed. The CLT needs many independent effects with finite variance. Financial returns violate both: volatility clusters, so the effects are dependent; and tails are fat enough that the finite-variance premise is marginal at best. A lesson that justifies Gaussianity to a finance student via the CLT is teaching something the student's own return series will contradict within a week."))
        .para(|p| p
            .text("The verdict to teach: a modelling convention, adopted because it is the family that closes, and defensible because among all beliefs carrying that mean and covariance it is the least committal one. Not a law of nature about data."))
        .para(|p| p
            .text("Bedrock: a convention chosen for consistency. The Gaussian's role is a choice — the unique choice that keeps the belief finite, and the unique choice that adds nothing to the two moments you already committed to. A choice made for a stated reason is not a fact with a further \"why\" behind it. Asking \"but why is the world Gaussian?\" is asking the wrong question: the world usually is not, and the filter never needed it to be."))
}

fn idea_three_at_work(b: LessonBuilder) -> LessonBuilder {
    b.heading("Idea 3 at work — a puck on the ice, and a filter that never gets fatter")
        .para(|p| p
            .text("In the physical world. Picture uncertainty as a shape on the ice. A puck slides on a frictionless rink. You do not know exactly where it is, or exactly how fast, so instead of drawing a dot you draw a fuzzy blob — the region the puck is probably in. A Gaussian's contour is an ellipse, so the blob is an ellipse."))
        .para(|p| p
            .text("Now let a second pass. Constant-velocity motion is the linear map"))
        .para(|p| p
            .text("One convention before that map, and it holds for every matrix in this lesson: a matrix is set on a single line with a semicolon between its rows, so ")
            .math(r"(\,a\ \ b\,;\ \ c\ \ d\,)")
            .text(" is the two-by-two array whose top row is ")
            .math(r"a\ \ b")
            .text(" and whose bottom row is ")
            .math(r"c\ \ d")
            .text("."))
        .explain(r"(\,a\ \ b\,;\ \ c\ \ d\,)", "A two-by-two matrix, set on one line",
            "The row form used for every matrix in this lesson: the semicolon separates one row from the next.")
        .explain(r"a\ \ b", "The top row",
            "The first row of the matrix.")
        .explain(r"c\ \ d", "The bottom row",
            "The second row of the matrix.")
        .display(r"\binom{p}{v}_{t+1} = (\,1\ \ \Delta t\,;\ \ 0\ \ 1\,)\binom{p}{v}_{t}")
        .explain(r"\binom{p}{v}_{t+1}", "Position and velocity, one step later",
            "The two states stacked as a column, at time t plus one.")
        .explain(r"(\,1\ \ \Delta t\,;\ \ 0\ \ 1\,)\binom{p}{v}_{t}", "The constant-velocity map applied to the state",
            "New position = old position + velocity times elapsed time; velocity unchanged.")
        .para(|p| p
            .text("which reads: new position = old position + velocity ")
            .math(r"\times")
            .text(" elapsed time; velocity unchanged. Geometrically that is a shear — grab the top of the blob and slide it sideways. A sheared ellipse is still an ellipse. It can stretch, tilt, flatten; it cannot bend into a banana or split in two. That is the closure property, seen with your eyes. Bending is precisely what a nonlinear map does — which is why the EKF has to exist."))
        .para(|p| p
            .text("Now a question. The puck's position is known to ")
            .math(r"\pm1")
            .text(" m and its velocity to ")
            .math(r"\pm1")
            .text(" m/s, and the two errors are unrelated — a round blob. One second later, how wide is the position uncertainty?"))
        .para(|p| p
            .text("You know a vehicle's position to ")
            .math(r"\pm1")
            .text(" m and its velocity to ")
            .math(r"\pm1")
            .text(" m/s, and the two errors are uncorrelated. You look away for exactly one second, during which the position becomes ")
            .math(r"p + v\,\Delta t")
            .text(" with ")
            .math(r"\Delta t = 1")
            .text(". How well do you know the position now?"))
        .explain(r"v\,\Delta t", "Velocity times elapsed time",
            "How far the vehicle travels in the second you look away.")
        .explain(r"\Delta t", "The elapsed time",
            "How long the step lasts — here exactly one second.")
        .rule()
        .note("Commit to a figure before you scroll: say out loud, in metres, how well you know the position after that second.")
        .para(|p| p
            .math(r"\pm1.41")
            .text(" m."))
        .explain(r"\pm1.41", "Give or take one metre forty-one",
            "How well the position is known one second later.")
        .para(|p| p
            .text("The tempting wrong answer is ")
            .math(r"\pm2")
            .text(" m — one metre of position error plus one metre of velocity error over one second, added. Why it feels right: the errors genuinely do combine, and every unit in sight is a metre, so adding them is the only visible arithmetic. The missing intuition: the map is ")
            .math(r"p^- = A x")
            .text(" with ")
            .math(r"A = (1\ \ \Delta t) = (1\ \ 1)")
            .text(", and covariance transforms as ")
            .math(r"\mathrm{Cov}(Ax) = A\Sigma A^\top")
            .text(" — quadratically in ")
            .math(r"A")
            .text(", never linearly. With ")
            .math(r"\Sigma = \mathrm{diag}(1,1)")
            .text(" that gives ")
            .math(r"1 + 1 = 2")
            .text(", so the standard deviation is ")
            .math(r"\surd 2 = 1.41")
            .text(" m. Standard deviations do not add; variances do. Adding the ")
            .math(r"\pm")
            .text("s would only be right if the two errors were perfectly correlated — that is, if the velocity error were a known multiple of the position error, which is exactly what \"uncorrelated\" denies."))
        .explain(r"\pm2", "Give or take two",
            "The tempting wrong answer: the two one-metre errors simply added.")
        .explain(r"p^-", "The predicted position",
            "Where the position has got to after the step, before any measurement.")
        .explain(r"A x", "The map applied to the state",
            "The row A times the state, which is the predicted position.")
        .explain(r"(1\ \ \Delta t)", "The one-by-two map",
            "The row that turns position and velocity into the position one step later.")
        .explain(r"(1\ \ 1)", "The same row with one second elapsed",
            "What that map comes to when the elapsed time is 1.")
        .explain(r"\mathrm{diag}(1,1)", "A diagonal covariance of ones",
            "Position known to one metre and velocity to one metre per second, uncorrelated: a round blob.")
        .explain(r"1.41", "One point four one",
            "The square root of two: the standard deviation of the position after the step.")
        .para(|p| p
            .text("Something else appeared that was not there before: the full result is ")
            .math(r"A\Sigma A^\top = (\,2\ \ 1\,;\ \ 1\ \ 1\,)")
            .text(", so position and velocity are now correlated at ")
            .math(r"1/\surd 2 \approx 0.71")
            .text(" — a vehicle that turns out to be fast is also one that has travelled further. The round blob has tilted, and that tilt is what Idea 6 later spends itself on. The same mistake with a different costume: a four-day forecast band on a random walk is twice the one-day band, not four times, and the square-root-of-time rule every risk desk uses is nothing but this fact applied ")
            .math(r"n")
            .text(" times."))
        .explain(r"(\,2\ \ 1\,;\ \ 1\ \ 1\,)", "The covariance after the step",
            "Position variance 2, velocity variance 1, and off the diagonal a covariance of 1 that was not there before.")
        .explain(r"1/\surd 2", "One over the square root of two",
            "The correlation between position and velocity that appeared out of nothing.")
        .explain(r"0.71", "About seventy-one hundredths",
            "What one over the square root of two comes to.")
        .para(|p| p
            .text("The tempting answer is ")
            .math(r"\pm2")
            .text(" m: one metre of doubt you already had, plus one metre the velocity doubt buys you. But variances add, not standard deviations. ")
            .math(r"1+1 = 2")
            .text(", so the answer is ")
            .math(r"\surd 2\approx1.41")
            .text(" m."))
        .para(|p| p
            .text("And something appeared that was not there before: position and velocity are now correlated, at ")
            .math(r"1/\surd 2\approx0.71")
            .text(", because a puck that turns out to be fast is also a puck that has travelled further. The blob has tilted. Add a gust of wind — an independent Gaussian, the ")
            .math(r"+Q")
            .text(" term — and the ellipse simply fattens. Ellipse plus ellipse, still an ellipse."))
        .figure(Figure::new(ILL_2_SVG, "The closure property, drawn. Starting from P0 = diag(1, 1) — position known to ±1 m, velocity to ±1 m/s, uncorrelated — one second of constant-velocity motion applies the shear F = (1 1; 0 1), giving P^- = F P0 F^T = (2 1; 1 1). Since P0 = I, the new 1σ contour is exactly the unit circle's image under F: its semi-axes are √((3 ± √5)/2) = 1.618 and 0.618, tilted 31.7°. Position doubt grows to √2 = 1.41 m, not 2 m, because variances add; velocity doubt is untouched at 1.00; and the tilt that appeared from nothing is the new correlation 1/√2 = 0.71 — a puck that turns out to be fast is also a puck that has travelled further.")
            .print_svg(ILL_2_SVG_PRINT)
            .width_percent(80))
        .para(|p| p
            .text("In finance. Closure is what lets a filter live on a live tick stream forever. The prediction step is"))
        .display(r"\hat x_t^- = F\hat x_{t-1}, \qquad P_t^- = FP_{t-1}F^\top + Q")
        .explain(r"\hat x_t^-", "The prior estimate at time t",
            "The best guess carried in before this tick's measurement arrives.")
        .explain(r"F\hat x_{t-1}", "Last tick's estimate pushed forward",
            "How the hidden quantities would evolve if nobody touched them.")
        .explain(r"\qquad P_t^-", "The prior covariance at time t",
            "How unsure you are before this tick's measurement arrives.")
        .explain(r"FP_{t-1}F^\top", "Last tick's covariance pushed through the map",
            "The sandwich that carries the old uncertainty forward through F.")
        .para(|p| p
            .text("Reading it: ")
            .math(r"\hat x")
            .text(" is your current best guess of the hidden quantities — say a drifting intercept and a drifting slope; ")
            .math(r"F")
            .text(" says how they would evolve if nobody touched them; ")
            .math(r"P")
            .text(" is how unsure you are, with variances down the diagonal and, off it, how the errors move together; and ")
            .math(r"Q")
            .text(" is how much fresh drift each tick injects."))
        .para(|p| p
            .text("Nothing on the right-hand side refers to history. Only to last tick's two objects. So the filter stores ")
            .math(r"n")
            .text(" numbers for the mean and ")
            .math(r"n(n+1)/2")
            .text(" for the covariance: for ")
            .math(r"n=2")
            .text(" states that is exactly 5 numbers, 40 bytes in double precision."))
        .explain(r"n(n+1)/2", "The number of distinct covariance entries",
            "How many numbers a covariance over n states needs.")
        .para(|p| p
            .text("Toy question: run that two-state filter from the open to the close on a stream of 10 ticks per second — ")
            .math(r"10\times6.5\times3600 = 234{,}000")
            .text(" updates. How much more memory does it need at the close than it needed at the open?"))
        .explain(r"6.5", "Six and a half",
            "The hours in a trading session.")
        .explain(r"3600", "Three thousand six hundred",
            "The seconds in an hour.")
        .explain(r"234", "Two hundred and thirty-four thousand",
            "The updates the filter performs between the open and the close.")
        .explain(r"000", "Two hundred and thirty-four thousand",
            "The updates the filter performs between the open and the close.")
        .para(|p| p
            .text("The tempting answer is \"a little more each tick — it has seen more data\". The correct answer is none. The same 5 numbers, the same 40 bytes, the same handful of multiplications per tick, on tick 234,000 as on tick 1. A rolling 250-day regression on the same two coefficients must physically hold its window — 500 numbers, 4,000 bytes — and disturb all of it every time the window rolls. The filter's window is infinite and costs nothing."))
        .para(|p| p
            .text("And here is the fun of it. Two systems open at 9:30 on the same two-state model. One is a Kalman filter on a ten-tick-per-second feed; by the close it has processed 234,000 updates. The other is a rolling 250-day regression on the same two coefficients. Guess which one needs more memory at 4pm. The filter holds 5 numbers — 40 bytes, fewer characters than this sentence — on tick 234,000 exactly as on tick 1. The rolling window holds 500 numbers, 4,000 bytes, and has to shove one observation out of the door every time a new one arrives. The one with the infinite window is the cheap one. Not because it is clever: because the belief's shape never changes, so there is never a third thing to store."))
        .para(|p| p
            .text("The honest counterpart, and you should hear it now rather than later, because this is where returns fight back. Under a Gaussian, ")
            .math(r"\Pr(|z|>4) = 6.334\times10^{-5}")
            .text(" — one day in 15,787, about one day in 62.6 trading years at 252 days a year. Anyone who has watched a market has seen more than one."))
        .explain(r"\Pr(|z|>4)", "The chance of a four-sigma move",
            "How often a Gaussian says a move that far out happens: one day in 15,787.")
        .explain(r"6.334", "Six point three three four",
            "The leading figure of that probability.")
        .explain(r"10^{-5}", "Ten to the minus five",
            "The scale of that probability: one part in a hundred thousand.")
        .para(|p| p
            .text("What that costs, and what it does not: the recursion still propagates the mean and covariance correctly and remains the best linear estimator, so the point estimate is not poisoned. What breaks is the reading of the bands. A gate set at ")
            .math(r"4\surd S_t")
            .text(" will fire far more often than once a lifetime. Size positions off experience, not off ")
            .math(r"P")
            .text("."))
        .explain(r"4\surd S_t", "A four-sigma gate",
            "A threshold set at four times the innovation's own standard deviation, which will fire far more often than once a lifetime.")
}

/// Figure 1: two beliefs multiply into a third that is narrower than both.
const ILL_1_SVG: &str = r##"<svg viewBox="0 0 440 250" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Three bell curves: a wide prior, a sharp measurement, and a posterior narrower than both">
  <!-- prior N(3, 1.7^2); measurement N(4.4, 0.6^2)  [the pair verified in core idea 2, Branch B]
       precisions: 1/2.89 = 0.3460208 ; 1/0.36 = 2.7777778 ; sum = 3.1237986
       posterior var = 1/3.1237986 = 0.3201231 -> sd = 0.5657942
       posterior mean = 0.3201231*(3/2.89 + 4.4/0.36) = 4.2449231
       peak height of N(m,s) is 1/(s*sqrt(2*pi)):
         prior 0.2346719 ; measurement 0.6649038 ; posterior 0.7051014
       px mapping: x -> 40 + 54*x   (x = 0 at px 40, x = 7 at px 418)
                   density -> 200 - 240*d   (baseline y = 200)
       so peaks land at y = 200-240*0.2346719 = 143.7 ; = 40.4 ; = 30.8 -->
  <line x1="40" y1="200" x2="424" y2="200" stroke="#E8E6E3" stroke-width="2"/>
  <line x1="40" y1="26" x2="40" y2="200" stroke="#E8E6E3" stroke-width="2"/>

  <polyline fill="none" stroke="#9A9793" stroke-width="2" points="40.0,188.1 67.0,180.9 94.0,171.8 121.0,161.8 148.0,152.6 175.0,146.1 202.0,143.7 229.0,146.1 256.0,152.6 283.0,161.8 310.0,171.8 337.0,180.9 364.0,188.1 391.0,193.2 418.0,196.5"/>
  <polyline fill="none" stroke="#60A5FA" stroke-width="2" points="169.6,199.4 183.1,197.7 196.6,193.0 210.1,181.8 223.6,160.2 237.1,126.9 250.6,87.2 264.1,53.7 277.6,40.4 291.1,53.7 304.6,87.2 318.1,126.9 331.6,160.2 345.1,181.8 358.6,193.0 372.1,197.7 385.6,199.4"/>
  <polyline fill="none" stroke="#FBBF24" stroke-width="2.5" points="169.6,199.2 183.1,196.8 196.6,190.0 210.1,174.0 223.6,144.5 237.1,102.6 250.6,59.5 264.1,33.1 269.2,30.8 277.6,37.0 291.1,69.0 304.6,113.4 318.1,152.9 331.6,178.9 345.1,192.3 358.6,197.7"/>

  <line x1="269.2" y1="30.8" x2="269.2" y2="200" stroke="#FBBF24" stroke-width="1" stroke-dasharray="4 4"/>
  <line x1="202" y1="143.7" x2="202" y2="200" stroke="#9A9793" stroke-width="1" stroke-dasharray="4 4"/>
  <line x1="277.6" y1="40.4" x2="277.6" y2="200" stroke="#60A5FA" stroke-width="1" stroke-dasharray="4 4"/>

  <text x="86" y="139" font-size="13" fill="#9A9793">what I believed</text>
  <text x="86" y="155" font-size="12" fill="#9A9793">σ = 1.70</text>
  <text x="300" y="62" font-size="13" fill="#60A5FA">what I measured</text>
  <text x="300" y="78" font-size="12" fill="#60A5FA">σ = 0.60</text>
  <text x="150" y="46" font-size="13" fill="#FBBF24">both together</text>
  <text x="150" y="62" font-size="12" fill="#FBBF24">σ = 0.566 — narrower</text>
  <text x="150" y="78" font-size="12" fill="#FBBF24">than either one</text>

  <g font-size="12" fill="#E8E6E3" text-anchor="middle">
    <text x="202" y="217">3.00</text>
    <text x="269.2" y="233">4.245</text>
    <text x="277.6" y="217">4.40</text>
    <text x="232" y="246">value of the thing being estimated</text>
  </g>
</svg>"##;

/// [`ILL_1_SVG`] as it was drawn, for white paper.
const ILL_1_SVG_PRINT: &str = r##"<svg viewBox="0 0 440 250" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Three bell curves: a wide prior, a sharp measurement, and a posterior narrower than both">
  <!-- prior N(3, 1.7^2); measurement N(4.4, 0.6^2)  [the pair verified in core idea 2, Branch B]
       precisions: 1/2.89 = 0.3460208 ; 1/0.36 = 2.7777778 ; sum = 3.1237986
       posterior var = 1/3.1237986 = 0.3201231 -> sd = 0.5657942
       posterior mean = 0.3201231*(3/2.89 + 4.4/0.36) = 4.2449231
       peak height of N(m,s) is 1/(s*sqrt(2*pi)):
         prior 0.2346719 ; measurement 0.6649038 ; posterior 0.7051014
       px mapping: x -> 40 + 54*x   (x = 0 at px 40, x = 7 at px 418)
                   density -> 200 - 240*d   (baseline y = 200)
       so peaks land at y = 200-240*0.2346719 = 143.7 ; = 40.4 ; = 30.8 -->
  <line x1="40" y1="200" x2="424" y2="200" stroke="#334155" stroke-width="2"/>
  <line x1="40" y1="26" x2="40" y2="200" stroke="#334155" stroke-width="2"/>

  <polyline fill="none" stroke="#64748b" stroke-width="2" points="40.0,188.1 67.0,180.9 94.0,171.8 121.0,161.8 148.0,152.6 175.0,146.1 202.0,143.7 229.0,146.1 256.0,152.6 283.0,161.8 310.0,171.8 337.0,180.9 364.0,188.1 391.0,193.2 418.0,196.5"/>
  <polyline fill="none" stroke="#1d4ed8" stroke-width="2" points="169.6,199.4 183.1,197.7 196.6,193.0 210.1,181.8 223.6,160.2 237.1,126.9 250.6,87.2 264.1,53.7 277.6,40.4 291.1,53.7 304.6,87.2 318.1,126.9 331.6,160.2 345.1,181.8 358.6,193.0 372.1,197.7 385.6,199.4"/>
  <polyline fill="none" stroke="#b45309" stroke-width="2.5" points="169.6,199.2 183.1,196.8 196.6,190.0 210.1,174.0 223.6,144.5 237.1,102.6 250.6,59.5 264.1,33.1 269.2,30.8 277.6,37.0 291.1,69.0 304.6,113.4 318.1,152.9 331.6,178.9 345.1,192.3 358.6,197.7"/>

  <line x1="269.2" y1="30.8" x2="269.2" y2="200" stroke="#b45309" stroke-width="1" stroke-dasharray="4 4"/>
  <line x1="202" y1="143.7" x2="202" y2="200" stroke="#64748b" stroke-width="1" stroke-dasharray="4 4"/>
  <line x1="277.6" y1="40.4" x2="277.6" y2="200" stroke="#1d4ed8" stroke-width="1" stroke-dasharray="4 4"/>

  <text x="86" y="139" font-size="13" fill="#64748b">what I believed</text>
  <text x="86" y="155" font-size="12" fill="#64748b">σ = 1.70</text>
  <text x="300" y="62" font-size="13" fill="#1d4ed8">what I measured</text>
  <text x="300" y="78" font-size="12" fill="#1d4ed8">σ = 0.60</text>
  <text x="150" y="46" font-size="13" fill="#b45309">both together</text>
  <text x="150" y="62" font-size="12" fill="#b45309">σ = 0.566 — narrower</text>
  <text x="150" y="78" font-size="12" fill="#b45309">than either one</text>

  <g font-size="12" fill="#334155" text-anchor="middle">
    <text x="202" y="217">3.00</text>
    <text x="269.2" y="233">4.245</text>
    <text x="277.6" y="217">4.40</text>
    <text x="232" y="246">value of the thing being estimated</text>
  </g>
</svg>"##;

/// Figure 2: a round uncertainty blob sheared into a tilted ellipse.
const ILL_2_SVG: &str = r##"<svg viewBox="0 0 440 285" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="A circular uncertainty blob sheared into a tilted ellipse: wider, same height">
  <!-- P0 = diag(1,1); F = [[1,1],[0,1]]; P- = F P0 F' = [[2,1],[1,1]]   (core idea 3, applied block)
       Because P0 = I, the 1-sigma contour of P- IS the image of the unit circle under F,
       so every point below is a mapped point, not a fitted curve:
         F(0,1)  = (1,1)   top of circle slides one unit right
         F(0,-1) = (-1,-1) bottom slides one unit left
         F(1,0)  = (1,0)   the horizontal diameter is fixed by the shear
       eigenvalues of [[2,1],[1,1]] = (3 +/- sqrt5)/2 = 2.618034, 0.381966
         semi-axes = sqrt of those = 1.618034, 0.618034 ; major axis at atan(0.618034) = 31.7175 deg
       horizontal half-extent = sqrt(P11) = sqrt2 = 1.41421   (not 1 + 1 = 2)
       vertical   half-extent = sqrt(P22) = sqrt1 = 1.00000   (unchanged)
       correlation = 1/sqrt(P11*P22) = 1/sqrt2 = 0.70711
       px: 60 px per unit, centre (200,120); SVG y points down, so the tilt is rotate(-31.7175)
         rx = 60*1.618034 = 97.08 ; ry = 60*0.618034 = 37.08
         vertical tangents at 200 +/- 60*1.41421 = 115.15 and 284.85
         horizontal tangents at 120 -/+ 60*1.0 = 60 and 180 (shared with the circle) -->
  <text x="46" y="26" font-size="13" fill="#E8E6E3">Stretch, tilt, flatten — never bend, never split.</text>

  <line x1="75" y1="120" x2="358" y2="120" stroke="#E8E6E3" stroke-width="1.5"/>
  <line x1="200" y1="36" x2="200" y2="208" stroke="#E8E6E3" stroke-width="1.5"/>

  <line x1="105" y1="60" x2="300" y2="60" stroke="#7F7C78" stroke-width="1" stroke-dasharray="5 4"/>
  <line x1="105" y1="180" x2="300" y2="180" stroke="#7F7C78" stroke-width="1" stroke-dasharray="5 4"/>
  <line x1="115.15" y1="46" x2="115.15" y2="212" stroke="#60A5FA" stroke-width="1" stroke-dasharray="5 4"/>
  <line x1="284.85" y1="46" x2="284.85" y2="212" stroke="#60A5FA" stroke-width="1" stroke-dasharray="5 4"/>

  <circle cx="200" cy="120" r="60" fill="none" stroke="#7F7C78" stroke-width="2" stroke-dasharray="6 4"/>
  <ellipse cx="200" cy="120" rx="97.08" ry="37.08" transform="rotate(-31.7175 200 120)" fill="#60A5FA" fill-opacity="0.10" stroke="#60A5FA" stroke-width="2.5"/>

  <line x1="200" y1="60" x2="252" y2="60" stroke="#FBBF24" stroke-width="2"/>
  <polygon points="252,55 262,60 252,65" fill="#FBBF24"/>
  <line x1="200" y1="180" x2="148" y2="180" stroke="#FBBF24" stroke-width="2"/>
  <polygon points="148,175 138,180 148,185" fill="#FBBF24"/>
  <text x="264" y="52" font-size="12" fill="#FBBF24">the shear</text>

  <line x1="140" y1="228" x2="260" y2="228" stroke="#7F7C78" stroke-width="2"/>
  <line x1="140" y1="223" x2="140" y2="233" stroke="#7F7C78" stroke-width="2"/>
  <line x1="260" y1="223" x2="260" y2="233" stroke="#7F7C78" stroke-width="2"/>
  <text x="270" y="232" font-size="12" fill="#9A9793">before: ±1.00 m</text>

  <line x1="115.15" y1="252" x2="284.85" y2="252" stroke="#60A5FA" stroke-width="2"/>
  <line x1="115.15" y1="247" x2="115.15" y2="257" stroke="#60A5FA" stroke-width="2"/>
  <line x1="284.85" y1="247" x2="284.85" y2="257" stroke="#60A5FA" stroke-width="2"/>
  <text x="294" y="256" font-size="12" fill="#60A5FA">after: ±1.41 m</text>

  <text x="60" y="277" font-size="12" fill="#FBBF24">Not ±2.00 m — variances add, spreads do not.</text>

  <line x1="318" y1="60" x2="318" y2="180" stroke="#9A9793" stroke-width="1.5"/>
  <line x1="313" y1="60" x2="323" y2="60" stroke="#9A9793" stroke-width="1.5"/>
  <line x1="313" y1="180" x2="323" y2="180" stroke="#9A9793" stroke-width="1.5"/>
  <text x="328" y="112" font-size="12" fill="#9A9793">velocity</text>
  <text x="328" y="128" font-size="12" fill="#9A9793">±1.00 m/s,</text>
  <text x="328" y="144" font-size="12" fill="#9A9793">unchanged</text>

  <text x="204" y="46" font-size="12" fill="#E8E6E3">velocity error (m/s)</text>
  <text x="300" y="137" font-size="12" fill="#E8E6E3">position error (m)</text>
  <text x="46" y="200" font-size="12" fill="#60A5FA">tilt = correlation 0.71</text>
</svg>"##;

/// [`ILL_2_SVG`] as it was drawn, for white paper.
const ILL_2_SVG_PRINT: &str = r##"<svg viewBox="0 0 440 285" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="A circular uncertainty blob sheared into a tilted ellipse: wider, same height">
  <!-- P0 = diag(1,1); F = [[1,1],[0,1]]; P- = F P0 F' = [[2,1],[1,1]]   (core idea 3, applied block)
       Because P0 = I, the 1-sigma contour of P- IS the image of the unit circle under F,
       so every point below is a mapped point, not a fitted curve:
         F(0,1)  = (1,1)   top of circle slides one unit right
         F(0,-1) = (-1,-1) bottom slides one unit left
         F(1,0)  = (1,0)   the horizontal diameter is fixed by the shear
       eigenvalues of [[2,1],[1,1]] = (3 +/- sqrt5)/2 = 2.618034, 0.381966
         semi-axes = sqrt of those = 1.618034, 0.618034 ; major axis at atan(0.618034) = 31.7175 deg
       horizontal half-extent = sqrt(P11) = sqrt2 = 1.41421   (not 1 + 1 = 2)
       vertical   half-extent = sqrt(P22) = sqrt1 = 1.00000   (unchanged)
       correlation = 1/sqrt(P11*P22) = 1/sqrt2 = 0.70711
       px: 60 px per unit, centre (200,120); SVG y points down, so the tilt is rotate(-31.7175)
         rx = 60*1.618034 = 97.08 ; ry = 60*0.618034 = 37.08
         vertical tangents at 200 +/- 60*1.41421 = 115.15 and 284.85
         horizontal tangents at 120 -/+ 60*1.0 = 60 and 180 (shared with the circle) -->
  <text x="46" y="26" font-size="13" fill="#334155">Stretch, tilt, flatten — never bend, never split.</text>

  <line x1="75" y1="120" x2="358" y2="120" stroke="#334155" stroke-width="1.5"/>
  <line x1="200" y1="36" x2="200" y2="208" stroke="#334155" stroke-width="1.5"/>

  <line x1="105" y1="60" x2="300" y2="60" stroke="#94a3b8" stroke-width="1" stroke-dasharray="5 4"/>
  <line x1="105" y1="180" x2="300" y2="180" stroke="#94a3b8" stroke-width="1" stroke-dasharray="5 4"/>
  <line x1="115.15" y1="46" x2="115.15" y2="212" stroke="#1d4ed8" stroke-width="1" stroke-dasharray="5 4"/>
  <line x1="284.85" y1="46" x2="284.85" y2="212" stroke="#1d4ed8" stroke-width="1" stroke-dasharray="5 4"/>

  <circle cx="200" cy="120" r="60" fill="none" stroke="#94a3b8" stroke-width="2" stroke-dasharray="6 4"/>
  <ellipse cx="200" cy="120" rx="97.08" ry="37.08" transform="rotate(-31.7175 200 120)" fill="#1d4ed8" fill-opacity="0.10" stroke="#1d4ed8" stroke-width="2.5"/>

  <line x1="200" y1="60" x2="252" y2="60" stroke="#b45309" stroke-width="2"/>
  <polygon points="252,55 262,60 252,65" fill="#b45309"/>
  <line x1="200" y1="180" x2="148" y2="180" stroke="#b45309" stroke-width="2"/>
  <polygon points="148,175 138,180 148,185" fill="#b45309"/>
  <text x="264" y="52" font-size="12" fill="#b45309">the shear</text>

  <line x1="140" y1="228" x2="260" y2="228" stroke="#94a3b8" stroke-width="2"/>
  <line x1="140" y1="223" x2="140" y2="233" stroke="#94a3b8" stroke-width="2"/>
  <line x1="260" y1="223" x2="260" y2="233" stroke="#94a3b8" stroke-width="2"/>
  <text x="270" y="232" font-size="12" fill="#64748b">before: ±1.00 m</text>

  <line x1="115.15" y1="252" x2="284.85" y2="252" stroke="#1d4ed8" stroke-width="2"/>
  <line x1="115.15" y1="247" x2="115.15" y2="257" stroke="#1d4ed8" stroke-width="2"/>
  <line x1="284.85" y1="247" x2="284.85" y2="257" stroke="#1d4ed8" stroke-width="2"/>
  <text x="294" y="256" font-size="12" fill="#1d4ed8">after: ±1.41 m</text>

  <text x="60" y="277" font-size="12" fill="#b45309">Not ±2.00 m — variances add, spreads do not.</text>

  <line x1="318" y1="60" x2="318" y2="180" stroke="#64748b" stroke-width="1.5"/>
  <line x1="313" y1="60" x2="323" y2="60" stroke="#64748b" stroke-width="1.5"/>
  <line x1="313" y1="180" x2="323" y2="180" stroke="#64748b" stroke-width="1.5"/>
  <text x="328" y="112" font-size="12" fill="#64748b">velocity</text>
  <text x="328" y="128" font-size="12" fill="#64748b">±1.00 m/s,</text>
  <text x="328" y="144" font-size="12" fill="#64748b">unchanged</text>

  <text x="204" y="46" font-size="12" fill="#334155">velocity error (m/s)</text>
  <text x="300" y="137" font-size="12" fill="#334155">position error (m)</text>
  <text x="46" y="200" font-size="12" fill="#1d4ed8">tilt = correlation 0.71</text>
</svg>"##;
