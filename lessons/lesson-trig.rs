//! Prerequisites:
//! - School algebra: reading a formula, substituting a number, rearranging.
//! - Coordinates: reading a point as an across-value and an up-value.
//! - Averages and standard deviation, at the level of "what a spreadsheet does".
//! - No trigonometry assumed. Sine, cosine, tangent and pi are all built here.
//!
//! Sine, cosine, tangent and pi from the ground up: the unit circle as the
//! single definition, radians as the honest unit for a turn, tangent as a
//! slope, the addition formulas as "turning twice", and then the finance
//! payoffs — seasonal models built from sine/cosine pairs, correlation as the
//! cosine of an angle, and pi inside the normal distribution. Cross-links:
//! lesson-limits (the squeeze that proves sin x over x goes to 1) and
//! lesson-exponents (the exponent rules Euler's formula leans on).
//!
//! Run it: cargo run --release --bin lesson-trig

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
    let b = Lesson::builder("Sine, Cosine, Tangent and Pi");
    let b = a_quantity_that_comes_back(b);
    let b = the_circle_that_defines_them(b);
    let b = measuring_the_turn(b);
    let b = the_triangle_is_the_circle_scaled(b);
    let b = tangent_is_a_slope(b);
    let b = turning_twice(b);
    let b = unrolling_the_circle(b);
    let b = fitting_something_that_comes_back(b);
    let b = correlation_is_a_cosine(b);
    let b = where_pi_shows_up_on_a_desk(b);
    let b = when_the_wave_story_is_a_lie(b);
    let b = turning_as_multiplication(b);
    let b = practice(b);
    let b = letter_overrides(b);
    b.build()
}

fn a_quantity_that_comes_back(b: LessonBuilder) -> LessonBuilder {
    b.heading("A quantity that comes back")
        .note("Hover any term in a formula to see what it means here. Three plots below have sliders — drag them and the curves follow.")
        .para(|p| p
            .text("Almost everything you can build out of ordinary arithmetic goes one way and keeps going. A polynomial climbs or dives and eventually runs off the page. An exponential grows or decays and never turns around. Add, subtract, multiply, raise to a power in any combination you like, and the result still has that character: it may wobble a few times, but sooner or later it commits to a direction and leaves."))
        .para(|p| p
            .text("A great deal of what a desk actually models does the opposite. A retailer's revenue peaks every December and troughs every summer, and then does it again. Trading volume spikes at the open and again at the close, every single day. Electricity demand, coupon dates, quarter-end flows, holiday effects, the working week: these come back. Not approximately, not eventually — on a schedule, forever."))
        .para(|p| p
            .text("Here is the whole idea of this lesson, and it is embarrassingly simple. If you want something that comes back, go round something. A point walking round a circle returns to where it started, exactly, forever, by construction — no amplitude lost on the way, no drift. So put a point on a circle, start it walking, and watch one of its coordinates. That coordinate is a quantity that comes back."))
        .note("That is the entire invention. Sine and cosine are the two coordinates of a point walking round a circle. Every rule, identity, graph and application in this lesson is that one sentence, read in a different direction.")
        .para(|p| p
            .text("Before any formulas, here is what that invention looks like from the ground. Picture a fairground carousel seen edge-on from a distance, so you cannot see any depth in it. A single painted horse goes round at a steady speed, but what you see is not a circle: it is one horse sliding out to the right, slowing as it goes, stopping dead at the far edge, then sliding back through the middle at full pelt, out to the far left, stopping again, and returning — forever, never a slower lap, never a shorter slide. That sliding is the sine. Where the picture stops being honest is the bobbing: a real horse also rises and falls on its pole, and that is a second, faster wave riding on this one, not the one being described. And notice what the viewing angle has done: the horse never changes speed, yet the thing you see pauses at both ends and moves fastest through the middle. That is why the wave you will meet later is flat at its peaks and steepest where it crosses the centre line."))
        .para(|p| p
            .text("Most people meet these functions as three ratios in a right-angled triangle, memorised through a mnemonic, and never quite recover from it. The triangle is real and it turns up in the fourth section, but it is a consequence, not a definition — and it is the reason so many people believe these functions stop working past a quarter turn. Start with the circle and nothing ever stops working."))
        .para(|p| p
            .text("By the end you will have built sine, cosine and tangent from the circle, understand what pi actually measures and why it is unavoidable, be able to rebuild the identities instead of memorising them, and — the part that pays — know why seasonal models are always fitted with sines and cosines in pairs, and why the correlation between two funds is literally the cosine of an angle."))
}

fn the_circle_that_defines_them(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("The circle that defines them")
        .para(|p| p
            .text("Draw a circle of radius 1, centred on the origin of a pair of axes. This is the unit circle, and radius 1 is the only special thing about it — it makes every length in the picture a plain number rather than a fraction of something."))
        .para(|p| p
            .text("Put a point at the far right, where the circle crosses the across-axis, and start walking it anticlockwise round the rim. Let ")
            .math(r"\theta")
            .text(" — the Greek letter theta — stand for how far round you have turned. Now simply read off where the point is:"))
        .display(r"(\cos\theta, \sin\theta)")
        .explain(r"(\cos\theta, \sin\theta)", "The point on the unit circle at angle theta",
            "The definition of both functions, in one pair of brackets. Cosine is the across-coordinate, sine is the up-coordinate, of the point you reach after turning through theta from the starting position. Everything else in this lesson is read off this point.")
        .explain(r"\cos\theta", "Cosine of theta",
            "How far across the point is after turning through theta: its horizontal coordinate. It starts at 1, falls to 0 at a quarter turn, reaches minus 1 at a half turn, and comes back. When theta is instead the angle between two directions, the same coordinate reads as the share of one that lies along the other — the projection reading the triangle section sets up and the correlation section cashes.")
        .explain(r"\sin\theta", "Sine of theta",
            "How far up the point is after turning through theta: its vertical coordinate. It starts at 0, peaks at 1 a quarter turn round, returns to 0 at a half turn, and dips to minus 1 at three quarters.")
        .para(|p| p
            .text("That is the definition. Not a ratio, not a triangle, not a mnemonic — two coordinates of one moving point. Cosine is across, sine is up. If you want a handhold for which is which: c comes before s in the alphabet exactly as x comes before y."))
        .figure(Figure::new(CIRCLE_SVG,
            "The unit circle with the point at fifty degrees round. The horizontal leg of the shaded triangle has length cos of theta, the vertical leg has length sin of theta, and the sloping side is the radius, which is 1. The small arc near the centre is the angle itself. Every value either function ever takes is a coordinate of this one point, at some position on this one circle.")
            .width_percent(70))
        .rule()
        .para(|p| p
            .text("Five facts now arrive free, and it is worth watching them arrive, because in a course built on the triangle definition each of them has to be taught separately and memorised."))
        .para(|p| p
            .text("First, the most-quoted identity in the subject. The point is on a circle of radius 1, so its coordinates satisfy the circle's own equation — across squared plus up squared equals 1:"))
        .display(r"\sin^2\theta + \cos^2\theta = 1")
        .explain(r"\sin^2\theta", "Sine squared",
            "The up-coordinate, multiplied by itself. Written with the exponent tucked in front of the angle, which is a convention and nothing more: it means the sine is taken first and the result squared.")
        .explain(r"\cos^2\theta", "Cosine squared",
            "The across-coordinate, multiplied by itself. Together with sine squared it is the sum of squares in Pythagoras' theorem, applied to a triangle whose sloping side is 1. And Pythagoras itself is not bedrock but one storey up: drop a perpendicular from the right angle onto the sloping side and the triangle falls into two smaller triangles, each a scaled copy of the whole, whose matching pieces a squared over c and b squared over c tile the sloping side — a squared plus b squared is c squared. The theorem stands on the same scale-invariance of flat space that makes pi one number for every circle.")
        .para(|p| p
            .text("This is Pythagoras' theorem and nothing else: a right-angled triangle whose sloping side is 1 has legs whose squares add to 1. It is not a new fact about sine and cosine. It is the statement that the point never leaves the circle."))
        .para(|p| p
            .text("Second, both functions are trapped between minus 1 and 1:"))
        .display(r"-1 \le \sin\theta \le 1")
        .explain(r"-1", "Minus one",
            "The most negative value either coordinate can take: the point at the far left of the circle, or at the bottom. The circle has radius 1, so no coordinate of a point on it can exceed 1 in size.")
        .para(|p| p
            .text("There is no angle whose sine is 2, and there never can be, because there is no point on a circle of radius 1 sitting two units above the centre. A calculator returning an error for that is not being difficult; it is being asked for a point that does not exist."))
        .para(|p| p
            .text("Third — the property the whole lesson was started for — they come back. Walk a full lap and the point is exactly where it began, so both coordinates are exactly what they were. Keep walking and it happens again. These functions repeat forever, with no decay and no drift, because the circle has no end and no memory."))
        .para(|p| p
            .text("Fourth, the signs. In a course built on triangles this is a table to be memorised, quadrant by quadrant. Here you just look at the picture. A quarter to a half turn round, the point is up and to the left, so its up-coordinate is positive and its across-coordinate is negative: sine positive, cosine negative. A half to three quarters, it is down and to the left, so both are negative. There is nothing to remember, because there is nothing that could have been otherwise."))
        .para(|p| p
            .text("Fifth, turning the other way. Walking clockwise instead of anticlockwise mirrors the point in the across-axis: same across-coordinate, opposite up-coordinate. So cosine ignores the sign of the angle and sine flips with it. Cosine is the symmetric one; sine is the one that cares which way you turned."))
        .note("Worth internalising early: cosine is not \"the other one\". It is the same walk, read along the other axis. Concretely: how far across the point is at any angle is how far up it will be a quarter turn later, so the two functions trace the identical shape and the cosine simply gets there a quarter lap sooner. Cosine is sine with a head start — the unrolling figure later in the lesson draws the two curves together, and that one quarter-lap offset is the whole of the difference between them.")
}

fn measuring_the_turn(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Measuring the turn, and where pi comes from")
        .para(|p| p
            .text("So far \"how far round you have turned\" has been a gesture. To compute anything it needs a number, and there are two conventions for supplying one. They are not equally good, and the reason is the whole content of this section."))
        .para(|p| p
            .text("Degrees split a full lap into 360 equal parts. The number is inherited from Babylonian arithmetic, which counted in sixties, and it survives because 360 divides so obligingly: into halves, thirds, quarters, fifths, sixths, eighths, ninths, tenths and twelfths, all without fractions. It is a convenient number. It is also a completely arbitrary one — there is nothing in a circle that knows about 360."))
        .para(|p| p
            .text("Radians make the opposite trade. Instead of choosing a number, they measure the turn by the distance the point has actually walked along the rim. Walk one radius-length round the edge and you have turned one radian. The angle is the arc length. Nothing has been chosen, so nothing is arbitrary."))
        .note("On the unit circle the radius is 1, so the arc walked and the angle turned are literally the same number. That coincidence is not a convenience — it is what the rest of this section cashes in.")
        .para(|p| p
            .text("Which raises the obvious question: how many radius-lengths are there in a full lap? That is asking for the circumference of a circle in units of its own radius."))
        .para(|p| p
            .text("Answer it with your hands — and commit to a guess first: surely the rim holds some whole number of radius-strings, and six would be tidy. Take a dinner plate, cut a string that just spans centre to rim, and walk it round the edge end over end, pinching each landing point with a thumbnail. Six lengths take you almost home — and then the rim runs out, with a stub left over. The half-lap is the cleaner count: three strings and a stub of about a seventh. Do it again with a coin, or with a bicycle wheel — a spoke is, near enough, the radius in steel — and the stub is a millimetre and a half on a small coin and five centimetres on a wheel with thirty-five-centimetre spokes, but always the same seventh of whatever string you cut, because the string grows with the circle. That count is the whole of pi: three-and-a-bit radius-strings to the half-lap, and three and a seventh comes to 3.142857 — which is exactly why 22/7 is the folk approximation, and why it is already wrong in the third decimal place. The stub is not your thumbnail's fault: no circle anyone will ever draw comes out even, and proving that took until 1761."))
        .para(|p| p
            .text("The count never changes because all circles are scaled copies of one another, and scaling divides out: enlarging a circle by some factor multiplies every length in the picture by that factor, rim and diameter alike, and a ratio of two lengths both multiplied by the same factor is unchanged. The factor divides out before you ever measure anything. That number is what pi names:"))
        .display(r"\pi \approx 3.14159")
        .explain(r"3.14159", "Pi, to five decimal places",
            "The ratio of any circle's circumference to its diameter. It has no exact decimal and no exact fraction — 22/7 is a common approximation and is already wrong in the third decimal place, at 3.142857. And its being one constant for all circles is a fact about flat pages, not about roundness: it leans on the Euclidean axiom that shapes can be scaled without distortion. On a sphere, where nothing can be scaled, the ratio depends on the circle's size — a circle whose surface-measured radius reaches a quarter of the way round the globe has a ratio of exactly 2. One more place it hides in plain sight: eighteenth-century proposals would have defined the metre as the length of the pendulum that beats seconds, and that definition forces the gravitational acceleration g to be exactly pi squared. The meridian metre that won instead landed within six parts in a thousand of it — which is why g in metres and seconds, 9.81, sits so suspiciously close to pi squared, 9.87. The something going round is the swing's own lap.")
        .para(|p| p
            .text("Two quiet steps in that scaling argument deserve daylight, because both reach bedrock. Scaling multiplies even a curved length by the same factor because the length of a curve means the total of ever-shorter straight steps chained along it — the same limiting move the limits lesson runs on — and scaling stretches every step equally. And every circle really is a scaled copy of every other because a circle of any radius is the unit circle enlarged that many times over; the plane itself has no built-in unit of length, so enlarging changes size and nothing else. That last assumption is a genuine axiom of flat geometry, and it is the one this whole subject stands on."))
        .para(|p| p
            .text("Distance round divided by distance across is pi, and the distance across is two radii, so the distance round is two pi radii. On the unit circle that is a lap of ")
            .math(r"2\pi")
            .text(" radius-lengths — which, since arc length is the angle, means a full lap is ")
            .math(r"2\pi")
            .text(" radians. Half a lap is pi. A quarter lap, the right angle, is pi over two."))
        .explain(r"2\pi", "Two pi: one full turn",
            "The circumference of the unit circle, and therefore — since arc length measures the angle — the number of radians in a complete lap. Whenever 2 pi appears inside a formula, something in that formula is going all the way round.")
        .para(|p| p
            .text("Here is the reframing worth carrying away, because it dissolves most of the mystique around this number. Pi is not a fact about roundness that happens to equal 3.14159. Pi is the number of radius-lengths in half a turn. That is why it turns up as the half-period of everything that oscillates, and it is why, when you meet pi in a formula with nothing visibly circular in it, the right instinct is to go looking for the thing that is going round. There always is one. A later section of this lesson goes hunting for the one hiding inside the normal distribution, and finds it."))
        .rule()
        .para(|p| p
            .text("Now the payoff that makes radians worth the trouble. For a small angle, the sine of the angle is the angle:"))
        .display(r"\sin x \approx x")
        .explain(r"\sin x", "Sine of a small angle x",
            "For angles near zero this is almost exactly x itself — provided x is in radians. It is the single most-used approximation in applied mathematics.")
        .para(|p| p
            .text("The sine of 0.1 is 0.0998334. The sine of 0.01 is 0.00999983. The approximation is not merely close; it gets relatively better the smaller the angle, and the reason is drawn below: the angle is the arc, the sine is the vertical drop, and over a short stretch the arc barely has room to curve, so the two are nearly the same segment."))
        .figure(Figure::new(RADIAN_STRING_SVG,
            "Pi counted with string, and the small-angle rule seen. On the left the rim of the unit circle is laid out in radius-lengths, the colour changing where each string ends: three strings reach almost exactly the half-lap — the pi mark sits a stub of 0.1416, about a seventh of a string, into the fourth — and six strings almost close the lap, leaving the amber leftover of 0.283, two stubs, to reach 2 pi at 6.28. On the right, a small angle up close: theta is 0.4 radians, the amber arc is the angle itself — in radians the angle is the arc — and the blue vertical is its sine, 0.3894. The two differ by 2.6 per cent, and the gap closes faster than the angle does, because the shorter the stretch the less room the arc has to curve. That is the approximation the pendulum passage below runs on, drawn rather than asserted.")
            .width_percent(90))
        .para(|p| p
            .text("The claim that this is the most-used approximation in applied mathematics deserves to be cashed once. A pendulum swinging through an angle is pulled back by a force proportional to the sine of that angle, and that equation has no clean solution — it needs machinery well beyond this lesson. Replace the sine by the angle and it collapses into an easy one, whose answer is that the swing takes the same time however wide it is. That is the whole reason a pendulum clock works: a clock cannot afford a period that changes as its swing dies down."))
        .para(|p| p
            .text("It is also exactly why such a clock has to swing narrowly. A pendulum a metre long takes about 2 seconds to go out and back. But the true period grows with the width of the swing, by roughly 1 plus the swing angle squared over 16, with the angle in radians. At 5 degrees, which is 0.0873 radians, that correction is under a twentieth of one per cent — invisible. At 30 degrees, 0.5236 radians, it is 1.71 per cent. So a clock regulated at a 5-degree swing but actually running at 30 degrees loses about 24 of the 1440 minutes in a day. Those 24 minutes are the price of leaving the region where the sine of the angle is the angle."))
        .note("Every \"small oscillations\" result in physics is this same substitution — a pendulum, a ship rolling, a molecule vibrating. Replace the sine by the angle and a hard equation becomes the wave of a later section. Worth being honest about what that means: the wave is not what the system truly does. It is what the system does while the angle stays small.")
        .para(|p| p
            .text("Stated exactly, the ratio of the two closes on 1:"))
        .display(r"\lim_{x \to 0} \frac{\sin x}{x} = 1")
        .explain(r"\lim_{x \to 0} \frac{\sin x}{x}", "The small-angle limit",
            "Where the ratio of the sine to the angle heads as the angle is driven to zero. It is exactly 1 in radians. The limits lesson proves it by trapping the ratio between the cosine and 1 and closing the trap — and notes there that you cannot get it by differentiating, because differentiating sine depends on this very limit.")
        .para(|p| p
            .text("Run the same thing in degrees and it fails. The sine of a tenth of a degree is 0.00174533, not 0.1 — out by a factor of about 57. The ratio settles not on 1 but on 0.0174533, which is pi over 180, and that factor would then attach itself to every derivative, every series and every oscillation formula in physics and finance, forever. Radians are not a deeper truth about angles. They are the unit chosen to make that constant equal to 1, and the entire convenience of the subject rests on that choice."))
        .note("Degrees are for talking to people. Radians are for calculating. Every mathematical library's sine expects radians, and mixing the two is one of the most common quiet numerical bugs there is — the code runs, the answer is wrong by a factor of 57.3.")
        .para(|p| p
            .text("The approximation is also the first term of the exact answer. A computer asked for a sine does not consult a circle; it adds up a series, and in radians the series opens with the angle itself:"))
        .display(r"\sin x = x - \frac{x^3}{3!} + \frac{x^5}{5!} - \ldots")
        .explain(r"\frac{x^3}{3!}", "The first correction",
            "The angle cubed, divided by 3 factorial, which is 6. It is the amount by which the straight-line approximation overshoots, and for small angles it is tiny: at x = 0.5 it is only 0.0208.")
        .explain(r"\frac{x^5}{5!}", "The second correction",
            "The angle to the fifth power over 5 factorial, which is 120. At x = 0.5 it contributes 0.00026 — the terms shrink fast, which is why a handful of them is enough for full precision.")
        .explain(r"\ldots", "And on forever",
            "The pattern continues with alternating signs and rising odd powers over their factorials. The series converges for every angle, however large.")
        .para(|p| p
            .text("Check it at 0.5 radians. The angle is 0.5, minus 0.0208 gives 0.4792, plus 0.00026 gives 0.479427. The true value is 0.4794255. Three terms, and the first five digits are already right — and notice that the first term being exactly the angle is the small-angle rule, sitting inside the exact answer as its leading piece. In degrees the series would open with a factor of pi over 180 and never look clean again."))
        .rule()
        .para(|p| p
            .text("Two facts about pi itself, because they explain why it is written as a letter rather than a number. It is irrational — Lambert proved in 1761 that it cannot be written as one whole number over another, so its decimal never terminates and never repeats. And it is transcendental — Lindemann proved in 1882 that it is not the solution of any polynomial equation with whole-number coefficients, which settled, after two thousand years of attempts, that a circle cannot be squared with ruler and compass. The letter is not shorthand for a decimal anyone could write out. It is the only way to name the number exactly."))
}

fn the_triangle_is_the_circle_scaled(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("The triangle is the circle, scaled")
        .para(|p| p
            .text("Now the definition most people were actually taught, arriving where it belongs — as a consequence."))
        .para(|p| p
            .text("Go back to the circle picture and drop a vertical line from the point down to the across-axis. That makes a right-angled triangle. Its sloping side is the radius, so it has length 1. Its horizontal leg reaches as far across as the point does, so it has length cosine of theta. Its vertical leg rises as far as the point does, so it has length sine of theta."))
        .para(|p| p
            .text("Now take any right-angled triangle in the world that contains the angle theta. It is a scaled copy of that one — same shape, different size — because two triangles with the same angles are similar, and a right angle plus theta fixes the third angle at whatever is left. So write ")
            .math("L")
            .text(" for the length of its sloping side and multiply the whole picture by ")
            .math("L")
            .text(". The two legs become"))
        .display(r"L\cos\theta, L\sin\theta")
        .explain(r"L\cos\theta", "The leg alongside the angle",
            "The side of a right-angled triangle that lies along the direction the angle is measured from, when the sloping side has length L. It is the unit-circle across-coordinate, scaled up by L.")
        .explain(r"L\sin\theta", "The leg across from the angle",
            "The side of a right-angled triangle facing the angle, when the sloping side has length L. It is the unit-circle up-coordinate, scaled up by L.")
        .para(|p| p
            .text("Turn those round and the school ratios fall out. Sine is the opposite side over the sloping side; cosine is the side alongside the angle over the sloping side. The mnemonic that generations have chanted is the circle definition, divided through by the length of the hypotenuse."))
        .note("And the fact that makes the whole subject possible is the similarity, not the ratio. Because every right-angled triangle with a given angle is a scaled copy of every other, the ratio of two of its sides depends on the angle alone and not at all on the size. That is why a single table of sines can serve every triangle there has ever been. And the similarity is the same scale-invariance of the plane that made pi one number for every circle — two pillars of the subject, one axiom under both.")
        .para(|p| p
            .text("Two entries of that table can be built by hand in a minute, and between them they account for most of the arithmetic in the rest of this lesson. Fold an equilateral triangle of side 1 down the middle. The fold halves the base and halves the top angle, leaving a right-angled triangle with a sloping side of 1, a base of one half, and a top angle of thirty degrees — so the sine of thirty degrees is exactly one half, not approximately, and Pythagoras gives the remaining leg as the square root of three quarters, 0.866. That is the cosine of thirty degrees, and equally the sine of sixty. The eighth of a turn is quicker still and needs no triangle at all: it is the point on the circle where across equals up, so twice one coordinate squared is 1, and each coordinate is the square root of a half — 0.7071. Every 0.5, 0.866 and 0.7071 that follows in this lesson comes out of one of those two pictures, and not one of them was looked up."))
        .para(|p| p
            .text("Worth knowing where the names came from, because one of them is an accident. Sanskrit astronomers tabulated the half-chord of an arc and called it, roughly, the bowstring; Arabic scholars transliterated the sound without writing the vowels; and a Latin translator read those bare consonants as the Arabic word for a bay or a fold, and wrote sinus. Sine means bay, because of a spelling accident in the twelfth century. Cosine is the honest one: it is the sine of the complementary angle — the angle left over from a right angle — which is precisely the quarter-turn head start noticed at the end of the second section."))
        .para(|p| p
            .text("It also shows exactly where the triangle definition runs out. A right-angled triangle's other two angles have to be less than a quarter turn each, so the ratios define sine and cosine only between zero and a quarter turn. Past that there is no triangle to point at. The circle has no such difficulty: the point keeps walking, the coordinates keep being coordinates, and negative values simply mean the point has crossed an axis. This is the entire reason to prefer the circle — not elegance, coverage."))
        .rule()
        .para(|p| p
            .text("One reading of that scaled triangle deserves to be pulled out on its own, because the last third of this lesson runs on it. Take an arrow of length ")
            .math("L")
            .text(" pointing at angle theta away from some reference direction. Its horizontal leg is how much of that arrow lies along the reference direction; its vertical leg is how much of it points off at right angles, contributing nothing in that direction at all."))
        .para(|p| p
            .text("Here is that reading with the sun in it. Hold a straight stick of length 1 up from flat ground at noon, sun directly overhead, and watch the ground: the shadow is exactly the cosine of the stick's angle. Lay the stick flat and the shadow is the full 1. Stand it upright and it vanishes. Tip it to sixty degrees and the shadow is half a stick — while the height, at 0.866 of a stick, is the larger share. The shadow is the part of the stick that lies along the ground; the height is the part the ground never sees. Read every cosine in the rest of this lesson as a noon shadow and you will always know what it is measuring."))
        .para(|p| p
            .text("Two places the picture breaks, and both teach. The shadow is the cosine only with the sun straight overhead: let the sun drop and the shadow stretches past the stick's own length and stops being the cosine of anything — the picture needs light arriving exactly perpendicular to the ground, which is what projection means. And a shadow makes the leftover part look like waste, since the stick's height casts nothing at all. In the correlation section that leftover is the part you are paying a second fund manager for. Same geometry, opposite value."))
        .para(|p| p
            .text("So cosine has a second job, and it is the one that matters in finance: cosine of the angle between two directions is the fraction of one that lies along the other. At zero degrees it is 1 — entirely aligned. At a right angle it is 0 — nothing of one lies along the other. Hold on to that sentence. In the correlation section it turns out to be the definition of correlation, word for word."))
}

fn tangent_is_a_slope(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Tangent is a slope")
        .para(|p| p
            .text("The third function is not a third mystery. It is the ratio of the first two:"))
        .display(r"\tan\theta = \frac{\sin\theta}{\cos\theta}")
        .explain(r"\tan\theta", "Tangent of theta",
            "Up divided by across, for the point at angle theta. That is rise over run, which is the slope of the line from the origin through that point. Tangent converts an angle into a slope.")
        .explain(r"\frac{\sin\theta}{\cos\theta}", "Up over across",
            "The up-coordinate divided by the across-coordinate. Because both are measured from the same origin, the ratio is exactly the gradient of the ray, and it is unchanged by how far out along the ray you look.")
        .para(|p| p
            .text("Read that as up over across and it becomes rise over run, which is the definition of a slope. So tangent is the gradient of the line from the centre out through the point. Feed it an angle, get a slope. And its inverse, arctangent, runs the other way: feed it a slope, get the angle. Those two conversions are what tangent is for."))
        .para(|p| p
            .text("Three consequences follow directly, and each is a fact that gets memorised elsewhere."))
        .para(|p| p
            .text("At a quarter turn the tangent has no value. The ray is pointing straight up, the across-coordinate is zero, and the division is by zero. Geometrically: a vertical line has no slope. Not an infinite slope, not a slope of zero — no slope, because rise over run needs a run. The gap in the tangent function and the missing gradient of a vertical line are the same fact reported twice."))
        .para(|p| p
            .text("Tangent repeats every half turn, not every full turn. Turn the ray through a half lap and it points the opposite way — but a line and its opposite have the same steepness, so the slope is unchanged. Sine and cosine need a full lap to return because they track a point; tangent needs only half because it tracks a direction, and a direction is blind to which end you are standing at."))
        .para(|p| p
            .text("And the name is a picture. Draw the vertical line touching the circle at the starting point — the tangent line, in the ordinary geometric sense of a line that grazes. Extend the ray from the centre until it strikes that line. The height at which it strikes is exactly the tangent of the angle, because the ray has run exactly one unit across and risen by its own slope. The function is named after a length measured on a tangent line."))
        .figure(Figure::new(TANGENT_LINE_SVG,
            "Where the name comes from. The vertical line grazes the circle at the starting point — a tangent line in the ordinary geometric sense — and the ray from the centre, extended, strikes it at a height of exactly the tangent of the angle. Notice where that strike sits at 50 degrees: 1.19, already above the top of the circle, which is why tangent is not trapped between minus 1 and 1 the way sine and cosine are. Then follow the pale ray. At 80 degrees the strike is 5.67, off the top of the picture, and at a quarter turn the ray is parallel to the line and never strikes it at all.")
            .width_percent(60))
        .rule()
        .para(|p| p
            .text("Now the finance use, which is a warning rather than a technique."))
        .para(|p| p
            .text("Every chart you have ever read reports a slope, but your eye does not measure slope — it measures angle. And the angle is not a property of the data. Plotting a series on axes converts the slope into an angle through the arctangent of the slope times whatever ratio the two axis scales happen to be in. Change the scales, change the angle, with the data untouched."))
        .para(|p| p
            .text("Take a series that gains 10 units over 100 days. Plot it with one day to a pixel across and one unit to a pixel up, and the line climbs at the arctangent of 0.1: about 5.7 degrees, visually flat, an asset going nowhere. Now stretch the vertical scale so one unit takes twenty pixels. The slope is still 0.1 units per day — the data has not moved — but the angle is now the arctangent of 2, about 63 degrees. A wall. Same series, same trend, same information, and a reader's instinct about \"how steep this is\" has been manufactured by an axis choice."))
        .figure(Figure::new(AXIS_SCALE_SVG,
            "The same series drawn twice, and the only difference between the panels is the span of the vertical axis. Read the axis labels first, then the two angle arcs: 5.7 degrees on the left, 63.4 on the right, from a line that gains 10 units over 100 days in both. The slope is the fact about the world and it never moved. The angle is the rendering, and it was chosen by whoever set the axis.")
            .width_percent(70))
        .note("Which is why the honest way to compare two charts' steepness is to compare their numbers, and the fastest way to mislead with a truthful chart is to rescale an axis. The slope is a fact about the world; the angle is a fact about the rendering.")
        .para(|p| p
            .text("The same conversion appears wherever a fitted line is described. A regression of a fund's returns on its index gives a slope — the beta — of, say, 0.8, meaning the fund moves 0.8 for each 1 of the index. On equally-scaled axes that fitted line sits at the arctangent of 0.8, about 38.7 degrees. That is a real angle, but be careful what it is not: it is the angle of the fitted line in the scatter plot, and it has no direct relationship to the angle in the correlation section further down, which is measured in an entirely different space. Two different angles, two different questions. Beta says how much the fund moves when the index moves; correlation says how reliably. A fund can have a beta of 0.8 with a correlation of 0.2, and the two angles then have nothing to say to each other."))
}

fn turning_twice(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Turning twice")
        .para(|p| p
            .text("Here is a question with an obvious answer that turns out to contain every identity in trigonometry. If you turn through one angle and then turn through another, where do you end up? Through the sum of them, plainly. So whatever sine and cosine do, they must satisfy something that says exactly that — and that something is the pair of formulas the subject is most famous for making people memorise:"))
        .display(r"\cos(\alpha + \beta) = \cos\alpha\cos\beta - \sin\alpha\sin\beta")
        .display(r"\sin(\alpha + \beta) = \sin\alpha\cos\beta + \cos\alpha\sin\beta")
        .explain(r"\cos(\alpha + \beta)", "Cosine of the total turn",
            "The across-coordinate after turning through alpha and then through beta — that is, after turning through their sum. The formula is the definition of a rotation — origin fixed, no distance changed — unpacked into coordinates; nothing in it is special to angles. And preserving distance really is enough to preserve the whole picture: straightness is itself a distance fact — a point lies on the straight path between two others exactly when the two part-lengths add to the whole, and any bend makes them overshoot — so a map that changes no distance can bend nothing, and parallelograms survive intact.")
        .explain(r"\sin(\alpha + \beta)", "Sine of the total turn",
            "The up-coordinate after turning through alpha and then through beta.")
        .explain(r"\cos\alpha\cos\beta", "Across times across",
            "One of the two contributions to the final across-coordinate. It is the part inherited from the first turn's across-coordinate.")
        .explain(r"\sin\alpha\sin\beta", "Up times up",
            "The other contribution to the final across-coordinate, subtracted rather than added — the second turn tips the first turn's upward part backwards, which is where the minus sign comes from. One quiet step deserves daylight: the turned up-arrow lands a quarter turn past the turned right-arrow because quarter-turn-then-beta is the same as beta-then-quarter-turn — turns about one centre compose by adding arc along the same rim, and addition does not care about order.")
        .explain(r"\sin\alpha\cos\beta", "Up times across",
            "One of the two contributions to the final up-coordinate.")
        .explain(r"\cos\alpha\sin\beta", "Across times up",
            "The other contribution to the final up-coordinate. Both add, which is why the sine formula has a plus where the cosine formula has a minus.")
        .para(|p| p
            .text("They look arbitrary and they are not. Here is where they come from, and it takes one idea: a turn moves the whole plane rigidly, so if you know where the two reference directions end up, you know where everything ends up, because every point is built out of those two."))
        .para(|p| p
            .text("Why rigidity is enough deserves one paragraph, because it is the step every derivation of these formulas quietly leans on. Saying a point is \"so much of the way along one direction plus so much along the other\" is not a formula but a picture: a parallelogram with the origin at one corner and the point at the opposite one. A rigid motion — and a rotation is, by definition, a motion that holds the origin still and changes no distance — carries a parallelogram to a parallelogram: every side keeps its length, so parallel sides stay parallel and equal. And because nothing rescales, a segment of length cosine alpha is still that length afterwards, still lying along its reference arrow, which has moved and taken the segment with it. The multipliers survive the turn; only the directions they multiply have changed."))
        .para(|p| p
            .text("Turn through beta. The rightward reference direction lands on the circle at angle beta, which by definition is at across-coordinate cosine beta and up-coordinate sine beta. The upward reference direction is a quarter turn ahead of it, so it lands a quarter turn further on — at across minus sine beta and up cosine beta, because a quarter turn swaps the two coordinates and hands the new across a minus sign."))
        .para(|p| p
            .text("Now the point at angle alpha is cosine alpha of the way along the rightward direction plus sine alpha of the way along the upward one. Turn the whole picture by beta and that recipe still holds, with the turned directions substituted in. Collect the across-parts and you get cosine alpha times cosine beta, minus sine alpha times sine beta. Collect the up-parts and you get sine alpha times cosine beta, plus cosine alpha times sine beta. The formulas are not facts about angles at all — they are bookkeeping for a rigid turn."))
        .figure(Figure::new(TURN_TWICE_SVG,
            "The derivation, drawn once with alpha at 40 degrees and beta at 30. On the left the point at 40 degrees is built from the two reference arrows: 0.766 of the rightward one, then 0.643 of the upward one. On the right the whole picture has been turned 30 degrees — the rightward arrow now sits at (0.866, 0.5), the upward one at (minus 0.5, 0.866) — and the multipliers are untouched: the same 0.766 rides the turned right-arrow, the same 0.643 rides the turned up-arrow, and the far corner lands at (0.342, 0.940), which is exactly the point at 70 degrees. Collect the across-parts and you are reading the cosine formula, minus sign included — it arrives because the turned up-arrow leans left.")
            .width_percent(85))
        .note("Worth doing once by hand and then never again. Once you have seen that the addition formulas are the statement \"turning twice is turning once by the total\", they stop being things to remember and become things you can rebuild in thirty seconds from a picture. One reading for later: preserving sums and scalings is exactly what \"linear\" means, so the two formulas are the sentence \"rotation is linear\" written out in coordinates — their four products are the four entries of a two-by-two turning matrix, the machinery the algebra-to-linear lesson builds.")
        .rule()
        .para(|p| p
            .text("The single most common error in the subject is worth stating as a temptation, because it is genuinely tempting. Everything else in school algebra distributes across a sum — and worse, sine itself feeds the instinct: near zero, sine is its angle, so small turns genuinely do add their sines. Double 0.1 radians and the sine goes from 0.0998 to 0.1987, a ratio of 1.99. The temptation is the small-angle rule from the radians section, over-extended to angles with room to curve — and at size the failure is not marginal. Take a thirty degree turn and a sixty degree turn. Together they make a right angle, whose sine is exactly 1. But the sine of thirty is 0.5 and the sine of sixty is 0.8660, and those add to 1.3660 — over by more than a third."))
        .para(|p| p
            .text("The reason is worth having. Sine is not a multiplier that scales its input; it is a coordinate read off a turn. Turns compose by rotating, and rotating mixes the two coordinates into each other — which is exactly what the addition formulas describe, and exactly why both formulas need both functions. There is no way to write the sine of a sum without the cosine appearing, because turning takes the across-part and tips it upward."))
        .plot(Plot::new(0.0..=6.28318531)
            .curve("the truth: sine of the total turn", "sin(x + second_turn)")
            .curve("the temptation: sine plus sine", "sin(x) + sin(second_turn)")
            .param("second_turn", 0.0..=3.14159265, 1.04719755)
            .hline(1.0)
            .hline(-1.0)
            .x_label("the first turn, in radians")
            .y_label("value")
            .height(300.0)
            .caption("Drag the second turn and watch the tempting rule fail in a way the arithmetic alone does not show. One curve is the honest sine of the total turn — exactly what the addition formula computes — and the other is what you get by adding the two sines. The honest one only ever slides sideways: a second turn moves where the peak sits and never how high it is. The tempting one does the exact opposite, never sliding at all, only lifting, because adding a constant is the only thing it can do. The lines at plus and minus 1 are the ceiling and floor the second section proved no sine can break, and at the default second turn of 1.047 radians — sixty degrees — the sum-of-sines curve spends nearly half the lap above that ceiling, peaking at 1.866, which is not a value any sine has. Park your eye at a first turn of 0.524, thirty degrees, and read the gap: 1 against 1.366, the arithmetic above, now a vertical distance. The two curves touch only where the settings make the total turn zero or a whole lap — drag the slider to the far left and they coincide everywhere, because the tempting rule is right precisely when there is no second turn to be wrong about."))
        .rule()
        .para(|p| p
            .text("Set both angles equal and the addition formulas collapse into the double-angle formulas, free of charge:"))
        .display(r"\sin 2\theta = 2\sin\theta\cos\theta")
        .explain(r"\sin 2\theta", "Sine of twice the angle",
            "What you get by putting alpha and beta both equal to theta in the addition formula. It is emphatically not twice the sine of theta — it is the tempting answer times a correction of exactly cos theta. Near zero that correction is nearly 1, which is why the temptation exists at all; at 30 degrees it is 0.866, and twice sin 30 times 0.866 is 0.866 — exactly sin 60, where twice the sine alone says 1.")
        .explain(r"2\sin\theta\cos\theta", "Twice the product of the two coordinates",
            "The two identical contributions from the addition formula, added. It peaks when the two coordinates are equal — at an eighth of a turn — which is the reason a projectile fired at forty-five degrees goes furthest.")
        .para(|p| p
            .text("That parenthetical about forty-five degrees is worth a few lines of arithmetic, because the double-angle formula does all the work and you can watch it arrive. Throw a ball at some speed, at an angle above level ground, and ignore the air. It travels across at the speed times the cosine of the angle, and it stays in the air for a time proportional to the speed times the sine of the angle, since that is the upward part gravity has to cancel. Multiply the two and the range carries the product of the sine and the cosine — the double-angle formula in the flesh — so the distance is the speed squared times the sine of twice the angle, divided by gravity."))
        .para(|p| p
            .text("Put numbers on it. Throw at 10 metres per second and call gravity 10 rather than 9.81, which costs about 2 per cent and makes the speed squared over gravity exactly 10 metres. At 45 degrees the sine of 90 degrees is 1, and the ball lands at 10 metres — the furthest it can possibly go, because 1 is the largest a sine ever is. At 30 degrees the sine of 60 degrees is 0.866, so 8.66 metres. And at 60 degrees the sine of 120 degrees is also 0.866: also 8.66 metres. Two quite different throws, the same landing spot. The pair 15 and 75 degrees does it too, at 5 metres each."))
        .para(|p| p
            .text("That pairing is not a coincidence and not a fact about gravity. Doubling turns two angles that add to a quarter turn into two angles that add to a half turn, and two angles adding to a half turn are mirror images across the up-axis — the same height on the circle. Equal heights, equal sines, equal range. The whole result is one glance at the picture from the second section."))
        .rule()
        .para(|p| p
            .text("Now the consequence that the applied half of this lesson runs on entirely. Take a cosine and a sine of the same angle, give each of them a weight, and add them. What shape is the result?"))
        .display(r"A\cos\theta + B\sin\theta = C\cos(\theta - \varphi)")
        .explain(r"A\cos\theta", "The cosine, weighted",
            "A cosine wave scaled by A. On its own it peaks at angle zero, wherever you happen to want the peak.")
        .explain(r"B\sin\theta", "The sine, weighted",
            "A sine wave scaled by B. On its own it peaks a quarter turn later than the cosine does.")
        .explain(r"C\cos(\theta - \varphi)", "One cosine, rescaled and shifted",
            "A single cosine wave of height C whose peak has been slid round to angle phi. The identity says any weighted mixture of a cosine and a sine of the same angle is one of these — mixing them never produces a new shape, only a new height and a new peak position. Height-and-shift and cosine-weight-and-sine-weight are the same point read in two coordinate systems: distance-and-angle against across-and-up.")
        .para(|p| p
            .text("The answer is: the same shape as either of them, just rescaled and slid along. Expanding the right-hand side with the addition formula shows why. Cosine of theta minus phi is cosine theta cosine phi plus sine theta sine phi, so the right-hand side is cosine theta times ")
            .math("C\\cos\\varphi")
            .text(", plus sine theta times ")
            .math("C\\sin\\varphi")
            .text(". Match those against the left-hand side and you need ")
            .math("A")
            .text(" to be ")
            .math("C\\cos\\varphi")
            .text(" and ")
            .math("B")
            .text(" to be ")
            .math("C\\sin\\varphi")
            .text(" — which is to say, the pair of weights is itself a point on a circle, at distance ")
            .math("C")
            .text(" and angle ")
            .math(r"\varphi")
            .text("."))
        .explain(r"C\cos\varphi", "The across-coordinate of the weight pair",
            "Reading the two weights as a point: this is how far across it sits, which the matching shows is exactly A.")
        .explain(r"C\sin\varphi", "The up-coordinate of the weight pair",
            "How far up the weight pair sits, which the matching shows is exactly B.")
        .para(|p| p
            .text("One step in that matching deserves its own sentence: such a pair always exists. Any point other than the origin sits at some positive distance from it, so it lies on the circle of that radius — the unit circle scaled up — and the walk round that circle visits it at exactly one angle per lap. Distance and angle name every point in the plane, once each. That is why the identity is not an occasional trick but a change of coordinates: nothing is lost in either direction, and no wave of this period is unreachable."))
        .para(|p| p
            .text("So the height is the distance of that point from the origin, straight out of Pythagoras:"))
        .display(r"C = (A^2 + B^2)^{1/2}")
        .explain(r"(A^2 + B^2)^{1/2}", "The distance to the weight pair",
            "The square root of the sum of the squared weights: how tall the combined wave is. With A = 10.392 and B = minus 6 it comes to exactly 12, because 108 plus 36 is 144.")
        .para(|p| p
            .text("and the shift is the angle of that point. Take the pair the seasonal section is about to need, 10.392 and minus 6. Divide each by the height of 12 and you get a cosine of 0.866 and a sine of minus one half — the thirty-degree pair folded out of an equilateral triangle two sections ago, with the sine now negative. So the angle is minus thirty degrees, and those two weights produce a single wave of height 12 whose peak has slid back by a twelfth of a lap."))
        .figure(Figure::new(WEIGHT_PAIR_SVG,
            "The weights are a point, and that is the whole content of the identity. Read the cosine weight as an across-coordinate and the sine weight as an up-coordinate, and the pair 10.392 and minus 6 becomes a single point in a plane of its own. Look at the two things that point carries: its distance from the origin is 12, which is the height of the combined wave, and its angle is minus 30 degrees, which is how far round the calendar the peak has slid. Two ordinary weights a regression can find, one point, and both the size of the season and its timing read straight off it.")
            .width_percent(65))
        .para(|p| p
            .text("The identity earns its keep a second way, which is worth one more set of weights. A wholesaler carries two seasonal costs. Inventory financing swings 3 million either side of its average and peaks in January. Overtime swings 4 million and peaks in April — three months later, which on a twelve-month cycle is exactly a quarter lap, so it is the sine partner to the first one's cosine. Their total is 3 times the cosine plus 4 times the sine of the same angle, and the identity says that is one wave, of height the square root of 9 plus 16. Which is 5."))
        .para(|p| p
            .text("Not 7. Two cost seasons peaking three months apart do not reinforce; together they swing only a quarter more than the larger one does alone, and a budget built by adding the two peaks would over-provision by 40 per cent. And the combined peak falls where the cosine is 3 over 5 and the sine is 4 over 5 — an angle of 53.1 degrees, which is 53.1 three-hundred-and-sixtieths of a year: month 1.77, late February. Neither January nor April, and pulled towards the larger of the two, exactly as the sizes of the weights say it should be."))
        .note("Read the identity the other way round and it becomes a technique, which is exactly how it is used in practice: any wave of any height with its peak anywhere at all can be written as a weighted sum of one fixed cosine and one fixed sine. Two ordinary numbers, and you have reached every wave of that period. The next two sections are that sentence, cashed.")
}

fn unrolling_the_circle(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Unrolling the circle into a wave")
        .para(|p| p
            .text("Everything so far has treated the angle as a position. Let time drive it instead: send the point round the circle at a steady rate and plot its height against the clock. What comes out is the familiar wave, and it is worth being clear that the wave is not a new object — it is the circular walk, unrolled."))
        .figure(Figure::new(WAVE_SVG,
            "The unrolling, drawn. On the left a point sits on the circle sixty degrees round; on the right, the same walk plotted against the angle, from zero to a full lap. The dashed amber line carries the point's height across to the blue curve: the height of the wave at any angle is the height of the point at that angle, and nothing more. One lap of the circle is one cycle of the wave, which is why the horizontal axis is marked in fractions of two pi. The green dashed curve is that same walk read along the other axis — the across-coordinate rather than the up one. Compare the two shapes rather than glancing at them: they are not merely similar, they are the identical curve, with the cosine a quarter lap ahead. It is already at its peak when the walk starts, while the sine still has a quarter lap to travel before reaching its own. That one offset is the entire difference between the two functions, and it is what the seasonal pair later exploits.")
            .width_percent(90))
        .plot(Plot::new(0.0..=6.28318531)
            .curve("the sine itself", "sin(x)")
            .curve("its slope, measured over a step", "(sin(x + step) - sin(x)) / step")
            .curve("cosine, a quarter lap ahead", "cos(x)")
            .param("step", 0.02..=2.5, 2.0)
            .hline(0.0)
            .x_label("angle, in radians")
            .y_label("value")
            .height(300.0)
            .caption("Measure the sine's steepness the way a spreadsheet would: step forward by a fixed amount, take the rise, divide by the step. At the default step of 2 radians the measured slope already has a familiar shape — it is exactly a cosine, just short and early: height 0.841, slid left by a full radian, which is half the step. Now drag the step towards zero and watch the measured slope climb and slide until it settles exactly onto the cosine. The head-start note from the second section was hiding a reason: where the sine crosses its centre line at full steepness, the cosine sits at its peak, and where the sine peaks and is momentarily flat, the cosine sits at zero — the quarter-lap-ahead curve is not merely the same shape as the sine, it is the sine's slope, point by point. And the settling is the radians payoff one more time: the measured slope's height is the sine of half the step over half the step, the very ratio the third section drove to 1. In degrees it would drive to 0.0175 instead, and the slope of sine would be 0.0175 cosines, forever."))
        .para(|p| p
            .text("Why the slope had to be the cosine is one quarter turn of reasoning. Let the point walk at one radian per unit of time, so clock and angle agree. Its speed along the rim is then exactly 1 — the arc is the angle — and its direction of travel is along the rim, at right angles to the radius. So the velocity is the position rotated a quarter turn, and the quarter turn was computed in the turning-twice section: coordinates swap, the new across takes the minus sign. The rate of change of the sine is the cosine, and one more quarter turn gives the rate of the rate: minus the sine. The wave accelerates towards its centre line by exactly its own displacement — which is the pendulum's small-angle equation from the pi section, and the reason a narrow swing traces this precise wave against the clock. It is also the carousel from the first page, cashed: the horse's visible speed is the cosine, full pelt as it crosses the middle, zero at the edges where the wave runs flat."))
        .para(|p| p
            .text("A wave of this kind is completely described by three numbers, and every applied use in the rest of this lesson consists of choosing them:"))
        .display(r"y = A\sin\left(\frac{2\pi t}{P} + \varphi\right)")
        .explain(r"A\sin\left(\frac{2\pi t}{P} + \varphi\right)", "A wave in full dress",
            "Three knobs and nothing else: A sets the height, P sets how long one cycle takes, and phi sets where in the cycle the clock starts. Any wave of this shape is one of these, and no fourth knob exists.")
        .explain(r"\frac{2\pi t}{P}", "The angle at time t",
            "How far round the circle the point has walked by time t. Dividing the elapsed time by the period gives the fraction of a lap completed, and multiplying by 2 pi turns that fraction into radians.")
        .para(|p| p
            .text("The amplitude ")
            .math("A")
            .text(" is the radius of the circle being walked: half the distance from trough to peak. The period ")
            .math("P")
            .text(" is how long one lap takes, in whatever units the clock is in — twelve months, twenty-four hours, one week."))
        .para(|p| p
            .text("The reason ")
            .math("P")
            .text(" enters the way it does is worth a sentence, because it is the one piece of the formula people copy without reading. The bracket has to advance by exactly one full lap — by ")
            .math(r"2\pi")
            .text(" — over one period. So the multiplier on time has to be ")
            .math(r"2\pi")
            .text(" divided by the period, and that is the whole derivation."))
        .para(|p| p
            .text("The phase ")
            .math(r"\varphi")
            .text(" is where on the lap the point was standing when the clock started. It slides the whole wave sideways and does nothing else at all — it cannot change the height and it cannot change the period. Three knobs, three jobs, no interference between them."))
        .plot(Plot::new(0.0..=24.0)
            .curve("the target: height 1, period 12, no shift", "sin(2*3.14159265*x/12)")
            .curve("your wave", "amplitude * sin(2*3.14159265*x/period + phase)")
            .param("amplitude", 0.2..=3.0, 2.0)
            .param("period", 3.0..=24.0, 18.0)
            .param("phase", -3.14159265..=3.14159265, 1.2)
            .hline(0.0)
            .x_label("time, in months")
            .y_label("height")
            .height(300.0)
            .caption("The three knobs, one at a time — the sliders are A, P and phi, spelled out. Try to land your wave exactly on the fixed target curve, and notice that each knob does one job and cannot do another's. Amplitude stretches the wave vertically while every crossing of the centre line stays exactly where it was. Period crowds the crossings together or spreads them apart while the height never moves. Phase slides the whole shape sideways, rigid. Only one setting of all three matches the target, and no amount of the other two will make up for a wrong one — which is the reason the next section has to fit all three, and the trouble it then runs into. Two things worth trying on the way: match amplitude and period to the target first and then push phase to either end, about 3.14, and you get the target's exact mirror, because half a lap of shift is the same as flipping the sign; and set period to 6 and the wave completes two cycles where the target completes one, which is the second Fourier term the seasonal section adds."))
        .para(|p| p
            .text("Two more words you will meet: the frequency, one over the period — laps per unit of time — and the angular frequency, two pi over the period — radians per unit of time. The same information in three units; different fields settled on different ones."))
        .note("A word of caution about the word \"cycle\". Everything in this section is deterministic: the same height, the same period, forever. Real series that come back rarely come back that obediently, and the final section is about the difference between a repeat with a cause and a repeat you found by looking.")
}

fn fitting_something_that_comes_back(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Fitting something that comes back")
        .para(|p| p
            .text("Here is the first payoff, and it answers a question that puzzles most people the first time they open a forecasting package: why do seasonal models always add sines and cosines in pairs? It looks like duplication. It is not — the pair is doing something a single term provably cannot."))
        .para(|p| p
            .text("Take a retailer's monthly revenue, in millions, over one year. Number the months from zero for January to eleven for December. The revenue runs 110.4, 106.0, 100.0, 94.0, 89.6, 88.0, 89.6, 94.0, 100.0, 106.0, 110.4, 112.0 — high in the new year, sagging through the summer, bottoming in June and peaking in December. The numbers here are constructed rather than scraped, so that the fit can be checked exactly; the shape is the ordinary shape of a retail year."))
        .para(|p| p
            .text("The obvious model is the wave from the last section: a base level, a height, and a peak month. In symbols, a level plus a cosine of height ")
            .math("C")
            .text(" whose peak has been slid to the right month. And there the trouble starts. The level and the height are ordinary coefficients, but the peak month sits inside the cosine, and a quantity inside a function is not something ordinary least squares can solve for. You would have to search for it."))
        .para(|p| p
            .text("The identity from the last section removes the search entirely. Any wave of a given period, however tall and wherever it peaks, is a weighted cosine plus a weighted sine of that same period. So write the model that way:"))
        .display(r"R_m = 100 + A\cos\frac{2\pi m}{12} + B\sin\frac{2\pi m}{12}")
        .explain(r"R_m", "Revenue in month m",
            "The modelled revenue, in millions, for the month numbered m — zero for January through eleven for December.")
        .explain("100", "The base level",
            "The average revenue the seasonal swing moves around, in millions. Estimated as an ordinary intercept.")
        .explain(r"A\cos\frac{2\pi m}{12}", "The cosine partner",
            "A cosine that completes exactly one cycle over twelve months, scaled by the coefficient A. On its own it can only peak in January or, with a negative A, in July.")
        .explain(r"B\sin\frac{2\pi m}{12}", "The sine partner",
            "A sine over the same twelve-month cycle, scaled by B. It is the term that lets the peak sit anywhere at all, rather than only at the two places the cosine alone can reach.")
        .explain(r"\frac{2\pi m}{12}", "The angle for month m",
            "The month number as a fraction of the twelve-month year, turned into radians by multiplying by a full lap. January is angle zero and December is eleven twelfths of the way round.")
        .para(|p| p
            .text("Now everything unknown is an ordinary coefficient multiplying an ordinary column of numbers. Two sine and cosine columns get computed once from the calendar, and a plain regression returns the weights. For this series it returns 10.392 and minus 6.000."))
        .para(|p| p
            .text("Then run the identity forwards to read the answer in human terms. The height is the square root of 108 plus 36, exactly 12 — so revenue swings 12 million either side of the 100 million base, a peak-to-trough range of 24. And the peak sits where the angle matches that of the weight pair: cosine 0.866, sine minus 0.5, which is minus a twelfth of a lap, which is eleven twelfths going forwards — month eleven. December. The fit recovered both the size of the season and its timing, and it never once had to search for a peak month."))
        .note("This is the entire reason the pair exists, and it is worth saying plainly: one term fixes the size of the season, the other fixes its timing, and neither can do the other's job. A cosine alone peaks in January or July and nowhere else, because those are the only two places a cosine of that period has a peak. Adding its sine partner buys every month in between — for the price of one more coefficient, and with no search.")
        .plot(Plot::new(0.0..=12.0)
            .scatter("actual revenue", revenue())
            .curve("your fit", "100 + A*cos(2*3.14159265*x/12) + B*sin(2*3.14159265*x/12)")
            .param("A", -14.0..=14.0, 12.0)
            .param("B", -14.0..=14.0, 0.0)
            .hline(100.0)
            .x_label("month, 0 for January through 11 for December")
            .y_label("revenue, millions")
            .height(320.0)
            .caption("Fit it by hand, and watch the sine partner earn its place. The curve starts as the cosine alone, with B at zero — a season of the right size peaking in January, when the data plainly peaks in December. Sweep A across its whole range and the failure never improves: positive values peak in January, negative ones peak in July, and there is no third option, because those are the only two places a cosine of this period has a peak. A sets height, not timing. Now bring B down through negative values and the peak slides round the calendar; at about minus 6, with A eased down to 10.4, the curve lands on every point. The flat line is the 100 million base the season swings around. The thing to feel is that you never dialled in a peak month — it emerged from the angle of the weight pair, which is exactly what lets a regression find it without searching.")
        )
        .note("Two pieces of small print, both of which bite in production. The peak is set by the weight pair — both coordinates, not their ratio. The ratio alone leaves a half-turn ambiguity: 10.392 and minus 6 share a ratio with minus 10.392 and 6, whose peak is six months away, and it is the signs that decide between them — which is exactly why numerical libraries read the angle with a two-argument arctangent. And if a regression returns both weights near zero, the season has no size, and a peak month computed from two noise-sized weights is an angle computed from noise, however confidently it prints.")
        .rule()
        .para(|p| p
            .text("Two extensions, briefly, because you will meet both."))
        .para(|p| p
            .text("One pair of terms can only make one smooth hump per year. Real seasons are often lumpier — a sharp December spike rather than a gentle swell, or two busy periods. The fix is to add a second pair at twice the frequency, completing two cycles a year rather than one, then a third at three times, and so on. Each pair adds shape and costs two coefficients. This is what a forecasting package means by the order of its seasonality: how many pairs it is allowed. Start with one, add another only while genuinely out-of-sample error falls, and stop at the first increase — past that you are fitting the noise in last year's calendar. And when the season itself drifts — a peak creeping earlier, an amplitude growing with the business — the Kalman filter lesson picks this exact pair up as a state: each month the weight pair is turned through one month's angle by the turning matrix of the turning-twice note, and the filter nudges it as each observation arrives, tracking a season a frozen regression would miss."))
        .para(|p| p
            .text("The second use is the same trick doing a different job, and it turns up in every machine-learning pipeline with a timestamp in it. Suppose a model needs the hour of the day. Feed it the raw number and you have told it that eleven at night and midnight are twenty-three units apart, while midnight and one in the morning are one unit apart — which is a lie about the clock, and a lie that costs accuracy at exactly the moment most systems are quietest."))
        .para(|p| p
            .text("Encode the hour as a position on a circle instead: take the cosine and the sine of the hour's fraction of a full lap. Midnight lands at across 1, up 0. Eleven at night lands at across 0.9659, up minus 0.2588, and one in the morning at across 0.9659, up plus 0.2588. Both are the same distance from midnight, 0.2611, which is what the clock actually means. The wrap-around has been built into the coordinates."))
        .para(|p| p
            .text("And here is why one column will not do, which is the same argument as the seasonal pair from a different angle. The sine alone gives three in the morning and nine in the morning the identical value of 0.7071 — the model cannot tell them apart. The cosine separates them at once: 0.7071 against minus 0.7071. You need two numbers because a position on a circle takes two coordinates, and no single number can name a point on a circle without a discontinuity somewhere. Cutting the circle anywhere is exactly the midnight problem you were trying to fix."))
}

fn correlation_is_a_cosine(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Correlation is a cosine")
        .para(|p| p
            .text("This is the deepest connection in the lesson, and the one most likely to change how you read a risk report. It has nothing to do with waves."))
        .para(|p| p
            .text("Take a fund's daily returns over some window — say twenty days — and instead of thinking of them as twenty numbers in a row, think of them as one arrow. One axis per day, twenty axes, and the arrow's coordinates are the returns. Nobody can picture twenty dimensions. Why nobody needs to is a debt this section owes you, and pays two paragraphs down."))
        .para(|p| p
            .text("Now subtract each fund's own average return from every one of its days, so each arrow is measured from the middle of its own behaviour rather than from zero. Do that for two funds and compute the angle between the two arrows. The result:"))
        .display(r"r = \frac{x \cdot y}{\|x\| \|y\|} = \cos\theta")
        .explain(r"\frac{x \cdot y}{\|x\| \|y\|}", "The correlation, and the cosine",
            "Multiply the two arrows day by day and add up the products, then divide by both arrows' lengths. Written out in full for two lists of demeaned returns, this is Pearson's correlation coefficient exactly as a spreadsheet computes it — and it is also, character for character, the standard formula for the cosine of the angle between two arrows. Two more facts live here. The ratio can never leave minus 1 to 1, because the piece of one arrow lying along another is never longer than the arrow itself — a bound usually taught as a property of Pearson's coefficient that is really a fact about projection. And in many dimensions this quantity is not discovered but installed: length and angle there are defined by it, in exactly the way that makes plane geometry survive verbatim on every flat sheet.")
        .explain(r"x \cdot y", "The day-by-day agreement",
            "Each day's deviation in one fund multiplied by the same day's deviation in the other, all summed. Days when both are above their average, or both below, add to it; days when they disagree subtract from it.")
        .explain(r"\|x\|", "The length of the first arrow",
            "The square root of the summed squared deviations — which, up to a constant, is the fund's standard deviation. Dividing by it is what strips the size of the moves out and leaves only their direction.")
        .para(|p| p
            .text("First, why that ratio is a cosine at all, rather than a formula that merely resembles one. Scale both arrows to length 1 — dividing by the two lengths is exactly that, since a common factor pulls out of a sum. A unit arrow at angle alpha sits on the unit circle, so its coordinates are cosine alpha and sine alpha: the circle definition, nothing else. Multiply two unit arrows coordinate by coordinate and add, and the product sum is cosine alpha times cosine beta plus sine alpha times sine beta. Now run the turning-twice formula on alpha and minus beta, remembering the fifth fact from the circle section — cosine ignores the sign of a turn, sine flips with it — and that expression is exactly the cosine of alpha minus beta: the cosine of the angle between the arrows. The formula does not resemble a cosine; it is the turning-twice identity read backwards, and dividing by the two lengths is what scales both arrows onto the unit circle where that identity lives."))
        .para(|p| p
            .text("Second, why twenty dimensions changes nothing. Two arrows from a common origin never need more than two dimensions between them: whatever the surrounding space, they span one flat sheet through the origin, and the whole triangle — both arrows and their difference — lies inside it. The angle between two twenty-day return arrows is an ordinary flat angle on an ordinary flat sheet. The twenty coordinates say only which sheet."))
        .para(|p| p
            .text("Correlation is not analogous to an angle, or a useful metaphor for one. The two formulas are the same formula. Pearson's coefficient is the cosine of the angle between two funds' demeaned return arrows, and every property of correlation that has to be learned as a rule is a property of cosine that can be seen."))
        .para(|p| p
            .text("Read the special cases straight off the circle. A correlation of 1 is an angle of zero: the arrows point the same way, and one fund's returns are a positive multiple of the other's. A correlation of 0 is a right angle — \"uncorrelated\" means perpendicular, in the literal geometric sense. A correlation of minus 1 is a half turn: exactly opposite. And a correlation of 0.5, which reads like \"halfway to unrelated\", is an angle of sixty degrees — two thirds of the way to a right angle already. Correlation compresses the angles near 1 and stretches them near 0, which is one reason a drop from 0.9 to 0.8 feels smaller than it is."))
        .rule()
        .para(|p| p
            .text("The geometry pays a real dividend when you ask what a correlation of 0.7 leaves behind."))
        .figure(Figure::new(PROJECT_SVG,
            "Two funds as arrows from a common origin, at sixty degrees to each other. Dropping a perpendicular from the tip of fund B onto fund A's line splits B into two pieces: the piece lying along A, whose length is cos of the angle times B's length, and the piece at right angles to A, whose length is sin of the angle times B's length. The first piece is the part of B that A already gives you; the second is the part it cannot. At sixty degrees the split is 0.5 and 0.866.")
            .width_percent(65))
        .para(|p| p
            .text("Drop a perpendicular from the tip of one arrow onto the other, exactly as in the fourth section. The arrow splits into a piece lying along the other — of length cosine of the angle times its own length — and a piece at right angles to it. The first piece is the part of this fund's behaviour the other one already accounts for. The second is genuinely its own."))
        .para(|p| p
            .text("So the correlation is the fraction lying along, and the fraction left over is the sine of the same angle. Because the two pieces are at right angles, Pythagoras relates them, and the perpendicular share works out at"))
        .display(r"(1 - r^2)^{1/2}")
        .explain(r"(1 - r^2)^{1/2}", "The independent share",
            "How much of one fund's variation is at right angles to the other's — the part no amount of the first fund can explain. At a correlation of 0.7 it is the square root of 0.51, about 0.714: nearly three quarters of the movement is still its own.")
        .explain(r"r^2", "Correlation squared",
            "The share of one fund's variance the other accounts for. It is a share of variance rather than of movement because variance is a squared length, and squared lengths are what Pythagoras adds.")
        .para(|p| p
            .text("Put a person in front of those numbers. An allocator is told to strip duplication out of a book: two funds, and the risk report calls them strongly correlated at 0.7, so the obvious move is to sell one, since seven tenths of it is apparently the other one already. Now measure it. A correlation of 0.7 is an angle of about 45.6 degrees — most of the way to perpendicular. The piece of the second fund lying along the first has length 0.7 of it; the piece at right angles has length the square root of 0.51, which is 0.714 — larger than the shared piece. Strongly correlated at 0.7, and more of the second fund is its own than is the first fund's. So selling it does not remove a duplicate. It removes a bet of which slightly more than half was never available from the first fund at any weighting at all."))
        .para(|p| p
            .text("And this is where the most-quoted number in regression comes from. The squared correlation is the share of variance explained — the famous r-squared — and the reason it is the square rather than the correlation itself is simply that variance is a squared length. The explained and unexplained pieces sit at right angles, so their squares add. R-squared is Pythagoras, applied to arrows made of returns."))
        .rule()
        .para(|p| p
            .text("Now the consequence that catches people out in practice, and it is pure geometry."))
        .para(|p| p
            .text("Consider three funds over three days. Fund A returns 2.5 per cent, then minus 0.5, then minus 0.5. Fund B returns minus 0.5, then 2.5, then minus 0.5. Fund C returns minus 0.5, minus 0.5, then 2.5. Three managers taking turns to have the one good day. Each averages 0.5 per cent, so each demeaned arrow is one day at plus 2 and two days at minus 1, in a different position."))
        .para(|p| p
            .text("Compute any pair. The day-by-day products are minus 2, minus 2 and plus 1, summing to minus 3. Each arrow's squared length is 4 plus 1 plus 1, which is 6, so the product of the two lengths is 6. The correlation is minus 3 over 6: exactly minus one half, for every pair. As an angle, exactly one hundred and twenty degrees."))
        .para(|p| p
            .text("Three arrows from a single point, each a hundred and twenty degrees from the other two. Three times a hundred and twenty is three hundred and sixty — a complete turn, used up exactly. These three funds are as mutually opposed as three things can possibly be, and the geometry says so by running out of room."))
        .note("Which answers a question every risk system eventually raises. If someone hands you a correlation matrix saying three assets are each correlated at minus 0.9 with the other two, you can reject it without computing anything. Minus 0.9 is an angle of 154.2 degrees, and three of those need 462 degrees of turning to fit around a point. There is not that much room in a turn.")
        .figure(Figure::new(NO_ROOM_SVG,
            "Correlations are angles, so they have to fit around a point. On the left, three funds correlated at minus one half: 120 degrees apiece, and the three arcs close the turn exactly, which is why minus one half is the floor for three assets. On the right, the same attempt at minus 0.9. Step 154.2 degrees round twice from arrow 1 and look at where arrow 3 has to land — in the amber gap, 51.7 degrees from arrow 1 rather than the 154.2 the matrix demands, which is a correlation of plus 0.62. The picture has run out of room, and finding that out took no arithmetic at all.")
            .width_percent(85))
        .para(|p| p
            .text("The picture is exact for three arrows, but it does not generalise — five arrows do not arrange round a point on a page. The floor for any number of assets comes instead from one line of arithmetic. Scale each fund's demeaned arrow to length 1, since correlation ignores scale, and add all of them into a single arrow. Its squared length is every pairwise product summed: each arrow with itself contributes 1, and each cross pair contributes the common correlation. For n arrows that is n, plus n times n minus 1 lots of the correlation. And a squared length cannot be negative — that is the entire input. Requiring that expression to be zero or more and rearranging gives the floor below; the 360 degrees the picture ran out of was this inequality wearing a protractor."))
        .para(|p| p
            .text("So correlations are not free to be whatever a spreadsheet contains — they have to be geometrically consistent, because they are angles between arrows that actually exist. For three assets sharing one common correlation, the floor is exactly minus one half, the cosine of a third of a turn. For ")
            .math("n")
            .text(" assets it is"))
        .display(r"\rho \ge -\frac{1}{n - 1}")
        .explain(r"\rho", "The common correlation",
            "The single correlation shared by every pair, when a whole book is modelled with one number. Greek rho is the conventional letter for a correlation in a model, as against r for one computed from data.")
        .explain(r"-\frac{1}{n - 1}", "The floor for n assets",
            "How negative a shared correlation can be before no set of arrows could produce it. Minus one half for three assets, minus a third for four, and closing on zero as the book grows — with many assets, they simply cannot all disagree with each other. At the floor itself the scaled arrows sum to exactly nothing, which in portfolio terms means some combination of the assets has zero variance: a perfect hedge. Below the floor, some portfolio would need negative variance, which is a length squared coming out negative.")
        .para(|p| p
            .text("Read the formula for a large book and it says something genuinely useful about diversification. With many assets the floor climbs towards zero, so a large portfolio cannot consist of assets that all hedge one another. There is not enough room in any number of dimensions for a thousand arrows to all point away from each other. Some common direction always survives — which is the geometric statement of why a market factor exists at all, and why broad diversification reduces risk without ever eliminating it."))
        .note("Apply the same no-negative-variance test to every weighted combination of the arrows, not just the equal-weight one, and you have rediscovered what a risk system means when it demands a correlation matrix be positive semi-definite. No eigenvalues required — just the rule that no portfolio can have negative variance.")
}

fn where_pi_shows_up_on_a_desk(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Where pi shows up on a trading desk")
        .para(|p| p
            .text("Nothing about an option is round, and nothing about a distribution of returns is round, yet pi is inside every option price on every screen. Following it home is the best possible demonstration of the claim made back in the third section — that pi appears exactly where something is going round, even when the roundness is buried."))
        .para(|p| p
            .text("An option price is written in terms of the standard normal distribution, whose bell curve carries a constant out in front:"))
        .display(r"\frac{1}{\sigma (2\pi)^{1/2}}")
        .explain(r"\frac{1}{\sigma (2\pi)^{1/2}}", "The normal distribution's constant",
            "The number the bell curve is divided by so that the total probability under it comes to exactly 1. The square root of two pi is about 2.5066. It is not a modelling choice — it is forced, as the only value that makes the area come out right. The 2.5066 has a provenance: squaring the area turns it into a ring-by-ring volume whose rings each contribute a full lap of two pi, and the volume comes to exactly two pi. The sigma then arrives by plain rescaling — stretching the curve by sigma stretches the area by sigma. And the squared area being a volume is itself one more definition down: add the volume up column by column, and the strip over each across-value holds the first curve's height there times the second curve's whole area — so the total is one area times the other.")
        .para(|p| p
            .text("That constant is not chosen for convenience or fitted to anything. It is forced: it is the one number that makes the area under the curve come to exactly 1, as any probability distribution must. So the question is why the area under a bell curve should involve pi at all."))
        .para(|p| p
            .text("The answer is in how the area is computed. The area under the bell curve cannot be found by ordinary integration — there is no formula for the antiderivative. So instead you square it: multiply the integral by a copy of itself in a second variable, which turns a length-like quantity into a volume over a flat plane. Now watch what the bell shape does. The height of that surface is the product of two exponentials, and multiplying exponentials adds their exponents — the first rule of the exponents lesson — so the combined exponent is minus half of across squared plus up squared. That sum is the squared distance from the origin, by Pythagoras. The surface's height depends on where you stand only through how far out you stand: it is perfectly circularly symmetric, and the bell curve is precisely the shape that converts Pythagoras' sum into a product."))
        .para(|p| p
            .text("Circular symmetry is what makes the volume computable, and it is where the pi gets in. Add the surface up ring by ring instead of square by square: the ring at a given distance has a circumference of two pi times that distance, so every ring contributes one full lap — and, unlike the original curve, the ring-by-ring sum has an elementary antiderivative. The volume comes out at exactly two pi. The volume is the square of the area, so the area is the square root of two pi, about 2.5066, and that is the number the curve must be divided by. Every digit of it is one lap, counted once per ring."))
        .note("So the roundness was there all along, hiding one dimension up — and it is a property, not a trick. The bell curve is the only density whose two-dimensional product depends solely on distance from the origin: ask for two coordinates that are independent of each other and jointly have no preferred direction, and the Gaussian is forced, which is how Maxwell first derived it for the velocities of gas molecules. Pi is not the receipt for a clever detour. It is in the constant because circular symmetry is the normal distribution's defining property.")
        .rule()
        .para(|p| p
            .text("Here is the same constant in a form you can use directly. For a normally distributed quantity, the average size of a move ignoring its sign is not the standard deviation — it is a fixed fraction of it:"))
        .display(r"E|X| = \sigma (2/\pi)^{1/2}")
        .explain(r"E|X|", "The expected absolute move",
            "The average size of a move, sign discarded — the mean absolute deviation. It answers \"how big is a typical day\" in a way a standard deviation does not, because it is not inflated by squaring.")
        .explain(r"\sigma (2/\pi)^{1/2}", "Four fifths of the standard deviation",
            "The constant is the square root of two over pi, about 0.7979. Under a normal distribution the average absolute move is always this same fraction of the standard deviation, whatever the scale. And it is the same pi as the lap in the normalising constant, arriving by no other route: the average absolute move integrates elementarily, leaving twice the reciprocal of the square root of two pi — which is the square root of two over pi. That is also why the ratio is one fixed number for every normal distribution, and why it breaks when the tails are not normal.")
        .para(|p| p
            .text("The constant is about 0.798, so the average absolute daily move is roughly four fifths of the daily standard deviation. Turn it round and multiplying an average absolute move by 1.2533 — the square root of pi over two — restates it on a standard-deviation footing."))
        .para(|p| p
            .text("Put it on something. Take a stock quoted at 16 per cent annualised volatility, with 252 trading days in the year. The square root of 252 is 15.87, near enough 16 — which is where the desk habit of dividing an annual volatility by 16 comes from — so the daily standard deviation is about 1 per cent. Multiply by 0.798 and the typical day is 0.8 per cent: on a $100 share, a standard deviation of a dollar but a typical day of 80 cents, and the 20-cent gap between those two numbers is pi."))
        .para(|p| p
            .text("The conversion runs backwards just as usefully. Take a year of a fund's daily moves, throw away the signs, and suppose the average comes to 0.9 per cent. Multiply by 1.2533 and the implied standard deviation is 1.13 per cent, which annualises back through the same factor of 16 to about 18 per cent. That is a volatility estimate that never squared anything — which is the point of it, because it is far less at the mercy of the single worst day in the sample."))
        .para(|p| p
            .text("Anyone who has compared a \"typical daily move\" figure to a volatility figure and found them stubbornly different has met this constant without being introduced. The gap between the two is not a data error and not a definitional quibble. It is pi."))
        .note("With one honest caveat that matters more than the constant does. That ratio assumes normality, and daily returns are not normal — they have fatter tails. Rare large days inflate a standard deviation faster than they inflate an average absolute move, so the empirical ratio typically comes in below 0.798. Which makes the conversion a useful sanity check in a second way: if a series' ratio is far below four fifths, that gap is itself a measurement of how fat the tails are.")
}

fn when_the_wave_story_is_a_lie(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("When the wave story is a lie")
        .para(|p| p
            .text("Everything so far has made waves look like the natural language for anything that repeats, and a lesson that stopped here would have taught a genuinely dangerous habit. So: where does this machinery legitimately apply to markets, and where does it manufacture nonsense?"))
        .para(|p| p
            .text("It applies wherever a calendar or a mechanism forces the repeat and you can name it in advance. Retail revenue peaks at Christmas because of Christmas. Intraday volume is U-shaped because exchanges open and close at fixed times and index funds trade at the close. Electricity demand follows daylight and working hours. Quarter-end and month-end flows exist because reporting dates exist; coupon and expiry dates exist because contracts say so. In every case the period is known before you look at any data, which is the property that makes the model honest — you are not discovering a cycle, you are measuring the size of one whose existence and timing were never in question."))
        .para(|p| p
            .text("Asset returns themselves are a different matter, and the difference is not a matter of degree. Their frequency content is close to flat: no period carries much more of the variation than any other, which is the technical content of saying returns are close to white noise. That is roughly what you would expect of a liquid market — a reliable repeat is a standing invitation to trade against it, and trading against it removes it."))
        .rule()
        .para(|p| p
            .text("And there is a specific trap here that has caught serious people, because it manufactures cycles out of nothing at all. In the 1920s Eugen Slutsky showed that if you take a stream of purely random numbers and replace each one with a moving average of its neighbours, the result does not look random. It looks like a business cycle: smooth, rolling waves of a fairly consistent period, with no cause whatsoever. He was able to match such a manufactured series against a real economic index and have it pass for the genuine article."))
        .para(|p| p
            .text("The mechanism is simple once stated. A moving average is a filter, and every filter treats some periods differently from others — it damps the fast wiggles hard and the slow ones barely at all. Run it over noise, which contains a little of everything, and what survives is the band the filter happened to favour. That surviving band looks exactly like a signal, because looking like a signal is what having a preferred frequency means."))
        .para(|p| p
            .text("You do not have to take this on trust, because the filter's preference is made of this lesson's own sines. Average w consecutive values of a wave of period P and the wave survives scaled by the sine of pi w over P, divided by w times the sine of pi over P. So a ten-day average kills a ten-day wave stone dead — the numerator is the sine of pi, which is zero — while a forty-day wave sails through at 90 per cent of full strength: the numerator is the eighth-turn value 0.7071, the denominator is, by the small-angle rule, near enough pi over 4, which is 0.7854, and the ratio is 0.90. So what a ten-day average leaves standing of noise is slow swells a few windows long. Widen the window to forty, touching not one draw, and the forty-day wave you were passing at 90 per cent is annihilated in its turn, while a 160-day wave becomes the 90-per-cent survivor. The period you would have quoted was never in the data. It was the window, wearing the data as a disguise. Here is the experiment performed rather than described — two minutes in a spreadsheet reproduces it:"))
        .figure(Figure::new(SLUTSKY_SVG,
            "Slutsky's demonstration, computed rather than sketched. The top strip is 300 draws from a fixed seeded random generator — by construction there is no cycle in them, and their sign flips 155 times. The middle strip replaces each draw by the average of its last ten: rolling waves appear, with 44 sign changes and a rhythm a chartist would happily name. The bottom strip averages the last forty instead: 4 sign changes, waves an order of magnitude slower. Not one of the 300 draws changed between the panels. The cycle and its period belong entirely to the smoother, and the period you would have quoted is a property of the window you chose.")
            .width_percent(90))
        .note("Which gives one rule worth more than the rest of this section: never smooth a series and then go looking for cycles in the smoothed version. Whatever you find will be a property of the smoother. And this is not an exotic mistake — most technical indicators are moving averages, and most cycle-hunting is done on their output. The 200-day average that anchors so many charts kills a 200-day cycle exactly and passes a three-year swell at nine tenths strength: any rhythm read off it lives at multi-year periods by construction.")
        .para(|p| p
            .text("So the working discipline. Fit a seasonal term when you can name the mechanism and state the period before fitting; then the model is only estimating an amplitude and a timing, and the tools in this lesson are exactly right for the job. If instead you searched a range of periods and kept the best-looking one, you have not found a cycle — you have found the maximum of a random field, and it will be somewhere different next year. Test it: generate a random series of the same length, run the identical search, and see how good the best cycle looks there. If it looks about as convincing, yours was noise. And prefer, always, the seasonality you could have written down before the data arrived."))
}

fn turning_as_multiplication(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("One floor down: turning as multiplication")
        .para(|p| p
            .text("There is a level beneath everything in this lesson, and it is worth a look even if you never use it, because it explains why the addition formulas had to be true rather than merely being true."))
        .para(|p| p
            .text("Start with a way of reading negative numbers that most people are never offered. Multiplying by minus 1 turns a number through half a lap: 3 becomes minus 3, on the other side of zero, at the same distance. Do it twice and you are back where you started, which is why minus times minus is plus. Multiplication by a number can be a rotation."))
        .para(|p| p
            .text("Now ask for the number that performs a quarter lap. Doing it twice must give a half lap — that is, must give multiplication by minus 1 — so this number, squared, is minus 1. No ordinary number does that, so mathematics names a new one:"))
        .display(r"i^2 = -1")
        .explain(r"i^2", "i squared",
            "The imaginary unit, multiplied by itself. Read as a rotation, i is a quarter turn, and doing a quarter turn twice is a half turn — which is multiplication by minus 1. The defining equation is that reading, written down. Two honesties keep the picture true: it is the whole plane that pivots about zero, not each number turning on its own spot — one click carries the point at 3 along an arc of radius 3 — and only numbers at distance 1 from zero are pure turns: multiply by 2i and everything doubles as it quarter-turns. A complex number is a turn and a stretch; i is the one that stretches by nothing.")
        .para(|p| p
            .text("Read that way, ")
            .math("i")
            .text(" stops being mysterious: it is the quarter turn. Count it like a ratchet — 1, then i, then minus 1, then minus i, then back to 1: four identical clicks to a lap — and everything else about complex numbers follows from taking that seriously."))
        .para(|p| p
            .text("With rotation available as multiplication, the point on the unit circle at angle theta is the number cosine theta plus ")
            .math("i")
            .text(" times sine theta — across, plus a quarter turn's worth of up. And that combination turns out to be exactly what the exponential function does with an imaginary exponent:"))
        .display(r"e^{i\theta} = \cos\theta + i\sin\theta")
        .explain(r"e^{i\theta}", "The unit circle, as an exponential",
            "Euler's formula. Read the exponential's usual meaning of steady growth, and an imaginary exponent turns steady growth sideways into steady turning: the result travels round the unit circle at one radian per unit of theta instead of growing along a line. Why it must: the exponential is defined by its growth rule — its rate of change is itself times the growth rate — and with rate i the velocity is always the position given a quarter turn. No part of the motion points outward, so the distance from the centre stays 1, and all the speed runs along the rim, covering arc — which is angle — at exactly one radian per unit.")
        .explain(r"i\sin\theta", "The up-part, turned",
            "The up-coordinate, multiplied by the quarter turn — which is what places it on the vertical axis rather than the horizontal one.")
        .para(|p| p
            .text("Now watch the section on turning twice evaporate. Turning by one angle and then another is multiplying the two corresponding numbers, and multiplying two exponentials adds their exponents — the first rule the exponents lesson establishes, and the only one needed here. So the angle-addition formulas are nothing but that rule, with the across-parts and up-parts separated afterwards. They were never facts about triangles. They were the rule that exponents add, wearing coordinates."))
        .para(|p| p
            .text("And at half a lap, where the angle is pi, the point on the circle is the far left: exactly minus 1. Which gives the most-quoted equation in mathematics, and it says something entirely concrete — turn the number 1 through half a lap and you land on minus 1."))
        .display(r"e^{i\pi} = -1")
        .explain(r"e^{i\pi}", "One, turned through half a lap",
            "Euler's formula at an angle of pi. The claim reads as mystical and is not: half a lap round the unit circle from the starting point is the far left of the circle, which is minus 1.")
        .figure(Figure::new(EULER_SVG,
            "The final section in one picture. Multiplying by i is a quarter turn, so the ladder from 1 to i to minus 1 to minus i is three quarter-turns, and two of them make multiplication by minus 1 — the defining equation of i, drawn. The moving point — placed at 50 degrees, the same angle as this lesson's very first figure — is cosine theta across plus i times sine theta up, which is Euler's formula read as coordinates: the exponential with an imaginary exponent is the unit-circle point, renamed. And at half a lap the point is the far left of the circle, which is the number minus 1. The most-quoted equation in mathematics is a location.")
            .width_percent(60))
        .note("This is the doorway rather than the room. Through it are Fourier analysis, signal processing, the characteristic functions that price options in models with no formula for their distribution, and the eigenvalues that make a system oscillate — all of which are this lesson's contents, taken seriously in the complex plane.")
}

fn practice(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Practice")
        .para(|p| p
            .text("Eight questions, all built from this lesson's own material. After the first, which is worked in full, each one stops and invites you to commit to an answer before the working appears — take the invitation. A guess you have written down is what makes the answer stick; a guess you skipped past leaves nothing behind."))
        .para(|p| p
            .text("First, one worked in full. A point on the unit circle has an across-coordinate of 0.6 and sits above the axis. What are its sine and its tangent?"))
        .para(|p| p
            .text("The across-coordinate is the cosine, so cosine theta is 0.6. The point is on the circle, so its coordinates satisfy across squared plus up squared equals 1 — that is Pythagoras, and it is the only fact needed. So the up-coordinate squared is 1 minus 0.36, which is 0.64, and the up-coordinate is 0.8 or minus 0.8. Above the axis picks the positive one, so the sine is 0.8. The tangent is up over across, 0.8 over 0.6, which is four thirds, about 1.333 — meaning the ray from the centre through this point climbs 4 for every 3 it runs. Note how little was memorised: one circle equation and one ratio."))
        .rule()
        .para(|p| p
            .text("Now one with the first half already done. A seasonal regression on monthly data returns a cosine weight of 8 and a sine weight of 6, on top of the level. Read the two weights as a point, exactly as the turning-twice section did: the height of the single wave they add up to is the distance of that point from the origin, the square root of 64 plus 36, which is the square root of 100, which is 10. So the season swings 10 either side of the level, a peak-to-trough range of 20. That is the height. The peak month is yours."))
        .note("You need one thing only: the angle whose cosine is 0.8 and whose sine is 0.6. Turn it into a month before reading on.")
        .para(|p| p
            .text("The peak sits at the angle of the weight pair. Dividing both weights by the height gives a cosine of 0.8 and a sine of 0.6, both positive, so the angle is in the first quarter of the lap: about 36.9 degrees. A calculator's arccos of 0.8 says the same 36.9 — but could not alone have said which side of the axis, because cosine reaches 0.8 twice per lap, at plus and minus 36.9 degrees, and arccos only ever answers from the top half; it is the sine's positive sign that completes the answer, the same sign-check the seasonal small print automates with the two-argument arctangent. As a fraction of a full lap that is 36.9 over 360, which is 0.102, and 0.102 of a twelve-month year is month 1.23. Months run from zero for January, so month 1 is February and the peak falls about a quarter of the way through it — early February. Check it against the model if you like: at month 1 the two terms give 6.93 and 3, adding to 9.93, and at month 2 they give 4 and 5.20, adding to 9.20 — both short of the 10 the peak reaches. Nothing here searched for a month. The month is the angle of a pair of ordinary regression weights."))
        .rule()
        .para(|p| p
            .text("Now one with no help at all, and it joins the two halves of this lesson. Two desks book seasonal revenue on the same twelve-month rhythm but different calendars: one peaks in December, the other two months later, in February. One season swings 12 million either side of its level, the other 3 million. Over a year of monthly data, demeaned, what is the correlation between the two series — and how much of the second desk's swing can the first desk's not account for?"))
        .note("Commit to a number before reading on — and decide first whether the 12 and the 3 matter.")
        .para(|p| p
            .text("The sizes do not matter: dividing by both arrows' lengths is precisely what strips them out, so this is two unit arrows at two seasonal angles. Each month contributes the product of the two waves' values, and over a whole year the piece of that product that depends on which month it is walks twice round the circle and sums to exactly nothing, leaving only the piece set by the gap between the peaks — the subtraction formula doing its work. The gap is two months, a sixth of a lap, sixty degrees, so the correlation is the cosine of sixty degrees: exactly 0.5, whatever the two sizes. The part the first desk cannot account for is the perpendicular share, the sine of the same angle: 0.866 of the movement, which in variance terms — squares, because variance is a squared length — leaves only 25 per cent explained. \"Correlation 0.5\" is a much weaker statement than it sounds, even between two perfectly deterministic seasons. And slide the second peak one more month, to a quarter lap: the correlation is the cosine of ninety degrees, zero. Uncorrelated means perpendicular, not unrelated — both series are pure clockwork."))
        .rule()
        .para(|p| p
            .text("A question with a tempting wrong answer. A quarter turn has a sine of exactly 1. What, then, is the sine of a quarter turn plus a quarter turn?"))
        .note("Write a number down before reading on. If it came quickly, it is probably the wrong one.")
        .para(|p| p
            .text("The tempting answer is 2: one for each quarter turn, added. It feels right because every other bracket in school algebra opens that way, and the notation for the sine of a sum looks exactly like a multiplier sitting in front of a bracket. It is also impossible, and this lesson said so two sections in: there is no point on a circle of radius 1 sitting two units above the centre, so no angle has a sine of 2. An answer outside minus 1 to 1 is the signature of a sine treated as a multiplier."))
        .para(|p| p
            .text("The true answer is 0. A quarter turn plus a quarter turn is a half turn, and the point after a half lap is at the far left of the circle: across minus 1, up 0. The double-angle formula agrees, and says exactly where the tempting 2 came from: the sine of twice an angle is twice the sine times a correction of cosine theta. Near zero the correction is nearly 1 and doubling nearly works — sin 0.2 over sin 0.1 is 1.99 — which is why the instinct exists. Here the angle is a quarter turn, the correction is the cosine of ninety degrees, which is zero, and the tempting 2 is corrected all the way down to the true 0. The instinct fails maximally at exactly the angle the question chose, because a sine is a coordinate read off a turn, not a factor applied to an angle."))
        .rule()
        .para(|p| p
            .text("A colleague smooths five years of daily returns with a 40-day moving average and finds smooth rolling waves — a cycle of a few months, consistent across the chart. As a check they re-run it with a 10-day average; the waves are still there, only faster. They take the second chart as confirmation: the pattern survived a change of method. Should they?"))
        .note("Accept or reject before reading on — and say what the second chart would have had to show to count as confirmation.")
        .para(|p| p
            .text("The tempting answer is yes, and it feels right for a good reason: results that survive a change of method usually are the real ones, and that instinct is sound in general. It is wrong here because the one thing that did not survive is the thing being claimed — the period. The waves sped up when the window shrank, which is the Slutsky signature: a moving average damps the fast wiggles and spares the band it favours, so run over anything noise-like it manufactures rolling waves whose period belongs to the window. Confirmation would have been the opposite chart: the same period at both windows. The honest tests are the ones the wave-story section gave — state the period before looking, and run the identical smooth-and-look on generated noise of the same length; if that looks about as convincing, the cycle was the smoother's."))
        .rule()
        .para(|p| p
            .text("A colleague reworks a chart, and the same price series that looked like a gentle drift now looks like a vertical ascent. Nothing about the data changed. Has the trend got steeper, and what number would settle it?"))
        .note("Answer yes or no, and name the one number that settles it, before reading on.")
        .para(|p| p
            .text("No. What changed is the angle, and the angle is not a property of the series — it is the arctangent of the slope multiplied by whatever ratio the axis scales are in. Stretching the vertical axis by twenty times turns a slope of 0.1, which draws at about 5.7 degrees, into an apparent slope of 2, which draws at about 63 degrees. The number that settles it is the slope itself in the units of the data — units per day — which is unchanged. Read the axis labels, not the picture."))
        .rule()
        .para(|p| p
            .text("A risk system reports a fund's daily standard deviation as 1.6 per cent. A trader who watches the same fund all day says a typical move is about 1.2 per cent. Both numbers come from the same returns. Is one of them wrong?"))
        .note("Decide before reading on — and if you say neither, produce the number that reconciles them.")
        .para(|p| p
            .text("Neither has to be wrong, because the two are not measuring the same thing, and the constant separating them is pi. Under a normal distribution the average size of a move, sign discarded, is the standard deviation times the square root of two over pi, about 0.798. A standard deviation of 1.6 therefore predicts a typical day of 1.28 per cent — much nearer the trader's 1.2 than the system's 1.6. The observed ratio is 1.2 over 1.6, which is 0.75, a little under the 0.798 normality would give, and that shortfall is worth reading rather than dismissing: rare large days inflate a standard deviation faster than they inflate an average absolute move, so a ratio below four fifths is a measurement of fat tails, not an error. Run it the other way as a check: the trader's 1.2 times 1.2533 gives 1.50, the standard deviation a typical day of 1.2 would imply under normality, against the 1.6 observed."))
        .rule()
        .para(|p| p
            .text("Last one, and it is a diagnostic rather than a calculation. A risk model is handed a correlation matrix for three assets in which every pair is correlated at minus 0.8. Every entry sits comfortably inside the range a correlation is allowed to take. Should the model accept it?"))
        .note("Accept or reject, before reading on. The check needs no linear algebra, only a protractor.")
        .para(|p| p
            .text("The tempting answer is yes, and it is tempting for a good reason: minus 0.8 is a perfectly legal correlation for any single pair, and a table made entirely of legal numbers looks as though it must be a legal table. The flaw is that the entries are not independent claims. They are angles between three arrows that all have to exist at the same time, in the same space, from the same point — so they constrain one another."))
        .para(|p| p
            .text("So reject it. Minus 0.8 is an angle of about 143.1 degrees, and three arrows each 143.1 degrees from the other two would need 429 degrees of turning to arrange around a point — more than the 360 a full turn contains. No three sets of returns can do it. The most negative a shared three-way correlation can be is minus one half, at which the three angles are 120 degrees each and use up the turn exactly. Someone has typed these numbers in rather than measured them."))
}

fn letter_overrides(b: LessonBuilder) -> LessonBuilder {
    b.explain_char('θ', "The angle",
        "How far round the circle the point has turned, measured anticlockwise from the far right. In radians unless a section says otherwise. In the correlation section it is the angle between two funds' return arrows.")
        .explain_char('π', "Pi",
            "The number of radius-lengths in half a turn, about 3.14159 — so a full lap is two pi radians. It appears in any formula where something is going round, including ones with no visible circle in them.")
        .explain_char('φ', "The phase",
            "Where in its cycle a wave stands when the clock starts, in radians. It slides the whole wave sideways and changes nothing else. In the identity for combining a cosine and a sine, it is the angle of the pair of weights.")
        .explain_char('α', "The first turn",
            "The angle turned through first, in the addition formulas.")
        .explain_char('β', "The second turn",
            "The angle turned through after alpha, in the addition formulas. Not the finance beta, which appears in this lesson only as the word.")
        .explain_char('A', "The amplitude, or the cosine weight",
            "Half the distance from trough to peak of a wave — the radius of the circle being walked. In the seasonal model it is the weight on the cosine column.")
        .explain_char('B', "The sine weight",
            "The weight on the sine column in the seasonal model. Paired with A, it is what lets the peak sit in any month.")
        .explain_char('C', "The combined height",
            "The height of the single wave that a weighted cosine plus a weighted sine adds up to: the distance from the origin to the point whose coordinates are the two weights.")
        .explain_char('L', "A length",
            "The length of the sloping side of a triangle, or of an arrow — the factor the whole unit-circle picture is scaled by.")
        .explain_char('P', "The period",
            "How long one full cycle takes, in the units of the clock: twelve for months in a year, twenty-four for hours in a day.")
        .explain_char('R', "Revenue",
            "The retailer's monthly revenue in millions, in the seasonal model.")
        .explain_char('m', "The month",
            "Which month it is, numbered from zero for January through eleven for December.")
        .explain_char('t', "Time",
            "The clock driving the point round the circle, in whatever units the period is measured in.")
        .explain_char('x', "The first return arrow",
            "One fund's demeaned daily returns, read as a single arrow with one coordinate per day. In the small-angle section it is instead a plain angle in radians.")
        .explain_char('y', "The second return arrow",
            "The other fund's demeaned daily returns, read as an arrow. In the wave section it is instead the height of the wave.")
        .explain_char('r', "The correlation",
            "Pearson's correlation coefficient computed from data — and, identically, the cosine of the angle between the two demeaned return arrows.")
        .explain_char('n', "How many assets",
            "The number of assets sharing a common correlation, which sets how negative that correlation is allowed to be.")
        .explain_char('i', "The quarter turn",
            "The imaginary unit: the number whose square is minus 1, read as a quarter turn about the origin. Used in this lesson only in the turning-as-multiplication section.")
        .explain_char('X', "A random quantity",
            "A normally distributed move — a daily return, in the desk section.")
        .explain_char('1', "Digit one",
            "The number one: the radius of the unit circle, and therefore the largest value a sine or cosine can take.")
        .explain_char('2', "Digit two",
            "The number two. As a superscript it means squared; in front of pi it makes a full lap out of a half one.")
}

/// [fig 1] The unit circle with the point at 50 degrees. Centre (310, 200),
/// radius 130, so the point sits at (393.6, 100.4) and the angle arc, drawn at
/// radius 40, runs from (350, 200) to (335.7, 169.4).
const CIRCLE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 400" font-family="sans-serif" font-size="12">
<rect x="0" y="0" width="620" height="400" rx="8" fill="#f8fafc"/>
<text x="310" y="26" fill="#64748b" text-anchor="middle">the unit circle: cosine is across, sine is up</text>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M150 200H490M310 55V345"/></g>
<circle cx="310" cy="200" r="130" fill="none" stroke="#94a3b8" stroke-width="1.6"/>
<path d="M310 200 L393.6 200 L393.6 100.4 Z" fill="#bfdbfe" fill-opacity="0.55" stroke="none"/>
<path d="M385.6 200 L385.6 192 L393.6 192" fill="none" stroke="#64748b" stroke-width="1"/>
<path d="M310 200 L393.6 100.4" stroke="#b45309" stroke-width="2.4" fill="none"/>
<path d="M393.6 200 L393.6 100.4" stroke="#2563eb" stroke-width="2.6" fill="none"/>
<path d="M310 200 L393.6 200" stroke="#16a34a" stroke-width="2.6" fill="none"/>
<path d="M350 200 A 40 40 0 0 0 335.7 169.4" fill="none" stroke="#b45309" stroke-width="1.6"/>
<circle cx="393.6" cy="100.4" r="5.5" fill="#b45309"/>
<circle cx="440" cy="200" r="4" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="333" y="192" fill="#b45309" font-size="13" text-anchor="end">&#952;</text>
<text x="351" y="219" fill="#16a34a" text-anchor="middle">cos &#952;</text>
<text x="404" y="154" fill="#2563eb">sin &#952;</text>
<text x="340" y="141" fill="#b45309" text-anchor="middle">1</text>
<text x="404" y="92" fill="#334155">(cos &#952;, sin &#952;)</text>
<text x="440" y="222" fill="#94a3b8" text-anchor="middle">start</text>
<text x="497" y="196" fill="#94a3b8">across</text>
<text x="310" y="48" fill="#94a3b8" text-anchor="middle">up</text>
<text x="310" y="378" fill="#64748b" text-anchor="middle">every value either function ever takes is a coordinate of this one point, somewhere on this one circle</text>
</svg>"##;

/// [fig 5] Circle on the left unrolled into a sine wave on the right. The
/// circle is centred (110, 160) with radius 80; the wave runs px 210 to 610
/// for one full lap, baseline 160, amplitude 80. The marked angle is 60
/// degrees, which puts both the circle point and the wave point at y = 90.7.
const WAVE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 320" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="640" height="320" rx="8" fill="#f8fafc"/>
<text x="320" y="24" fill="#64748b" text-anchor="middle">the same walk, twice: round the circle, and unrolled against the angle</text>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M20 160H195M110 70V250"/></g>
<circle cx="110" cy="160" r="80" fill="none" stroke="#94a3b8" stroke-width="1.6"/>
<path d="M110 160 L150 90.7" stroke="#b45309" stroke-width="2.2" fill="none"/>
<path d="M150 160 L150 90.7" stroke="#2563eb" stroke-width="2.4" fill="none"/>
<circle cx="150" cy="90.7" r="5" fill="#b45309"/>
<text x="128" y="152" fill="#b45309" font-size="12" text-anchor="end">&#952;</text>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M210 160H620"/></g>
<g stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 4" fill="none"><path d="M210 80H620M210 240H620"/></g>
<path d="M 210.0 80.0 L 218.3 80.7 L 226.7 82.7 L 235.0 86.1 L 243.3 90.7 L 251.7 96.5 L 260.0 103.4 L 268.3 111.3 L 276.7 120.0 L 285.0 129.4 L 293.3 139.3 L 301.7 149.6 L 310.0 160.0 L 318.3 170.4 L 326.7 180.7 L 335.0 190.6 L 343.3 200.0 L 351.7 208.7 L 360.0 216.6 L 368.3 223.5 L 376.7 229.3 L 385.0 233.9 L 393.3 237.3 L 401.7 239.3 L 410.0 240.0 L 418.3 239.3 L 426.7 237.3 L 435.0 233.9 L 443.3 229.3 L 451.7 223.5 L 460.0 216.6 L 468.3 208.7 L 476.7 200.0 L 485.0 190.6 L 493.3 180.7 L 501.7 170.4 L 510.0 160.0 L 518.3 149.6 L 526.7 139.3 L 535.0 129.4 L 543.3 120.0 L 551.7 111.3 L 560.0 103.4 L 568.3 96.5 L 576.7 90.7 L 585.0 86.1 L 593.3 82.7 L 601.7 80.7 L 610.0 80.0" fill="none" stroke="#16a34a" stroke-width="2" stroke-dasharray="6 4"/>
<path d="M 210.0 160.0 L 218.3 149.6 L 226.7 139.3 L 235.0 129.4 L 243.3 120.0 L 251.7 111.3 L 260.0 103.4 L 268.3 96.5 L 276.7 90.7 L 285.0 86.1 L 293.3 82.7 L 301.7 80.7 L 310.0 80.0 L 318.3 80.7 L 326.7 82.7 L 335.0 86.1 L 343.3 90.7 L 351.7 96.5 L 360.0 103.4 L 368.3 111.3 L 376.7 120.0 L 385.0 129.4 L 393.3 139.3 L 401.7 149.6 L 410.0 160.0 L 418.3 170.4 L 426.7 180.7 L 435.0 190.6 L 443.3 200.0 L 451.7 208.7 L 460.0 216.6 L 468.3 223.5 L 476.7 229.3 L 485.0 233.9 L 493.3 237.3 L 501.7 239.3 L 510.0 240.0 L 518.3 239.3 L 526.7 237.3 L 535.0 233.9 L 543.3 229.3 L 551.7 223.5 L 560.0 216.6 L 568.3 208.7 L 576.7 200.0 L 585.0 190.6 L 593.3 180.7 L 601.7 170.4 L 610.0 160.0" fill="none" stroke="#2563eb" stroke-width="2.6"/>
<text x="224" y="74" fill="#16a34a" font-size="12">cos &#952;</text>
<text x="318" y="74" fill="#2563eb" font-size="12">sin &#952;</text>
<path d="M150 90.7 H276.7" stroke="#b45309" stroke-width="1.2" stroke-dasharray="4 4" fill="none"/>
<path d="M276.7 90.7 L276.7 160" stroke="#94a3b8" stroke-width="1" stroke-dasharray="3 3" fill="none"/>
<circle cx="276.7" cy="90.7" r="5" fill="#b45309"/>
<g fill="#94a3b8" text-anchor="middle"><text x="210" y="182">0</text><text x="310" y="182">&#960;/2</text><text x="410" y="182">&#960;</text><text x="510" y="182">3&#960;/2</text><text x="610" y="182">2&#960;</text></g>
<text x="626" y="84" fill="#94a3b8" text-anchor="end">+1</text>
<text x="626" y="252" fill="#94a3b8" text-anchor="end">&#8722;1</text>
<text x="110" y="62" fill="#b45309" text-anchor="middle" font-size="10">the same height, carried across</text>
<text x="320" y="300" fill="#64748b" text-anchor="middle">one lap of the circle is one cycle of the wave &#8212; the wave is not a new object, it is the walk drawn against the angle</text>
</svg>"##;

/// [fig 6] Two return arrows at 60 degrees, with the perpendicular dropped.
/// Origin (60, 230); arrow A runs 220px right; arrow B runs 180px at 60
/// degrees, so its tip is (150, 74.1) and its foot on A is (150, 230).
const PROJECT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 460 300" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="460" height="300" rx="8" fill="#f8fafc"/>
<text x="230" y="26" fill="#64748b" text-anchor="middle">two funds as arrows: correlation is the cosine of the angle between them</text>
<defs>
  <marker id="ah-a" markerWidth="9" markerHeight="9" refX="8" refY="4.5" orient="auto"><path d="M0 0 L9 4.5 L0 9 z" fill="#16a34a"/></marker>
  <marker id="ah-b" markerWidth="9" markerHeight="9" refX="8" refY="4.5" orient="auto"><path d="M0 0 L9 4.5 L0 9 z" fill="#2563eb"/></marker>
</defs>
<path d="M60 230 L272 230" stroke="#16a34a" stroke-width="2.6" fill="none" marker-end="url(#ah-a)"/>
<path d="M60 230 L145 82.4" stroke="#2563eb" stroke-width="2.6" fill="none" marker-end="url(#ah-b)"/>
<path d="M150 74.1 L150 230" stroke="#b45309" stroke-width="2.2" stroke-dasharray="5 4" fill="none"/>
<path d="M150 222 L142 222 L142 230" fill="none" stroke="#64748b" stroke-width="1"/>
<path d="M60 236 L150 236" stroke="#b45309" stroke-width="3" fill="none"/>
<path d="M105 230 A 45 45 0 0 0 82.5 191" fill="none" stroke="#64748b" stroke-width="1.4"/>
<text x="96" y="216" fill="#64748b" font-size="12">&#952;</text>
<text x="278" y="234" fill="#16a34a">fund A</text>
<text x="150" y="72" fill="#2563eb" text-anchor="middle">fund B</text>
<text x="105" y="253" fill="#b45309" text-anchor="middle">the part of B that A already gives you</text>
<text x="105" y="268" fill="#b45309" text-anchor="middle">length = cos &#952; &#215; |B| = 0.5 &#215; |B|</text>
<text x="163" y="150" fill="#b45309">the part A cannot give you</text>
<text x="163" y="165" fill="#b45309">= sin &#952; &#215; |B| = 0.866 &#215; |B|</text>
<text x="230" y="290" fill="#64748b" text-anchor="middle">at &#952; = 60&#176;, r = cos &#952; = 0.5 &#8212; and the two pieces are at right angles, so their squares add</text>
</svg>"##;

/// [fig 2] Tangent as a height on the tangent line. Centre (140, 250), radius
/// 110, so the tangent line is x = 250 and one unit of run is 110px. At 50
/// degrees the circle point is (210.7, 165.7) and the ray, extended, strikes
/// the line at 110 * tan 50 = 131.09 above centre, at (250, 118.9). The pale
/// ray is 80 degrees, whose strike at 110 * 5.6713 is far off the canvas; it
/// is drawn to its exit point (176.7, 42). The angle arc is radius 40.
const TANGENT_LINE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 520 400" font-family="sans-serif" font-size="12">
<rect x="0" y="0" width="520" height="400" rx="8" fill="#f8fafc"/>
<text x="260" y="26" fill="#64748b" text-anchor="middle">tangent: the height at which the ray strikes the line that grazes the circle</text>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M25 250H310M140 130V370"/></g>
<circle cx="140" cy="250" r="110" fill="none" stroke="#94a3b8" stroke-width="1.6"/>
<path d="M250 60V350" stroke="#94a3b8" stroke-width="1.6" fill="none"/>
<path d="M140 250 L176.7 42" stroke="#cbd5e1" stroke-width="2" fill="none"/>
<path d="M140 250 L250 118.9" stroke="#b45309" stroke-width="2.4" fill="none"/>
<path d="M140 250 L250 250" stroke="#16a34a" stroke-width="2.6" fill="none"/>
<path d="M250 250 L250 118.9" stroke="#2563eb" stroke-width="3" fill="none"/>
<path d="M242 250 L242 242 L250 242" fill="none" stroke="#64748b" stroke-width="1"/>
<path d="M180 250 A 40 40 0 0 0 165.7 219.4" fill="none" stroke="#b45309" stroke-width="1.6"/>
<circle cx="210.7" cy="165.7" r="5" fill="#b45309"/>
<circle cx="250" cy="118.9" r="5.5" fill="#2563eb"/>
<circle cx="250" cy="250" r="4" fill="#f8fafc" stroke="#94a3b8" stroke-width="2"/>
<text x="188" y="234" fill="#b45309" font-size="13">&#952;</text>
<text x="195" y="268" fill="#16a34a" text-anchor="middle">run = 1</text>
<text x="258" y="190" fill="#2563eb">tan &#952;</text>
<text x="258" y="114" fill="#2563eb">1.19 up, so tan 50&#176; = 1.19</text>
<text x="168" y="64" fill="#94a3b8" font-size="11" text-anchor="end">at 80&#176;: 5.67, off the top</text>
<text x="258" y="330" fill="#64748b" font-size="11">the tangent line: it grazes the circle</text>
<text x="258" y="345" fill="#64748b" font-size="11">at the start, and nowhere else</text>
<text x="260" y="374" fill="#64748b" font-size="11" text-anchor="middle">the ray runs exactly 1 across, so it rises by exactly its own slope</text>
<text x="260" y="389" fill="#64748b" font-size="11" text-anchor="middle">at a quarter turn the ray is parallel to the line and never strikes at all</text>
</svg>"##;

/// [fig 3] The same 10-units-over-100-days series at two vertical scales. Both
/// panels run 110px for 100 days. The left axis gives 110px to 100 units, so
/// the 10-unit rise draws as 11px: slope 0.1, arctan 5.7106 degrees. The right
/// axis gives 220px to 10 units, so the same rise draws as 220px: slope 2,
/// arctan 63.4349 degrees. Angle arcs are radius 55 in both panels.
const AXIS_SCALE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 520 380" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="520" height="380" rx="8" fill="#f8fafc"/>
<text x="260" y="26" fill="#64748b" text-anchor="middle">the same series twice: nothing changed but the span of the vertical axis</text>
<g stroke="#94a3b8" stroke-width="1.4" fill="none"><path d="M75 190V300H190"/><path d="M335 80V300H450"/></g>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M71 245H75M71 190H75M331 190H335M331 80H335"/></g>
<path d="M75 300 L185 289" stroke="#2563eb" stroke-width="2.6" fill="none"/>
<path d="M335 300 L445 80" stroke="#2563eb" stroke-width="2.6" fill="none"/>
<path d="M130 300 A 55 55 0 0 0 129.73 294.53" fill="none" stroke="#b45309" stroke-width="1.6"/>
<path d="M390 300 A 55 55 0 0 0 359.6 250.81" fill="none" stroke="#b45309" stroke-width="1.6"/>
<g fill="#94a3b8" text-anchor="end"><text x="69" y="304">0</text><text x="69" y="249">50</text><text x="69" y="194">100</text>
<text x="329" y="304">0</text><text x="329" y="194">5</text><text x="329" y="84">10</text></g>
<g fill="#94a3b8" text-anchor="middle"><text x="75" y="316">0</text><text x="185" y="316">100 days</text><text x="335" y="316">0</text><text x="445" y="316">100 days</text></g>
<text x="140" y="285" fill="#b45309" text-anchor="middle">5.7&#176;</text>
<text x="398" y="252" fill="#b45309">63.4&#176;</text>
<text x="130" y="342" fill="#64748b" text-anchor="middle">vertical axis spans 0 to 100</text>
<text x="390" y="342" fill="#64748b" text-anchor="middle">vertical axis spans 0 to 10</text>
<text x="260" y="366" fill="#64748b" text-anchor="middle" font-size="10">both lines gain 10 over 100 days &#8212; a slope of 0.1 a day. The slope is the fact; the angle is the rendering.</text>
</svg>"##;

/// [fig 4] The weight pair as a point. Origin (120, 130) with 18px to the unit,
/// so A = 10.392 draws as 187.06 across and B = -6 as 108 down, putting the
/// point at (307.1, 238). Its distance is 216px, which is 12 units, and its
/// angle is -30 degrees; the dashed arc is that same radius swept from the
/// across-axis, and the small arc is the angle, at radius 60.
const WEIGHT_PAIR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 470 330" font-family="sans-serif" font-size="12">
<rect x="0" y="0" width="470" height="330" rx="8" fill="#f8fafc"/>
<text x="235" y="24" fill="#64748b" text-anchor="middle">the two weights read as one point: its distance is the height, its angle is the shift</text>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M45 130H400M120 74V272"/></g>
<path d="M336 130 A 216 216 0 0 1 307.1 238" fill="none" stroke="#94a3b8" stroke-width="1.4" stroke-dasharray="5 4"/>
<path d="M120 130 L307.1 130" stroke="#16a34a" stroke-width="2.6" fill="none"/>
<path d="M307.1 130 L307.1 238" stroke="#2563eb" stroke-width="2.6" fill="none"/>
<path d="M120 130 L307.1 238" stroke="#b45309" stroke-width="2.6" fill="none"/>
<path d="M299.1 130 L299.1 138 L307.1 138" fill="none" stroke="#64748b" stroke-width="1"/>
<path d="M180 130 A 60 60 0 0 1 172 160" fill="none" stroke="#b45309" stroke-width="1.6"/>
<circle cx="307.1" cy="238" r="5.5" fill="#b45309"/>
<text x="213" y="121" fill="#16a34a" text-anchor="middle">A = 10.392</text>
<text x="302" y="204" fill="#2563eb" text-anchor="end">B = &#8722;6</text>
<text x="185" y="192" fill="#b45309" text-anchor="middle">C = 12</text>
<text x="192" y="156" fill="#b45309">&#966; = &#8722;30&#176;</text>
<text x="315" y="256" fill="#334155">the pair (10.392, &#8722;6)</text>
<text x="344" y="192" fill="#94a3b8" font-size="11">circle of radius C</text>
<text x="404" y="134" fill="#94a3b8">A</text>
<text x="114" y="64" fill="#94a3b8" text-anchor="end">B</text>
<text x="112" y="146" fill="#94a3b8" text-anchor="end">0</text>
<text x="235" y="312" fill="#64748b" text-anchor="middle">the height is Pythagoras on the two weights; the shift is the angle they make</text>
</svg>"##;

/// [fig 7] Three arrows from a point, possible and impossible. Left: centre
/// (150, 175), length 95, at 90, 210 and 330 degrees — 120 apart, three arcs
/// closing the turn exactly. Right: centre (400, 175), same length, stepping
/// 154.158 degrees (the angle whose cosine is -0.9) from 90, so the arrows sit
/// at 90, 244.158 and 398.316. Three steps need 462.47 degrees, so the third
/// arrow lands at 38.316 — only 51.68 from the first, drawn as the amber gap.
/// Shafts stop at 92 of 95 to leave room for the arrowhead.
const NO_ROOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 560 340" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="560" height="340" rx="8" fill="#f8fafc"/>
<text x="280" y="22" fill="#64748b" text-anchor="middle">a correlation is an angle, and a turn has only 360 degrees of room in it</text>
<defs><marker id="tri" markerWidth="9" markerHeight="9" refX="8" refY="4.5" orient="auto"><path d="M0 0 L9 4.5 L0 9 z" fill="#475569"/></marker></defs>
<g stroke="#475569" stroke-width="2.4" fill="none" marker-end="url(#tri)">
<path d="M150 175 L150 83"/><path d="M150 175 L70.33 221"/><path d="M150 175 L229.67 221"/>
<path d="M400 175 L400 83"/><path d="M400 175 L359.9 257.8"/><path d="M400 175 L472.18 117.96"/>
</g>
<g stroke="#94a3b8" stroke-width="1.4" fill="none">
<path d="M150 137 A 38 38 0 0 0 117.09 194"/><path d="M117.09 194 A 38 38 0 0 0 182.91 194"/><path d="M182.91 194 A 38 38 0 0 0 150 137"/>
<path d="M400 137 A 38 38 0 0 0 383.44 209.2"/><path d="M383.44 209.2 A 38 38 0 0 0 429.81 151.44"/>
</g>
<path d="M429.81 151.44 A 38 38 0 0 0 400 137" fill="none" stroke="#b45309" stroke-width="3"/>
<g fill="#94a3b8" text-anchor="middle" font-size="10">
<text x="101.5" y="151">120&#176;</text><text x="150" y="235">120&#176;</text><text x="198.5" y="151">120&#176;</text>
<text x="341.5" y="165">154.2&#176;</text><text x="446.8" y="216">154.2&#176;</text>
</g>
<text x="426.2" y="121" fill="#b45309" text-anchor="middle" font-size="10">51.7&#176;</text>
<g fill="#334155" text-anchor="middle"><text x="150" y="70">A</text><text x="62" y="236">B</text><text x="238" y="236">C</text>
<text x="400" y="70">1</text><text x="350" y="276">2</text><text x="486" y="110">3</text></g>
<text x="150" y="305" fill="#64748b" text-anchor="middle">&#8722;0.5 each: 120&#176; &#215; 3 = 360&#176;, exactly</text>
<text x="400" y="305" fill="#64748b" text-anchor="middle">&#8722;0.9 each: 154.2&#176; &#215; 3 = 462&#176;, impossible</text>
<text x="280" y="328" fill="#b45309" text-anchor="middle" font-size="10">arrow 3 lands 51.7&#176; from arrow 1, not 154.2&#176; &#8212; those three sets of returns do not exist</text>
</svg>"##;

/// [fig 8] Radians, both halves. Left: rim of the unit circle laid out in
/// radius-strings, centre (150, 190), r 120; string k ends at
/// (150 + 120 cos k, 190 - 120 sin k): 1:(214.8,89.0) 2:(100.1,80.9)
/// 3:(31.2,173.1) 4:(71.6,280.8) 5:(184.0,305.1) 6:(265.2,223.5); pi sits
/// 0.1416 into the fourth string at (30,190), and the leftover from 6 to 2pi
/// is 0.283 rad, 34px of rim. Right: centre (360, 330), r 230; the 0.4-rad
/// point is (571.8, 240.4); arc 92px against a drop of 89.6px = 230 sin 0.4.
const RADIAN_STRING_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 660 400" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="660" height="400" rx="8" fill="#f8fafc"/>
<text x="330" y="24" fill="#64748b" text-anchor="middle">the angle is the arc: count the rim in radius-strings, then look at a small angle up close</text>
<circle cx="150" cy="190" r="120" fill="none" stroke="#e2e8f0" stroke-width="5"/>
<g fill="none" stroke-width="5">
<path d="M270 190 A120 120 0 0 0 214.8 89.0" stroke="#2563eb"/>
<path d="M214.8 89.0 A120 120 0 0 0 100.1 80.9" stroke="#16a34a"/>
<path d="M100.1 80.9 A120 120 0 0 0 31.2 173.1" stroke="#2563eb"/>
<path d="M31.2 173.1 A120 120 0 0 0 71.6 280.8" stroke="#16a34a"/>
<path d="M71.6 280.8 A120 120 0 0 0 184.0 305.1" stroke="#2563eb"/>
<path d="M184.0 305.1 A120 120 0 0 0 265.2 223.5" stroke="#16a34a"/>
<path d="M265.2 223.5 A120 120 0 0 0 270 190" stroke="#b45309"/>
</g>
<path d="M150 190 L270 190" stroke="#475569" stroke-width="2" fill="none"/>
<text x="200" y="206" fill="#475569" text-anchor="middle">one radius</text>
<g fill="#334155" text-anchor="middle"><text x="222.9" y="80">1</text><text x="93.8" y="71">2</text><text x="16.3" y="175">3</text><text x="61.8" y="296">4</text><text x="188.3" y="323">5</text><text x="284" y="232">6</text></g>
<text x="278" y="186" fill="#94a3b8" font-size="10">0</text>
<circle cx="30" cy="190" r="3.5" fill="#f8fafc" stroke="#334155" stroke-width="2"/>
<text x="44" y="181" fill="#334155">half a lap: &#960;</text>
<text x="44" y="196" fill="#334155">= 3 strings + 0.1416</text>
<text x="280" y="214" fill="#b45309" font-size="10">0.283</text>
<text x="150" y="356" fill="#64748b" text-anchor="middle">six strings and a bit: a full lap is 2&#960; &#8776; 6.28</text>
<text x="490" y="218" fill="#64748b" text-anchor="middle">zoomed: a small angle, &#952; = 0.4 rad</text>
<path d="M340 330 H630" stroke="#cbd5e1" stroke-width="1" fill="none"/>
<path d="M360 330 L571.8 240.4" stroke="#94a3b8" stroke-width="1" stroke-dasharray="3 3" fill="none"/>
<path d="M420 330 A60 60 0 0 0 415.3 306.6" stroke="#475569" stroke-width="1.2" fill="none"/>
<text x="428" y="320" fill="#475569" font-size="10">&#952;</text>
<path d="M590 330 A230 230 0 0 0 571.8 240.4" stroke="#b45309" stroke-width="3.5" fill="none"/>
<path d="M571.8 240.4 L571.8 330" stroke="#2563eb" stroke-width="2.6" fill="none"/>
<path d="M571.8 322 L563.8 322 L563.8 330" stroke="#64748b" stroke-width="1" fill="none"/>
<circle cx="571.8" cy="240.4" r="4.5" fill="#b45309"/>
<text x="571.8" y="228" fill="#b45309" text-anchor="middle">the arc = the angle = 0.4000</text>
<text x="566" y="290" fill="#2563eb" text-anchor="end">the drop = sin &#952; = 0.3894</text>
<text x="490" y="356" fill="#64748b" text-anchor="middle">2.6% apart, and shrinking faster than &#952; does: sin x &#8776; x</text>
</svg>"##;

/// [fig 9] The addition formulas as one rigid turn, alpha 40, beta 30. Left
/// centre (150, 205), r 120: the 40-degree point (241.9, 127.9) built as
/// 0.766*120 = 91.9px across then 0.643*120 = 77.1px up. Right centre
/// (470, 205): turned arrows tip at (573.9, 145) and (410, 101.1); the same
/// legs ride them, (470,205) -> (549.6,159.0) -> (511.0,92.3), and the direct
/// 70-degree point (470 + 120*0.342, 205 - 120*0.940) = (511.0, 92.2)
/// confirms the landing.
const TURN_TWICE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 400" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="640" height="400" rx="8" fill="#f8fafc"/>
<text x="320" y="24" fill="#64748b" text-anchor="middle">the addition formulas are one rigid turn: the multipliers survive, only the arrows move</text>
<defs><marker id="ah-s" markerWidth="9" markerHeight="9" refX="8" refY="4.5" orient="auto"><path d="M0 0 L9 4.5 L0 9 z" fill="#475569"/></marker></defs>
<text x="150" y="50" fill="#64748b" text-anchor="middle">before: the point at 40&#176;</text>
<text x="470" y="50" fill="#64748b" text-anchor="middle">after: the whole picture turned 30&#176;</text>
<circle cx="150" cy="205" r="120" fill="none" stroke="#e2e8f0" stroke-width="1.6"/>
<circle cx="470" cy="205" r="120" fill="none" stroke="#e2e8f0" stroke-width="1.6"/>
<g stroke="#475569" stroke-width="2.2" fill="none" marker-end="url(#ah-s)">
<path d="M150 205 L270 205"/><path d="M150 205 L150 85"/>
<path d="M470 205 L573.9 145"/><path d="M470 205 L410 101.1"/>
</g>
<text x="276" y="220" fill="#475569" font-size="10">(1, 0)</text>
<text x="156" y="80" fill="#475569" font-size="10">(0, 1)</text>
<text x="634" y="168" fill="#475569" font-size="10" text-anchor="end">(0.866, 0.5)</text>
<text x="402" y="96" fill="#475569" font-size="10" text-anchor="end">(&#8722;0.5, 0.866)</text>
<path d="M150 205 L241.9 205" stroke="#16a34a" stroke-width="3" fill="none"/>
<path d="M241.9 205 L241.9 127.9" stroke="#2563eb" stroke-width="3" fill="none"/>
<circle cx="241.9" cy="127.9" r="5" fill="#b45309"/>
<text x="196" y="220" fill="#16a34a" text-anchor="middle">0.766 of &#8594;</text>
<text x="248" y="170" fill="#2563eb">0.643 of &#8593;</text>
<text x="242" y="112" fill="#334155" text-anchor="middle">(0.766, 0.643)</text>
<path d="M195 205 A45 45 0 0 0 184.5 176.1" stroke="#b45309" stroke-width="1.4" fill="none"/>
<text x="201" y="193" fill="#b45309" font-size="10">40&#176;</text>
<path d="M470 205 L549.6 159.0" stroke="#16a34a" stroke-width="3" fill="none"/>
<path d="M549.6 159.0 L511.0 92.3" stroke="#2563eb" stroke-width="3" fill="none"/>
<circle cx="511.0" cy="92.3" r="5" fill="#b45309"/>
<text x="498" y="164" fill="#16a34a" text-anchor="middle" font-size="10">same 0.766</text>
<text x="508" y="120" fill="#2563eb" font-size="10" text-anchor="end">same 0.643</text>
<text x="511" y="80" fill="#334155" text-anchor="middle">(0.342, 0.940) = the point at 70&#176;</text>
<path d="M520 205 A50 50 0 0 0 513.3 180" stroke="#b45309" stroke-width="1.4" fill="none"/>
<text x="527" y="193" fill="#b45309" font-size="10">30&#176;</text>
<text x="320" y="372" fill="#64748b" text-anchor="middle">0.766&#183;(0.866, 0.5) + 0.643&#183;(&#8722;0.5, 0.866) = (0.342, 0.940) = (cos 70&#176;, sin 70&#176;)</text>
<text x="320" y="388" fill="#64748b" text-anchor="middle">the minus sign in the cosine formula is the turned up-arrow leaning left</text>
</svg>"##;

/// [fig 10] Slutsky's manufactured cycle, computed. 300 draws from the LCG
/// x <- (1664525 x + 1013904223) mod 2^32, seed 12345, mapped to -1..1 by
/// v = 2 x / 2^32 - 1, then trailing moving averages over windows 10 and 40.
/// Layout: x = 40 + n * 580/299; strip centres y = 90, 210, 330; the raw
/// strip maps +-1 to 42px, both average strips map +-0.5 to 42px (max |MA10|
/// = 0.459, max |MA40| = 0.266, so nothing clips). Sign changes: 155/44/4.
const SLUTSKY_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 660 430" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="660" height="430" rx="8" fill="#f8fafc"/>
<text x="330" y="22" fill="#64748b" text-anchor="middle">Slutsky's demonstration: smooth pure noise, and a cycle appears</text>
<g stroke="#cbd5e1" stroke-width="1" stroke-dasharray="2 4" fill="none"><path d="M40 90H620M40 210H620M40 330H620"/></g>
<text x="40" y="40" fill="#334155">the 300 raw draws &#8212; pure noise: no cycle, no memory, no trend</text>
<path d="M 40.0 130.3 L 41.9 130.6 L 43.9 86.4 L 45.8 78.7 L 47.8 55.6 L 49.7 122.6 L 51.6 90.3 L 53.6 85.9 L 55.5 81.9 L 57.5 66.2 L 59.4 128.7 L 61.3 69.5 L 63.3 50.9 L 65.2 125.8 L 67.2 73.2 L 69.1 93.3 L 71.0 98.7 L 73.0 51.4 L 74.9 97.9 L 76.9 52.0 L 78.8 48.5 L 80.7 76.2 L 82.7 96.5 L 84.6 90.0 L 86.6 107.9 L 88.5 96.4 L 90.4 75.3 L 92.4 119.0 L 94.3 124.5 L 96.3 60.4 L 98.2 105.8 L 100.1 131.4 L 102.1 93.1 L 104.0 107.5 L 106.0 83.9 L 107.9 51.2 L 109.8 130.7 L 111.8 66.5 L 113.7 106.6 L 115.7 105.0 L 117.6 51.8 L 119.5 86.1 L 121.5 75.9 L 123.4 76.8 L 125.4 79.6 L 127.3 118.5 L 129.2 54.0 L 131.2 67.5 L 133.1 93.6 L 135.1 71.0 L 137.0 104.1 L 138.9 72.1 L 140.9 83.6 L 142.8 68.0 L 144.7 94.8 L 146.7 114.7 L 148.6 93.6 L 150.6 75.6 L 152.5 88.2 L 154.4 58.7 L 156.4 99.1 L 158.3 54.2 L 160.3 69.7 L 162.2 120.0 L 164.1 74.4 L 166.1 102.6 L 168.0 101.9 L 170.0 65.4 L 171.9 65.1 L 173.8 117.2 L 175.8 122.0 L 177.7 98.1 L 179.7 58.5 L 181.6 71.2 L 183.5 87.0 L 185.5 111.2 L 187.4 99.1 L 189.4 58.9 L 191.3 63.7 L 193.2 100.8 L 195.2 62.1 L 197.1 73.9 L 199.1 87.6 L 201.0 56.8 L 202.9 79.0 L 204.9 54.7 L 206.8 99.8 L 208.8 66.2 L 210.7 62.7 L 212.6 123.9 L 214.6 80.1 L 216.5 80.7 L 218.5 122.6 L 220.4 83.4 L 222.3 57.3 L 224.3 124.3 L 226.2 108.8 L 228.2 66.5 L 230.1 70.6 L 232.0 100.4 L 234.0 116.9 L 235.9 108.3 L 237.9 58.0 L 239.8 99.1 L 241.7 66.9 L 243.7 71.7 L 245.6 124.4 L 247.6 70.0 L 249.5 61.2 L 251.4 85.0 L 253.4 114.1 L 255.3 56.1 L 257.3 98.4 L 259.2 114.8 L 261.1 75.0 L 263.1 63.1 L 265.0 48.0 L 267.0 131.7 L 268.9 76.7 L 270.8 130.2 L 272.8 102.4 L 274.7 122.8 L 276.7 89.1 L 278.6 55.4 L 280.5 49.2 L 282.5 106.4 L 284.4 103.5 L 286.4 53.9 L 288.3 56.1 L 290.2 116.5 L 292.2 71.1 L 294.1 81.4 L 296.1 96.7 L 298.0 103.1 L 299.9 49.9 L 301.9 67.3 L 303.8 76.9 L 305.8 94.2 L 307.7 91.7 L 309.6 90.9 L 311.6 100.6 L 313.5 102.2 L 315.5 52.8 L 317.4 99.8 L 319.3 122.3 L 321.3 79.5 L 323.2 75.7 L 325.2 94.3 L 327.1 60.6 L 329.0 120.2 L 331.0 121.1 L 332.9 81.4 L 334.8 83.7 L 336.8 78.4 L 338.7 94.1 L 340.7 93.4 L 342.6 83.7 L 344.5 101.7 L 346.5 78.7 L 348.4 121.7 L 350.4 106.0 L 352.3 116.5 L 354.2 83.4 L 356.2 131.9 L 358.1 123.7 L 360.1 58.6 L 362.0 111.1 L 363.9 55.1 L 365.9 124.2 L 367.8 102.2 L 369.8 85.5 L 371.7 131.4 L 373.6 114.1 L 375.6 119.7 L 377.5 67.3 L 379.5 113.4 L 381.4 130.9 L 383.3 104.2 L 385.3 82.4 L 387.2 100.1 L 389.2 74.9 L 391.1 71.0 L 393.0 122.1 L 395.0 123.8 L 396.9 80.8 L 398.9 121.3 L 400.8 99.1 L 402.7 125.6 L 404.7 109.1 L 406.6 67.2 L 408.6 109.5 L 410.5 87.8 L 412.4 105.2 L 414.4 97.4 L 416.3 70.4 L 418.3 69.5 L 420.2 121.2 L 422.1 77.6 L 424.1 48.8 L 426.0 124.1 L 428.0 111.7 L 429.9 99.6 L 431.8 59.1 L 433.8 53.7 L 435.7 97.1 L 437.7 82.4 L 439.6 97.2 L 441.5 113.4 L 443.5 114.9 L 445.4 72.1 L 447.4 70.3 L 449.3 60.2 L 451.2 105.8 L 453.2 96.8 L 455.1 98.9 L 457.1 50.7 L 459.0 119.5 L 460.9 87.1 L 462.9 49.0 L 464.8 71.1 L 466.8 107.2 L 468.7 104.8 L 470.6 74.4 L 472.6 99.4 L 474.5 62.0 L 476.5 88.3 L 478.4 62.0 L 480.3 78.1 L 482.3 69.3 L 484.2 72.3 L 486.2 52.3 L 488.1 93.2 L 490.0 61.8 L 492.0 131.4 L 493.9 75.0 L 495.9 127.8 L 497.8 59.3 L 499.7 122.7 L 501.7 85.8 L 503.6 83.8 L 505.6 58.2 L 507.5 116.7 L 509.4 81.9 L 511.4 68.4 L 513.3 56.3 L 515.3 119.6 L 517.2 116.9 L 519.1 84.5 L 521.1 92.7 L 523.0 103.9 L 524.9 81.8 L 526.9 89.0 L 528.8 73.8 L 530.8 83.6 L 532.7 107.8 L 534.6 102.8 L 536.6 62.3 L 538.5 117.4 L 540.5 71.1 L 542.4 119.6 L 544.3 88.3 L 546.3 100.8 L 548.2 104.6 L 550.2 126.1 L 552.1 116.5 L 554.0 114.9 L 556.0 52.4 L 557.9 108.0 L 559.9 109.1 L 561.8 57.0 L 563.7 64.6 L 565.7 59.4 L 567.6 106.3 L 569.6 118.5 L 571.5 58.5 L 573.4 80.9 L 575.4 54.1 L 577.3 123.4 L 579.3 119.0 L 581.2 120.6 L 583.1 97.3 L 585.1 88.4 L 587.0 108.1 L 589.0 74.7 L 590.9 103.5 L 592.8 61.0 L 594.8 95.9 L 596.7 95.9 L 598.7 101.1 L 600.6 127.3 L 602.5 93.3 L 604.5 102.3 L 606.4 103.0 L 608.4 115.0 L 610.3 66.9 L 612.2 60.2 L 614.2 117.0 L 616.1 92.4 L 618.1 103.6 L 620.0 72.7" fill="none" stroke="#475569" stroke-width="1"/>
<text x="624" y="54" fill="#94a3b8" font-size="10">+1</text><text x="624" y="136" fill="#94a3b8" font-size="10">&#8722;1</text>
<text x="40" y="160" fill="#334155">the same draws, each replaced by the average of its last 10 &#8212; rolling waves</text>
<path d="M 57.5 215.7 L 59.4 215.4 L 61.3 203.1 L 63.3 196.0 L 65.2 205.5 L 67.2 209.0 L 69.1 203.2 L 71.0 204.8 L 73.0 197.9 L 74.9 201.1 L 76.9 198.3 L 78.8 182.3 L 80.7 183.6 L 82.7 192.7 L 84.6 185.6 L 86.6 192.5 L 88.5 193.1 L 90.4 188.4 L 92.4 201.9 L 94.3 207.3 L 96.3 208.9 L 98.2 220.4 L 100.1 231.4 L 102.1 230.8 L 104.0 234.3 L 106.0 229.5 L 107.9 220.4 L 109.8 231.5 L 111.8 221.0 L 113.7 217.4 L 115.7 226.3 L 117.6 215.5 L 119.5 206.5 L 121.5 203.0 L 123.4 196.9 L 125.4 196.1 L 127.3 209.5 L 129.2 194.2 L 131.2 194.4 L 133.1 191.8 L 135.1 185.0 L 137.0 195.4 L 138.9 192.7 L 140.9 194.2 L 142.8 192.4 L 144.7 195.5 L 146.7 194.7 L 148.6 202.6 L 150.6 204.3 L 152.5 203.2 L 154.4 200.7 L 156.4 199.7 L 158.3 196.1 L 160.3 193.3 L 162.2 203.7 L 164.1 199.7 L 166.1 197.2 L 168.0 198.9 L 170.0 196.8 L 171.9 192.2 L 173.8 203.9 L 175.8 208.5 L 177.7 217.3 L 179.7 215.0 L 181.6 205.3 L 183.5 207.8 L 185.5 209.5 L 187.4 209.0 L 189.4 207.7 L 191.3 207.4 L 193.2 204.1 L 195.2 192.1 L 197.1 187.3 L 199.1 193.1 L 201.0 190.2 L 202.9 188.6 L 204.9 177.3 L 206.8 177.4 L 208.8 178.9 L 210.7 178.7 L 212.6 183.3 L 214.6 186.9 L 216.5 188.3 L 218.5 195.3 L 220.4 200.6 L 222.3 196.3 L 224.3 210.2 L 226.2 212.0 L 228.2 212.0 L 230.1 213.6 L 232.0 208.9 L 234.0 216.3 L 235.9 221.8 L 237.9 208.9 L 239.8 212.0 L 241.7 213.9 L 243.7 203.4 L 245.6 206.5 L 247.6 207.2 L 249.5 205.4 L 251.4 202.3 L 253.4 201.7 L 255.3 191.3 L 257.3 199.4 L 259.2 202.5 L 261.1 204.1 L 263.1 202.4 L 265.0 187.1 L 267.0 199.5 L 268.9 202.6 L 270.8 211.6 L 272.8 209.3 L 274.7 222.6 L 276.7 220.7 L 278.6 208.9 L 280.5 203.7 L 282.5 212.4 L 284.4 223.5 L 286.4 207.9 L 288.3 203.8 L 290.2 201.1 L 292.2 194.8 L 294.1 186.5 L 296.1 188.0 L 298.0 197.6 L 299.9 197.7 L 301.9 189.9 L 303.8 184.6 L 305.8 192.7 L 307.7 199.8 L 309.6 194.6 L 311.6 200.5 L 313.5 204.7 L 315.5 195.9 L 317.4 195.3 L 319.3 209.7 L 321.3 212.2 L 323.2 211.9 L 325.2 212.0 L 327.1 205.7 L 329.0 211.6 L 331.0 215.7 L 332.9 211.5 L 334.8 217.7 L 336.8 213.4 L 338.7 207.8 L 340.7 210.6 L 342.6 212.1 L 344.5 213.6 L 346.5 217.3 L 348.4 217.6 L 350.4 214.5 L 352.3 221.6 L 354.2 221.5 L 356.2 232.2 L 358.1 238.1 L 360.1 231.2 L 362.0 236.7 L 363.9 227.3 L 365.9 236.4 L 367.8 232.5 L 369.8 228.4 L 371.7 231.4 L 373.6 237.6 L 375.6 235.1 L 377.5 223.8 L 379.5 234.8 L 381.4 238.8 L 383.3 248.6 L 385.3 240.2 L 387.2 239.8 L 389.2 237.7 L 391.1 225.6 L 393.0 227.2 L 395.0 228.0 L 396.9 230.7 L 398.9 232.3 L 400.8 226.0 L 402.7 230.2 L 404.7 235.6 L 406.6 229.0 L 408.6 236.0 L 410.5 239.3 L 412.4 235.9 L 414.4 230.6 L 416.3 228.5 L 418.3 218.2 L 420.2 222.6 L 422.1 213.0 L 424.1 200.9 L 426.0 212.3 L 428.0 212.7 L 429.9 215.1 L 431.8 205.9 L 433.8 197.2 L 435.7 202.5 L 437.7 205.1 L 439.6 200.3 L 441.5 207.4 L 443.5 220.6 L 445.4 210.2 L 447.4 201.9 L 449.3 194.1 L 451.2 203.4 L 453.2 212.0 L 455.1 212.4 L 457.1 206.0 L 459.0 210.5 L 460.9 205.2 L 462.9 192.1 L 464.8 191.9 L 466.8 199.3 L 468.7 208.2 L 470.6 201.9 L 472.6 202.4 L 474.5 195.0 L 476.5 202.6 L 478.4 191.1 L 480.3 189.3 L 482.3 193.3 L 484.2 193.5 L 486.2 182.6 L 488.1 180.2 L 490.0 177.7 L 492.0 184.1 L 493.9 186.7 L 495.9 194.6 L 497.8 194.1 L 499.7 203.0 L 501.7 206.3 L 503.6 208.6 L 505.6 209.8 L 507.5 214.5 L 509.4 218.5 L 511.4 205.9 L 513.3 202.2 L 515.3 200.5 L 517.2 212.0 L 519.1 204.4 L 521.1 205.8 L 523.0 209.8 L 524.9 214.5 L 526.9 209.0 L 528.8 207.4 L 530.8 210.4 L 532.7 220.7 L 534.6 217.4 L 536.6 206.4 L 538.5 213.0 L 540.5 208.7 L 542.4 211.8 L 544.3 213.1 L 546.3 215.5 L 548.2 221.7 L 550.2 230.2 L 552.1 231.9 L 554.0 234.3 L 556.0 232.4 L 557.9 230.5 L 559.9 238.1 L 561.8 225.6 L 563.7 220.8 L 565.7 212.5 L 567.6 212.9 L 569.6 211.3 L 571.5 199.8 L 573.4 192.9 L 575.4 193.3 L 577.3 196.4 L 579.3 198.3 L 581.2 211.1 L 583.1 217.6 L 585.1 223.4 L 587.0 223.7 L 589.0 215.0 L 590.9 224.0 L 592.8 220.0 L 594.8 228.4 L 596.7 222.8 L 598.7 219.3 L 600.6 220.6 L 602.5 219.8 L 604.5 222.6 L 606.4 221.6 L 608.4 229.7 L 610.3 222.3 L 612.2 222.2 L 614.2 226.4 L 616.1 225.7 L 618.1 226.2 L 620.0 215.3" fill="none" stroke="#2563eb" stroke-width="1.8"/>
<text x="624" y="176" fill="#94a3b8" font-size="10">+0.5</text><text x="624" y="256" fill="#94a3b8" font-size="10">&#8722;0.5</text>
<text x="40" y="280" fill="#334155">the same draws again, averaged over 40 &#8212; slower, statelier waves</text>
<path d="M 115.7 332.3 L 117.6 328.4 L 119.5 326.2 L 121.5 325.6 L 123.4 325.5 L 125.4 326.8 L 127.3 326.5 L 129.2 324.7 L 131.2 323.8 L 133.1 324.4 L 135.1 324.6 L 137.0 323.4 L 138.9 323.5 L 140.9 325.2 L 142.8 322.3 L 144.7 323.4 L 146.7 324.4 L 148.6 324.2 L 150.6 325.4 L 152.5 324.9 L 154.4 325.2 L 156.4 327.8 L 158.3 326.7 L 160.3 325.3 L 162.2 326.8 L 164.1 325.2 L 166.1 325.5 L 168.0 326.8 L 170.0 324.1 L 171.9 321.1 L 173.8 324.0 L 175.8 324.8 L 177.7 323.1 L 179.7 321.4 L 181.6 319.6 L 183.5 319.7 L 185.5 322.7 L 187.4 321.2 L 189.4 320.8 L 191.3 318.6 L 193.2 318.4 L 195.2 318.9 L 197.1 318.3 L 199.1 318.9 L 201.0 317.9 L 202.9 317.9 L 204.9 314.7 L 206.8 317.0 L 208.8 316.9 L 210.7 315.4 L 212.6 318.0 L 214.6 316.8 L 216.5 317.2 L 218.5 319.2 L 220.4 319.9 L 222.3 318.1 L 224.3 318.6 L 226.2 319.3 L 228.2 318.9 L 230.1 318.0 L 232.0 320.1 L 234.0 320.9 L 235.9 323.7 L 237.9 323.1 L 239.8 322.0 L 241.7 321.6 L 243.7 320.1 L 245.6 321.2 L 247.6 321.5 L 249.5 321.3 L 251.4 319.7 L 253.4 319.3 L 255.3 317.2 L 257.3 319.2 L 259.2 321.3 L 261.1 320.7 L 263.1 318.3 L 265.0 315.8 L 267.0 319.4 L 268.9 320.1 L 270.8 321.5 L 272.8 323.5 L 274.7 326.0 L 276.7 326.1 L 278.6 326.0 L 280.5 324.5 L 282.5 327.1 L 284.4 327.3 L 286.4 326.7 L 288.3 326.3 L 290.2 326.0 L 292.2 325.5 L 294.1 325.6 L 296.1 324.3 L 298.0 325.2 L 299.9 324.9 L 301.9 322.0 L 303.8 320.4 L 305.8 321.8 L 307.7 322.9 L 309.6 322.4 L 311.6 321.6 L 313.5 321.3 L 315.5 321.0 L 317.4 321.1 L 319.3 323.8 L 321.3 324.2 L 323.2 321.8 L 325.2 323.0 L 327.1 323.0 L 329.0 324.7 L 331.0 325.1 L 332.9 326.3 L 334.8 325.6 L 336.8 323.8 L 338.7 324.7 L 340.7 326.3 L 342.6 328.0 L 344.5 326.5 L 346.5 326.6 L 348.4 326.2 L 350.4 326.4 L 352.3 326.1 L 354.2 325.8 L 356.2 329.6 L 358.1 333.3 L 360.1 331.0 L 362.0 331.3 L 363.9 331.4 L 365.9 334.8 L 367.8 334.1 L 369.8 334.8 L 371.7 337.3 L 373.6 338.2 L 375.6 339.0 L 377.5 339.9 L 379.5 342.2 L 381.4 344.9 L 383.3 345.4 L 385.3 344.9 L 387.2 345.4 L 389.2 344.1 L 391.1 342.5 L 393.0 346.0 L 395.0 347.2 L 396.9 345.1 L 398.9 347.2 L 400.8 348.4 L 402.7 349.9 L 404.7 352.4 L 406.6 349.7 L 408.6 349.2 L 410.5 349.5 L 412.4 350.5 L 414.4 351.5 L 416.3 350.3 L 418.3 349.1 L 420.2 351.0 L 422.1 349.8 L 424.1 348.3 L 426.0 348.4 L 428.0 348.7 L 429.9 347.9 L 431.8 346.6 L 433.8 342.7 L 435.7 341.4 L 437.7 342.6 L 439.6 341.9 L 441.5 344.8 L 443.5 344.3 L 445.4 342.8 L 447.4 342.1 L 449.3 338.5 L 451.2 338.1 L 453.2 337.0 L 455.1 338.5 L 457.1 335.4 L 459.0 334.8 L 460.9 334.0 L 462.9 332.3 L 464.8 330.9 L 466.8 332.5 L 468.7 334.2 L 470.6 331.8 L 472.6 330.6 L 474.5 329.6 L 476.5 328.0 L 478.4 326.1 L 480.3 323.7 L 482.3 321.7 L 484.2 322.0 L 486.2 319.1 L 488.1 319.4 L 490.0 317.2 L 492.0 318.9 L 493.9 319.2 L 495.9 322.1 L 497.8 319.0 L 499.7 321.2 L 501.7 323.1 L 503.6 321.1 L 505.6 318.4 L 507.5 319.2 L 509.4 320.4 L 511.4 321.1 L 513.3 319.1 L 515.3 320.9 L 517.2 321.9 L 519.1 320.5 L 521.1 319.4 L 523.0 321.0 L 524.9 321.5 L 526.9 323.0 L 528.8 321.4 L 530.8 320.7 L 532.7 321.2 L 534.6 323.8 L 536.6 320.9 L 538.5 322.4 L 540.5 323.5 L 542.4 325.9 L 544.3 325.0 L 546.3 324.8 L 548.2 326.3 L 550.2 327.7 L 552.1 330.4 L 554.0 331.7 L 556.0 331.2 L 557.9 332.7 L 559.9 334.7 L 561.8 334.0 L 563.7 334.6 L 565.7 332.9 L 567.6 335.1 L 569.6 334.5 L 571.5 333.6 L 573.4 331.3 L 575.4 331.0 L 577.3 331.1 L 579.3 332.7 L 581.2 334.6 L 583.1 336.5 L 585.1 335.1 L 587.0 336.4 L 589.0 336.7 L 590.9 339.1 L 592.8 336.2 L 594.8 335.1 L 596.7 335.7 L 598.7 336.1 L 600.6 337.3 L 602.5 337.8 L 604.5 338.5 L 606.4 340.0 L 608.4 341.5 L 610.3 339.5 L 612.2 337.4 L 614.2 340.1 L 616.1 338.8 L 618.1 340.5 L 620.0 338.1" fill="none" stroke="#16a34a" stroke-width="1.8"/>
<text x="624" y="296" fill="#94a3b8" font-size="10">+0.5</text><text x="624" y="376" fill="#94a3b8" font-size="10">&#8722;0.5</text>
<text x="330" y="404" fill="#64748b" text-anchor="middle">not one of the 300 draws changed between the panels &#8212; the cycle, and its period, belong to the smoother</text>
<text x="330" y="420" fill="#64748b" text-anchor="middle">sign changes: 155 raw, 44 at window 10, 4 at window 40</text>
</svg>"##;

/// [fig 11] The complex plane, closing the lesson where it opened. Centre
/// (240, 215), r 130: 1 = (370,215), i = (240,85), -1 = (110,215),
/// -i = (240,345). The moving point at 50 degrees — the opening figure's
/// angle — is (240 + 130*0.6428, 215 - 130*0.7660) = (323.6, 115.4). The
/// three quarter-turn arcs sit at r 148, the half-lap angle arc at r 55.
const EULER_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 520 440" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="520" height="440" rx="8" fill="#f8fafc"/>
<text x="260" y="24" fill="#64748b" text-anchor="middle">turning as multiplication: the unit circle in the complex plane</text>
<g stroke="#cbd5e1" stroke-width="1" fill="none"><path d="M70 215H410M240 45V385"/></g>
<circle cx="240" cy="215" r="130" fill="none" stroke="#94a3b8" stroke-width="1.6"/>
<defs><marker id="ah-i" markerWidth="9" markerHeight="9" refX="8" refY="4.5" orient="auto"><path d="M0 0 L9 4.5 L0 9 z" fill="#b45309"/></marker></defs>
<g stroke="#b45309" stroke-width="2" fill="none" marker-end="url(#ah-i)">
<path d="M388 215 A148 148 0 0 0 240 67"/>
<path d="M240 67 A148 148 0 0 0 92 215"/>
<path d="M92 215 A148 148 0 0 0 240 363"/>
</g>
<g fill="#b45309" text-anchor="middle" font-size="12"><text x="354.6" y="102">&#215;i</text><text x="125.4" y="102">&#215;i</text><text x="122" y="338">&#215;i</text></g>
<path d="M295 215 A55 55 0 0 0 185 215" stroke="#475569" stroke-width="1.4" stroke-dasharray="4 3" fill="none"/>
<text x="240" y="150" fill="#475569" text-anchor="middle" font-size="10">half a lap: &#952; = &#960;</text>
<path d="M240 215 L323.6 115.4" stroke="#475569" stroke-width="2" fill="none"/>
<path d="M240 215 L323.6 215" stroke="#16a34a" stroke-width="2.6" fill="none"/>
<path d="M323.6 215 L323.6 115.4" stroke="#2563eb" stroke-width="2.6" fill="none"/>
<path d="M323.6 207 L315.6 207 L315.6 215" fill="none" stroke="#64748b" stroke-width="1"/>
<circle cx="323.6" cy="115.4" r="5.5" fill="#b45309"/>
<text x="316" y="108" fill="#334155" text-anchor="end">e^i&#952;</text>
<text x="280" y="230" fill="#16a34a" text-anchor="middle">cos &#952;</text>
<text x="331" y="170" fill="#2563eb">i sin &#952;</text>
<circle cx="370" cy="215" r="4" fill="#475569"/><circle cx="240" cy="85" r="4" fill="#475569"/><circle cx="240" cy="345" r="4" fill="#475569"/>
<circle cx="110" cy="215" r="5.5" fill="#b45309"/>
<text x="374" y="232" fill="#334155">1</text><text x="250" y="82" fill="#334155">i</text><text x="250" y="352" fill="#334155">&#8722;i</text>
<text x="118" y="200" fill="#b45309" text-anchor="middle">e^i&#960; = &#8722;1</text>
<text x="416" y="219" fill="#94a3b8">real</text>
<text x="240" y="40" fill="#94a3b8" text-anchor="middle">imaginary</text>
<text x="260" y="412" fill="#64748b" text-anchor="middle">&#215;i is a quarter turn; two of them make &#215;(&#8722;1), and half a lap from 1 lands exactly on &#8722;1</text>
<text x="260" y="428" fill="#64748b" text-anchor="middle">the moving point is cos &#952; + i sin &#952; &#8212; drawn at 50&#176;, the same angle as the first figure of the lesson</text>
</svg>"##;

/// The retailer's twelve monthly revenues, in millions, built from a base of
/// 100 and a swing of 12 peaking in December: 100 + 12 cos(pi (m - 11) / 6),
/// with m running 0 for January to 11 for December. Month 12 repeats January's
/// value, so the plot closes the year.
fn revenue() -> Vec<[f64; 2]> {
    vec![
        [0.0, 110.39],
        [1.0, 106.00],
        [2.0, 100.00],
        [3.0, 94.00],
        [4.0, 89.61],
        [5.0, 88.00],
        [6.0, 89.61],
        [7.0, 94.00],
        [8.0, 100.00],
        [9.0, 106.00],
        [10.0, 110.39],
        [11.0, 112.00],
        [12.0, 110.39],
    ]
}
