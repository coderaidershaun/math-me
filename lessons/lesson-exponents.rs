//! Exponents — built from `.scratch/lesson-exponents.md`'s composed script
//! (the `# Lesson: Exponents` section, pass-2 refined).

use math_me::prelude::*;

fn main() -> math_me::Result<()> {
    let mut b = Lesson::builder("Exponents");

    b = b.note("Hover any term to learn what it is telling you. Sections marked \"going deeper\" are optional — skip them and the spine still holds together. Drag any slider under a plot and the curves redraw as you go.");

    // ---------------------------------------------------------------
    // The scribe who was five days early
    // ---------------------------------------------------------------
    b = b.heading("The scribe who was five days early");
    b = b.para(|p| p
        .text("Somewhere between 2000 and 1700 BC, a clerk in Mesopotamia pressed a question into wet clay: how long does money take to double at 20% a year, with the interest joining the principal every year? He was being asked to solve"));
    b = b.display(r"1.2^{\,n} = 2");
    b = b.para(|p| p
        .text("four thousand years before anyone would write it that way. He did the only sensible thing available to him: he worked out the two years that bracket the answer — ")
        .math(r"1.2^3 = 1.728")
        .text(", not yet doubled, and ")
        .math(r"1.2^4 = 2.0736")
        .text(", just past — saw the doubling happens somewhere inside the fourth year, and divided that year with a ruler, as though the money grew by equal amounts across it."));
    b = b.para(|p| p
        .text("His answer: 3 years and 283 days. The true answer, on his 360-day year, is 3 years and 288 days. He is five days early — and you can predict the direction of his error before you check it. A straight line drawn between ")
        .math(r"1.2^3")
        .text(" and ")
        .math(r"1.2^4")
        .text(" sits above the curve it stands in for, because each of those twelve months multiplies rather than adds, so the later months contribute more than the earlier ones. A line that runs high reaches 2 too soon."));
    b = b.para(|p| p
        .text("That is the oldest exponential equation anyone has found, and it is a loan problem. It is also the first recorded instance of a mistake that has never gone away — reading a multiplying process as though it added. You will meet that person several more times in this lesson: a piano tuner, a coffee drinker, a risk manager, a fund manager, and about a third of a representative sample of American adults, all making the scribe's move with better equipment."));
    b = b.note("Count the factors. Do not add the amounts. That sentence is the whole lesson — everything below is what it means, why it is forced to be true, and what it is worth in money.");

    b = b.heading("How to read this");
    b = b.para(|p| p
        .text("You need arithmetic with whole numbers, negatives, fractions and decimals, you need to know what \"per cent\" means, and you need to be comfortable reading a letter as a stand-in for a number. Nothing else — in particular, you do not need calculus, and none is used."));
    b = b.para(|p| p
        .text("Three kinds of material, and skip freely: Spine — five core ideas and the rules they generate, a complete lesson on its own. Optional depth — the \"Going deeper\" notes, answering why all the way down to where mathematics stops proving things and starts declaring them; nothing later depends on having read them. Appendix — Further notes at the end, kept because it is worth having, not because the spine needs it. The applications lean towards money, because that is where exponents do the most damage when misunderstood — the physical world is kept alongside, because the same arithmetic runs a cooling cup of coffee and a savings account."));
    b = b.rule();

    // ===================================================================
    // Part one — Five ideas that generate everything
    // ===================================================================
    b = b.heading("Part one — Five ideas that generate everything");
    b = b.para(|p| p
        .text("There are exactly five ideas here, and the reason for having five rather than fifty rules is a documented failure. A student who meets the exponent laws as a heap of unrelated formulas finds the wrong variant exactly as plausible as the right one — which is why ")
        .math(r"x^2\cdot x^3 = x^6")
        .text(" and ")
        .math(r"\left(x^2\right)^3 = x^5")
        .text(" get written by the same person in the same exercise, each error holding the other's answer. Each of the five is stated as an intuition you can rebuild the rules from, after you have forgotten them."));

    // -------------------------------------------------------------
    // Idea 1
    // -------------------------------------------------------------
    b = b.heading("Idea 1 — An exponent is a headcount of factors");
    b = b.para(|p| p
        .text("Write ")
        .math(r"b^n")
        .text(". Almost everyone's first reading is \"multiply ")
        .math(r"b")
        .text(" by ")
        .math(r"n")
        .text("\". It is not that. It records how many copies of ")
        .math(r"b")
        .text(" sit in the product:"));
    b = b.display(r"b^n = b \times b \times \cdots \times b \quad (n \ \mathrm{factors})");
    b = b.explain(r"b^n = b \times b \times \cdots \times b \quad (n \ \mathrm{factors})", "The headcount definition",
        "b is the base — the thing being tallied. n is the exponent — the tally itself. b^n is n copies of b multiplied together, not b multiplied by n.");
    b = b.para(|p| p
        .text("So ")
        .math(r"2^3")
        .text(" is not ")
        .math(r"6")
        .text("; it is ")
        .math(r"2\times2\times2=8")
        .text(", three twos counted. Once that lands, every law becomes the same question — how many factors do I end up with? — and not one of them needs memorising separately."));
    b = b.explain_char('b', "The base", "The thing being tallied in b^n — the factor that gets repeated.");
    b = b.explain_char('n', "The exponent", "The tally itself — how many copies of the base sit in the product.");
    b = b.para(|p| p
        .text("Vocabulary, so other books do not throw you: ")
        .math(r"b^n")
        .text(" is read \"b to the nth power\" or \"b raised to the power n\". ")
        .math(r"b^2")
        .text(" is \"b squared\", from the area of a square of side b; ")
        .math(r"b^3")
        .text(" is \"b cubed\", from the volume of a cube — which is exactly why powers above three had no natural name for centuries. British textbooks say index (plural indices) and \"laws of indices\"; US and international texts say exponent. The raised superscript is a convention from 1637, not a necessity — in plain text you will meet `^` and `**`, and in C and its descendants `^` means bitwise XOR, not exponentiation, a classic and silent bug."));

    b = b.heading("Four laws, and why none of them needs memorising");
    b = b.para(|p| p
        .text("Set ")
        .math(r"m")
        .text(" factors of ")
        .math(r"b")
        .text(" beside ")
        .math(r"n")
        .text(" factors of ")
        .math(r"b")
        .text(". How many factors have you got? ")
        .math(r"m+n")
        .text(". That is the whole proof of the product rule:"));
    b = b.display(r"b^m \cdot b^n = b^{m+n}");
    b = b.explain(r"b^m \cdot b^n = b^{m+n}", "The product rule",
        "Lay m factors of b beside n factors of b: m+n factors in total. Archimedes stated and proved this for base 10 around 250 BC, roughly 1,900 years before the notation existed.");
    b = b.para(|p| p
        .text("Now cancel. Each ")
        .math(r"b")
        .text(" underneath kills one ")
        .math(r"b")
        .text(" on top, so ")
        .math(r"n")
        .text(" cancellations leave ")
        .math(r"m-n")
        .text(" factors — the quotient rule, ")
        .math(r"b^m/b^n = b^{m-n}")
        .text(" for ")
        .math(r"b\neq0")
        .text(", because you cannot cancel zeros. Now take ")
        .math(r"n")
        .text(" groups, each of ")
        .math(r"m")
        .text(" factors: that is ")
        .math(r"mn")
        .text(" factors, so the counts multiply:"));
    b = b.display(r"\left(b^m\right)^n = b^{mn}");
    b = b.explain(r"\left(b^m\right)^n = b^{mn}", "Power of a power",
        "n groups of m factors is mn factors — counts multiply because you are stacking groups of factors, not laying factors end to end.");
    b = b.para(|p| p
        .text("And ")
        .math(r"(bc)^n")
        .text(" is ")
        .math(r"(bc)(bc)\cdots(bc)")
        .text(" re-sorted into all the ")
        .math(r"b")
        .text("s followed by all the ")
        .math(r"c")
        .text("s — power of a product, ")
        .math(r"(bc)^n = b^nc^n")
        .text("; run the same argument over division for power of a quotient, ")
        .math(r"(b/c)^n = b^n/c^n")
        .text(" for ")
        .math(r"c\neq0")
        .text(". Five laws, one question asked five times. Notice the pair that gets swapped, now that the difference is visible rather than memorised: in ")
        .math(r"b^m\cdot b^n")
        .text(" you lay factors end to end, so the tally adds; in ")
        .math(r"\left(b^m\right)^n")
        .text(" you stack groups of factors, so the tally multiplies."));

    b = b.para(|p| p
        .text("Analogy — the photocopier with two buttons. Set a copier to 90% reduction and feed each output back in. The dial reads 90 every time — that is the base. The number of passes is the exponent. Three passes and then four more is seven passes: ")
        .math(r"0.9^3\cdot0.9^4=0.9^7")
        .text(". Now the trap: the machine also has a \"batch\" button running three passes at once. Press batch four times and you have twelve passes, not seven: ")
        .math(r"\left(0.9^3\right)^4=0.9^{12}")
        .text(". Predict which sheet is smaller before you compute — seven passes gives 0.478; four batches of three gives 0.282. One more factor adds to the tally; one more group of factors multiplies it."));
    b = b.para(|p| p
        .text("Where the analogy breaks down: a real copier degrades the image as well as shrinking it, so copies accumulate noise. The exponent tracks only the scale factor and knows nothing about accumulating error."));
    b = b.figure(Figure::new(ILL8_SVG, "The photocopier's two buttons: one dial reading 90%, and the whole difference between adding passes and multiplying batches of them. Feeding sheets back one at a time is 0.9^3 * 0.9^4 = 0.9^7 = 0.478 — the passes add, 3+4=7. Pressing a \"batch of three\" button four times is (0.9^3)^4 = 0.9^12 = 0.282 — the groups multiply, 3x4=12. Because a copier's percentage scales each side, the areas fall as the square: 0.9^14 = 0.229 and 0.9^24 = 0.080."));

    b = b.para(|p| p
        .text("Two more facts fall straight out of the count: ")
        .math(r"b^1=b")
        .text(" (one factor), and a negative base contributes one minus sign per factor, so ")
        .math(r"(-b)^n=(-1)^nb^n")
        .text(" — positive for even ")
        .math(r"n")
        .text(", negative for odd ")
        .math(r"n")
        .text("."));

    b = b.heading("Where the laws stop — the half nobody teaches");
    b = b.para(|p| p
        .text("A headcount needs a common thing to count, and that single requirement fixes the scope of every rule. The product rule needs equal bases: ")
        .math(r"2^3\cdot2^4=2^7")
        .text(" works, because everything in sight is a two. The power-of-a-product rule needs equal exponents: ")
        .math(r"2^3\cdot5^3=10^3")
        .text(" works, because each 2 can be married to a 5 and nothing is left over. A mixed product like ")
        .math(r"2^3\cdot5^2")
        .text(" obeys neither: you can marry a 2 to a 5 only as often as the smaller pile allows, so ")
        .math(r"2^3\cdot5^2=(2\cdot5)^2\cdot2=200")
        .text(", one lonely 2 left over. \"Equal exponents\" is precisely the condition that the pairing exhausts both piles."));
    b = b.para(|p| p
        .text("And a sum inside a bracket cannot be re-sorted at all — re-sorting is a theorem about factors, and ")
        .math(r"a+b")
        .text(" is one factor, not two. Multiplying out ")
        .math(r"(a+b)(a+b)")
        .text(" forces every term of the first bracket to meet every term of the second:"));
    b = b.display(r"(a+b)^n \neq a^n + b^n");
    b = b.explain(r"(a+b)^n \neq a^n + b^n", "The freshman's dream",
        "The single most persistent error in algebra. Dies to one number: (3+4)^2 = 49, while 3^2+4^2 = 25. The missing 24 is 2ab, the cross terms.");
    b = b.para(|p| p
        .text("This is the freshman's dream, and it dies to one number: ")
        .math(r"(3+4)^2=49")
        .text(", while ")
        .math(r"3^2+4^2=25")
        .text(". The missing 24 is ")
        .math(r"2ab")
        .text(", the cross terms — and it has a disguise: ")
        .math(r"(x^2+y^2)^{1/2} = x+y")
        .text(" is the same error wearing a root."));
    b = b.para(|p| p
        .text("A close relative worth catching in yourself: write down a value for ")
        .math(r"y^4+y^4")
        .text(" before reading on. If you wrote ")
        .math(r"y^8")
        .text(", you tallied factors where the notation tallies terms — adding counts terms, ")
        .math(r"y^4+y^4=2y^4")
        .text(", the distributive law read backwards, ")
        .math(r"y^4+y^4=(1+1)y^4")
        .text("."));
    b = b.para(|p| p
        .text("The exact accounting of which terms met which is the binomial theorem, with the coefficients laid out in Pascal's triangle (predating Pascal — Pingala has it in the 3rd century BC):"));
    b = b.display(r"(a+b)^n = \sum_{k=0}^{n}\binom{n}{k}a^{n-k}b^k");
    b = b.explain(r"(a+b)^n = \sum_{k=0}^{n}\binom{n}{k}a^{n-k}b^k", "The binomial theorem",
        "The exact accounting of which terms met which when a bracket raised to the n is multiplied out. Read through this theorem, (1+r)^n is the anatomy of compound interest: nr is simple interest, C(n,2) r^2 is interest on interest, and so on.");
    b = b.para(|p| p
        .text("Fun fact: Pascal's triangle is printed on your interest. Put $100 at 10% for three years: the growth factor is 1.331. Read the digits — 1, 3, 3, 1 — the third row of Pascal's triangle: $100 of principal, $30 of plain interest, $3 of interest on interest, and 10 cents of interest on interest on interest. Row four, 1.1^4 = 1.4641, reads straight off 1, 4, 6, 4, 1 — but row five breaks the trick, because once a coefficient reaches 10 it no longer fits in a decimal place and the digits start carrying: 1.1^5 = 1.61051."));
    b = b.para(|p| p
        .text("Drop the positive tail instead of counting it and you have Bernoulli's inequality, ")
        .math(r"(1+x)^n \ge 1+nx")
        .text(" for ")
        .math(r"x\ge-1")
        .text(" and whole ")
        .math(r"n\ge1")
        .text(" — read financially, compounding is never worse than simple interest, and strictly better as soon as ")
        .math(r"n\ge2")
        .text(" and ")
        .math(r"x>0")
        .text("."));
    b = b.para(|p| p
        .text("Counting also settles two structural facts. Because the base and the exponent are tallied differently, exponentiation is neither commutative nor associative: ")
        .math(r"2^3=8\neq9=3^2")
        .text(" (2 and 4 are the only distinct positive whole numbers with ")
        .math(r"a^b=b^a")
        .text("), and ")
        .math(r"\left(2^3\right)^2=64\neq512=2^{\left(3^2\right)}")
        .text(", which is why a tower ")
        .math(r"a^{b^c}")
        .text(" is read from the top down. Exponentiation also binds tighter than unary minus, so ")
        .math(r"-3^2=-(3^2)=-9")
        .text(" while ")
        .math(r"(-3)^2=9")
        .text(", and tighter than multiplication too, so ")
        .math(r"2\cdot3^2=18")
        .text(", not 36."));

    b = b.note("Going deeper (optional): the four laws each spend a specific axiom of multiplication — associativity for the product rule, distributivity-in-the-exponent for power-of-a-power, commutativity for power-of-a-product — which is exactly why the last one dies on a sum and why (a+b)^n cannot be resorted. Chase \"why is any of this true\" far enough and you reach one of a small number of floors: the field axioms of the real numbers, Peano's induction axiom, or (for the right-associative reading of a tower) a naming convention with real dissenters — Excel and Python disagree about -3^2 and about 2^3^2, and neither program is computing anything false. Skip to \"The idea at work\" with nothing lost.");

    b = b.heading("The idea at work");
    b = b.para(|p| p
        .text("In the physical world, the headcount sits in plain sight in the units. An area in ")
        .math(r"\mathrm{m}^2")
        .text(" is two length factors tallied; a volume in ")
        .math(r"\mathrm{m}^3")
        .text(" is three; multiplying a floor area by a ceiling height is the product rule spelled out in units, ")
        .math(r"\mathrm{m}^2\times\mathrm{m}^1=\mathrm{m}^3")
        .text(". The same accounting forces an exponent to be a pure number: in ")
        .math(r"2^{-t/t_{1/2}}")
        .text(" the exponent is a time divided by a time, and 11,460 years of carbon-14 against a 5,730-year half-life gives an exponent of exactly 2 — a count of halvings, no units at all."));
    b = b.para(|p| p
        .text("The picture to hold is Galileo's. Scale a model bridge — or an animal — up by 3 in every direction. Its weight goes up by how much? It is 27: weight follows volume, three length factors, ")
        .math(r"3^3")
        .text(". But the legs holding that weight up resist by cross-sectional area, two length factors, ")
        .math(r"3^2=9")
        .text(". So the stress in the legs rises by ")
        .math(r"27/9=3")
        .text(". That one division is the square–cube law, described by Galileo in Two New Sciences (1638), and it is why no land animal has ever been shaped like a scaled-up mouse."));
    b = b.figure(Figure::new(ILL1_SVG, "Galileo's two cubes, drawn to scale: doubling every length turns one unit cube into eight — each face of the big cube holds 2^2 = 4 unit squares, its body holds 2^3 = 8 unit cubes. Surface goes 6 -> 24 cm^2 and volume 1 -> 8 cm^3, so the surface-to-volume ratio falls from 6 to 3 per cm, and the stress in the supports rises by 2^3/2^2 = 2. One scale factor, two different exponents."));
    b = b.para(|p| p
        .text("Non-distribution has a two-second physical demonstration: walk 3 m east, then 4 m north."));
    b = b.rule();
    b = b.note("Before reading on: write down two numbers. How far have you walked, and how far are you from where you started?");
    b = b.para(|p| p
        .text("You have walked 7 m, but you are standing 5 m from where you started, because ")
        .math(r"\left(3^2+4^2\right)^{1/2}=5\neq3+4")
        .text(". The tempting wrong answer is 7 for both, and its algebraic twin is ")
        .math(r"\left(a^2+b^2\right)^{1/2}=a+b")
        .text(" — the freshman's dream wearing a root. The missing 2 m is exactly the cross terms the freshman's dream throws away: you cannot walk in two directions at once and have the distances add."));
    b = b.figure(Figure::new(ILL2_SVG, "Where the cross terms live: the square of side 3+4=7 has area 7^2=49. The freshman's dream (a+b)^2=a^2+b^2 keeps only the two squares on the diagonal, 3^2=9 and 4^2=16, total 25, and throws away the two shaded rectangles, 3x4=12 each. The discarded 2ab=24 is almost half the picture: 25+24=49."));

    b = b.para(|p| p
        .text("In a financial time series, the headcount is the number of periods elapsed. That is the whole translation — the exponent laws are not applied to finance, they are the period bookkeeping."));
    b = b.display(r"P_t = P_0(1+r)^t");
    b = b.explain(r"P_t = P_0(1+r)^t", "Compound growth",
        "P0 is the money you start with. r is the return per period, carried as a decimal. 1+r is one period's growth factor. t is the tally — how many copies of that factor sit in the product. P_t is what you end with. Nothing here multiplies by t; the t only says how many times.");
    b = b.para(|p| p
        .text("Two consequences the headcount makes unmissable. First, the exponent is dimensionless, and the rate in the base must be per that same period — \"3 years\" may not be dropped into a formula whose base is a monthly factor; re-periodising is power-of-a-power, ")
        .math(r"\left((1+r_{\mathrm{month}})^{12}\right)^y=(1+r_{\mathrm{month}})^{12y}")
        .text(". Second, deflating one series by another pairs the counts off, power-of-a-quotient:"));
    b = b.display(r"\frac{P_0(1+i)^t}{(1+\pi)^t} = P_0\left(\frac{1+i}{1+\pi}\right)^{t}");
    b = b.para(|p| p
        .text("Toy numbers: $100 in an account quoted at 12% nominal, compounded monthly. Two years later, how much is there? The tempting answer takes the headline rate and counts the years: ")
        .math(r"100\times1.12^2=\$125.44")
        .text(". It is wrong: the rate in the base has to be per the period the exponent counts. The monthly rate is ")
        .math(r"0.12/12=0.01")
        .text(" and two years is 24 months, so ")
        .math(r"100\times1.01^{24}=\$126.97")
        .text(". The headcount law audits the $1.53 gap: ")
        .math(r"1.01^{24}=\left(1.01^{12}\right)^2")
        .text(", so the effective annual factor is 1.126825, an APY of 12.6825% — the guess had the right shape and fed the nominal rate in where the effective one belongs."));
    b = b.para(|p| p
        .text("The same headcount runs inside a volatility model. GARCH(1,1) — Bollerslev, 1986 — says ")
        .math(r"\sigma_t^2 = \omega + \alpha\,\epsilon_{t-1}^2 + \beta\,\sigma_{t-1}^2")
        .text(", where ")
        .math(r"\alpha+\beta")
        .text(" is the persistence: the share of an excess in variance that survives one day. Survive ")
        .math(r"k")
        .text(" days and it has been multiplied by that share ")
        .math(r"k")
        .text(" times, ")
        .math(r"(\alpha+\beta)^k")
        .text(" — a headcount of days. With ")
        .math(r"\alpha=0.05")
        .text(" and ")
        .math(r"\beta=0.90")
        .text(", persistence is 0.95. A shock has left variance well above normal; how much of the excess is still there ten trading days later? Not \"half\" and not \"none\" — the 5% comes off what is left, ")
        .math(r"0.95^{10}=0.5987")
        .text(": sixty percent of the excess is still there after two full trading weeks."));

    b = b.rule();
    b = b.note("Before reading on: write down a number for each square — x^2 . x^3 = x^? and (x^2)^3 = x^?");
    b = b.para(|p| p
        .text("[q-1] ")
        .math(r"x^2\cdot x^3=x^5")
        .text(", and ")
        .math(r"\left(x^2\right)^3=x^6")
        .text(". The tempting wrong answers are ")
        .math(r"x^6")
        .text(" and ")
        .math(r"x^5")
        .text(" — the right two numbers, each in the wrong place, because both expressions look like the same instruction, \"combine the exponents\", once you have decided which arithmetic that means you do it to both. Stop recalling and start counting: ")
        .math(r"x^2\cdot x^3")
        .text(" sets two ")
        .math(r"x")
        .text("s beside three ")
        .math(r"x")
        .text("s, five in the product, counts add. ")
        .math(r"\left(x^2\right)^3")
        .text(" takes three groups of two ")
        .math(r"x")
        .text("s, six in total, counts multiply. Check on numbers: at ")
        .math(r"x=2")
        .text(", ")
        .math(r"4\times8=32=2^5")
        .text(", while ")
        .math(r"4^3=64=2^6")
        .text("."));

    // -------------------------------------------------------------
    // Idea 2
    // -------------------------------------------------------------
    b = b.rule();
    b = b.heading("Idea 2 — The count starts at 1, so the ladder runs both ways with no seam");
    b = b.para(|p| p
        .text("There is a better way to say what ")
        .math(r"b^n")
        .text(" means: b^n means start at 1 and multiply by b, n times. So ")
        .math(r"b^3=1\times b\times b\times b")
        .text(". The 1 is not decoration — it is multiplication's do-nothing element, the value you are left holding when nothing happens. Stepping the exponent up multiplies by ")
        .math(r"b")
        .text("; stepping it down divides by ")
        .math(r"b")
        .text("; and nothing about that stops at zero. Perform no multiplications and you are left with the 1 you started from:"));
    b = b.display(r"b^0 = 1 \qquad (b \neq 0)");
    b = b.explain(r"b^0 = 1 \qquad (b \neq 0)", "b to the zero",
        "A base case, not a convention bolted on afterwards: perform no multiplications and you are left holding the 1 you started from. This is the empty-product convention — a product of no factors is 1, exactly as a sum of no terms is 0.");
    b = b.para(|p| p
        .text("Three independent reasons converge on this, which is what makes it feel inevitable rather than decreed: the empty product (no multiplications leaves the starting 1); forced by the product rule (")
        .math(r"b^n\cdot b^0=b^{n+0}=b^n")
        .text(", so ")
        .math(r"b^0")
        .text(" must leave ")
        .math(r"b^n")
        .text(" alone); and the ladder (one step below ")
        .math(r"b^1=b")
        .text(" is ")
        .math(r"b/b=1")
        .text(")."));
    b = b.rule();
    b = b.note("Before reading on: which are you pulled towards, x^0 = 0 or x^0 = x?");
    b = b.para(|p| p
        .text("Both are wrong, and each fails for a different reason. ")
        .math(r"x^0=0")
        .text(" imports addition's do-nothing element into multiplication: ")
        .math(r"0+x=x")
        .text(" is true, but ")
        .math(r"0\times x=0")
        .text(", so starting the tally at zero would collapse every power to zero and destroy the exponent's information entirely. ")
        .math(r"x^0=x")
        .text(" reads the exponent as \"how many times x appears\", so zero occurrences ought to leave... something — but what is left when nothing happens is the thing you were holding before anything happened, and that is 1. One related fact for free: ")
        .math(r"1^x=1")
        .text(" for every ")
        .math(r"x")
        .text(", because multiplying by 1 any number of times changes nothing."));

    b = b.heading("Down the ladder: what a negative exponent is");
    b = b.para(|p| p
        .text("Keep dividing past zero. One step below ")
        .math(r"b^0=1")
        .text(" is ")
        .math(r"1/b")
        .text("; one below that is ")
        .math(r"1/b^2")
        .text(":"));
    b = b.display(r"b^{-n} = \frac{1}{b^n} \qquad (b\neq0)");
    b = b.explain(r"b^{-n} = \frac{1}{b^n} \qquad (b\neq0)", "Negative exponents",
        "The product rule demands b^n . b^-n = b^0 = 1, so b^-n is the number that multiplies b^n to give 1 — the reciprocal, and nothing else. A negative exponent is a position across the fraction bar. It is never a minus sign on the answer.");
    b = b.para(|p| p
        .text("The product rule forces it: whatever ")
        .math(r"b^{-n}")
        .text(" is, the rule demands ")
        .math(r"b^n\cdot b^{-n}=b^{n+(-n)}=b^0=1")
        .text(", so ")
        .math(r"b^{-n}")
        .text(" is the number that multiplies ")
        .math(r"b^n")
        .text(" to give 1 — the reciprocal, and nothing else. Test the rival reading: if ")
        .math(r"2^{-3}")
        .text(" were ")
        .math(r"-8")
        .text(", then ")
        .math(r"2^3\cdot2^{-3}=8\times(-8)=-64")
        .text(", not 1, and the ladder would snap. A negative exponent is a position across the fraction bar. It is never a minus sign on the answer: ")
        .math(r"2^{-3}=\frac18")
        .text(" — write this as ")
        .math(r"2^{-3}=1/8")
        .text(" — a positive number. A useful corollary: ")
        .math(r"(a/b)^{-n}=(b/a)^n")
        .text(" — a negative exponent turns a fraction upside down. The one exception is not really about exponents: ")
        .math(r"0^{-n}")
        .text(" is undefined because it is ")
        .math(r"1/0^n=1/0")
        .text(" — division by zero."));
    b = b.para(|p| p
        .text("Analogy — the minus is a lift floor, not a thermometer. On a thermometer, -2°C is genuinely less than nothing. In a lift, -2 is a position: two floors below the lobby. In ")
        .math(r"2^{-3}")
        .text(" the minus is the lift, not the thermometer — three floors below the lobby, and the lobby is 1, the floor you stand on when you have taken no trips at all. Test any candidate against the one question a lift cannot dodge: down three and up three must put you back in the lobby. ")
        .math(r"\frac18\times8=1")
        .text(" passes; ")
        .math(r"-8\times8=-64")
        .text(" leaves the building."));
    b = b.para(|p| p
        .text("Where the analogy breaks down: lift floors are equally spaced in metres, while exponent rungs are equally spaced multiplicatively — floor -1 is half the value, floor -2 is half of that, so the physical drops shrink while the floor numbers do not."));
    b = b.figure(Figure::new(ILL3_SVG, "The ladder with no seam: powers of 2 drawn to scale at 32 px per unit. The rungs 3, 2, 1, 0, -1, -2 are equally spaced, while the bars are 8, 4, 2, 1, 1/2, 1/4 -- each exactly half the one above. Nothing happens at the highlighted 2^0=1 rung: the halving across 2^1 -> 2^0 -> 2^-1 is the same halving as everywhere else."));
    b = b.para(|p| p
        .text("Fun fact — the mistake you can make with your mouth. Say ")
        .math(r"-9^{3/2}")
        .text(" out loud. Now say ")
        .math(r"(-9)^{3/2}")
        .text(" out loud, differently — you will have to invent a spoken bracket, because there isn't one. In Cangelosi, Madrid, Cooper, Olson and Hartter (2013), 13 of 18 interviewed US college students read those two expressions aloud identically, and only 29% of second-semester calculus students simplified ")
        .math(r"-9^{3/2}")
        .text(" correctly. And on ")
        .math(r"2^{-3}")
        .text(", 10 of 18 got it wrong, but only 2 produced a negative answer — the famous \"the minus is the answer's sign\" mistake is the rare one; the common failure is knowing something must be flipped and not knowing what."));
    b = b.note("Going deeper (optional): the recursive definition b^0=1, b^(n+1)=b^n . b is what a mathematician or a computer actually uses -- it is the savings-account recurrence B_0=P, B_(n+1)=B_n(1+r), and every pow() implementation ever written. The chain of why bottoms out at the field axioms of the real numbers (there exists a multiplicative identity 1, and it is unique by a one-line proof) and, one level further, at Peano's induction axiom -- the same floor idea 1's laws bottom out at. The measured fact worth carrying: this confusion about what a negative exponent means survives four college courses, which is an empirical finding about learners, not a fact about numbers.");

    b = b.heading("The idea at work");
    b = b.para(|p| p
        .text("In the physical world, almost nothing is measured absolutely — physics measures a ratio against a chosen reference, exactly the rung where the exponent is 0 and the value is 1. That is what 0 dB means: a decibel level is ")
        .math(r"10\log_{10}(I/I_0)")
        .text(", so 0 dB means the ratio is 1, not silence — a 0 dB amplifier is a piece of wire. Above the reference the exponent is positive, below it negative, and nothing happens at the crossing: +10 dB is ×10, -10 dB is ×1/10, -20 dB is ×1/100."));
    b = b.para(|p| p
        .text("Toy numbers: carbon-14 decays as ")
        .math(r"N=N_0\,2^{-t/t_{1/2}}")
        .text(" with a 5,730-year half-life. A bone is measured at one-eighth of the living carbon-14 level — how old is it? Not \"divide by eight\": ")
        .math(r"\frac18=2^{-3}")
        .text(" — write this as ")
        .math(r"1/8=2^{-3}")
        .text(" — so three half-lives have passed, ")
        .math(r"3\times5{,}730=17{,}190")
        .text(" years."));

    b = b.para(|p| p
        .text("In a financial time series, every exponential weighting scheme is this ladder, laid along the calendar with ")
        .math(r"b^0=1")
        .text(" standing on the most recent observation and the rungs descending into the past. RiskMetrics' EWMA volatility estimator:"));
    b = b.display(r"\sigma_t^2 = (1-\lambda)\sum_{k\ge0}\lambda^{k}\,r_{t-1-k}^2");
    b = b.para(|p| p
        .text("with ")
        .math(r"\lambda=0.94")
        .text(" for daily data. The case ")
        .math(r"k=0")
        .text(" is yesterday, receiving ")
        .math(r"\lambda^0=1")
        .text(" — full weight, undiminished, because no steps back have been taken. Toy numbers: with a trading month of 21 days, what weight does a month-old return carry relative to yesterday's? The linear reading says 21 days of 6%-per-day decay consumes 126% of the weight — gone. It is badly wrong: the 6% comes off what remains, ")
        .math(r"\lambda^{21}=0.94^{21}=0.2727")
        .text(" — a month-old return still carries 27% of yesterday's weight."));
    b = b.para(|p| p
        .text("The negative half of the ladder is discounting:"));
    b = b.display(r"PV = FV(1+r)^{-t}");
    b = b.explain(r"PV = FV(1+r)^{-t}", "Discounting",
        "FV is a sum arriving t periods from now; 1+r is one period's growth factor; the minus is a position across the fraction bar, not a sign on the money -- (1+r)^-t means 1/(1+r)^t, strictly between 0 and 1.");
    b = b.para(|p| p
        .text("Toy numbers: $100 arrives in three years at a 10% discount rate. The sticky-sign error reads ")
        .math(r"1.1^{-3}")
        .text(" as -1.331 and hands you -$133.10; the roaming-reciprocal error linearises and subtracts 30%, giving $70. Neither: the minus flips, it does not negate, ")
        .math(r"100\times1.1^{-3}=100/1.331=\$75.13")
        .text(". Check it forwards: ")
        .math(r"75.13\times1.1^3=\$100.00")
        .text(" — up three rungs and down three rungs are the same three rungs."));

    b = b.rule();
    b = b.note("Before reading on: write down your value for 5^-2. Then test it by multiplying it by 5^2 = 25 -- what should that product come to, and does yours?");
    b = b.para(|p| p
        .text("[q-3] ")
        .math(r"5^{-2}=1/25=0.04")
        .text(", and the test is ")
        .math(r"25\times(1/25)=1")
        .text(". The tempting wrong answer is a whole family rather than one number — ")
        .math(r"-25")
        .text(", ")
        .math(r"-1/25")
        .text(", or ")
        .math(r"5^{1/2}")
        .text(" — and that the wrong answers scatter is itself the finding: on this exact item, 10 of 18 interviewees got it wrong, but only 2 produced a negative answer. \"Flip it\" names an action on symbols; multiplicative inverse names a checkable invariant, that the product must be 1. That test settles every candidate: ")
        .math(r"25\times(1/25)=1")
        .text(" passes, ")
        .math(r"25\times(-25)=-625")
        .text(" fails, ")
        .math(r"25\times5^{1/2}=55.9")
        .text(" fails."));
    b = b.rule();
    b = b.para(|p| p
        .text("[q-4] $100 arrives in four years at 10%. One year's growth factor is 1.1, and four of them is ")
        .math(r"1.1^4=1.4641")
        .text(". The minus in ")
        .math(r"1.1^{-4}")
        .text(" says divide by that, not negate it: ")
        .math(r"PV=100\times1.1^{-4}=100/1.4641=\$68.30")
        .text(". Check it forwards — ")
        .math(r"68.30\times1.4641=\$100.00")
        .text(" — four rungs down and four rungs back up land where you started, which is the multiplicative-inverse test settling it without consulting anything else. Both rivals fail that test: linearising to -40% gives $60, and ")
        .math(r"60\times1.4641=\$87.85")
        .text(", not $100."));

    // -------------------------------------------------------------
    // Idea 3
    // -------------------------------------------------------------
    b = b.rule();
    b = b.heading("Idea 3 — Every new kind of exponent is forced, not chosen");
    b = b.para(|p| p
        .text("Nobody ever decided what ")
        .math(r"b^0")
        .text(", ")
        .math(r"b^{-n}")
        .text(", ")
        .math(r"b^{1/n}")
        .text(" or ")
        .math(r"b^{\surd2}")
        .text(" ought to mean. The laws are held fixed, and each new exponent is handed the only value that lets them keep working — extension by forcing, not by decree. You have already seen the machine run twice: set ")
        .math(r"m=n")
        .text(" in the quotient rule and ")
        .math(r"b^0")
        .text(" has to be 1; demand ")
        .math(r"b^n\cdot b^{-n}=b^0=1")
        .text(" and ")
        .math(r"b^{-n}")
        .text(" has to be the reciprocal. Now run it two more times."));

    b = b.heading("Fractional exponents");
    b = b.para(|p| p
        .text("What could ")
        .math(r"b^{1/n}")
        .text(" possibly mean? Demand that power-of-a-power keeps working:"));
    b = b.display(r"\left(b^{1/n}\right)^n = b^{(1/n)\cdot n} = b^1 = b");
    b = b.para(|p| p
        .text("So ")
        .math(r"b^{1/n}")
        .text(" is a number whose ")
        .math(r"n")
        .text("th power is ")
        .math(r"b")
        .text(" — the ")
        .math(r"n")
        .text("th root, and nothing else was ever available. For a general fraction, take the root first, because it keeps the numbers small:"));
    b = b.display(r"b^{m/n} = \left(b^{1/n}\right)^m");
    b = b.explain(r"b^{m/n} = \left(b^{1/n}\right)^m", "Fractional exponents",
        "b^(1/n) is the nth root of b -- the only number whose nth power is b, forced by demanding power-of-a-power keep working. Root-first keeps the numbers small: 8^(2/3) = (8^(1/3))^2 = 2^2 = 4, versus cubing 8 to 512 first.");
    b = b.para(|p| p
        .text("For example ")
        .math(r"8^{2/3}=\left(8^{1/3}\right)^2=2^2=4")
        .text(", beating cubing 8 to 512 and then hunting for a cube root."));
    b = b.para(|p| p
        .text("Before you go on: a very common reading of ")
        .math(r"x^{1/2}")
        .text(" is \"x divided by 2\". Test it against the one thing ")
        .math(r"x^{1/2}")
        .text(" has to satisfy, ")
        .math(r"\left(x^{1/2}\right)^2=x")
        .text(". Does ")
        .math(r"(x/2)^2")
        .text(" equal ")
        .math(r"x")
        .text("? It equals ")
        .math(r"x^2/4")
        .text(". The fraction is in the exponent, so it acts on the tally, not on the base."));

    b = b.heading("Irrational exponents");
    b = b.para(|p| p
        .text("What about ")
        .math(r"2^{\surd2}")
        .text(" — write this as ")
        .math(r"2^{1.41421\ldots}")
        .text("? Squeeze it. For a fixed ")
        .math(r"b>0")
        .text(", ")
        .math(r"b^r")
        .text(" is monotonic in ")
        .math(r"r")
        .text(", so trapping ")
        .math(r"\surd2")
        .text(" between rationals traps ")
        .math(r"2^{\surd2}")
        .text(" between their values, and the brackets close: ")
        .math(r"\left[2^{1.4},2^{1.5}\right]=[2.6390,2.8284]")
        .text(", then ")
        .math(r"\left[2^{1.41},2^{1.42}\right]=[2.6574,2.6759]")
        .text(", then ")
        .math(r"\left[2^{1.414},2^{1.415}\right]=[2.66475,2.66660]")
        .text(", closing on ")
        .math(r"2^{\surd2}=2.665144\ldots")
        .text(". Exactly one number survives the pincer, and every law survives the limit. A fund that triples in three years has an annual growth factor of ")
        .math(r"3^{1/3}=1.44224\ldots")
        .text(" — no fraction whatsoever cubes to 3, so the most routine calculation in the subject already lives out here. (The picture of this squeeze, three brackets closing on 2^root2, is placed at the end of layer 2 below, where the same forcing argument is developed as a real function.)"));

    b = b.heading("Where the forcing stops");
    b = b.para(|p| p
        .text("Where forcing produces two answers, the extension is illegitimate and the domain must be restricted. ")
        .math(r"1/3")
        .text(" and ")
        .math(r"2/6")
        .text(" are the same number, so any honest ")
        .math(r"b^{m/n}")
        .text(" must agree on both spellings, but:"));
    b = b.display(r"(-8)^{1/3} = -2, \qquad (-8)^{2/6} = \left((-8)^2\right)^{1/6} = 64^{1/6} = +2");
    b = b.para(|p| p
        .text("One exponent, two answers. The standard fix is to require ")
        .math(r"b>0")
        .text(" for non-integer exponents. Related boundary facts: even roots of negative numbers do not exist in the reals — ")
        .math(r"(-4)^{1/2}")
        .text(" is not a real number (in the complex numbers it is ")
        .math(r"2i")
        .text(", the doorway to layer 8). And ")
        .math(r"\left(x^2\right)^{1/2}=|x|")
        .text(", not ")
        .math(r"x")
        .text(" — write this as ")
        .math(r"\surd(x^2)=|x|")
        .text(" — the radical denotes the principal, non-negative root, so squaring and then rooting loses the sign. And where the value stops being unique altogether, the ladder ends: for complex ")
        .math(r"z")
        .text(", ")
        .math(r"b^z")
        .text(" is multivalued, so a branch must be chosen. Every earlier extension was forced and single-valued; that one is not — layer 8 opens it."));
    b = b.note("Going deeper (optional): why is \"the laws force it\" allowed to count as a definition? Because in each case law-preservation is a constraint with exactly one solution, and pinning down exactly one object is what a definition is permitted to do -- the modern standard is existence-and-uniqueness, shown as a theorem rather than merely asserted (Peacock's 1830 \"permanence of equivalent forms\" was the slogan; the rigorous replacement came later, and the principle's own author, Hankel, proved in 1867 that no hypercomplex number system can retain every law of ordinary arithmetic). The existence half needs completeness of the real numbers -- b^(1/n) literally IS the least upper bound of {t>0 : t^n <= b}, which is why sqrt(2) does not exist inside the rationals even though the forcing argument is just as valid there. Bedrock: axiom (Dedekind completeness) for existence; axiom (the ordered-field axioms, \"squares are non-negative\") for why negative bases stay irreparable; convention for the choice of principal branch once complex numbers discard the order.");

    b = b.heading("The idea at work");
    b = b.para(|p| p
        .text("The most audible case of forcing is a piano keyboard. Analogy — the piano nobody voted on. An octave must double the frequency, and all twelve semitone steps must be the same multiplier, or a tune changes character when started on a different key: two requirements, one unknown, ")
        .math(r"s^{12}=2")
        .text(". Try the additive guess first, because everyone does — an octave is +100%, twelve steps, so 8.33% each. Test it: ")
        .math(r"\left(1+\frac1{12}\right)^{12}=2.613")
        .text(" — write this as ")
        .math(r"1.0833^{12}=2.613")
        .text(", not 2. You have overshot the octave by a factor of 1.3."));
    b = b.para(|p| p
        .text("The forcing leaves exactly one survivor, ")
        .math(r"s=2^{1/12}=1.05946")
        .text(". Nobody voted on it; two requirements returned it. Seven steps up gives ")
        .math(r"2^{7/12}=1.4983")
        .text(", within 0.11% of the pure 3:2 fifth, which is exactly why the ear forgives the compromise. And 2.613 — the wrong number — is precisely the monthly-compounding figure on the road to ")
        .math(r"e")
        .text(": the mistuned piano and the misquoted savings rate are one arithmetic mistake in two costumes."));
    b = b.para(|p| p
        .text("Where the analogy breaks down: forcing is only ever as strong as the requirements fed into it. Insist instead that the fifth be exactly 3:2 and the twelve steps can no longer all be the same multiplier."));
    b = b.figure(Figure::new(ILL4_SVG, "The octave, split twelve ways, twice: two rows of twelve steps from 440 Hz on one linear-in-frequency axis. Row A multiplies by the forced step s = 2^(1/12) = 1.05946 each time and lands exactly on 880 Hz. Row B takes the tempting additive step of +8.33% and drifts right from the first step, reaching 440 x 2.613 = 1149.7 Hz -- past the octave by a factor of 1.3, better than four semitones sharp."));

    b = b.para(|p| p
        .text("Fractional exponents are all over physics. Kepler's third law says ")
        .math(r"T=a^{3/2}")
        .text(", period in years against distance in astronomical units. Toy numbers: an asteroid orbits at 4 AU — what is its year? Not 4 years: ")
        .math(r"4^{3/2}=\left(4^{1/2}\right)^3=2^3=8")
        .text(" years — write ")
        .math(r"4^{1/2}")
        .text(" as \"root 4\"; Jupiter at 5.2 AU gives ")
        .math(r"5.2^{3/2}=11.86")
        .text(" years, its actual orbital period to four figures."));
    b = b.para(|p| p
        .text("Kleiber's law says basal metabolic rate scales as ")
        .math(r"M^{3/4}")
        .text(". An animal sixteen times heavier needs how much more food per day? Not sixteen times: ")
        .math(r"16^{3/4}=\left(16^{1/4}\right)^3=2^3=8")
        .text(" — eight times the total food, hence half the food per kilogram, which is why mice eat frantically and elephants graze calmly. (Kleiber's 3/4 is measured, not forced by an equation, and genuinely contested — surface-to-volume reasoning predicts 2/3, and the disagreement changes the answer from 6.35x to 8x.)"));
    b = b.para(|p| p
        .text("Fractal dimension is a fractional exponent solving a shrink-the-ruler question: shrink your ruler to a third of its length and a Koch coastline needs not 3 but 4 times as many ruler-lengths, so the exponent ")
        .math(r"D")
        .text(" solving ")
        .math(r"3^D=4")
        .text(" is ")
        .math(r"D=\ln4/\ln3=1.2619")
        .text(" — a dimension between a line and a plane, measuring roughness. Mandelbrot's 1967 paper puts the west coast of Great Britain near 1.25."));

    b = b.para(|p| p
        .text("In finance, this is where the forced exponents get used. CAGR:"));
    b = b.display(r"\mathrm{CAGR} = \left(\frac{V_{\mathrm{end}}}{V_{\mathrm{start}}}\right)^{1/n} - 1");
    b = b.explain(r"\mathrm{CAGR} = \left(\frac{V_{\mathrm{end}}}{V_{\mathrm{start}}}\right)^{1/n} - 1", "CAGR",
        "The total growth factor over the whole run, raised to 1/n -- the single number which, applied n times, reproduces that total growth. Subtracting 1 turns the factor back into a rate. The exponent 1/n is forced, not stylistic: whatever x^(1/n) means, it must satisfy (x^(1/n))^n = x.");
    b = b.rule();
    b = b.note("Before reading on: a fund goes from $100 to $121 over two years. What was the annual rate?");
    b = b.para(|p| p
        .text("[q-5] The tempting answer halves the 21% total: 10.5% a year. Test it — ")
        .math(r"1.105^2=1.221025")
        .text(", which would have produced $122.10, more than actually happened. The forced answer is ")
        .math(r"1.21^{1/2}=1.1")
        .text(" — write ")
        .math(r"1.21^{1/2}")
        .text(" as \"the square root of 1.21\" — so 10.0% a year, and ")
        .math(r"1.1^2=1.21")
        .text(" exactly. The missing half a percentage point is the cross term: in year two the growth applies to year one's growth as well as to the principal."));
    b = b.para(|p| p
        .text("[q-6] Now you do one: a different fund goes from $100 to $144 over two years. Its factor ")
        .math(r"g")
        .text(" satisfies ")
        .math(r"g^2=1.44")
        .text(", so ")
        .math(r"g=1.44^{1/2}=1.2")
        .text(", a rate of 20% a year — check, ")
        .math(r"1.2^2=1.44")
        .text(". The tempting answer, half of 44%, gives 22% a year and ")
        .math(r"1.22^2=1.4884")
        .text(", overshooting the real $144 by $4.84. The direction of the error is always the same: dividing a total return by the number of years counts every year of growth but no growth on growth."));

    b = b.para(|p| p
        .text("The exponent 1/2 also annualises volatility:"));
    b = b.display(r"\sigma_{\mathrm{ann}} = \sigma_{\mathrm{daily}}\times252^{1/2}");
    b = b.para(|p| p
        .text("Variances add across independent periods, so 252 days carry ")
        .math(r"252\,\sigma_{\mathrm{daily}}^2")
        .text("; getting from variance back to volatility means undoing a square, the exponent 1/2 — not a fudge factor, the only exponent that converts \"252 times the variance\" into a number denominated in returns."));
    b = b.rule();
    b = b.note("Before reading on: a stock typically moves about 2% a day. What is its annual volatility -- and why is the exponent 1/2 and not 1?");
    b = b.para(|p| p
        .text("[q-7] ")
        .math(r"\sigma_{\mathrm{ann}}=0.02\times252^{1/2}=0.02\times15.8745=31.75\%")
        .text(" a year. Scaling by the period count instead of its square root would give ")
        .math(r"0.02\times252=504\%")
        .text(", not a plausible number for anything. Sanity-check on four days: variance ")
        .math(r"=4\times(0.02)^2=0.0016")
        .text(", so ")
        .math(r"\sigma=0.0016^{1/2}=0.04")
        .text(" — 4% over four days, not 8%: four times the time buys twice the move. And the 252 is a calendar convention, not a constant — an asset trading every day of the year annualises with ")
        .math(r"365^{1/2}=19.1050")
        .text(", about a fifth more volatility from the calendar alone."));

    b = b.para(|p| p
        .text("Irrational exponents are not exotic — they are in the parameter sheet. Ask the EWMA the most natural question available, how far back is half of the weight? — solving ")
        .math(r"\lambda^h=1/2")
        .text(" gives ")
        .math(r"h=\ln2/\ln(1/\lambda)=11.20")
        .text(" days at ")
        .math(r"\lambda=0.94")
        .text(", not a whole number, so ")
        .math(r"0.94^{11.20}")
        .text(" has to mean something — the least upper bound of ")
        .math(r"0.94^q")
        .text(" over rationals ")
        .math(r"q\le11.2023")
        .text("."));
    b = b.para(|p| p
        .text("And where the forcing stops, live in practice: CAGR is undefined — not merely very negative — when the ending value is negative, since an even root of a negative real does not exist. Story — the account with no annualised return: a levered position closes the year with negative equity, and the reporting system is asked for a CAGR. The right output is not a large negative number — there is no annualised figure to report at all, and software that returns one has invented it. The machinery genuinely broke, at the same seam as ")
        .math(r"(-8)^{1/3}=-2")
        .text(" against ")
        .math(r"(-8)^{2/6}=+2")
        .text("."));

    // -------------------------------------------------------------
    // Idea 4
    // -------------------------------------------------------------
    b = b.rule();
    b = b.heading("Idea 4 — Equal steps added to the exponent multiply the value by equal factors");
    b = b.display(r"b^{x+y} = b^x\cdot b^y");
    b = b.explain(r"b^{x+y} = b^x\cdot b^y", "The functional equation",
        "Move a fixed distance along the exponent and the value is multiplied by a fixed factor -- wherever you started from. This single mechanism, additive input and multiplicative output, is what the word \"exponential\" actually means. It is the only law in the whole topic that is sufficient, not just necessary: assume only this equation plus one mild regularity condition and f is forced to be b^x, nothing else.");
    b = b.para(|p| p
        .text("Move a fixed distance along the exponent, and the value is multiplied by a fixed factor — wherever you started from. That single mechanism, additive input and multiplicative output, is what the word \"exponential\" actually means. Divide both sides by ")
        .math(r"b^x")
        .text(" and the content is even starker:"));
    b = b.display(r"\frac{b^{x+T}}{b^{x}} = b^{T}");
    b = b.para(|p| p
        .text("The multiplier over a window depends only on the window's width, never on where the window starts. That one line is the entire proof that a doubling time or a half-life exists at all and is the same number everywhere on the curve."));
    b = b.para(|p| p
        .text("Analogy — the nucleus that is never \"due\". You already own the opposite intuition: the feeling that a coin which has come up tails eight times running is somehow due. A uranium nucleus has no such feeling — one that has already survived a billion years is exactly as likely to decay in the next second as one forged this morning. Check it on money: at 7% a year the doubling time is 10.24 years, for a £100 pot and a £100m pot alike. Then try to name the doubling time of a straight line: £100 growing at a flat £10 a year doubles in 10 years; £1,000 growing at a flat £10 a year takes 100. There is no such number — \"doubling time\" exists only because the growth is multiplicative."));
    b = b.para(|p| p
        .text("Where the analogy breaks down: a real sample does eventually run out, because atoms are discrete and the last one goes — \"never reaches zero\" is the curve's answer, not the sample's."));

    b = b.heading("What the shape has to be");
    b = b.para(|p| p
        .text("Constant multiplicative steps make the shape inevitable. Increasing for ")
        .math(r"b>1")
        .text(", decreasing for ")
        .math(r"0<b<1")
        .text(" — ")
        .math(r"(1/2)^x=2^{-x}")
        .text(", so a base below 1 is a base above 1 with the exponent's sign reversed, and raising a number between 0 and 1 to a higher power gives a smaller result: ")
        .math(r"0.9^{10}\approx0.349")
        .text(". Always positive — a product of positive numbers is positive, so the curve has a horizontal asymptote at 0 and never crosses it. And a fixed multiplier eventually and permanently beats any polynomial: ")
        .math(r"10^x")
        .text(" eventually passes ")
        .math(r"x^{10}")
        .text(" — at exactly ")
        .math(r"x=10")
        .text(" — and never looks back."));
    b = b.para(|p| p
        .text("That forces a distinction people get wrong for years: a power function fixes the exponent and varies the base (")
        .math(r"f(x)=x^n")
        .text("), an exponential function fixes the base and varies the exponent (")
        .math(r"f(x)=b^x")
        .text("). \"")
        .math(r"x^2")
        .text(" is exponential growth\" is not loose vocabulary — it is a claim that a fading multiplier and a constant one are the same kind of object. The discriminator: ")
        .math(r"b^x")
        .text(" multiplies by a fixed ")
        .math(r"b")
        .text(" per unit step regardless of position, while ")
        .math(r"n^p")
        .text(" multiplies by a step that fades to 1 as ")
        .math(r"n")
        .text(" grows."));

    b = b.heading("Reading it backwards: a logarithm is an exponent");
    b = b.para(|p| p
        .text("Every use of ")
        .math(r"b^x")
        .text(" so far has asked: given the exponent, what is the value? The reverse question — what exponent produced this value? — is a logarithm, and one sentence gets you through the rest of this lesson:"));
    b = b.note("log_b(x) is the exponent that b must be raised to, in order to give x.");
    b = b.para(|p| p
        .text("It is not a new subject — it is this subject read right to left. \"ln\" means the logarithm to base ")
        .math(r"e")
        .text(" (met properly at layer 5); any base works provided you use the same one top and bottom. Three uses justify the detour. One: solving for time — ")
        .math(r"(1+r)^t=2")
        .text(" asks how long money takes to double, and taking logs of both sides gives"));
    b = b.display(r"t = \frac{\ln 2}{\ln(1+r)}");
    b = b.explain(r"t = \frac{\ln 2}{\ln(1+r)}", "Doubling time",
        "At 7% this is 0.693147/0.067659 = 10.24 years. The same formula with the sign flipped is a half-life -- the physical and financial versions are literally the same algebra, not analogues.");
    b = b.para(|p| p
        .text("At 7% that is 10.24 years. The Rule of 72 is this formula done in your head: doubling time ≈ ")
        .math(r"72/(100r)")
        .text(". The mathematically clean constant is ")
        .math(r"100\ln2=69.3")
        .text(", exact under continuous compounding; 72 wins in practice over the 6–10% band investment returns live in, because it divides cleanly by more numbers, while demography and ecology use the Rule of 70 because their 1–3% rates sit where 70 is the closer constant. Two: log returns, which the rest of this idea is built on. Three: reading a log-scaled chart, which is what makes forty years of price history legible on one page."));
    b = b.para(|p| p
        .text("History: Michael Stifel's Arithmetica integra (1544) — the book that coined the word \"exponent\" — printed an arithmetic progression beside a geometric one and observed that addition in one column corresponds to multiplication in the other, seventy years before Napier turned it into logarithms in 1614 explicitly to convert multiplication into addition for astronomy and navigation. Edmund Gunter ruled a logarithmic scale along a rule in 1620 and multiplied by stepping distances off it with dividers; Oughtred set two such scales sliding against each other around 1622, producing the slide rule — the product rule made of wood."));
    b = b.note("Going deeper (optional): why does b^(x+y)=b^x b^y deserve to be called THE definition, rather than one law among the others? Because it is the only statement in the topic that is sufficient -- assume nothing about f except this equation plus one mild regularity condition (continuity at a point, or just monotonicity, is enough) and f is forced to be b^x; every other law drops out as a consequence, propagated one rational value at a time. Without any regularity condition at all, the equation admits monstrous discontinuous solutions built with the axiom of choice -- objects nobody has ever written down, whose graph is dense in the plane. Bedrock: axiom (the axiom of choice, for whether the monsters exist at all) -- one of only two places in this lesson the chain reaches that particular floor. And why is compound interest not merely modelled by exponentials but literally the same statement? Because holding a return for m periods then n more, versus m+n periods at once, are the same act described twice -- which IS the functional equation, with \"period\" as the exponent.");

    b = b.heading("The idea at work");
    b = b.para(|p| p
        .text("In the physical world, this law is why physics quotes half-lives and time constants rather than end dates — one number describes the entire process forever. Imagine — the coffee everyone kills ten minutes early. Wrap both hands round a mug. The room is at 20°C, the coffee was poured at 100°C, and ten minutes later it reads 60°C."));
    b = b.rule();
    b = b.note("Before reading on: say your number for the twenty-minute reading out loud.");
    b = b.para(|p| p
        .text("Almost everyone subtracts again — dropped 40 degrees, so 20°C, room temperature, finished. It reads 40°C. What halves is not the temperature but the excess over the room: the excess went 80→40, so the next equal stretch of time takes it 40→20, giving 20+20=40°C. Ten more minutes: 30°C. Ten more: 25°C. Equal stretches of time, equal factors on the excess — which is why the mug slides toward the room forever and arrives never."));
    b = b.para(|p| p
        .text("The same law runs in space: Beer–Lambert attenuation multiplies by an equal factor for every equal thickness of material, which is why the natural unit is the half-value layer — one sheet of tinted film halves the light, ten sheets leave ")
        .math(r"2^{-10}=1/1024")
        .text(", under 0.1%. And running upward, exponentials outrun every intuition: E. coli doubles about every 20 minutes, so one cell after eight hours has had 24 doublings — ")
        .math(r"2^{24}=16{,}777{,}216")
        .text(". Fold a sheet of paper 0.1 mm thick 42 times and it passes the Moon."));

    b = b.para(|p| p
        .text("In financial time series, this law is why everything is analysed in logs — probably the single most consequential idea on this list for practical work:"));
    b = b.display(r"r_t = \ln\frac{P_t}{P_{t-1}}");
    b = b.explain(r"r_t = \ln\frac{P_t}{P_{t-1}}", "Log return",
        "P_t is today's price, P_(t-1) yesterday's, so P_t/P_(t-1) is one day's growth factor. The ln asks the backwards question, what exponent produced this factor, and the output is the log (continuously compounded) return, in units of exponent per day.");
    b = b.para(|p| p
        .text("Read ")
        .math(r"b^{x+y}=b^xb^y")
        .text(" backwards and a product of factors becomes a sum of exponents — and the sum telescopes:"));
    b = b.display(r"\sum_{t=1}^{n} r_t = \ln\frac{P_1}{P_0} + \ln\frac{P_2}{P_1} + \cdots + \ln\frac{P_n}{P_{n-1}} = \ln\frac{P_n}{P_0}");
    b = b.para(|p| p
        .text("because every intermediate price appears once on top and once underneath and cancels. This is an exact identity, not an approximation, which is why means, variances and regressions run on log returns rather than on prices."));
    b = b.rule();
    b = b.note("Before reading on: a stock goes $100 -> $110 -> $99, so its two simple returns are +10% and -10%. What do the two log returns add up to?");
    b = b.para(|p| p
        .text("[q-8, told as the document's own worked example] If the simple returns look as though they cancel to zero, the natural expectation is that the logs cancel too. They do not: ")
        .math(r"\ln1.10=+0.0953102")
        .text(" and ")
        .math(r"\ln0.90=-0.1053605")
        .text(" sum to ")
        .math(r"-0.0100503")
        .text(". The down-step's log is larger in size, because getting back from 110 to 100 means dividing by 1.1, a bigger move than multiplying by 0.9 — and the sum reads out the answer directly, ")
        .math(r"e^{-0.0100503}=0.99")
        .text(", so you hold $99."));
    b = b.para(|p| p
        .text("Independent question — a stock closes at $100, then $120, then $90, then $108. Show its three log returns sum to ")
        .math(r"\ln(108/100)")
        .text(" without computing a single logarithm, then say what the three simple returns sum to."));
    b = b.rule();
    b = b.note("Work it out before reading on.");
    b = b.para(|p| p
        .text("[q-8] Read ")
        .math(r"b^{x+y}=b^xb^y")
        .text(" backwards: a sum of logs is the log of a product, ")
        .math(r"\ln\frac{120}{100}+\ln\frac{90}{120}+\ln\frac{108}{90}=\ln\left(\frac{120}{100}\times\frac{90}{120}\times\frac{108}{90}\right)")
        .text(" — every intermediate price appears once on top and once underneath: the 120s cancel, the 90s cancel, leaving ")
        .math(r"108/100=1.08")
        .text(", so the sum is ")
        .math(r"\ln1.08")
        .text(", the log return of the whole run — no logarithm ever evaluated. The simple returns behave completely differently: they are +20%, -25% and +20%, summing to +15%, while the run actually returned +8%, because each simple return is a percentage of a different starting price and adding them tallies nothing."));

    b = b.para(|p| p
        .text("Equal steps, drawn. Take logs of the compound growth path and ")
        .math(r"\ln P_t=\ln P_0+t\,\ln(1+r)")
        .text(": the log of the price is a straight line in ")
        .math(r"t")
        .text(", with slope ")
        .math(r"\ln(1+r)")
        .text(" — one period's log return, identical at every ")
        .math(r"t")
        .text(". That is \"equal steps in the exponent multiply the value by equal factors\" written as a graph, and it is what a semi-log price chart is for: a constant-growth investment plots as a straight line, and equal growth rates plot as parallel lines regardless of price level, because on a log axis the distance between ")
        .math(r"x")
        .text(" and ")
        .math(r"kx")
        .text(" is ")
        .math(r"\log k")
        .text(", independent of ")
        .math(r"x")
        .text(" — multiplication by a fixed factor is a rigid translation up the page. Drag the sliders below: watch the raw-dollar view flatten into a line-then-cliff shape while the log-price view stays a straight line whose slope is the rate, and watch path B slide up the page in perfect parallel as its starting level changes a thousandfold."));
    b = b.plot(Plot::new(0.0..=40.0)
        .curve("A — $100 at rate rA", "100 * (1 + rA)^x")
        .curve("B — same rate, different start", "P0B * (1 + rA)^x")
        .curve("C — fixed 7%, $100", "100 * 1.07^x")
        .param("rA", -0.10..=0.15, 0.10)
        .param("P0B", 2.0..=2000.0, 2.0)
        .x_label("years")
        .y_label("account value ($)")
        .caption("Linear-dollar view: the first two decades look flat and the last few look like a cliff, and nothing about the investment changed -- it is the axis, not the money. Drag rA down through zero and watch growth become decay with no seam."))
    ;
    b = b.plot(Plot::new(0.0..=40.0)
        .curve("ln(A) — $100 at rate rA", "ln(100) + x * ln(1 + rA)")
        .curve("ln(B) — same rate, different start", "ln(P0B) + x * ln(1 + rA)")
        .curve("ln(C) — fixed 7%, $100", "ln(100) + x * ln(1.07)")
        .param("rA", -0.10..=0.15, 0.10)
        .param("P0B", 2.0..=2000.0, 2.0)
        .x_label("years")
        .y_label("ln(account value) — straight means constant rate")
        .caption("The same three paths on a semi-log axis: straightness IS the constant rate, and the slope is ln(1+rA). Drag P0B from $2 to $2,000 and path B slides up the page without tilting -- a return describes the investment, not the cheque size."))
    ;

    b = b.para(|p| p
        .text("Reading the exponent backwards is the whole of decay-parameter design. Set ")
        .math(r"\lambda^k=c")
        .text(" and take a logarithm, ")
        .math(r"k=\ln c/\ln\lambda")
        .text(": for the half-life, ")
        .math(r"c=1/2")
        .text(" gives 11.20 days at ")
        .math(r"\lambda=0.94")
        .text(" and 22.76 days at ")
        .math(r"\lambda=0.97")
        .text("; for the effective window at 1% tolerance, ")
        .math(r"k=\ln(0.01)/\ln(0.94)=74.4")
        .text(", matching RiskMetrics' own published Table 5.7 of 74 days. The same backwards reading fixes the EMA smoothing constant of technical analysis, ")
        .math(r"\alpha=2/(N+1)")
        .text(", otherwise pure folklore: setting the EMA's average age equal to an ")
        .math(r"N")
        .text("-bar simple moving average's average age and solving gives that formula exactly, so a \"19-day EMA\" is named for the SMA whose centre of gravity it copies, and for nothing else."));

    // -------------------------------------------------------------
    // Idea 5
    // -------------------------------------------------------------
    b = b.rule();
    b = b.heading("Idea 5 — A percentage change is a factor, not an addend, which is why money is an exponent problem");
    b = b.para(|p| p
        .text("\"Up 8%\" is ×1.08. \"Down 8%\" is ×0.92. A rate is carried as the growth factor ")
        .math(r"1+r")
        .text(", and successive changes therefore multiply. Repeated proportional change is by definition repeated multiplication, so it is literally an exponent — and every counterintuitive fact about returns is multiplication refusing to behave like addition. This idea is not deducible from the notation and cannot be rebuilt from the other four; it has to be handed to you."));
    b = b.para(|p| p
        .text("Analogy — the sentence with a missing noun. A percentage is not a quantity. It is an instruction with a blank in it: \"take 0.08 of ______ and add it on.\" Fill it with this period's balance and each step multiplies, ")
        .math(r"V_0(1+r)^n")
        .text(". Fill it with the original principal, frozen, and each step adds the same absolute amount, ")
        .math(r"V_0(1+nr)")
        .text(". At 8% for thirty years those read ")
        .math(r"1.08^{30}=10.06")
        .text(" and ")
        .math(r"1+30(0.08)=3.4")
        .text(". The entire gap between a curve and a straight line is which noun went into the blank."));
    b = b.para(|p| p
        .text("Where the analogy breaks down: the two fills are not right and wrong. Which noun goes in the blank is settled by a document, not a theorem, and the law has repeatedly put the frozen one there. Classical Roman law prohibited compounding (anatocismus) precisely so interest would accrue linearly; German BGB §248(1) voids an agreement made in advance that interest itself bear interest, with an exception for savings banks; French law permits capitalisation only after interest has been due a full year; and US Regulation Z ships both an actuarial (compounding) method and a United States Rule (no compounding) as legitimate. Compounding is a clearing rule, not a truer model."));

    b = b.heading("Which noun goes in the blank — and who decides");
    b = b.para(|p| p
        .text("The mechanism that moves the referent has a name: capitalisation. Interest is credited into the account, at which point it stops being \"interest owed\" and becomes part of the balance — the base the next calculation is applied to. A savings account adds the $8 to the $100 and computes next period on $108. A coupon bond does the opposite: the coupon is paid out, it leaves, the face value is untouched, and a 5% bond on $100 face delivers $5 a year forever — $250 of coupons over 30 years, against the $432 the same 5% would reach if each payment rejoined the base. What compounds is never \"money\"; it is a retained stock, and retention is the switch."));
    b = b.para(|p| p
        .text("The same switch explains equities: the S&P 500's price-only record since 1928 is roughly 6% a year, its total-return record with dividends reinvested roughly 10% — over 30 years those two exponents differ by about 3×, and the entire difference is whether the dividend rejoined the base. That is also why APR and APY are separate legally mandated disclosures on opposite sides of the balance sheet — Regulation Z requires APR on credit, Regulation DD requires APY on deposits, because Congress found that inconsistent bank methods of describing interest made accounts impossible to compare."));

    b = b.heading("What multiplication refuses to do");
    b = b.para(|p| p
        .text("Undoing a loss means dividing, not subtracting. Imagine — pour half the water out of the glass. Pour exactly half the water out of a full glass. Now say what percentage you must add to fill it again."));
    b = b.rule();
    b = b.note("Before reading on: what percentage refills the glass?");
    b = b.para(|p| p
        .text("The mouth answers \"fifty percent\", because fifty came out. The hands say otherwise: what is left is half a glass, and you must pour in as much again as is left — +100%. The percentage always answers to whatever is currently in the glass, so a fall of ")
        .math(r"f")
        .text(" needs a gain of ")
        .math(r"f/(1-f)")
        .text(" and never ")
        .math(r"f")
        .text(": -20% needs +25%, -50% needs +100%, -90% needs +900%."));
    b = b.rule();
    b = b.note("Before reading on: a fund falls 40%. What gain does it need to get back to where it started?");
    b = b.para(|p| p
        .text("[q-10] +66.67%. The tempting wrong answer is +40%, because the loss and the recovery share a name and two things wearing the same number are assumed the same size. Test it: ")
        .math(r"0.6\times1.4=0.84")
        .text(", so losing 40% and gaining 40% back leaves you 16% down. What is actually needed is the factor that undoes 0.6, its reciprocal ")
        .math(r"1/0.6=1.6667")
        .text(", a gain of +66.67% — ")
        .math(r"0.6\times1.6667=1.00")
        .text(" confirms it."));
    b = b.para(|p| p
        .text("Percentage changes commute but do not cancel: +10% then -10% equals -10% then +10%, because multiplication is commutative — and both leave you at 0.99, not 1. A compounded outcome is governed by the geometric mean: \"+50% then -50% averages 0%\" is really ")
        .math(r"1.5\times0.5=0.75")
        .text(", a 25% loss, whose constant rate is ")
        .math(r"0.75^{1/2}-1=-13.4\%")
        .text(" per period — write ")
        .math(r"0.75^{1/2}")
        .text(" as \"the square root of 0.75\". AM–GM guarantees the arithmetic mean is never below the geometric mean, so an advertised \"average annual return\" is not achievable compound growth, and the gap grows with volatility, roughly half of the variance."));
    b = b.para(|p| p
        .text("Fractional periods are not linear: the monthly equivalent of a 12% annual rate is ")
        .math(r"1.12^{1/12}-1=0.949\%")
        .text(", not 1% — twelve multiplications, not twelve additions. Inflation composes multiplicatively, the exact Fisher relation ")
        .math(r"(1+i)=(1+r)(1+\pi)")
        .text(", because two successive scalings of purchasing power compose the way any two growth factors do. Fees compound too, inside the exponent: a 1% fee against a 7% return is about 14% of the return, and over 30 years the fee factor alone is ")
        .math(r"0.99^{30}=0.7397")
        .text(", roughly a quarter of terminal wealth gone. And a boundary case that matters more than it looks: a -100% return is an absorbing state — the growth factor is 0, every subsequent factor multiplies zero, and the geometric mean collapses to 0 no matter what follows. It is the mathematical statement of \"don't go bust\"."));
    b = b.note("Going deeper (optional): why do people get compound growth wrong so reliably? Not innumeracy -- the errors have a direction and a shape. People LINEARISE: they extrapolate an exponential series as though it were a straight line, systematically underestimating growing series and overestimating decaying ones. Three independent research routes converge on it (Wagenaar and Sagaria 1975 onward; Stango and Zinman 2009 on real households' loan-rate perception; Levy and Tasoff 2016, whose incentivised study finds roughly a third of subjects fully biased, median bias 0.6, and 96% underestimating compound growth). Whether the bias is innate or installed by schooling in linear functions is explicitly unresolved in the literature. Bedrock: empirical fact -- a replicated, measured finding about learners, not a fact derivable from the mathematics.");

    b = b.heading("The idea at work");
    b = b.para(|p| p
        .text("In the physical world, efficiencies, transmissions and losses are growth factors, chaining by multiplication for the same reason returns do. Toy numbers: three stages each 90% efficient — a turbine, a gearbox, a generator. The additive reading says 10% lost three times, so 70% out. It is ")
        .math(r"0.9^3=0.729")
        .text(", 72.9% — multiplication is kinder than addition here, which surprises people who have only met the punitive version. Reverse the sign and the trap bites the other way: two filters that each block half the light block ")
        .math(r"1-0.5^2=75\%")
        .text(" between them, not 100%."));
    b = b.para(|p| p
        .text("Fun fact — your liver runs simple interest on exactly one drug. Start two things at 0.12: a blood alcohol concentration of 0.12, and a drug that also starts at 0.12 with a four-hour half-life. At four hours both read 0.06."));
    b = b.rule();
    b = b.note("Before reading on: predict hour eight.");
    b = b.para(|p| p
        .text("Almost everyone halves both again to 0.03. The drug does, and keeps halving forever without reaching zero, clearing by first-order kinetics — a constant fraction per unit time, moving referent. The alcohol is at ")
        .math(r"0.12-8\times0.015=0.00")
        .text(" — gone, on a straight line, at a definite time, because the metabolising enzyme is saturated and clears ethanol by zero-order kinetics, a constant amount per hour. Frozen referent and moving referent, running in the same bloodstream: same start, same midpoint, opposite endings."));

    b = b.para(|p| p
        .text("In a financial time series, idea 5 is what turns a column of prices into something you can do statistics on — stop storing levels, start storing factors:"));
    b = b.display(r"P_t = P_0\prod_{s=1}^{t}\left(1+r_s\right)");
    b = b.explain(r"P_t = P_0\prod_{s=1}^{t}\left(1+r_s\right)", "Growth factors compound",
        "r_s is the return in period s, a different number every period now, not a constant. This is P_t = P_0(1+r)^t with the constant-rate assumption dropped -- when every r_s happens to be equal the bag of factors collapses back into a power.");
    b = b.rule();
    b = b.note("Before reading on: a stock returns +3%, then -1%, then +2%. What is the three-day return?");
    b = b.para(|p| p
        .text("[q-9] +4.0094%, not +4%. A percentage change is a factor: \"up 3%\" is ×1.03, \"down 1%\" is ×0.99, \"up 2%\" is ×1.02. Multiply, because that is what \"and then\" means once the blank is filled that way: ")
        .math(r"1.03\times0.99\times1.02=1.040094")
        .text(". Adding gives 3-1+2=+4%, and that is what almost everyone writes, including production code; the extra 0.0094 percentage points is the cross terms — the third day earning on the first day's gain."));
    b = b.figure(Figure::new(ILL5_SVG, "Eight ways to earn: multiplying (1+r)^3 out one bracket at a time produces 2^3=8 choice-words, one per path through the three years. Sorting them by how many years earned piles them 1, 3, 3, 1 -- the third row of Pascal's triangle. At r=10% the piles are worth 1, 3r=0.300, 3r^2=0.030 and r^3=0.001, summing to 1.331. Simple interest keeps the first two columns and throws the 0.031 tail away."));

    // -------------------------------------------------------------
    // What the five ideas share
    // -------------------------------------------------------------
    b = b.rule();
    b = b.heading("What the five ideas turn out to share");
    b = b.para(|p| p
        .text("Read across the five ideas rather than down them, and the chains converge on a very small number of floors. Completeness of the real numbers is this lesson's one and only non-algebraic ingredient, bought once and bought early — the existence of nth roots, an irrational exponent, the logarithm as a total function, and the number e all cash the same cheque. The field axioms plus induction are the floor under every counting law: the laws of exponents are not facts about exponents, they are the axioms of multiplication, counted, and which axiom each law spends is what distinguishes them. The order axioms decide where the subject stops — losing order (in the complex numbers) is exactly why complex exponentiation goes multivalued. Convention chosen for consistency accounts for eleven separate human decisions in this lesson (the restriction to positive bases, the base e, 72 over 69.3, the principal branch, and more), each a choice with a reason and several with live dissenters. Empirical fact accounts for a handful of branches clustering in the applied and machine layers — the 252 trading days, RiskMetrics' λ=0.94, the referent of a percentage being a contractual clearing rule — each measured and each capable of changing. And exactly one chain in this whole lesson stops at neither an axiom nor a convention nor a measurement but at an unproven conjecture: that factoring is hard, underneath RSA."));
    b = b.para(|p| p
        .text("Idea 1 — the exponent as a headcount of factors — is primary: it is the only one that is simultaneously a definition and a proof technique, and the only one that can be given first to a reader starting from zero. Idea 2 fixes where the count starts; idea 3 is the discipline of preserving the count once it stops being a whole number; idea 4 is the same law promoted to a continuously varying exponent; idea 5 is the count applied to growth factors instead of plain numbers. If you stop here, you have the subject — everything below is the progression outward."));

    // ===================================================================
    // Part two — Outward from the spine
    // ===================================================================
    b = b.rule();
    b = b.heading("Part two — Outward from the spine");
    b = b.para(|p| p
        .text("Eleven layers, ordered so each is reachable from the one before it. Layers 1–3 finish the exponent itself; layer 4 turns it around; layers 5–7 are where the finance lives; layers 8–10 are the frontier; layer 11 names the doors this lesson deliberately leaves shut. A reader who stops after layer 7 has lost nothing they will need."));

    b = b.heading("Layer 1 — Fractional exponents and the return of roots");
    b = b.para(|p| p
        .text("Idea 3 established that ")
        .math(r"b^{1/n}")
        .text(" is forced; this layer is the craft that goes with it. ")
        .math(r"b^{m/n}=\left(b^{1/n}\right)^m")
        .text(" — root-first is the only order a human can execute unaided: two years of growth out of a three-year factor of 1.331 is ")
        .math(r"1.331^{2/3}")
        .text(", and root-first gives ")
        .math(r"1.331^{1/3}=1.1")
        .text(" then ")
        .math(r"1.1^2=1.21")
        .text(", while power-first means cubing 1.331 to 1.771561 and then hunting for its cube root. Something stronger is true for ")
        .math(r"b>0")
        .text(": representation-independence is itself a theorem — ")
        .math(r"b^{m/n}")
        .text(" depends only on the value of the exponent, never on which fraction you happened to write, which is precisely why the textbook \"lowest terms\" proviso is provably redundant for positive bases."));
    b = b.para(|p| p
        .text("Applied — physical: the cleanest fractional exponent in physics is Kepler's ")
        .math(r"T=a^{3/2}")
        .text(", held as \"one full copy of the distance, times the square root of another\". Applied — finance: the 1/n exponent is de-annualising, the most routine conversion in the subject. Toy numbers: a fund goes from $100 to $133.10 in three years. Dividing the total, ")
        .math(r"33.1\%/3=11.03\%")
        .text(", applies arithmetic to something multiplicative. The root gives ")
        .math(r"1.331^{1/3}=1.10")
        .text(" exactly, 10% — the 1.03 percentage points of difference are the interest-on-interest the division discarded."));

    b = b.heading("Layer 2 — Real exponents, and exponentiation becoming a function");
    b = b.para(|p| p
        .text("With rational exponents in hand, idea 3's squeeze fills the gap to the irrationals, and ")
        .math(r"b^x")
        .text(" is now a function on the whole real line: strictly increasing for ")
        .math(r"b>1")
        .text(", strictly decreasing for ")
        .math(r"0<b<1")
        .text(", always positive, asymptotic to zero, never crossing it. \"Every law survives the limit\" is very slightly too strong: strict inequalities do not automatically transfer (limits preserve ≤ but not <), the scope condition ")
        .math(r"b>0")
        .text(" stops being a convenience and becomes mandatory, and power-of-a-power needs two stages rather than one."));
    b = b.para(|p| p
        .text("A fixed multiplier eventually and permanently beats any polynomial — the threshold is fully explicit and calculus-free, Bernoulli's inequality gives it directly. For a savings-account base, ")
        .math(r"b=1.08,\ p=10")
        .text(": the ratio turns upward at year 130 and does not cross 1 until between year 881 and 882 — the exponential spends 750 years gaining before it is visibly ahead, which is exactly why exponential growth bias survives observation."));
    b = b.para(|p| p
        .text("Applied — physical: inverse-square (")
        .math(r"I\propto d^{-2}")
        .text(") is a power function, fixed exponent varying base; Beer–Lambert (")
        .math(r"I=I_0e^{-\alpha x}")
        .text(") is an exponential function, fixed base varying exponent — distance spreads light out, fog eats a fixed fraction per metre. Toy numbers: in fog thick enough that each metre halves the light, at 2 m both laws leave 1/4 — dead level — but at 20 m inverse-square leaves 1/400 while the fog leaves ")
        .math(r"2^{-20}=1/1{,}048{,}576")
        .text(", about 2,600 times more punishing, and the gap widens forever."));
    b = b.para(|p| p
        .text("Applied — finance: genuinely real exponents arrive the moment a cash flow is not a whole number of periods away, which is almost always — the exponent is a year fraction from a day-count convention. Toy numbers: a $100 payment falls due in 137 days at 5% ACT/365, ")
        .math(r"t=137/365=0.375342")
        .text(" years, giving ")
        .math(r"100\times1.05^{-0.375342}=\$98.1854")
        .text(". Pro-rating the rate instead — a genuine market convention, not simply an error — gives $98.1579, a gap of 2.8 cents per $100."));
    b = b.figure(Figure::new(ILL6_SVG, "Three brackets closing on 2^root2: one shared scale, three brackets. [2^1.4, 2^1.5] is 0.1894 wide, [2^1.41, 2^1.42] is 0.0185, and [2^1.414, 2^1.415] is 0.00185 -- a tenfold narrowing each step, none of them zoomed. The red line at 2^root2 = 2.665144 sits inside all three, because the brackets are nested -- exactly one number survives the pincer, which is why b^x for irrational x is forced, not chosen."));

    b = b.heading("Layer 3 — Where the rules bend: boundary cases, conventions and machine limits");
    b = b.para(|p| p
        .text("0^0, and both answers are right in their own field. In algebra and combinatorics, ")
        .math(r"0^0=1")
        .text(": it is the empty product, it counts the one function from the empty set to any set, and the binomial theorem needs it. Knuth argued firmly for 1; IEEE 754 specifies it. In analysis, ")
        .math(r"0^0")
        .text(" is an indeterminate form: if ")
        .math(r"f(x)\to0")
        .text(" and ")
        .math(r"g(x)\to0")
        .text(", ")
        .math(r"f(x)^{g(x)}")
        .text(" can approach anything at all depending on the path — for any target ")
        .math(r"0<c<1")
        .text(", set ")
        .math(r"a_n=c^n")
        .text(" and ")
        .math(r"b_n=1/n")
        .text(": both tend to 0, yet ")
        .math(r"a_n^{b_n}=c")
        .text(" exactly, for every ")
        .math(r"n")
        .text(". These are statements about different things — 0^0=1 is a definition about the expression, \"indeterminate\" is a statement about limits — and they are not in conflict."));
    b = b.para(|p| p
        .text("The other boundary values: ")
        .math(r"0^n=0")
        .text(" for ")
        .math(r"n>0")
        .text(", but ")
        .math(r"0^{-n}")
        .text(" is ")
        .math(r"1/0")
        .text(" — division by zero, not a special exponent rule. Negative bases with fractional exponents are refused, for the reason idea 3 exposed. And exponentiation is right-associative, so ")
        .math(r"2^{3^2}=512")
        .text(", not 64 — a spreadsheet that says 64 is disagreeing with mathematics rather than being wrong about numbers. That last one is a genuine convention with live dissenters: Excel, MATLAB, Octave and PostgreSQL left-associate, giving 64; Python, Ruby, Perl, Fortran and Mathematica right-associate, giving 512. Excel goes further and gives unary minus higher precedence than exponentiation, returning 9 for ")
        .math(r"-3^2")
        .text(" while Microsoft's own VBA returns -9 — two products from one company disagreeing."));
    b = b.rule();
    b = b.note("Before reading on: evaluate -3^2 by hand and commit to an answer. Then say what -9^(3/2) is, and what (-9)^(3/2) is.");
    b = b.para(|p| p
        .text("[q-11] ")
        .math(r"-3^2=-9")
        .text(". And ")
        .math(r"-9^{3/2}=-27")
        .text(", while ")
        .math(r"(-9)^{3/2}")
        .text(" is not a real number at all. The tempting wrong answer, ")
        .math(r"-3^2=+9")
        .text(", is tempting enough that widely used software agrees with it: Excel evaluates ")
        .math(r"-3^2")
        .text(" as 9, Python as -9 — two programs, no arithmetic error in either, and a different number out, the cleanest available proof that precedence is a convention rather than a fact about numbers. The exponent binds tighter than the minus, so the power is taken first and the minus applied last: ")
        .math(r"-3^2")
        .text(" means ")
        .math(r"-(3^2)=-9")
        .text("; only ")
        .math(r"(-3)^2=9")
        .text(" makes the base negative. The same reading settles the harder pair: ")
        .math(r"-9^{3/2}=-\left(9^{3/2}\right)=-\left(9^{1/2}\right)^3=-27")
        .text(", write ")
        .math(r"9^{1/2}")
        .text(" as \"root 9\", whereas ")
        .math(r"(-9)^{3/2}")
        .text(" asks for the root of a negative number, which the reals do not supply. A rate calculation whose value changes when it is ported has a convention bug, not an arithmetic bug — the fix is a bracket, not a rewrite."));
    b = b.para(|p| p
        .text("Applied — finance, three boundary cases that are live hazards in valuation code. One: the annuity factor is 0/0 at a zero rate — ")
        .math(r"PV=PMT\times(1-(1+r)^{-n})/r")
        .text(" — write this as PMT times the fraction (1 minus (1+r) to the minus n) over r — returns NaN at ")
        .math(r"r=0")
        .text(" from a naive implementation, though the true answer is plainly ")
        .math(r"n")
        .text(" undiscounted payments; and zero is not hypothetical — the ECB's deposit facility was negative from 2014 to 2022. Two: ")
        .math(r"0^n=0")
        .text(" but ")
        .math(r"0^{-n}")
        .text(" is undefined — a position that loses everything has growth factor exactly 0, and five years of +50% do not repair it, ")
        .math(r"0\times1.5^5=0")
        .text(". Three: negative bases are refused, and finance meets the refusal at negative equity — a margin account closing negative supplies a negative base, and a geometric return is not reported for a blown-up account rather than reported as a large negative number."));

    b = b.heading("Layer 4 — The inverse question: logarithms, just far enough");
    b = b.para(|p| p
        .text("Idea 4 introduced the one sentence: a logarithm is an exponent. This layer earns it, and marks where the minimum stops being enough. The inversion is legal because for ")
        .math(r"b>1")
        .text(", ")
        .math(r"b^x")
        .text(" is strictly increasing, hence one-to-one, and it hits every positive value exactly once, so ")
        .math(r"b^x")
        .text(" is a bijection from the reals onto the positive reals and ")
        .math(r"\log_b")
        .text(" is defined on the whole positive half-line. How little is genuinely needed: for the doubling-time formula, exactly one law; for log returns and log charts, one law again, ")
        .math(r"\ln(uv)=\ln u+\ln v")
        .text(", the product rule read backwards."));
    b = b.para(|p| p
        .text("Where the minimum stops being enough — and it is a sharp line — is the first moment a logarithm meets an average or an expectation. Every log law is a statement about a single argument being transformed; averaging is entirely a question about shape. Someone armed only with \"logs add\" computes the mean of daily log returns and reports ")
        .math(r"e^{\bar r}-1")
        .text(" as \"the average daily return\" — that number is the geometric mean simple return, systematically smaller, because ")
        .math(r"\ln(x+y)\neq\ln x+\ln y")
        .text(" is the freshman's dream wearing a logarithm. The gap is smaller than it looks: midpoint concavity of \"ln\" is literally AM–GM, which this lesson already owns. What it withholds is the name — Jensen's inequality."));
    b = b.para(|p| p
        .text("Applied — physical: every scale spanning many orders of magnitude is a ruler laid along the exponent. Toy numbers: a quiet room is about 30 dB, a vacuum cleaner about 70 dB — how many times more sound energy? Not \"about twice\", nor \"forty times\": ")
        .math(r"L=10\log_{10}(I/I_0)")
        .text(", so a gap of 40 dB is ")
        .math(r"10^4")
        .text(" — ten thousand times the intensity."));
    b = b.para(|p| p
        .text("Applied — finance: a stock falls 40%. Earning 8% a year, how long until it is whole? The tempting answer needs +40%, and ")
        .math(r"40/8=5")
        .text(" years. Wrong twice over: it needs +66.7%, the gain earned on the smaller base, and the years are an exponent, not a division — ")
        .math(r"0.6\times1.08^t=1")
        .text(" gives ")
        .math(r"t=\ln(1/0.6)/\ln1.08=6.64")
        .text(" years, understating the wait by more than eighteen months."));

    b = b.heading("Layer 5 — e, and what happens when the steps get infinitely small");
    b = b.para(|p| p
        .text("Compounding more often earns more, but not without limit:"));
    b = b.display(r"\lim_{n\to\infty}\left(1+\frac rn\right)^n = e^r, \qquad\mathrm{hence}\qquad A = Pe^{rt}");
    b = b.explain(r"\lim_{n\to\infty}\left(1+\frac rn\right)^n = e^r, \qquad\mathrm{hence}\qquad A = Pe^{rt}", "Continuous compounding",
        "As the compounding period shrinks toward nothing and the number of periods grows without bound, the growth factor climbs toward e^r and stops there -- it does not explode. At r=5% the ladder annual to continuous reads 5.000%, 5.063%, 5.095%, 5.116%, 5.127%, 5.127% -- the whole infinite refinement is worth about 13 basis points.");
    b = b.para(|p| p
        .text("annual → semiannual → quarterly → monthly → daily → continuous, at ")
        .math(r"r=5\%")
        .text(", reads 5.000%, 5.063%, 5.095%, 5.116%, 5.127%, 5.127% — the whole infinite refinement is worth about 13 basis points, and believing more frequent compounding grows without limit is a common misconception this ladder refutes."));
    b = b.para(|p| p
        .text("Why it converges instead of exploding: halving the period halves the rate but doubles the number of multiplications, and the two effects very nearly — but not exactly — cancel. The residue is interest earned on interest within the year, and it accumulates to a definite ceiling. The two famous definitions of e — the compound-interest limit and the factorial series ")
        .math(r"\sum1/k!")
        .text(" — are not two facts about e; they are one algebraic identity plus a limit, and the same three-line bound that shows ")
        .math(r"e<3")
        .text(" is the geometric series from the annuity formula doing the work."));
    b = b.para(|p| p
        .text("Applied — physical: one sentence covers an enormous range — the rate of change is proportional to how much is left. Toy numbers: an RC circuit loses 63% of its charge in the first second — when is it empty? Not about 1.6 seconds: it loses 63% of what is left, so after 1 s, 36.8% remains, after 2 s, 13.5%, after 3 s, 5.0%. Mathematically it is never empty. And: something decays at 100% per year — how much is left after one year? Not nothing: apply it in twelve monthly slices and ")
        .math(r"(1-1/12)^{12}=0.352")
        .text(" survives; in the continuous limit, ")
        .math(r"e^{-1}=0.3679")
        .text(" — about 37% survives a 100% annual decay rate, and that number is 1/e."));
    b = b.para(|p| p
        .text("Applied — finance: continuous compounding is where e earns its keep. Toy numbers: $100 due in two years at 5%. Continuous gives ")
        .math(r"100e^{-0.10}=\$90.4837")
        .text("; annual gives ")
        .math(r"100/1.05^2=\$90.7029")
        .text(". 5% continuously compounded is a higher rate than 5% annually compounded — it grows a pound to ")
        .math(r"e^{0.05}=1.05127")
        .text(" rather than 1.05 — so it discounts harder, and the annual figure is the larger. They are not one deal quoted twice; they are two deals."));

    b = b.heading("Layer 6 — Compound interest and the time value of money");
    b = b.para(|p| p
        .text("The first full applied layer, and the one most readers came for. $100 today is not merely $100 — it is $100 plus the ability to lend it out, so discounting is ")
        .math(r"A=P(1+r)^t")
        .text(" solved for ")
        .math(r"P")
        .text(", not a second law bolted on. And the price is not opinion: the law of one price is enforced by a trade you can write out — sell an over-priced future claim, lend the proceeds, and pocket riskless money today; the only price at which no such trade is available is the discounted one."));
    b = b.display(r"A = P\left(1+\frac rn\right)^{nt}");
    b = b.explain(r"A = P\left(1+\frac rn\right)^{nt}", "Compound interest, forward",
        "P is the principal, r the nominal annual rate, n the number of compounding periods per year so r/n is the rate actually applied once per period, nt is a dimensionless count of periods. The unit check that kills most errors: the base is a ratio, the exponent is a count.");
    b = b.rule();
    b = b.note("Before reading on: $1,000 is deposited at 6% nominal, compounded monthly, for five years. Pick one -- 1000 x 1.005^5, or 1000 x 1.06^5, or 1000 x 1.005^60.");
    b = b.para(|p| p
        .text("[q-12] ")
        .math(r"1000\times1.005^{60}=1000\times1.348850=\$1{,}348.85")
        .text(". The tempting wrong answer, ")
        .math(r"1000\times1.005^5=\$1{,}025.25")
        .text(", feels right because both ingredients are individually correct — 0.005 really is the monthly rate, 5 really is the number of years — which is exactly why the error survives being checked at all. The base and the exponent must be measured on the same clock: the base is one period's growth factor, the exponent a count of those same periods. Pair a monthly base with a count of years and you have priced five months, not five years. The count is ")
        .math(r"5\times12=60")
        .text(" months. The middle candidate, ")
        .math(r"1000\times1.06^5=\$1{,}338.23")
        .text(", is not a mistake of clocks at all — it is the right answer to a different contract, 6% compounded annually, and the $10.62 between the two is the entire value of compounding twelve times a year instead of once."));
    b = b.para(|p| p
        .text("A stream of payments is a geometric series, which is where the rest of the subject comes from:"));
    b = b.display(r"PV = PMT\times\frac{1-(1+r)^{-n}}{r}");
    b = b.explain(r"PV = PMT\times\frac{1-(1+r)^{-n}}{r}", "Annuity present value",
        "One formula covers annuities, mortgages, bond prices and pensions -- the geometric series collapsed to closed form. Let n grow without bound and it collapses further, to PMT/r: a perpetuity.");
    b = b.para(|p| p
        .text("Toy numbers: a $200,000 mortgage, 30 years, 6% nominal compounded monthly, so ")
        .math(r"r=0.005")
        .text(" per month and ")
        .math(r"n=360")
        .text(" months, never 30. With no interest the payment would be $555.56; the actual payment is $1,199.10, and the total paid over the loan is $431,676 — more than twice the sum borrowed, with 53.7% of every dollar paid going to interest."));
    b = b.para(|p| p
        .text("Perpetuities are the same series with no last term: a growing one (Gordon) is ")
        .math(r"P=D_1/(r-g)")
        .text(", converging only when ")
        .math(r"g<r")
        .text(". Toy numbers: ")
        .math(r"D_1=\$4")
        .text(", ")
        .math(r"r=8\%")
        .text(", ")
        .math(r"g=3\%")
        .text(" gives ")
        .math(r"P=\$80")
        .text(". Raise the growth assumption two points, to 5% — the value moves not 2% and not 25%, but to ")
        .math(r"4/0.03=\$133.33")
        .text(", a 67% jump, because the change lands in the denominator's distance to zero."));
    b = b.para(|p| p
        .text("Bond pricing, YTM and IRR are the same sum solved for the rate instead of the value — pricing is polynomial evaluation, easy; yielding is polynomial root-finding, not automatically unique. Lorie and Savage's 1955 example, ")
        .math(r"-1600,+10000,-10000")
        .text(", has two internal rates of return, 25% and 400%, both genuine — which is the structural reason NPV is generally preferred to IRR."));
    b = b.para(|p| p
        .text("Applied — physical: compounding is not a financial phenomenon, it is what happens in any system where the increment rejoins the stock. Toy numbers: one E. coli cell divides every 20 minutes; left 24 hours with unlimited food, that is 72 doublings, ")
        .math(r"2^{72}=4.7\times10^{21}")
        .text(" cells — roughly 4,700 tonnes, a small ship, from one cell, in one day. And a physical perpetuity: a ball dropped from 1 m rebounds to three-quarters of its height each bounce. It travels ")
        .math(r"1+2(0.75+0.75^2+\cdots)=7")
        .text(" metres before stopping — infinitely many bounces, finite distance, the same geometric series that makes a perpetuity worth a finite amount."));

    b = b.heading("Layer 7 — Growth rates of a series that moves both ways");
    b = b.para(|p| p
        .text("Layer 6 assumed a constant rate. Real financial series do not have one, and almost every well-known trap in investing lives in the gap. Because growth factors multiply, the average that governs terminal wealth is the geometric one, not the arithmetic one. Toy numbers: a fund returns +50%, then -50%. The arithmetic mean is 0%, and the tempting conclusion is \"flat\" — but $100 → $150 → $75: the investor is down a quarter, and the geometric mean is ")
        .math(r"0.75^{1/2}-1=-13.397\%")
        .text(" a year. AM–GM is exact and always holds: the arithmetic mean can never be below the geometric mean, with equality only when every return is identical. The size of the gap is approximately half the variance — and that is an approximation, unlike AM–GM itself."));
    b = b.para(|p| p
        .text("The exponent 1/2 scales risk across horizons because variance — the squared quantity — is what adds across independent periods, so T days carry ")
        .math(r"T\sigma^2")
        .text(" and volatility carries ")
        .math(r"T^{1/2}")
        .text(". The 252 is an exchange calendar rounded and frozen by agreement, not a mathematical constant — the true count moves year to year, and crypto, which never closes, reports volatility about 20% higher than an equity convention would for identical daily moves."));
    b = b.para(|p| p
        .text("Exponential weighting puts the exponent on time rather than on money. In ")
        .math(r"\sigma_t^2=\lambda\sigma_{t-1}^2+(1-\lambda)r_{t-1}^2")
        .text(", an observation ")
        .math(r"k")
        .text(" days old carries weight ")
        .math(r"w_k=(1-\lambda)\lambda^{k}")
        .text(" — a geometric sequence, so a day's influence is an exponential in its age. Drag the slider below and watch what three hundredths of movement in λ does to the market's memory."));
    b = b.plot(Plot::new(0.0..=160.0)
        .curve("weight w_k = (1-lambda) lambda^k", "(1 - lambda) * lambda^x")
        .param("lambda", 0.90..=0.97, 0.94)
        .vline(11.20)
        .x_label("age of the observation, k trading days")
        .y_label("weight in today's variance estimate")
        .caption("At lambda=0.94 (RiskMetrics' published daily figure) the dashed line marks the half-life, 11.20 days -- drag lambda toward 0.97 and watch it stretch toward 22.76 days. The curve's height at k=0 is exactly 1-lambda: raising lambda does not add weight to the past, it takes weight off yesterday and spreads it backwards, with the total pinned at 1 throughout."));
    b = b.para(|p| p
        .text("Toy numbers: with ")
        .math(r"\lambda=0.94")
        .text(", yesterday's return was -3%, three times the usual daily move, and yesterday's estimate corresponded to ")
        .math(r"\sigma=1.00\%")
        .text(". Today's estimate: not a large jump — ")
        .math(r"\sigma_t^2=0.94(0.0001)+0.06(0.0009)=0.000148")
        .text(", so ")
        .math(r"\sigma_t=1.2166\%")
        .text(": one extreme day moves the estimate from 1.00% to 1.22%, because it enters with weight 0.06, not weight 1."));
    b = b.para(|p| p
        .text("GARCH puts the exponent on persistence: with the long-run variance ")
        .math(r"\bar\sigma^2=\omega/(1-\alpha-\beta)")
        .text(", the k-step forecast reverts toward it as ")
        .math(r"(\alpha+\beta)^k")
        .text(", the same exponent as compound interest, running downhill. Toy numbers: persistence 0.98, a crisis has pushed today's estimate to 2% against a normal level of 1%. A month later, is it back to normal? The half-life is 34.3 days, so a month is not even one half-life: ")
        .math(r"0.98^{30}=0.5455")
        .text(", and volatility is still 1.62% — 62% above normal. This is the arithmetic behind \"volatility clusters\"."));
    b = b.para(|p| p
        .text("The lognormal price model closes the layer:"));
    b = b.display(r"S_t = S_0\exp\left[\left(\mu-\frac{\sigma^2}{2}\right)t+\sigma W_t\right]");
    b = b.para(|p| p
        .text("Everything sits inside an exponential, so the price can approach zero but never pass through it. The ")
        .math(r"-\sigma^2/2")
        .text(" is the arithmetic-versus-geometric gap in exact form — the mean grows at ")
        .math(r"\mu")
        .text(" while the median grows at ")
        .math(r"\mu-\sigma^2/2")
        .text(", so ")
        .math(r"\mu")
        .text(" is not a return anyone actually earns; it is the average across paths, carried by an increasingly thin tail."));
    b = b.para(|p| p
        .text("Three shorter consequences of the same multiplication: recovery asymmetry, the gain that undoes a loss ")
        .math(r"d")
        .text(" is ")
        .math(r"1/(1-d)-1")
        .text(", not ")
        .math(r"d")
        .text("; power-law tails, doubling the size of a market move divides its frequency by about 8, not by the vastly larger factor a normal distribution would demand out in the tail; and Kelly staking, which is maximising the exponent — a bet paying +50% or -40% on a fair coin staked at full size has a geometric mean of -5.13% per bet, a positive-expectation bet that reliably goes broke, while the Kelly fraction of 25% turns the same bet into steady long-run growth."));
    b = b.para(|p| p
        .text("Applied — physical: the exponent 1/2 in this layer is diffusion. Einstein's 1905 result is that the typical distance a randomly bombarded particle travels from its start is proportional to the square root of time — the exact same theorem as ")
        .math(r"\sigma\surd{T}")
        .text(", not a financial convention borrowed by analogy. Toy numbers: a drunk leaves a lamppost and takes 100 one-metre steps, each north or south by a coin flip. The average displacement is genuinely 0, but the average distance is ")
        .math(r"100^{1/2}=10")
        .text(" m; four hundred steps gives 20 m, not 40 — four times the time buys twice the distance."));

    b = b.heading("Layer 8 — Exponents beyond the real line");
    b = b.para(|p| p
        .text("The last extension of the ladder, and the first one that is not forced. Euler's formula, ")
        .math(r"e^{i\theta}=\cos\theta+i\sin\theta")
        .text(", gives the identity ")
        .math(r"e^{i\pi}=-1")
        .text(". With complex exponents, ")
        .math(r"b^z")
        .text(" becomes multivalued, because the complex logarithm is — an angle is defined only up to a full turn — so naming \"the\" value requires choosing a branch. Every earlier extension had exactly one candidate and the laws picked it; here the laws permit infinitely many, and a convention has to do the work. Which laws survive the branch cut and which die is clean: the addition laws (")
        .math(r"z^az^b=z^{a+b}")
        .text(") survive exactly, while power-of-a-power and power-of-a-product fail, with explicit correction factors — the very law that did all the earlier forcing is the one that breaks at the end of the ladder."));
    b = b.para(|p| p
        .text("Applied — physical: the mental picture is one sentence — multiplying by a real number bigger than one stretches, multiplying by a number of size one turns. Imagine — the capacitor that refuses to warm up. An ideal capacitor sits in a mains circuit with a real current flowing and a real voltage across it. Put a hand on it and predict the warmth: power is current times voltage, both nonzero, so surely it gets hot."));
    b = b.rule();
    b = b.note("Before reading on: will the capacitor warm up?");
    b = b.para(|p| p
        .text("It produces exactly zero heat. At 50 Hz the current leads the voltage by exactly a quarter cycle, five milliseconds, so their product is positive for half of each cycle and negative for the other half, averaging to nothing: energy flows in and straight back out. Engineers write the impedance as imaginary, and the ")
        .math(r"i")
        .text(" is not an apology for a number that does not exist — it is five milliseconds, and you could time it with a stopwatch. Damped oscillation is one complex exponent doing two jobs at once: the real part of the exponent shrinks, the imaginary part turns — a plucked guitar string, a struck bell, a swinging door easing shut, and a black hole's ringdown are all this one formula."));
    b = b.para(|p| p
        .text("Applied — finance, an honest verdict: nothing here is reachable at this lesson's level, and the layer should be taken as theory with a single pointer. Writing a complex number as size-and-turn, ")
        .math(r"\rho e^{i\theta}")
        .text(", and raising it to the power ")
        .math(r"k")
        .text(" gives ")
        .math(r"\rho^ke^{ik\theta}")
        .text(" — the modulus decays geometrically while the angle accumulates, a damped cycle. When a fitted time-series model's roots come out complex, that is exactly what the model is saying about the series — a hand-off to a time-series lesson, not lesson material here."));

    b = b.heading("Layer 9 — Exponentiation as an abstract operation");
    b = b.para(|p| p
        .text("Strip away the numbers and the definition still works: in any group, ")
        .math(r"g^n")
        .text(" is the n-fold product starting from the identity. This setting shows precisely which laws were about counting and which were about numbers: ")
        .math(r"g^{m+n}=g^mg^n")
        .text(" and ")
        .math(r"\left(g^m\right)^n=g^{mn}")
        .text(" hold in any associative structure, no commutativity anywhere — but ")
        .math(r"(gh)^n=g^nh^n")
        .text(" needs commutativity. The first two laws only ever re-bracket; the third re-orders, and re-ordering is the only thing commutativity was ever paying for. Sorting ")
        .math(r"(gh)^n")
        .text(" into ")
        .math(r"g^nh^n")
        .text(" costs exactly ")
        .math(r"\binom n2")
        .text(" adjacent swaps — the same ")
        .math(r"\binom n2")
        .text(" that multiplies ")
        .math(r"r^2")
        .text(" in the binomial expansion of ")
        .math(r"(1+r)^n")
        .text(", because both are counts of pairs chosen from ")
        .math(r"n")
        .text(" things."));
    b = b.para(|p| p
        .text("Analogy — turn the book two ways. Do this with your hands. Pick up a book. Rotate it 90° about the vertical axis, then 90° about the horizontal axis pointing away from you, and note which way the cover faces. Put it back, and commit to a prediction before repeating the two turns in the other order — surely turns just add up."));
    b = b.rule();
    b = b.note("Now do it. Which way does the cover face this time -- the same, or different?");
    b = b.para(|p| p
        .text("The book ends up genuinely differently oriented. You have just disproved ")
        .math(r"(gh)^n=g^nh^n")
        .text(" with your wrists. Turning ")
        .math(r"(gh)^3=ghghgh")
        .text(" into ")
        .math(r"g^3h^3=ggghhh")
        .text(" means moving every ")
        .math(r"h")
        .text(" rightwards past every ")
        .math(r"g")
        .text(" that follows it — exactly ")
        .math(r"\binom32=3")
        .text(" swaps. Where the analogy breaks down: only one of the three laws dies — the other two hold in any associative structure with no commutativity anywhere, which is exactly why a transition-matrix power still obeys ")
        .math(r"P^{m+n}=P^mP^n")
        .text("."));
    b = b.figure(Figure::new(ILL7_SVG, "Three swaps, and what commutativity was paying for: turning (gh)^3 = ghghgh into g^3h^3 = ggghhh takes three adjacent swaps, each sliding one h rightwards past one g. The price of the re-ordering is exactly C(3,2)=3, the same number sitting in front of the r^2 term of (1+r)^3, because both are counts of pairs chosen from n=3 things."));
    b = b.para(|p| p
        .text("Applied — physical: rotations are the everyday case, ")
        .math(r"R(\theta)^n=R(n\theta)")
        .text(". Turn a square tile by 90° and four turns is the identity, so turning back once is the same physical act as turning forward three times. That finite list is a law of crystals: the crystallographic restriction theorem allows only 1-, 2-, 3-, 4- and 6-fold rotational symmetry in a periodic lattice, which is why no ordinary crystal has five-fold symmetry, and why Dan Shechtman's ten-fold diffraction pattern in 1982 was met with disbelief before earning him the 2011 Nobel Prize in Chemistry."));
    b = b.para(|p| p
        .text("Applied — finance: raising a Markov transition matrix to a power, which is how multi-period credit-rating migration and default probabilities are produced — not an analogy, prescribed in J.P. Morgan's CreditMetrics Technical Document (1997). ")
        .math(r"P^{m+n}=P^mP^n")
        .text(" is the Chapman–Kolmogorov equation, the product rule for exponents in a non-numeric setting, holding because matrix multiplication is associative with no commutativity required anywhere."));
    b = b.para(|p| p
        .text("Toy numbers (survival): two states, A and Default, with a 10% one-year default probability. The two-year default probability is not 20% — survival compounds, ")
        .math(r"0.9^2=0.81")
        .text(", so it is 19%: you can only default once, and naive doubling counts \"defaults in year 1 and defaults in year 2\" twice."));
    b = b.para(|p| p
        .text("Toy numbers (migration): three states A, B, D with A's row ")
        .math(r"(0.90,0.08,0.02)")
        .text(", B's row ")
        .math(r"(0.10,0.80,0.10)")
        .text(", D absorbing. An A-rated borrower's two-year default probability is not ")
        .math(r"2\times2\%=4\%")
        .text(": multiplying out the A row of ")
        .math(r"P^2")
        .text(" gives A→D of 4.6% — 2.0% default in year one, plus 1.8% that stay A and then default, plus 0.8% downgraded to B and then default from there, a path the naive answer had no way to see. A power enumerates every path, and the cross terms are the paths nobody counted by hand — the same anatomy as ")
        .math(r"(1+r)^n")
        .text("'s binomial expansion, in another setting."));

    b = b.heading("Layer 10 — Up the ladder, and the numbers exponentiation manufactures");
    b = b.para(|p| p
        .text("The frontier, and the layer that puts exponentiation in its place. This layer is theory — it has no application, and that is a finding rather than an omission: both the physical and the financial lenses were searched deliberately and both came back empty. Exponentiation is rung four of the hyperoperation ladder — successor, addition, multiplication, exponentiation, tetration, pentation, each rung iterating the one below. Commutativity and associativity both die at exponentiation, not one rung later as the usual story has it: ")
        .math(r"2^3=8\neq9=3^2")
        .text(" and ")
        .math(r"\left(2^3\right)^2=64\neq512=2^{3^2}")
        .text(". What dies at the next rung, tetration, is the laws that survived this far — and they die catastrophically: a product-rule analogue at tetration would predict a two-digit number where the true value has 19,729 digits."));
    b = b.para(|p| p
        .text("Knuth's up-arrow notation (1976) names the rungs above ordinary exponentiation — it was needed because combinatorics had produced numbers exponentiation could no longer write down, and Graham's number is what it was built for: at introduction, the largest specific integer ever used in a published proof. The Gelfond–Schneider theorem (1934) says that an algebraic number (other than 0 or 1) raised to an algebraic irrational power is transcendental, settling the seventh of Hilbert's 1900 problems and making ")
        .math(r"2^{\surd2}")
        .text(" and ")
        .math(r"e^\pi")
        .text(" transcendental — in a 1919 lecture, Hilbert ranked the transcendence of ")
        .math(r"2^{\surd2}")
        .text(" as harder than the Riemann hypothesis and did not expect anyone in the hall to live to see it proved; it fell in 1934."));
    b = b.para(|p| p
        .text("No application, stated plainly: in finance there is nothing at all — no interest convention, no pricing model, no risk measure and no estimator iterates exponentiation. Nothing in finance is built above rung four of this ladder. You now know where the subject's ceiling is, and that it was not arbitrary."));

    b = b.heading("Layer 11 — The doors this lesson leaves shut");
    b = b.para(|p| p
        .text("Every item here is a door you are now equipped to walk through. Logarithms as a subject in their own right — developed here only far enough to solve the doubling-time equation, justify log returns, and read a log-scaled chart. The calculus of exponentials — ")
        .math(r"d/dx\,e^x=e^x")
        .text(", flagged where it explains why e is called natural, but never developed, because continuous compounding is reachable as a limit of ordinary compounding without it. Itô calculus, which is where layer 7's geometric-Brownian-motion formula actually comes from, and the ")
        .math(r"-\sigma^2/2")
        .text(" with it. Complex exponentiation beyond layer 8's stated facts. Full derivations of the financial models named but not opened — Black–Scholes, GARCH-family estimation by maximum likelihood, and the numerical root-finders that actually solve for IRR and YTM."));
    b = b.para(|p| p
        .text("Why the no-calculus route worked, and exactly what it cost: the compounding limit is a limit of a sequence of ordinary numbers, and everything needed to show it exists is algebra this lesson already owns — the binomial theorem, Bernoulli's inequality, AM–GM — plus one property of the real numbers, completeness, paid for once and paid for early. Calculus is not the short road to e; it is the longer road textbooks happen to pave, because they want the derivative afterwards anyway. What you are left unable to prove, named exactly: that ")
        .math(r"d/dx\,e^x=e^x")
        .text(", the entire content of the word \"natural\"; that continuous compounding solves the instantaneous-rate model rather than merely equalling its limit; the exact size (not the existence) of the arithmetic-versus-geometric gap; and the exact size of the Rule-of-72 correction. In every case the usable statement survives and only the derivation is withheld — and in both finance cases the direction of the error is provable even when its magnitude is not."));
    b = b.para(|p| p
        .text("If you read one sequel, read this one: concavity and Jensen's inequality, the logarithm hand-off — it bites immediately and silently, the moment you average log returns and call the result \"the average return\" you have reported the geometric mean without knowing it. Then GARCH-family estimation, then numerical root-finding for YTM and IRR, then Itô calculus last."));

    // ===================================================================
    // Part three — The sixteen equations, in one place
    // ===================================================================
    b = b.rule();
    b = b.heading("Part three — The sixteen equations, in one place");
    b = b.para(|p| p
        .text("Reference material — everything here has already been taught above; this is the collected sheet, ordered so each equation is built from the one before it. Two standing conditions govern every line: the exponent counts periods and carries no units, so the rate in the base must be quoted per that same period; and compounding is a contractual convention, not a law of nature — where the contract freezes the referent, simple interest is the correct model and these formulas are the wrong tool rather than a better one."));
    b = b.para(|p| p
        .text("1. The growth factor, ")
        .math(r"P_t=P_{t-1}(1+r_t)")
        .text(" — the translation step everything stands on. 2. Compound growth, ")
        .math(r"P_t=P_0(1+r)^t")
        .text(" — the forward engine. 3. Compounding frequency, ")
        .math(r"\mathrm{APY}=(1+r/n)^n-1")
        .text(", tending to ")
        .math(r"e^r-1")
        .text(" — makes rate quotes commensurable. 4. Discounting, ")
        .math(r"PV=FV(1+r)^{-t}")
        .text(" — the negative exponent runs time backwards; it is not a negative amount."));
    b = b.para(|p| p
        .text("5. A stream of payments, ")
        .math(r"PV=\mathrm{PMT}\cdot\left(1-(1+r)^{-n}\right)/r")
        .text(" — the geometric series collapsed; covers annuities, mortgages, bond prices and pensions in one formula. 6. CAGR, ")
        .math(r"\mathrm{CAGR}=(V_T/V_0)^{1/T}-1")
        .text(" — the fractional exponent earning its keep, the geometric mean of the growth factors. 7. Log returns, ")
        .math(r"\ell_t=\ln(P_t/P_{t-1})")
        .text(" — the change of units under which a price series becomes something a statistical model can work on."));
    b = b.para(|p| p
        .text("8. Arithmetic versus geometric mean, ")
        .math(r"(1+g)^n=\prod_t(1+R_t)")
        .text(", with ")
        .math(r"g\le\bar R")
        .text(" always — AM–GM is exact and unconditional, half-the-variance is an approximation. 9. Volatility scaling, ")
        .math(r"\sigma_h=\sigma_1\,h^{1/2}")
        .text(", so ")
        .math(r"\sigma_{\mathrm{ann}}=\sigma_{\mathrm{daily}}\times252^{1/2}")
        .text(". 10. EWMA, ")
        .math(r"\sigma_t^2=\lambda\sigma_{t-1}^2+(1-\lambda)r_{t-1}^2")
        .text(", the exponent applied to time rather than money. 11. EMA smoothing, ")
        .math(r"S_t=\alpha P_t+(1-\alpha)S_{t-1}")
        .text(" with ")
        .math(r"\alpha=2/(N+1)")
        .text(" — the same decay, differently parameterised."));
    b = b.para(|p| p
        .text("12. GARCH(1,1), ")
        .math(r"\sigma_t^2=\omega+\alpha\epsilon_{t-1}^2+\beta\sigma_{t-1}^2")
        .text(", a shock's influence ")
        .math(r"k")
        .text(" steps later proportional to ")
        .math(r"(\alpha+\beta)^k")
        .text(". 13. The lognormal price model, ")
        .math(r"S_t=S_0\exp\left[(\mu-\sigma^2/2)t+\sigma W_t\right]")
        .text(" — everything inside an exponential, so the price can approach zero but never pass through it. 14. Power-law tails, ")
        .math(r"P(|r|>x)\sim x^{-\alpha}")
        .text(" with ")
        .math(r"\alpha\approx3")
        .text(" — the one place the exponent sits on the variable rather than on time."));
    b = b.para(|p| p
        .text("15. Costs compound too, ")
        .math(r"W_T=W_0\left[(1+r)(1-f)\right]^T")
        .text(" — the power-of-a-product rule with money on it, licensing a fee as its own compounding factor rather than a subtraction from the return. 16. Real versus nominal, ")
        .math(r"1+i=(1+r)(1+\pi)")
        .text(" — the exact Fisher relation, multiplying because two successive rescalings of purchasing power compose the way any two growth factors do. A rounding error in the base becomes a real error once an exponent gets hold of it."));

    // ===================================================================
    // Where all this came from
    // ===================================================================
    b = b.rule();
    b = b.heading("Where all this came from");
    b = b.para(|p| p
        .text("The order in which the idea and its notation actually arrived. The word came first; the symbol came two thousand years later. Euclid's Greek term for the square of a line segment gave English \"power\". Archimedes proved the first law of exponents around 250 BC — in The Sand Reckoner he effectively stated and proved ")
        .math(r"10^a\cdot10^b=10^{a+b}")
        .text(" while building a number system large enough to bound the grains of sand that would fill the universe."));
    b = b.para(|p| p
        .text("The oldest known exponential equation is a finance problem — the Babylonian tablet that opened this lesson, somewhere between 2000 and 1700 BC. Medieval Arabic algebra named powers rather than numbering them. Nicolas Chuquet's Le Triparty (1484) used raised numerals and was arguably the first to use zero and negative numbers as exponents. Michael Stifel coined the word \"exponent\" in Arithmetica integra (1544). Robert Recorde's Whetstone of Witte (1557) had to name every power for lack of general notation — square, cube, zenzizenzic (4th), sursolid (5th), all the way to zenzizenzizenzic (8th) — the absurdity of the vocabulary is the argument for notation. René Descartes' La Géométrie (1637) wrote raised ordinary decimal numerals, the notation that became universal."));
    b = b.para(|p| p
        .text("Compound interest was itself contested history: it was formerly called anatocism, and charging it was condemned by Roman law as the worst kind of usury. Luca Pacioli's Summa de arithmetica (1494) gave systematic treatments of simple and compound interest and stated the Rule of 72, in the same book that codified double-entry bookkeeping. Richard Witt's Arithmeticall Questions (1613) was the first book wholly devoted to compound interest. Napier's logarithms (1614) were invented to turn multiplication into addition, and Jacob Bernoulli found e in a compound-interest question in 1683, asking what happens to $1 at 100% annual interest as the compounding is split ever finer — he never named the number. Euler named and analysed it in Introductio in analysin infinitorum (1748), computing it to 18 decimal places and making the exponential a function of a continuously varying exponent rather than a table of powers."));
    b = b.para(|p| p
        .text("Gelfond and Schneider settled the seventh of Hilbert's 1900 problems in 1934. Donald Knuth introduced up-arrow notation in 1976, extending the ladder past exponentiation because combinatorics had produced numbers exponentiation could no longer write down. And J.P. Morgan's RiskMetrics (1994) put exponential weighting into daily risk practice with λ=0.94 — an exponent applied to time rather than to money."));

    // ===================================================================
    // Eight things worth repeating at dinner
    // ===================================================================
    b = b.rule();
    b = b.heading("Eight things worth repeating at dinner");
    b = b.para(|p| p
        .text("The oldest exponential equation we know of is a loan problem — and its author got it wrong in exactly the modern way: three years 283 days on the tablet against a true three years 288 days, exponential growth bias, four thousand years before it was named."));
    b = b.para(|p| p
        .text("\"Compound interest is the eighth wonder of the world\" is not an Einstein quote. It first appears in print in a 1983 advertisement — 28 years after his death — and appears nowhere in his collected papers. The most-cited endorsement of exponential growth in finance is a fabrication."));
    b = b.para(|p| p
        .text("The chessboard would bankrupt the world. One grain on the first square, doubling each square, totals ")
        .math(r"2^{64}-1=18{,}446{,}744{,}073{,}709{,}551{,}615")
        .text(" grains — roughly 2,800 times the world's annual wheat production. The legend is first recorded by Ibn Khallikan in 1256."));
    b = b.para(|p| p
        .text("Fold a sheet of paper 42 times and it reaches the Moon. At 0.1 mm per sheet, 30 folds passes the 100 km edge of space, and 42 folds is about 439,800 km against the Moon's average 384,400 km. Real paper folds seven or eight times."));
    b = b.para(|p| p
        .text("Before exponent notation, the 8th power had to be called zenzizenzizenzic — Recorde, 1557, \"the square of squares squaredly\". It is now best known as the English word with the most z's. Descartes' raised numeral, eighty years later, retired the entire vocabulary."));
    b = b.para(|p| p
        .text("A googol was named by a nine-year-old. Edward Kasner asked his nephew Milton Sirotta for a name for ")
        .math(r"10^{100}")
        .text(" during a walk in 1938; \"googolplex\" followed. Google is a misspelling of the word."));
    b = b.para(|p| p
        .text("Hyperinflation is exponential growth you can watch in real time. At Zimbabwe's November 2008 peak, prices doubled roughly every 24.7 hours; in Hungary in 1945–46, the record, about every 15 hours."));
    b = b.para(|p| p
        .text("A musical semitone is a fractional exponent you can hear. Equal temperament multiplies frequency by ")
        .math(r"2^{1/12}\approx1.0595")
        .text(" per semitone — about 5.95%, roughly a year of decent equity returns — and twelve of them compound to exactly 2. Same arithmetic as a savings account, running at twelve compounding periods per doubling."));

    // ===================================================================
    // What you can now re-derive
    // ===================================================================
    b = b.rule();
    b = b.heading("What you can now re-derive");
    b = b.para(|p| p
        .text("This is the test the whole lesson was built to pass: not can you recall the rules, but can you regenerate them. From idea 1's headcount: the product, quotient, power-of-a-power, power-of-a-product and power-of-a-quotient rules; why the product rule needs equal bases and power-of-a-product needs equal exponents; why (a+b)^n is not a^n+b^n; the binomial theorem and Pascal's triangle; Bernoulli's inequality; and why exponentiation is neither commutative nor associative."));
    b = b.para(|p| p
        .text("From idea 2's start-at-1: ")
        .math(r"b^0=1")
        .text(" and ")
        .math(r"1^x=1")
        .text("; ")
        .math(r"b^{-n}=1/b^n")
        .text(" and why ")
        .math(r"0^{-n}")
        .text(" is undefined; and discounting as the ladder walked backwards. From idea 3's forcing: ")
        .math(r"b^{1/n}")
        .text(" as the nth root; why non-integer exponents need ")
        .math(r"b>0")
        .text("; CAGR and volatility-annualising as forced fractional exponents; and why complex exponents are multivalued."));
    b = b.para(|p| p
        .text("From idea 4's equal steps: ")
        .math(r"b^{x+y}=b^xb^y")
        .text(" as the definition itself, not just a law; why ")
        .math(r"b^x>0")
        .text(" and asymptotic to zero; the power-versus-exponential distinction; doubling time and half-life; the Rule of 72; and why log returns add while simple returns do not. From idea 5's factors-not-addends: \"up 8%\" is ×1.08; the exponent counts periods, never years; recovery asymmetry; why returns are governed by the geometric mean; and why fees and inflation compound rather than subtract."));
    b = b.para(|p| p
        .text("And now the promise made at the top can be settled. The scribe had the right question, the right two brackets, and a ruler — and the ruler was the entire error, because a ruler adds equal amounts across a year that multiplies. Since then you have watched the same move fail on a piano tuned by division, on a mug of coffee killed ten minutes early, on a month-old return declared out of the sample, on a fund told it needed +40% back, and on about a third of a representative sample of American adults who had calculators to hand. They are one habit wearing five costumes, and you were the one who caught them."));
    b = b.note("Count the factors; do not add the amounts.");

    // ===================================================================
    // Further notes (appendix)
    // ===================================================================
    b = b.rule();
    b = b.heading("Further notes");
    b = b.para(|p| p
        .text("Appendix. Genuine material kept out of the taught path on purpose — either a tangent the spine does not need, or depth that belongs beside the reader rather than inside the argument."));
    b = b.heading("Speaking in exponents");
    b = b.para(|p| p
        .text("Scientific notation writes a number as ")
        .math(r"a\times10^n")
        .text(" with ")
        .math(r"1\le|a|<10")
        .text(", so two numbers can be compared by exponent first. Engineering notation restricts ")
        .math(r"n")
        .text(" to multiples of 3, mapping onto the SI prefixes — kilo, mega, giga, milli, micro. Moore's law — transistor density doubling every 18 to 24 months since the mid-1960s — is the canonical technological exponential, and the reason the word \"exponential\" entered business vocabulary at all. Powers of two run through computing: ")
        .math(r"2^{10}=1024")
        .text(", address spaces, hash-table sizes. And computational complexity is a verdict written as an exponent: ")
        .math(r"O(n^2)")
        .text(" against ")
        .math(r"O(2^n)")
        .text(" is the difference between a hard problem and an impossible one."));
    b = b.heading("Sundry facts and cautions kept for reference");
    b = b.para(|p| p
        .text("A beginner-level non-commuting pair, in money: with g = \"grow by 10%\" and f = \"deposit £100\", the product rule still holds, but grow-then-deposit twice reaches £210 while growing twice and depositing twice reaches £200 — the £10 gap is the interest the first deposit earned, and it is precisely why an annuity is a geometric series rather than a lump sum times a power."));
    b = b.para(|p| p
        .text("The lognormal mean-to-median ratio ")
        .math(r"e^{s^2/2}")
        .text(" is the arithmetic-versus-geometric factor in exact form. The textbook claim that YTM assumes coupons are reinvested at the YTM rate is challenged in the literature — YTM may simply be the single discount rate equating price to discounted cash flows, with no reinvestment condition required. Credit card and payday debt are the same exponent with the sign of ownership reversed — compounding at 20–30% APR working against the borrower."));
    b = b.para(|p| p
        .text("A provenance flag worth honouring: the claim that a photocopier's \"90% reduction\" scales each side length, so area falls as the square, was supplied during this lesson's construction rather than taken from a cited source — used in [ill-8] as a statement about exponents, and worth independently sourcing before any built version asserts it as a fact about copiers specifically."));
    b = b.heading("Open questions recorded rather than resolved");
    b = b.para(|p| p
        .text("Whether exponential growth bias is innate or installed by early schooling in linear functions is explicitly unresolved in the literature. Whether \"volatility drag\" is the right name for the arithmetic-versus-geometric gap is contested — the mathematics is not in dispute, the framing is; this lesson uses the arithmetic-versus-geometric framing throughout and flags the popular name as contested. And how much logarithm a zero-to-hero reader needs before the finance layers land is an open scoping question this lesson resolved narrowly, teaching only \"an exponent read backwards\" plus three working uses."));

    // ===================================================================
    // Sources
    // ===================================================================
    b = b.rule();
    b = b.heading("Sources");
    b = b.para(|p| p
        .text("Everything asserted in this lesson traces to a 320-entry source list spanning history and notation; the laws and their proofs; foundations and completeness; the functional equation and Cauchy's characterisation; e and continuous compounding; logarithms and log scales; compound interest, annuities and bond mathematics; volatility, EWMA and GARCH; floating point and IEEE 754; complex exponentiation; abstract algebra and matrix exponentials; hyperoperations and transcendence; the education-research literature on exponent errors; and the legal and regulatory material on capitalisation. Where a claim is contested — the \"volatility drag\" naming, Kleiber's exponent, the YTM reinvestment assumption, the root cause of exponent errors — both sides are on record; where a claim is a measurement rather than a theorem — the 252 trading days, λ=0.94, the IEEE exponent width, the referent of a percentage — the primary source and date are on record too, because a measurement can change."));

    let lesson = b.build();

    // The audit call belongs inside the assert: left outside it, a release
    // build would still compile every formula and then throw the answer
    // away. This is a one-shot build (advisor/auditor/polish waived), so the
    // finding list is not guaranteed empty — see the build report.
    debug_assert!(
        lesson.audit().is_empty(),
        "math errors, unexplained terms or unusable curves: {:?}",
        lesson.audit()
    );

    lesson.show()
}

/// [ill-1] Galileo's two cubes, drawn to scale.
const ILL1_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 312" font-family="sans-serif" font-size="12">
<text x="200" y="18" text-anchor="middle" fill="#475569">Same cube, every length doubled (k = 2)</text>
<line x1="16" y1="230" x2="310" y2="230" stroke="#94a3b8" stroke-width="1.5"/>
<g stroke="#334155" stroke-width="1.5" stroke-linejoin="round">
  <polygon points="30,180 80,180 98,162 48,162" fill="#dde5ec"/>
  <polygon points="80,180 80,230 98,212 98,162" fill="#c9d5e0"/>
  <rect x="30" y="180" width="50" height="50" fill="#f2f6fa"/>
  <polygon points="150,130 250,130 286,94 186,94" fill="#dde5ec"/>
  <polygon points="250,130 250,230 286,194 286,94" fill="#c9d5e0"/>
  <rect x="150" y="130" width="100" height="100" fill="#f2f6fa"/>
</g>
<g stroke="#0f766e" stroke-width="1">
  <line x1="200" y1="230" x2="200" y2="130"/>
  <line x1="150" y1="180" x2="250" y2="180"/>
  <line x1="200" y1="130" x2="236" y2="94"/>
  <line x1="168" y1="112" x2="268" y2="112"/>
  <line x1="250" y1="180" x2="286" y2="144"/>
  <line x1="268" y1="212" x2="268" y2="112"/>
</g>
<text x="294" y="124" fill="#0f766e" font-size="11">count 4 squares</text>
<text x="294" y="138" fill="#0f766e" font-size="11">on each face</text>
<text x="294" y="172" fill="#b45309" font-size="11">count 8 cubes</text>
<text x="294" y="186" fill="#b45309" font-size="11">in the body</text>
<text x="64" y="250" text-anchor="middle" fill="#334155">side 1 cm</text>
<text x="64" y="266" text-anchor="middle" fill="#0f766e" font-size="11">surface 6 cm²</text>
<text x="64" y="280" text-anchor="middle" fill="#b45309" font-size="11">volume 1 cm³</text>
<text x="218" y="250" text-anchor="middle" fill="#334155">side 2 cm</text>
<text x="218" y="266" text-anchor="middle" fill="#0f766e" font-size="11">surface 24 cm²  ×4 = k²</text>
<text x="218" y="280" text-anchor="middle" fill="#b45309" font-size="11">volume 8 cm³  ×8 = k³</text>
<text x="200" y="302" text-anchor="middle" fill="#475569">weight ×8 ÷ cross-section ×4  =  <tspan fill="#be123c" font-weight="600">stress ×2</tspan></text>
</svg>"##;

/// [ill-2] Where the cross terms live.
const ILL2_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 400" font-family="sans-serif" fill="#1c2530">
  <text x="200" y="26" text-anchor="middle" font-size="13">side 3 + 4 = 7 · whole square 7² = 49</text>
  <rect x="70" y="60" width="120" height="120" fill="#dce4f0" stroke="#4a6a99"/>
  <rect x="190" y="180" width="160" height="160" fill="#dce4f0" stroke="#4a6a99"/>
  <rect x="190" y="60" width="160" height="120" fill="#f2b134" stroke="#8a5a00" stroke-width="2.5"/>
  <rect x="70" y="180" width="120" height="160" fill="#f2b134" stroke="#8a5a00" stroke-width="2.5"/>
  <rect x="70" y="60" width="280" height="280" fill="none" stroke="#1c2530" stroke-width="2"/>
  <text x="130" y="125" text-anchor="middle" font-size="14">3²  =  9</text>
  <text x="270" y="125" text-anchor="middle" font-size="14">3 × 4  =  12</text>
  <text x="130" y="265" text-anchor="middle" font-size="14">3 × 4  =  12</text>
  <text x="270" y="265" text-anchor="middle" font-size="14">4²  =  16</text>
  <g stroke="#1c2530" stroke-width="1">
    <path d="M70 52v8 M190 52v8 M350 52v8"/>
    <path d="M62 60h8 M62 180h8 M62 340h8"/>
  </g>
  <text x="130" y="48" text-anchor="middle" font-size="13">3</text>
  <text x="270" y="48" text-anchor="middle" font-size="13">4</text>
  <text x="56" y="124" text-anchor="end" font-size="13">3</text>
  <text x="56" y="264" text-anchor="end" font-size="13">4</text>
  <rect x="72" y="356" width="13" height="13" fill="#dce4f0" stroke="#4a6a99"/>
  <text x="93" y="367" font-size="12">kept by the dream:  9 + 16  =  25</text>
  <rect x="72" y="376" width="13" height="13" fill="#f2b134" stroke="#8a5a00" stroke-width="2"/>
  <text x="93" y="387" font-size="12">thrown away:  12 + 12  =  24</text>
</svg>"##;

/// [ill-8] The photocopier's two buttons.
const ILL8_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 420 300" font-family="sans-serif" role="img" aria-label="Three sheets of paper nested at a shared top-left corner: the original, the sheet after seven passes at 90 percent scaled to 0.478 of the side, and the sheet after twelve passes scaled to 0.282 of the side.">
  <text x="210" y="24" text-anchor="middle" font-size="13" font-weight="600" fill="#1e293b">One dial reading 90%. Two buttons.</text>
  <text x="210" y="42" text-anchor="middle" font-size="11" fill="#64748b">the dial is the base; the number of passes is the exponent</text>
  <rect x="40" y="60" width="150" height="190" fill="#f1f5f9" stroke="#64748b" stroke-width="1.5"/>
  <rect x="40" y="60" width="71.74" height="90.88" fill="#dbeafe" stroke="#1d4ed8" stroke-width="1.5"/>
  <rect x="40" y="60" width="42.36" height="53.66" fill="#fed7aa" stroke="#c2410c" stroke-width="1.5"/>
  <g stroke-width="1" fill="none">
    <line x1="82.36" y1="86" x2="230" y2="86" stroke="#c2410c"/>
    <line x1="111.74" y1="134" x2="230" y2="134" stroke="#1d4ed8"/>
    <line x1="190" y1="204" x2="230" y2="204" stroke="#64748b"/>
  </g>
  <g font-size="12" font-weight="600">
    <text x="236" y="82" fill="#c2410c">12 passes — batch of 3, ×4</text>
    <text x="236" y="130" fill="#1d4ed8">7 passes — 3, then 4 more</text>
    <text x="236" y="200" fill="#64748b">original · 0.9⁰ = 1</text>
  </g>
  <g font-size="11" fill="#475569">
    <text x="236" y="99">3 × 4 = 12 · side ×0.282 · area ×0.080</text>
    <text x="236" y="147">3 + 4 = 7 · side ×0.478 · area ×0.229</text>
    <text x="236" y="217">no passes yet · side ×1 · area ×1</text>
  </g>
  <line x1="40" y1="262" x2="380" y2="262" stroke="#cbd5e1" stroke-width="1"/>
  <text x="210" y="280" text-anchor="middle" font-size="12" fill="#1e293b">one more factor ADDS to the tally · one more group MULTIPLIES it</text>
  <text x="44" y="296" font-size="11" fill="#1d4ed8">passes ADD:  x²·x³ = x⁵</text>
  <text x="376" y="296" text-anchor="end" font-size="11" fill="#c2410c">batches MULTIPLY:  (x²)³ = x⁶</text>
</svg>"##;

/// [ill-3] The ladder with no seam.
const ILL3_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 300" role="img" aria-label="Six equally spaced rungs for exponents 3, 2, 1, 0, minus 1, minus 2, with bars of length 8, 4, 2, 1, one half and one quarter drawn to scale; each bar is exactly half the one above, including across the highlighted 2 to the 0 equals 1 rung.">
  <g font-family="sans-serif" fill="#1f2933">
    <text x="8" y="22" font-size="13" font-weight="bold">Powers of 2, drawn to scale</text>
    <text x="8" y="38" font-size="11" fill="#6b7280">equal steps down · half the value every time</text>
    <rect x="8" y="166" width="384" height="28" fill="#fdf0d5"/>
    <line x1="92" y1="48" x2="92" y2="266" stroke="#d0d5dc" stroke-width="1"/>
    <g fill="#2f6fb0">
      <rect x="92" y="51" width="256" height="18"/>
      <rect x="92" y="91" width="128" height="18"/>
      <rect x="92" y="131" width="64" height="18"/>
      <rect x="92" y="171" width="32" height="18"/>
      <rect x="92" y="211" width="16" height="18"/>
      <rect x="92" y="251" width="8" height="18"/>
    </g>
    <g font-size="13" text-anchor="end">
      <text x="40" y="64">2³</text>
      <text x="40" y="104">2²</text>
      <text x="40" y="144">2¹</text>
      <text x="40" y="184" fill="#b45309" font-weight="bold">2⁰</text>
      <text x="40" y="224">2⁻¹</text>
      <text x="40" y="264">2⁻²</text>
    </g>
    <g font-size="12">
      <text x="354" y="64">8</text>
      <text x="226" y="104">4</text>
      <text x="162" y="144">2</text>
      <text x="130" y="184" fill="#b45309" font-weight="bold">1</text>
      <text x="114" y="224">½</text>
      <text x="106" y="264">¼</text>
    </g>
    <g font-size="10" fill="#6b7280" text-anchor="middle">
      <text x="64" y="83">÷2</text>
      <text x="64" y="123">÷2</text>
      <text x="64" y="163">÷2</text>
      <text x="64" y="203">÷2</text>
      <text x="64" y="243">÷2</text>
    </g>
    <text x="150" y="184" font-size="11" fill="#b45309">no seam here — just another halving</text>
    <text x="96" y="280" font-size="12" fill="#6b7280" text-anchor="middle">⋮</text>
    <text x="112" y="294" font-size="11" fill="#6b7280">shrinking toward 0, never reaching it</text>
  </g>
</svg>"##;

/// [ill-4] The octave, split twelve ways, twice.
const ILL4_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 420 250" font-family="sans-serif">
  <text x="210" y="20" font-size="13" fill="#333333" text-anchor="middle">Twelve steps up from 440 Hz, on a frequency axis that is linear</text>
  <text x="210" y="38" font-size="12" fill="#555555" text-anchor="middle">scale: x = 40 + (f - 440) × 0.5 px</text>
  <text x="40" y="60" font-size="12" fill="#1560bd">Row A  ×1.05946 = 2^(1/12), twelve times → 880.00 Hz</text>
  <line x1="260" y1="86" x2="260" y2="192" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <text x="260" y="80" font-size="12" fill="#111111" text-anchor="middle">880 Hz — the octave</text>
  <line x1="36" y1="120" x2="406" y2="120" stroke="#555555" stroke-width="1"/>
  <g stroke="#1560bd" stroke-width="2">
    <line x1="40.00" y1="92" x2="40.00" y2="120"/>
    <line x1="53.08" y1="92" x2="53.08" y2="120"/>
    <line x1="66.94" y1="92" x2="66.94" y2="120"/>
    <line x1="81.63" y1="92" x2="81.63" y2="120"/>
    <line x1="97.18" y1="92" x2="97.18" y2="120"/>
    <line x1="113.66" y1="92" x2="113.66" y2="120"/>
    <line x1="131.13" y1="92" x2="131.13" y2="120"/>
    <line x1="149.63" y1="92" x2="149.63" y2="120"/>
    <line x1="169.23" y1="92" x2="169.23" y2="120"/>
    <line x1="189.99" y1="92" x2="189.99" y2="120"/>
    <line x1="212.00" y1="92" x2="212.00" y2="120"/>
    <line x1="235.30" y1="92" x2="235.30" y2="120"/>
    <line x1="260.00" y1="88" x2="260.00" y2="120"/>
  </g>
  <g stroke="#d1590d" stroke-width="2">
    <line x1="40.00" y1="120" x2="40.00" y2="148"/>
    <line x1="58.33" y1="120" x2="58.33" y2="148"/>
    <line x1="78.19" y1="120" x2="78.19" y2="148"/>
    <line x1="99.71" y1="120" x2="99.71" y2="148"/>
    <line x1="123.02" y1="120" x2="123.02" y2="148"/>
    <line x1="148.27" y1="120" x2="148.27" y2="148"/>
    <line x1="175.63" y1="120" x2="175.63" y2="148"/>
    <line x1="205.26" y1="120" x2="205.26" y2="148"/>
    <line x1="237.37" y1="120" x2="237.37" y2="148"/>
    <line x1="272.15" y1="120" x2="272.15" y2="148"/>
    <line x1="309.83" y1="120" x2="309.83" y2="148"/>
    <line x1="350.65" y1="120" x2="350.65" y2="148"/>
    <line x1="394.87" y1="120" x2="394.87" y2="152"/>
  </g>
  <text x="40" y="168" font-size="12" fill="#d1590d">Row B  ×1.08333 (+8.33%) twelve times → 1149.7 Hz</text>
  <text x="40" y="208" font-size="12" fill="#555555" text-anchor="middle">440 Hz</text>
  <text x="398" y="208" font-size="12" fill="#d1590d" text-anchor="end">1149.7 Hz = 440 × 2.613</text>
  <text x="210" y="232" font-size="12" fill="#555555" text-anchor="middle">Equal ratios make unequal gaps on a linear axis.</text>
</svg>"##;

/// [ill-6] Three brackets closing on 2^root2.
const ILL6_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 248" font-family="sans-serif" font-size="12" fill="#222">
  <text x="121.66" y="13" text-anchor="middle" fill="#c0392b">2^√2 = 2.665144</text>
  <text x="121.66" y="25" text-anchor="middle" font-size="10.5" fill="#c0392b">inside all three brackets</text>
  <line x1="121.658" y1="31" x2="121.658" y2="152" stroke="#c0392b" stroke-width="1"/>
  <g fill="#1a5490">
    <rect x="92.900" y="41" width="208.340" height="10"/>
    <rect x="113.140" y="73" width="20.350" height="10"/>
    <rect x="121.225" y="105" width="2.035" height="10"/>
    <text text-anchor="end" font-weight="bold"><tspan x="87.9" y="50">1</tspan><tspan x="108.1" y="82">2</tspan><tspan x="116.2" y="114">3</tspan></text>
  </g>
  <polyline points="124.6,110 150,128 170,128" fill="none" stroke="#1a5490" stroke-width="1"/>
  <text x="174" y="132" font-size="11" fill="#1a5490">bar 3 — 2 px wide on this scale</text>
  <path d="M50 152H380 M50 152v6 M105 152v6 M160 152v6 M215 152v6 M270 152v6 M325 152v6 M380 152v6" fill="none" stroke="#555"/>
  <text y="170" font-size="11" fill="#444" text-anchor="middle"><tspan x="50">2.60</tspan><tspan x="105">2.65</tspan><tspan x="160">2.70</tspan><tspan x="215">2.75</tspan><tspan x="270">2.80</tspan><tspan x="325">2.85</tspan><tspan x="380">2.90</tspan></text>
  <text x="14" y="190" font-size="11" fill="#444">one shared scale — each bracket ≈10× narrower than the last</text>
  <text y="206"><tspan x="14" fill="#1a5490" font-weight="bold">1</tspan><tspan x="30">[2^1.4, 2^1.5] = [2.6390, 2.8284]</tspan><tspan x="300">width 0.1894</tspan></text>
  <text y="222"><tspan x="14" fill="#1a5490" font-weight="bold">2</tspan><tspan x="30">[2^1.41, 2^1.42] = [2.6574, 2.6759]</tspan><tspan x="300">width 0.0185</tspan></text>
  <text y="238"><tspan x="14" fill="#1a5490" font-weight="bold">3</tspan><tspan x="30">[2^1.414, 2^1.415] = [2.66475, 2.66660]</tspan><tspan x="300">width 0.00185</tspan></text>
</svg>"##;

/// [ill-5] Eight ways to earn.
const ILL5_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 264" font-family="sans-serif" font-size="12" text-anchor="middle">
  <title>Eight ways to earn: the 8 choice-words of (1+r)^3 at r = 0.10, sorted by how many years said r</title>
  <text x="200" y="18" font-size="13" font-weight="600" fill="#16233c">(1 + r)³ at r = 0.10 — the 2³ = 8 ways to earn</text>
  <text x="200" y="34" font-size="10" fill="#6b7891">one cell per year, left to right: 1 = leave it alone, r = earn on it</text>
  <g font-size="9" fill="#78849b">
    <text x="62" y="66">principal</text><text x="154" y="66">simple interest</text>
    <text x="246" y="56">interest on</text><text x="246" y="66">interest</text>
    <text x="338" y="46">interest on</text><text x="338" y="56">interest on</text><text x="338" y="66">interest</text>
  </g>
  <g font-size="11" fill="#3a4761"><text x="62" y="82">0 r · 1 way</text><text x="154" y="82">1 r · 3 ways</text><text x="246" y="82">2 r · 3 ways</text><text x="338" y="82">3 r · 1 way</text></g>
  <g font-size="13" font-weight="600" fill="#16233c"><text x="62" y="97">1</text><text x="154" y="97">0.300</text><text x="246" y="97">0.030</text><text x="338" y="97">0.001</text></g>
  <path d="M22 104h80M114 104h80M206 104h80M298 104h80M22 196h356" fill="none" stroke="#dbe1ec"/>
  <g transform="translate(41,112)"><rect width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="7" y="13" fill="#5a6884">1</text><rect x="14" width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="21" y="13" fill="#5a6884">1</text><rect x="28" width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="35" y="13" fill="#5a6884">1</text></g>
  <g transform="translate(133,112)"><rect width="14" height="18" rx="2" fill="#2b6cb0"/><text x="7" y="13" fill="#ffffff">r</text><rect x="14" width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="21" y="13" fill="#5a6884">1</text><rect x="28" width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="35" y="13" fill="#5a6884">1</text></g>
  <g transform="translate(133,138)"><rect width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="7" y="13" fill="#5a6884">1</text><rect x="14" width="14" height="18" rx="2" fill="#2b6cb0"/><text x="21" y="13" fill="#ffffff">r</text><rect x="28" width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="35" y="13" fill="#5a6884">1</text></g>
  <g transform="translate(133,164)"><rect width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="7" y="13" fill="#5a6884">1</text><rect x="14" width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="21" y="13" fill="#5a6884">1</text><rect x="28" width="14" height="18" rx="2" fill="#2b6cb0"/><text x="35" y="13" fill="#ffffff">r</text></g>
  <g transform="translate(225,112)"><rect width="14" height="18" rx="2" fill="#2b6cb0"/><text x="7" y="13" fill="#ffffff">r</text><rect x="14" width="14" height="18" rx="2" fill="#2b6cb0"/><text x="21" y="13" fill="#ffffff">r</text><rect x="28" width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="35" y="13" fill="#5a6884">1</text></g>
  <g transform="translate(225,138)"><rect width="14" height="18" rx="2" fill="#2b6cb0"/><text x="7" y="13" fill="#ffffff">r</text><rect x="14" width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="21" y="13" fill="#5a6884">1</text><rect x="28" width="14" height="18" rx="2" fill="#2b6cb0"/><text x="35" y="13" fill="#ffffff">r</text></g>
  <g transform="translate(225,164)"><rect width="14" height="18" rx="2" fill="#eef1f7" stroke="#ccd6e5"/><text x="7" y="13" fill="#5a6884">1</text><rect x="14" width="14" height="18" rx="2" fill="#2b6cb0"/><text x="21" y="13" fill="#ffffff">r</text><rect x="28" width="14" height="18" rx="2" fill="#2b6cb0"/><text x="35" y="13" fill="#ffffff">r</text></g>
  <g transform="translate(317,112)"><rect width="14" height="18" rx="2" fill="#2b6cb0"/><text x="7" y="13" fill="#ffffff">r</text><rect x="14" width="14" height="18" rx="2" fill="#2b6cb0"/><text x="21" y="13" fill="#ffffff">r</text><rect x="28" width="14" height="18" rx="2" fill="#2b6cb0"/><text x="35" y="13" fill="#ffffff">r</text></g>
  <text x="200" y="216" font-size="13" fill="#16233c">1 + 0.300 + 0.030 + 0.001 = <tspan font-weight="600">1.331</tspan></text>
  <text x="200" y="234" font-size="10" fill="#6b7891">column sizes 1, 3, 3, 1 — the third row of Pascal's triangle</text>
  <text x="200" y="252" font-size="10" fill="#6b7891">simple interest keeps 1 + 0.300 = 1.300 and throws away the 0.031 tail</text>
</svg>"##;

/// [ill-7] Three swaps, and what commutativity was paying for.
const ILL7_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 240" font-family="sans-serif">
<text x="200" y="18" text-anchor="middle" font-size="13" font-weight="600" fill="#1d2b36">(gh)&#179; &#8594; g&#179;h&#179; in three adjacent swaps</text>
<g text-anchor="middle" font-size="9" fill="#8a99a6">
  <text x="100" y="29">1</text><text x="138" y="29">2</text><text x="176" y="29">3</text>
  <text x="214" y="29">4</text><text x="252" y="29">5</text><text x="290" y="29">6</text>
</g>
<g fill="#cfe0f4" stroke="#5a86ad">
  <rect x="84" y="34" width="32" height="28"/><rect x="160" y="34" width="32" height="28"/><rect x="236" y="34" width="32" height="28"/>
  <rect x="84" y="74" width="32" height="28"/><rect x="122" y="74" width="32" height="28"/><rect x="236" y="74" width="32" height="28"/>
  <rect x="84" y="114" width="32" height="28"/><rect x="122" y="114" width="32" height="28"/><rect x="198" y="114" width="32" height="28"/>
  <rect x="84" y="154" width="32" height="28"/><rect x="122" y="154" width="32" height="28"/><rect x="160" y="154" width="32" height="28"/>
</g>
<g fill="#f7dfc0" stroke="#b9865a">
  <rect x="122" y="34" width="32" height="28"/><rect x="198" y="34" width="32" height="28"/><rect x="274" y="34" width="32" height="28"/>
  <rect x="160" y="74" width="32" height="28"/><rect x="198" y="74" width="32" height="28"/><rect x="274" y="74" width="32" height="28"/>
  <rect x="160" y="114" width="32" height="28"/><rect x="236" y="114" width="32" height="28"/><rect x="274" y="114" width="32" height="28"/>
  <rect x="198" y="154" width="32" height="28"/><rect x="236" y="154" width="32" height="28"/><rect x="274" y="154" width="32" height="28"/>
</g>
<g font-size="14" font-weight="600" text-anchor="middle" fill="#1d2b36">
  <text x="100" y="53">g</text><text x="138" y="53">h</text><text x="176" y="53">g</text><text x="214" y="53">h</text><text x="252" y="53">g</text><text x="290" y="53">h</text>
  <text x="100" y="93">g</text><text x="138" y="93">g</text><text x="176" y="93">h</text><text x="214" y="93">h</text><text x="252" y="93">g</text><text x="290" y="93">h</text>
  <text x="100" y="133">g</text><text x="138" y="133">g</text><text x="176" y="133">h</text><text x="214" y="133">g</text><text x="252" y="133">h</text><text x="290" y="133">h</text>
  <text x="100" y="173">g</text><text x="138" y="173">g</text><text x="176" y="173">g</text><text x="214" y="173">h</text><text x="252" y="173">h</text><text x="290" y="173">h</text>
</g>
<g fill="none" stroke="#c0392b" stroke-width="2">
  <rect x="118" y="70" width="78" height="36"/>
  <rect x="194" y="110" width="78" height="36"/>
  <rect x="156" y="150" width="78" height="36"/>
</g>
<g text-anchor="end" font-size="12" font-weight="600" fill="#1d2b36">
  <text x="76" y="53">(gh)&#179;</text>
  <text x="76" y="173">g&#179;h&#179;</text>
</g>
<g font-size="11" fill="#c0392b">
  <text x="314" y="93">swap 1</text>
  <text x="314" y="133">swap 2</text>
  <text x="314" y="173">swap 3</text>
</g>
<text x="200" y="204" text-anchor="middle" font-size="12" fill="#1d2b36">3 adjacent swaps = C(3,2) = 3 pairs chosen from n = 3</text>
<text x="200" y="222" text-anchor="middle" font-size="11" fill="#5c6b78">the same C(3,2) sits in front of r&#178; in (1+r)&#179; &#8212; both count pairs</text>
</svg>"##;

