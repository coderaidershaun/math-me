//! From algebra to linear algebra: how to read a timeseries equation written
//! in scalar algebra (AR(1), moving averages, EWMA) and rewrite it as an
//! equivalent vector–matrix expression — and why the two forms are the same
//! equations — using spreadsheet instincts as the bridge. The reading is then
//! spent where desks spend it: look-ahead bias as triangularity, a two-spread
//! VAR, least squares as perpendicularity, and the front door of the Kalman
//! filter. Each section function is named after the heading it renders and
//! chained in document order.
//!
//! Prerequisites:
//! - School algebra: comfortable rearranging an equation like x = a·y + b.
//! - Everyday spreadsheet fluency: columns, relative references, SUMPRODUCT.
//! - No linear algebra assumed — vectors and matrices are built from zero.
//!
//! Run it: cargo run --release --bin lesson-algebra-to-linear

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
    let b = Lesson::builder("From Algebra to Linear Algebra");
    let b = two_photographs_of_one_equation(b);
    let b = a_column_is_a_vector(b);
    let b = sumproduct_is_the_atom(b);
    let b = sumproduct_dragged_down(b);
    let b = one_row_per_date(b);
    let b = the_cell_above(b);
    let b = ar1_in_matrix_dress(b);
    let b = stack_the_recent_past(b);
    let b = reading_one_in_the_wild(b);
    let b = practice(b);
    let b = letter_overrides(b);
    b.build()
}

fn two_photographs_of_one_equation(b: LessonBuilder) -> LessonBuilder {
    b.heading("Two photographs of one equation")
        .note("Hover any term in a formula to see what it means here. Names set in bold are vectors or matrices — whole columns and blocks; plain ink is a single number. Two plots below have sliders — drag them and watch the curves follow.")
        .para(|p| p
            .text("Here is a sentence written in algebra you already own:"))
        .display(r"x_t = a x_{t-1} + \varepsilon_t")
        .explain(r"x_t", "The value at time t",
            "One cell of the series: the number in row t of the column. In the running example, the spread on day t.")
        .explain(r"a x_{t-1}", "Yesterday's value, scaled by a",
            "The fraction of yesterday that survives into today. With a = 0.9, ninety percent of yesterday's spread carries over.")
        .explain(r"x_{t-1}", "The value one row up",
            "Yesterday's entry in the column: the same series, read one time step earlier.")
        .explain(r"\varepsilon_t", "Today's shock",
            "The new, unforecastable piece that arrives with date t — the day's surprise, unknowable the day before.")
        .para(|p| p
            .text("Read it aloud: today's value is a fraction ")
            .math("a")
            .text(" of yesterday's, plus whatever news arrived overnight. Put it in a finance dress: ")
            .math("x")
            .text(" is the spread between two bond yields, ")
            .math("a")
            .text(" = 0.9 says ninety percent of yesterday's gap survives one more day, and ")
            .math(r"\varepsilon_t")
            .text(" is the day's surprise. Desks trade on equations of this shape; so do volatility models, discount curves and every autoregression ever fitted."))
        .para(|p| p
            .text("And here is the same sentence, photographed from a different angle:"))
        .display(r"x = a L x + \varepsilon")
        .explain(r"a L x", "The whole column, slid down and scaled",
            "Apply L to slide the column down one row — so each cell holds yesterday's value — then scale every cell by a.")
        .explain(r"\varepsilon", "The column of shocks",
            "All the surprises at once: one shock per date, stacked into a single named column.")
        .para(|p| p
            .text("Nothing new has been said. The first photograph shows one row of a spreadsheet — a single cell's formula. The second shows the whole column at once: ")
            .math("x")
            .text(" is no longer one number but the entire column, ")
            .math("L")
            .text(" is the instruction \"slide the column down one row\", and the equation claims the column equals nine tenths of its own slid-down self plus a column of surprises. Linear algebra is not deeper mathematics than algebra. It is the same mathematics with the camera pulled back."))
        .para(|p| p
            .text("By the end of this lesson you will translate freely in both directions, and — more useful — you will know why the two forms must agree. The only equipment assumed is school algebra and the spreadsheet reflexes you already have."))
}

fn a_column_is_a_vector(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("A column is a vector, a block is a matrix")
        .para(|p| p
            .text("Put six days of closing prices in a sheet: cells B1 to B6 hold 100, 103, 106, 103, 100, 97. Column B is six numbers, but you already treat it as one thing — you name the range, sum it, chart it, drag formulas along it. Mathematics does exactly this and no more: the whole column gets a single name, ")
            .math("x")
            .text(", and the entry in row ")
            .math("t")
            .text(" is written ")
            .math(r"x_t")
            .text(". That is the entire definition of a vector, as far as this lesson needs one: a named column of numbers whose subscript is the row number — and in a timeseries, the row number is the date. From here on, every name that stands for a whole column — and, shortly, a whole block — is set in bold, so a glance tells a spreadsheet object from a single number."))
        .figure(Figure::new(SHEET_SVG,
            "The whole of the notation in one screenshot. Column B, named as one object, is the vector x; its cell B4 is x with subscript 4, value 103. The shaded block C1:E6 is a matrix, here named A; its entry A with subscripts 4,2 sits in the block's row 4, column 2 — spreadsheet cell D4. Note the collision of conventions: the sheet's address D4 reads column-then-row, while the matrix subscript reads row-then-column. More translation errors start there than anywhere else."))
        .para(|p| p
            .text("Two print conventions to absorb now. Vectors are columns unless a paper says otherwise; a row is a transposed column, and \"transpose\" means tip it on its side — for a block, flip it over its diagonal, so the entry in row i, column j moves to row j, column i. That sounds like typography; hold it, because on a sheet whose rows are dates the flip will turn out to mean something physical. And the name's weight is only ever a costume: this lesson prints it bold, another paper might arrow it or leave it plain, and a paper often simply writes ")
            .math(r"x \in \mathbb{R}^n")
            .text(", which reads \"")
            .math("x")
            .text(" is a list of ")
            .math("n")
            .text(" real numbers\". A six-day price history lives in ")
            .math(r"\mathbb{R}^6")
            .text("."))
        .explain(r"\mathbb{R}^n", "The space of length-n lists",
            "All possible columns of n real numbers. Saying x lives here says only how long the column is.")
        .explain(r"\mathbb{R}^6", "The space of length-6 lists",
            "All possible columns of six real numbers — every conceivable six-day history at once.")
        .para(|p| p
            .text("A matrix is the same act of naming applied to a rectangular block of cells: one name, ")
            .math("A")
            .text(", and the entry in row ")
            .math("i")
            .text(", column ")
            .math("j")
            .text(" is ")
            .math(r"A_{ij}")
            .text(". Nothing about a matrix is more exotic than that — it is a named range that happens to be two-dimensional, exactly as a sheet is."))
        .explain(r"A_{ij}", "One cell of the block",
            "The entry of A in row i, column j. Row first, column second — the reverse of a spreadsheet address like D4.")
        .para(|p| p
            .text("Naming blocks of numbers is not a modern habit. Around two thousand years ago the Chinese text known as the Nine Chapters on the Mathematical Art solved grain-and-sheaf problems by laying the coefficients out as a rectangular array on a counting board and reducing the columns step by step — recognisably the method taught today as Gaussian elimination. The first linear algebra was, quite literally, done on a spreadsheet."))
        .para(|p| p
            .text("So far nothing has happened. Naming is free, and a name computes nothing. The power is in the operations defined on the names — and there is really only one, which you already use daily."))
}

fn sumproduct_is_the_atom(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("SUMPRODUCT is the atom")
        .para(|p| p
            .text("Your book holds 10 shares of one stock priced at $100, 5 of another at $20, and 2 of a third at $50. The book's value is 10 times 100, plus 5 times 20, plus 2 times 50: $1,200. In a sheet you would line the two columns up and write =SUMPRODUCT(holdings, prices): multiply row by row, then add. Mathematics calls the identical operation the dot product:"))
        .display(r"h \cdot p = h_1 p_1 + h_2 p_2 + h_3 p_3")
        .explain(r"h_1 p_1", "First row times first row",
            "Holdings of the first asset times its price: 10 times 100 = 1{,}000 in the running example.")
        .explain(r"h_2 p_2", "Second row times second row",
            "Holdings of the second asset times its price: 5 times 20 = 100.")
        .explain(r"h_3 p_3", "Third row times third row",
            "Holdings of the third asset times its price: 2 times 50 = 100.")
        .para(|p| p
            .text("Before the notation, do it with your hands. Put a left finger on the holdings column and a right finger on the prices, and walk them down in step: 10 times 100 — say \"a thousand\" — 5 times 20 — \"eleven hundred\" — 2 times 50 — \"twelve hundred\". Two fingers, one running total. Every dot product in this lesson, and every row-meets-column in every paper you will ever read, is that two-finger walk."))
        .para(|p| p
            .text("Two columns in, one number out. The same atom is everywhere once you look for it. A portfolio return: weights 0.5, 0.3, 0.2 against asset returns of 4%, minus 2%, and 10% is 2% minus 0.6% plus 2% — a 3.4% return, and a dot product. A weighted average is a dot product with weights that sum to one. \"Discount each cash flow and add\" is the same atom: a bond paying $5, $5 and $105 over three years, against discount factors 0.95, 0.90 and 0.86, is worth 4.75 plus 4.50 plus 90.30 — $99.55, the bond's price in one SUMPRODUCT. Pricing, hedging and risk are dot products stacked to the horizon."))
        .figure(Figure::new(DOT_SVG,
            "The atom, drawn once. Holdings times prices, row by row, then added: the book's value, 1,200. The blue middle column is where the sheet's =h*p stops — the elementwise product; the dot product is its SUM, collapsing two aligned columns into one number. Every row-meets-column in the rest of the lesson, including every highlighted row of the matrix figures, is this picture again."))
        .para(|p| p
            .text("So take this as the reading rule for everything that follows: whenever two columns meet under a dot — or, in the notation coming next, whenever a row of a block meets a column — hear SUMPRODUCT."))
}

fn sumproduct_dragged_down(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("A matrix times a vector is SUMPRODUCT dragged down")
        .para(|p| p
            .text("One SUMPRODUCT yields one number. A timeseries recipe needs one number per date — a whole output column. The spreadsheet move is to drag the formula down the sheet, letting the weight cells shift as it goes. The mathematical move has a two-letter name:"))
        .display(r"y = A x")
        .explain(r"Ax", "Every SUMPRODUCT at once",
            "The matrix–vector product: one output cell per row of A, each the SUMPRODUCT of that row with the column x.")
        .para(|p| p
            .text("and its meaning, row by row, is nothing but the atom repeated:"))
        .display(r"y_i = \sum_j A_{ij} x_j")
        .explain(r"y_i", "Output cell i",
            "Row i of the output column: what the drag-down formula leaves in row i.")
        .explain(r"\sum_j", "Add over the columns",
            "Run j across every column of the block, adding the products up — the SUM half of SUMPRODUCT.")
        .explain(r"A_{ij} x_j", "Weight times input",
            "Row i's weight for input cell j, times that input cell — the PRODUCT half of SUMPRODUCT.")
        .para(|p| p
            .text("Row ")
            .math("i")
            .text(" of ")
            .math("A")
            .text(" holds the weights for output ")
            .math("i")
            .text("; SUMPRODUCT it against ")
            .math("x")
            .text(" and you have that output cell. A block with ")
            .math("m")
            .text(" rows and ")
            .math("n")
            .text(" columns eats a length-")
            .math("n")
            .text(" column and emits a length-")
            .math("m")
            .text(" one. The inner sizes must agree, for the same reason SUMPRODUCT refuses ranges of different lengths."))
        .para(|p| p
            .text("The notation was built on purpose. It is engineered so that a whole stack of weigh-and-add recipes collapses into two letters that can then be moved around by school algebra: Sylvester coined the word \"matrix\" in 1850, and Cayley's 1858 memoir wrote out its algebra precisely so systems of equations could be handled as single symbols. Whether the arithmetic underneath was also a choice is a fair question — hold it; the end of this section answers no. And once ")
            .math("AB")
            .text(" is required to mean \"do B, then A\", the multiplication rule stops being a further choice: track any single output cell of the chained recipe and regroup its arithmetic, and the combined weight table is forced to be rows of A against columns of B. No other rule gives back the two recipes run back to back."))
        .para(|p| p
            .text("One trap, and it bites spreadsheet natives hardest. In a sheet, =A1:A3 * B1:B3 multiplies cell by cell — the elementwise product, which mathematicians call the Hadamard product and almost never mean. The row-dot-column product is MMULT. When a paper writes two names side by side, ")
            .math("AB")
            .text(", it always means the MMULT kind. And order matters: ")
            .math("AB")
            .text(" and ")
            .math("BA")
            .text(" are generally different recipes, and often the shapes do not even fit both ways round."))
        .explain(r"AB", "First B, then A",
            "The composite recipe: apply B to a column, then apply A to the result. Cell i of A(Bx) is a sum over j of A_ij times a sum over k of B_jk x_k; regroup — the distributive law — and the weight on x_k must be row i of A against column k of B. The rows-dot-columns rule is forced by composition.")
        .explain(r"BA", "First A, then B",
            "The same two recipes in the other order — in general a different recipe, when the shapes fit at all.")
        .para(|p| p
            .text("One more boundary, and it is the subject's own. Only weigh-and-add recipes stack into matrices, and every weigh-and-add recipe passes two tests: scale the input column and the output scales with it; and — the sharper test — add two input columns and the outputs simply add, because the distributive law splits each weighted sum of a sum into two weighted sums. \"Replace every negative cell with zero\" — the payoff clerk of an options book — fails the adding test, and two cells of arithmetic prove it. Take positions worth 5 and minus 5. Net the book first and the clerk pays nothing; pay the two off separately and it pays 5. Netting changed the answer, and netting can never change a weigh-and-add answer, because weigh-and-add nets before it looks. Note what the clerk does pass: doubling — double every position and every payoff doubles. \"Linear means it scales\" is therefore a half-truth; linearity is scaling and adding, and the adding is what option payoffs refuse. Max, absolute value, thresholds, the series multiplied by itself — the linear camera cannot photograph these whole. Everything linear algebra buys, it buys by staying linear; part of reading an equation is noticing when it has left that country."))
        .para(|p| p
            .text("And the boundary runs both ways — which settles the question left open above. Was the weigh-and-add table itself a choice? Feed any recipe the unit column ")
            .math(r"e_j")
            .text(" — a lone 1 in row ")
            .math("j")
            .text(", zeros elsewhere — and file whatever comes out as column ")
            .math("j")
            .text("; six feedings interrogate a six-row recipe completely. Now write any input as its cells times their unit columns — that is just what the entries of a column mean — and let the two tests fire: adding splits the recipe's answer into one piece per unit column, scaling pulls each cell's value out front, and the output is forced to be the first cell times column 1, plus the second cell times column 2, and onward. Any recipe that scales and adds already is a weigh-and-add table; the arithmetic was never up for grabs, only the packaging — the row-then-column layout the sheet figure warned about. And the forcing hands you a gift on the way out: a second reading of the product. ")
            .math("Ax")
            .text(" is also a mix of ")
            .math("A")
            .text("'s columns, weighted by the input's cells. Hold on to that; the payoff section will spend it."))
        .explain(r"e_j", "The unit column",
            "A 1 in row j and zeros everywhere else — the sheet's probe. Feeding it to a recipe reads off what the recipe does to row j alone, which is column j of the recipe's table.")
}

fn one_row_per_date(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("One row per date")
        .para(|p| p
            .text("Now the jump itself, performed on the most familiar recipe on any price chart — the three-day moving average:"))
        .display(r"y_t = \frac{x_t + x_{t-1} + x_{t-2}}{3}")
        .explain(r"\frac{x_t + x_{t-1} + x_{t-2}}{3}", "The average of the last three days",
            "Today's value, yesterday's and the day before's, added and divided by three: equal weights of 1/3 on the three most recent cells.")
        .explain(r"x_{t-2}", "The value two rows up",
            "The entry two time steps back: the day before yesterday's cell in the column.")
        .explain(r"y_t", "The smoothed value at time t",
            "Row t of the output column: the moving average as it stands on day t.")
        .para(|p| p
            .text("In a sheet this is =AVERAGE(B2:B4) written next to row 4 and dragged down. To convert it — or any equation shaped like it — use one recipe. First, name the columns: ")
            .math("x")
            .text(" in, ")
            .math("y")
            .text(" out. Second, freeze one generic row ")
            .math("t")
            .text(" and list the weight every input cell receives: here rows ")
            .math("t")
            .text(", ")
            .math("t-1")
            .text(" and ")
            .math("t-2")
            .text(" get 1/3 each and every other row gets 0. That list is row ")
            .math("t")
            .text(" of a matrix. Third, stack the rows, one per date:"))
        .explain(r"t-1", "One step back",
            "The row above row t: the previous date in the column.")
        .explain(r"t-2", "Two steps back",
            "Two rows above row t: two dates earlier.")
        .display(r"y = W x")
        .explain(r"Wx", "The whole smoothing at once",
            "The moving-average recipe applied to the entire history in one multiplication: every row's AVERAGE computed simultaneously.")
        .para(|p| p
            .text("and row by row it is the atom again:"))
        .display(r"y_t = \sum_s W_{ts} x_s")
        .explain(r"\sum_s", "Add over the dates",
            "Run s across every date in the sample, adding the weighted values — the drag-down formula's SUM.")
        .explain(r"W_{ts} x_s", "Row t's weight on day s",
            "The weight the recipe gives day s when producing output t — here 1/3 if s is one of the three most recent days, 0 otherwise — times day s's value.")
        .figure(Figure::new(BAND_SVG,
            "The moving average as a block of cells. Row 4 of W, highlighted, holds 1/3 in columns 2, 3, 4 and nothing elsewhere; SUMPRODUCT of that row with x is (103 + 106 + 103) / 3 = 104, which is exactly y in row 4. Empty cells are zeros. Rows 1 and 2 are the boundary: a three-day average does not exist until three days exist."))
        .para(|p| p
            .text("To read someone else's ")
            .math(r"y = W x")
            .text(", run the recipe backwards: pick a row, SUMPRODUCT it against ")
            .math("x")
            .text(", and you land back on the scalar equation for that date. This is the central fact of the whole subject, so it deserves stating baldly: a matrix equation is a column of scalar equations, one per row, and it contains exactly zero information the scalar equations did not. If you cannot recover the scalar row, you have not yet read the matrix equation — you have only looked at it."))
        .para(|p| p
            .text("Now the first practical dividend. What do rows 1 and 2 of ")
            .math("W")
            .text(" contain? The scalar formula dodges the question — it quietly refers to ")
            .math(r"x_0")
            .text(" and ")
            .math(r"x_{-1}")
            .text(", cells above the top of the sheet. Drag =AVERAGE(B-1:B1) into row 1 and the sheet says #REF!. The matrix cannot dodge: rows 1 and 2 must hold something. Leave them empty and you have declared the first two outputs undefined; put 1/3-weights on the fewer days available and you have invented a different formula for the young sample; assume the world was quiet before day 1 and you have invented data. All three are done in practice — the point is that the matrix form drags the choice into daylight, where the subscript notation let it hide."))
        .explain(r"x_0", "The value just before the sample",
            "The cell one row above the top of the sheet — a value the notation refers to but the data does not contain.")
        .explain(r"x_{-1}", "The value two rows before the sample",
            "Two rows above the top of the sheet: further into the pre-sample past the formula quietly leans on.")
        .para(|p| p
            .text("Notice ")
            .math("W")
            .text("'s texture, too: the same three weights, sliding one step right with every row down. \"The same formula dragged down the sheet\" prints as a matrix that is constant along its diagonals. Such matrices are called Toeplitz matrices, after Otto Toeplitz, and timeseries work mass-produces them — every fixed recipe with relative references makes one."))
}

fn the_cell_above(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("The cell above")
        .para(|p| p
            .text("Scalar algebra hides its one genuinely time-series move inside a subscript: ")
            .math(r"x_{t-1}")
            .text(". On a sheet the move is plain to see — cell C4 containing =B3, a relative reference to the cell one row up. Apply the conversion recipe: producing output row ")
            .math("t")
            .text(" takes weight 1 on input row ")
            .math("t-1")
            .text(" and 0 everywhere else. Ones one step below the diagonal:"))
        .display(r"(Lx)_t = x_{t-1}")
        .explain(r"(Lx)_t", "Row t of the slid column",
            "Apply the lag matrix L to x, then read row t of the result: it holds what row t minus 1 of x held.")
        .figure(Figure::new(LAG_SVG,
            "The lag matrix at work: ones one step below the diagonal, and every value slides down one row. Row 1 of the output is 0 — there is no cell above the top of the sheet, so the convention \"the world was quiet before day 1\" fills it. In sheet terms, L is the formula =B3 sitting in row 4, dragged down the column."))
        .para(|p| p
            .text("Powers of ")
            .math("L")
            .text(" are longer slides: ")
            .math(r"L^2")
            .text(" shifts the column two rows, ")
            .math(r"L^k")
            .text(" shifts it ")
            .math("k")
            .text(". And on a six-row sample, ")
            .math(r"L^6 = 0")
            .text(": slide everything off the bottom of the sheet and nothing is left. Pause on how strange that is by number standards: no number except zero can be multiplied by itself into nothing — if q is not zero, no power of q is zero, school algebra's quiet guarantee. Yet L is visibly not the zero recipe — it moves every cell it touches — and six slides annihilate every six-row column all the same. The sheet's edge does it: each slide pushes one more value off the bottom, and numbers never meet an edge. Hold that thought — it is about to make an infinite-looking series terminate on its own."))
        .explain(r"L^2", "Slide down twice",
            "The lag matrix applied twice: every value moves two rows down, so row t holds the value from t minus 2.")
        .explain(r"L^k", "Slide down k times",
            "The lag applied k times over: row t holds the value from k periods earlier.")
        .explain(r"L^6", "Six slides",
            "On a six-row sample, sliding six rows pushes every value off the bottom: the zero matrix, the recipe that returns an all-zero column.")
        .explain(r"L^3", "Slide down three times",
            "The lag applied three times: every value moves three rows down the column.")
        .para(|p| p
            .text("With the lag in hand, differencing is free. Daily P&L out of cumulative P&L is ")
            .math(r"y_t = x_t - x_{t-1}")
            .text(", and the recipe converts it instantly — weight 1 on today, weight minus 1 on yesterday:"))
        .display(r"y = (I - L) x")
        .explain(r"(I - L)", "The differencing recipe",
            "Do nothing, minus slide-down-one: applied to a column, row t reads x at t minus x at t-1 — the one-day change, for all dates at once.")
        .para(|p| p
            .text("Here ")
            .math("I")
            .text(" is the identity matrix — ones down the diagonal, the recipe that hands the column back unchanged. It is the matrix world's number 1, and it exists for the same reason 1 does: so that \"do nothing\" has a name that algebra can manipulate."))
        .para(|p| p
            .text("Now the opposite direction. A running total — cumulative P&L from daily P&L, =SUM(B$1:B4) dragged down, note the anchored $ — gives every day up to ")
            .math("t")
            .text(" a weight of 1:"))
        .display(r"y_t = \sum_{s=1}^{t} x_s")
        .explain(r"\sum_{s=1}^{t}", "Add from day 1 through day t",
            "Run s from the first date to date t inclusive, adding as you go: the running total's reach.")
        .explain(r"x_s", "The value on day s",
            "The entry in row s of the input column, as the summation index s passes over it.")
        .display(r"y = S x")
        .explain(r"Sx", "The running total at once",
            "The cumulative-sum recipe applied to the whole column in one multiplication: every SUM-so-far computed simultaneously.")
        .para(|p| p
            .text("As a block, ")
            .math("S")
            .text(" is a triangle of ones — row ")
            .math("t")
            .text(" holds 1 in columns 1 through ")
            .math("t")
            .text(" and nothing beyond. And differencing undoes cumulating exactly: take running totals, then daily changes, and the original column returns. Watch it with three days of P&L — daily 3, minus 1, 4 cumulates to 3, 2, 6; difference that back, 3 minus 0, 2 minus 3, 6 minus 2, and out comes 3, minus 1, 4 again, row 1 leaning on the quiet-before-day-1 convention for its missing cell above. Undoing has a notation:"))
        .display(r"S = (I - L)^{-1}")
        .explain(r"(I - L)^{-1}", "The undo of differencing",
            "The inverse matrix: the recipe that reverses (I - L). Feed it the changes and it rebuilds the levels — which is exactly what a running total does. Why exact: row t of Sx holds x_1 through x_t added, the row above holds the same sum without x_t; subtract, everything older cancels in pairs, and x_t alone remains.")
        .para(|p| p
            .text("The superscript is deliberately the reciprocal's: an inverse matrix is to a matrix what 1/7 is to 7, the thing that cancels it. And you can see this particular inverse with your eyes. Add up the slides ")
            .math("I")
            .text(" + ")
            .math("L")
            .text(" + ")
            .math(r"L^2")
            .text(" + ")
            .math(r"L^3")
            .text(" and so on: copies of the column slid down 0, 1, 2, 3 rows, all added. Each output cell collects everything on its own row and above — precisely the running total, precisely the triangle of ones. The sum ends by itself, because on a finite sheet a long enough slide is the zero matrix."))
        .para(|p| p
            .text("A quiet bonus falls out of everything being built from ")
            .math("L")
            .text(". Give the moving average its quiet-before-day-1 boundary and it, too, is a polynomial in the lag:"))
        .display(r"W = \frac{I + L + L^2}{3}")
        .explain(r"\frac{I + L + L^2}{3}", "Three slides, averaged",
            "Do nothing, slide once and slide twice, added and divided by three: the three-day average as arithmetic on the lag matrix itself.")
        .para(|p| p
            .text("Read its top rows before moving on, because ")
            .math("W")
            .text(" has quietly changed clothes. The band figure's ")
            .math("W")
            .text(" left rows 1 and 2 empty — outputs declared undefined. The polynomial cannot: row 1 of ")
            .math("I")
            .text(" puts a 1/3 there while rows 1 of ")
            .math("L")
            .text(" and ")
            .math(r"L^2")
            .text(" are empty, so the polynomial's row 1 is forced to hold a lone 1/3, and row 2 a pair — writing \"polynomial in the lag\" is choosing the quiet-before-day-1 convention, and the empty-rows ")
            .math("W")
            .text(" of the figure is not a polynomial in ")
            .math("L")
            .text(" at all. The choice has a printed cost. On the price column the polynomial's first two outputs are 100/3, about 33.3, and 203/3, about 67.7 — a series that never strays far from 100 opening its smoothed life with a crash to a third of its value — and from row 3 the two conventions agree exactly, 103, as in the figure. The error is large, known in advance, and confined to the first two rows: which is why every practitioner who takes the convention also discards a burn-in. The algebra below is bought at that price."))
        .para(|p| p
            .text("And recipes that are all arithmetic in the same ")
            .math("L")
            .text(" commute with each other: difference the three-day average or average the three-day differences, and the same column comes out — ")
            .math(r"W(I - L)")
            .text(" equals ")
            .math(r"(I - L)W")
            .text(", because both multiply out to the same polynomial in ")
            .math("L")
            .text(". The general warning stands — order matters — but the timeseries toolbox is the lucky corner where the standard tools all come from one factory and happily swap places. Powers of ")
            .math("L")
            .text(" also read directly as calendar: on a sheet of trading days, ")
            .math(r"L^5")
            .text(" is \"same day last week\", and a seasonal model is just a recipe reaching for it."))
        .explain(r"W(I - L)", "Smooth, then difference",
            "The moving average applied first, differencing applied to the result — one combined recipe, the product of the two blocks.")
        .explain(r"(I - L)W", "Difference, then smooth",
            "Differencing applied first, the moving average applied to the result. Equal to the other order — but only for the polynomial W. The figure's empty-rows W fails the swap one row into the sample: on the prices, average-then-difference gives 103 at row 3, difference-then-average 35.3. The commuting theorem is bought with the boundary convention.")
        .explain(r"L^5", "Slide down five: same day last week",
            "The lag applied five times — on a sheet of trading days, the value one calendar week earlier.")
        .para(|p| p
            .text("One more move completes the toolbox: run the slide backwards. Tip ")
            .math("L")
            .text(" over its diagonal — transpose it — and its ones move one step above the diagonal. Applied to a column, the transposed lag slides everything up a row:"))
        .display(r"(L'x)_t = x_{t+1}")
        .explain(r"(L'x)_t", "Row t of the slid-up column",
            "Apply the transposed lag to x, then read row t of the result: it holds what row t plus 1 of x holds. Row 6 of the output is empty — quiet after day 6, the boundary convention mirrored at the bottom of the sheet.")
        .explain(r"x_{t+1}", "The value one row down",
            "Tomorrow's entry in the column — the same series, read one time step later.")
        .para(|p| p
            .text("The row above held yesterday; the row below holds tomorrow. ")
            .math("L")
            .text(" slid down and read the past; its transpose ")
            .math(r"L'")
            .text(" slides up and reads the future: transposing a date-indexed recipe reverses time. That gives \"tip it on its side\" its promised content, and it turns a glance at any block into a safety check. Every matrix in this lesson keeps its ink on or below the diagonal — ")
            .math("L")
            .text(", ")
            .math("S")
            .text(", ")
            .math("W")
            .text(", the differencer, the undo about to arrive — because every one of them builds today from today and the past. Lower-triangular is what causal looks like, printed."))
        .figure(Figure::new(TIME_TEXTURE_SVG,
            "Time, read as texture. L's ones sit one step below the diagonal: every row reaches only into the past, and the column slides down. Tip it over and the transpose L' has ones one step above: it slides the column up, handing each row tomorrow's value. The centred three-day average straddles the line — its 1/3 band above the diagonal means row 4's output uses day 5's price, unknowable on day 4. One glance at where a recipe's weights sit relative to the diagonal tells you whether its signal is tradable."))
        .para(|p| p
            .text("The glance earns money the first time you meet a chartist's centred average — each day smoothed with a neighbour on both sides, a default in charting packages because it hugs the data with no lag. As a recipe it is ")
            .math(r"L'W")
            .text(": the trailing average, delivered a day early — and one third of its every value is tomorrow's price. On the price column the centred smooth prints its down-tick against day 4, so a backtest sells the turn at 103 — using a number that needs day 5's close to exist. The trailing smooth prints the same down-tick on day 5, and the live desk sells at 100. The backtest books that 3-point difference at every turn and the desk never sees a penny of it; the lie was visible before a single trade was simulated, as ink above the diagonal — a recipe reading tomorrow's newspaper. Where the glance has no power: a block whose rows and columns are not dates — the sheet figure's block, any cross-sectional table — has no time direction, and triangularity there says nothing. The test reads the calendar, not just the matrix."))
        .explain(r"L'", "The lead: slide up one",
            "The transposed lag — ones one step above the diagonal. Applied to a column it slides everything up a row, so each cell holds the next date's value.")
        .explain(r"L'W", "The trailing average, slid a day early",
            "Slide the whole smoothed column up one row and each date shows the average it should only see tomorrow: the centred smoother, and the anatomy of look-ahead bias.")
}

fn ar1_in_matrix_dress(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("The payoff: the opening equation, solved by school algebra")
        .para(|p| p
            .text("Return to the opening pair. The scalar photograph said ")
            .math(r"x_t = a x_{t-1} + \varepsilon_t")
            .text("; the conversion recipe turns \"a times the cell above, plus a shock\" into"))
        .display(r"x = a L x + \varepsilon")
        .para(|p| p
            .text("and this time every symbol is loaded: check row ")
            .math("t")
            .text(" and the scalar equation falls out. Now do what school taught you and collect the ")
            .math("x")
            .text(" terms on one side. Subtracting ")
            .math(r"a L x")
            .text(" from both sides is legal because columns add cell by cell — the adding and collecting rules of school algebra carry over wholesale. Multiplication is where dues are paid: the freedom to reorder is gone, and so is the guarantee that every nonzero recipe can be undone — the slide pushed 97 off the bottom of the sheet, and no recipe can bring back a cell that is simply gone. Division must now be earned recipe by recipe:"))
        .display(r"(I - a L) x = \varepsilon")
        .explain(r"(I - a L)", "Today minus a times yesterday",
            "The whole-column form of the AR(1)'s left-hand side: applied to x, row t reads x at t minus a times x at t-1.")
        .para(|p| p
            .text("Then divide through — that is, apply the undo, which this particular recipe has earned. Given the output column, the first input is the first output, and each later input is its output plus ")
            .math("a")
            .text(" times the input just recovered: forward substitution rebuilds ")
            .math("x")
            .text(" cell by cell, so nothing was destroyed and the inverse exists:"))
        .display(r"x = (I - a L)^{-1} \varepsilon")
        .explain(r"(I - a L)^{-1}", "The undo of the AR(1)'s left side",
            "The inverse recipe: applied to the shock column it performs every substitution of the recursion into itself, for every date, in one stroke. Its entries are powers of a. Inverse means exactly this: composed with (I - aL), it is \"do nothing\".")
        .para(|p| p
            .text("The inverse expands by the same reflex you use for the geometric series ")
            .math(r"\frac{1}{1-q}")
            .text(" = ")
            .math(r"1 + q + q^2 + q^3")
            .text(" and so on — only with ")
            .math(r"q = a L")
            .text(": the undo is ")
            .math("I")
            .text(" plus ")
            .math(r"a L")
            .text(" plus ")
            .math(r"a^2 L^2")
            .text(" plus ")
            .math(r"a^3 L^3")
            .text(" and onward, slides of every length, each carrying one more factor of ")
            .math("a")
            .text(". And the reflex is no loose analogy but a checkable identity: multiply ")
            .math(r"(I - a L)")
            .text(" against that stack and the cross terms cancel in pairs, leaving ")
            .math("I")
            .text(" at the front and a sixth power of ")
            .math("a")
            .text(" times ")
            .math(r"L^6")
            .text(" at the back — and on a six-row sheet ")
            .math(r"L^6")
            .text(" is the zero recipe, so the product is exactly ")
            .math("I")
            .text(", no smallness of ")
            .math("a")
            .text(" required. Passing that check is not evidence that the stack is the inverse; it is the definition of being the inverse, satisfied. Read row ")
            .math("t")
            .text(" of the stack and the whole model opens up:"))
        .explain(r"\frac{1}{1-q}", "The geometric limit",
            "What the series 1 + q + q^2 + … adds up to when the ratio q is smaller than 1 in size.")
        .explain(r"q^2", "The ratio, twice",
            "The common ratio applied twice: the third term of the geometric series.")
        .explain(r"q^3", "The ratio, three times",
            "The common ratio applied three times: the fourth term of the geometric series.")
        .explain(r"a L", "Scale by a, slide down one",
            "The matrix playing the geometric ratio's role: fade the column by a and slide it one row down. Each extra power is one more day of fading and one more row of slide.")
        .explain(r"a^2 L^2", "Two days of fade, two rows of slide",
            "The slide-twice matrix scaled by a squared: how the shock from two days ago enters today.")
        .explain(r"a^3 L^3", "Three days of fade, three rows of slide",
            "The slide-three-times matrix scaled by a cubed: the three-day-old shock's route into today.")
        .display(r"x_t = \sum_{k=0}^{t-1} a^k \varepsilon_{t-k}")
        .explain(r"\sum_{k=0}^{t-1}", "Add over every age of shock",
            "Run k from 0 (today's shock) back to the oldest shock in the sample, adding each one's surviving weight.")
        .explain(r"a^k \varepsilon_{t-k}", "A k-day-old shock, faded",
            "The shock that arrived k days ago, still carrying weight a to the power k today — geometric discounting by age.")
        .para(|p| p
            .text("Today's spread is every shock that ever hit it, each discounted by ")
            .math("a")
            .text(" per day of age. That is what a mean-reverting market is: a memory shaped like ")
            .math(r"a^k")
            .text("."))
        .explain(r"a^k", "The survival weight at age k",
            "The fraction of a shock still present k days after it arrived: a multiplied by itself k times.")
        .figure(Figure::new(INVERSE_SVG,
            "The undo, printed in full at the opening example's a = 0.9, entries rounded to two decimals; each cell's shading equals its weight, so the geometric fade is visible as texture. Read the highlighted row across and the scalar solution falls out: x with subscript 6 is every shock so far, discounted by age. Read the highlighted column down and you get one shock's future footprint — the memory curve a^k the plot below draws. Empty cells are zeros: the future never reaches back. And at a = 1 the triangle fills with ones and becomes S, the running total."))
        .para(|p| p
            .text("The amber column is the unit-column reading from the SUMPRODUCT section, paying out. Feed the undo a column that is a lone 1 on day 1 — one unit of shock, then silence forever — and the mix-of-columns reading hands back column 1 unread: no SUMPRODUCTs, just that shock's entire future, fading down the page. So every product in this lesson reads two ways, and part of reading is choosing which: across a row, what went into today; down a column, where one day's shock goes. The column is what a desk calls an impulse response, and every column of the undo is the memory curve of the plot below, started on a different day."))
        .para(|p| p
            .text("The triangle has a second life on the far side of its own diagonal. Transpose the undo — time reversed — and row ")
            .math("t")
            .text(" of the flipped block reads ")
            .math(r"a^{s-t}")
            .text(" against everything from ")
            .math("t")
            .text(" onward: discount each later cell by its distance, then add. That is the bond clerk of the SUMPRODUCT section, mechanised — at a flat per-period discount factor ")
            .math("a")
            .text(", present value is one row of the transposed memory matrix, and the amber column, summed, prices six days of 1-a-day income at 4.69 as of day 1. The same six cells, two readings: backwards they are memory, forwards they are discounting. Mean reversion and present value are transposes of each other. The identification leans on the curve being flat — one ")
            .math("a")
            .text(" for every period; the clerk's own factors 0.95, 0.90, 0.86 are deliberately not powers of one number, and that gap is exactly what \"the discount curve is not flat\" means."))
        .explain(r"a^{s-t}", "The fade across the gap",
            "a multiplied by itself once per period between dates t and s. Read backwards in time it is surviving memory; read forwards it is a discount factor — the value on day t of a unit arriving on day s.")
        .para(|p| p
            .text("You did not need matrices to discover this. Substitute the recursion into itself, one day at a time:"))
        .display(r"x_t = \varepsilon_t + a \varepsilon_{t-1} + a^2 \varepsilon_{t-2} + a^3 x_{t-3}")
        .explain(r"a \varepsilon_{t-1}", "Yesterday's shock, one day faded",
            "The surprise from one day ago, carrying weight a today — the first substitution's leftover.")
        .explain(r"a^2 \varepsilon_{t-2}", "The two-day-old shock, twice faded",
            "The surprise from two days ago, at weight a squared — the second substitution's leftover.")
        .explain(r"a^3 x_{t-3}", "The remainder, three days back",
            "What is still unexpanded after three substitutions: the series as it stood three days ago, at weight a cubed. Keep substituting and it dissolves into shocks.")
        .para(|p| p
            .text("Keep going and the remainder dissolves into shocks — the same answer, one substitution per line. The matrix inverse performed every substitution, for every date, simultaneously. That is the honest sales pitch of linear algebra: not new mathematics, but all of the old mathematics executed at once."))
        .plot(Plot::new(0.0..=24.0)
            .curve("weight of a shock, memory a", "pow(a, x)")
            .curve("weight at a = 0.5", "pow(0.5, x)")
            .param("a", 0.50..=1.10, 0.94)
            .hline(0.5)
            .vline(11.2)
            .x_label("days since the shock arrived")
            .y_label("weight still present today")
            .height(280.0)
            .caption("The memory curve a^k: one column of the inverse (I - aL)^{-1}, read downwards. The dashed lines mark the half-life at the default a = 0.94: the age at which a^k = 1/2, which is k = ln(1/2)/ln(a) — about 11.2 days here. Drag a and watch the curve's crossing walk along the half-weight line, tracking that formula; the fixed curve halves every single day, half-life exactly 1. Push a all the way to 1 and the fading stops dead — every shock kept whole forever: the random walk. Past 1, each day compounds what it inherited: the bubble. One slider crosses three market regimes."))
        .para(|p| p
            .text("The same curve sits on your desk. Newton's law of cooling says a hot drink sheds heat in proportion to how far it stands above the room, so the gap between coffee and room temperature obeys the opening equation with the shocks switched off: this minute's gap is ")
            .math("a")
            .text(" times the last minute's. Sixty degrees above the room with ")
            .math("a")
            .text(" = 0.9 per minute runs 54, 48.6, 43.7 — under half by minute seven, since 0.9 to the seventh is about 0.48 — and the mean this series reverts to is something you can touch: room temperature. Even ")
            .math("a")
            .text(" = 1 has a physical costume, the perfect thermos, where the gap never fades — which is exactly the random walk of the next paragraph."))
        .para(|p| p
            .text("Two special settings close the loop. Set ")
            .math("a")
            .text(" = 1 and the undo becomes the triangle of ones: a random walk is an undiscounted running total of shocks — literally =SUM(news so far). Push ")
            .math("a")
            .text(" above 1 and the weights compound instead of fading: an explosive series. One number is an entire market regime: below 1, mean reversion; at 1, a random walk; above 1, a bubble dynamic."))
        .para(|p| p
            .text("An honest edge case belongs here. On a finite sheet the expansion always terminates — a long enough slide is the zero matrix — so nothing stops you setting ")
            .math("a")
            .text(" = 1, or 2, inside a sample. The trouble arrives with an idealisation papers make constantly: letting history run back forever. Then the expansion is a genuinely infinite series, and it only adds up under ")
            .math(r"|a| < 1")
            .text(" — the same condition the numeric geometric series demands. At ")
            .math("a")
            .text(" = 1 the weights never fade, each day piles one more undiscounted shock onto the total, and the series' spread grows without limit — which is exactly why random walks wander away from wherever they started, and why \"is ")
            .math("a")
            .text(" truly below 1?\" is a live question on every desk that trades mean reversion."))
        .explain(r"|a|", "The size of a, sign aside",
            "The absolute value: how big the memory coefficient is, ignoring whether it flips signs. Under 1, old shocks die out and the infinite sum converges.")
        .para(|p| p
            .text("Under that idealisation the partial sums of the weights climb toward the ceiling ")
            .math(r"\frac{1}{1-a}")
            .text(" and never touch it — a limit, the exact object the limits lesson in this series opens with, and ")
            .math(r"|a| < 1")
            .text(" is precisely the condition for it to exist. The ceiling also measures how far away forever is: at ")
            .math("a")
            .text(" = 0.94 sixty days of history collects nearly all of the ceiling of about 17; at 0.99 the ceiling is 100 and sixty days collects less than half. The closer ")
            .math("a")
            .text(" stands to 1, the further away forever is — and at 1 there is no ceiling at all."))
        .explain(r"\frac{1}{1-a}", "The ceiling",
            "What the weights 1 + a + a^2 + … add up to under an infinite past: about 16.7 at a = 0.94, 100 at 0.99. Also where the series would settle if an identical unit shock arrived every single day.")
        .para(|p| p
            .text("And you now sight-read a famous equation. The risk-desk workhorse for daily volatility — the exponentially weighted moving average — is"))
        .display(r"\sigma_t^2 = \lambda \sigma_{t-1}^2 + (1 - \lambda) r_{t-1}^2")
        .explain(r"\sigma_t^2", "Today's variance estimate",
            "The running estimate of return variance as of day t — the square of the volatility the desk quotes.")
        .explain(r"\lambda \sigma_{t-1}^2", "Yesterday's estimate, mostly kept",
            "The old variance estimate carried forward at weight lambda — the persistence term, exactly the a x of the AR(1).")
        .explain(r"(1 - \lambda)", "The weight on fresh evidence",
            "One minus the persistence: the small fraction of today's estimate handed to the newest observation.")
        .explain(r"r_{t-1}^2", "Yesterday's squared return",
            "The newest piece of evidence about variance — this equation's shock term.")
        .para(|p| p
            .text("Same shape as the opening equation, with ")
            .math(r"\lambda")
            .text(" playing ")
            .math("a")
            .text(" and squared returns playing the shocks — so without solving anything you already know the answer: today's variance estimate is a geometrically discounted running total of past squared returns. The classic setting ")
            .math(r"\lambda")
            .text(" = 0.94, standardised by the RiskMetrics methodology in the 1990s, gives a volatility shock a half-life of about eleven days — a number you can now make rather than memorise. Ask when ")
            .math(r"a^k")
            .text(" first falls to a half. A question whose unknown is an exponent is answered by a logarithm — the exponents lesson's refrain, a logarithm is an exponent read backwards — so the half-life is ")
            .math(r"\frac{\ln(1/2)}{\ln(a)}")
            .text(": 11.2 days at 0.94, which is the crosshair drawn on the memory plot above, and why its slider defaults to 0.94. The coffee already computed the same number for ")
            .math("a")
            .text(" = 0.9 — \"under half by minute seven\" is the 6.6-minute half-life of 0.9. And the two clocks are one clock: ")
            .math(r"a^k = e^{k \ln(a)}")
            .text(", so a geometric fade in discrete time is a continuous exponential decay photographed once per period. Coffee, radioactive half-lives and RiskMetrics variance share a single curve, read at different shutter speeds."))
        .explain(r"\frac{\ln(1/2)}{\ln(a)}", "The half-life recipe",
            "The exponent that turns a into one half — a logarithm question, because the unknown is an exponent. It returns 11.2 days at a = 0.94, 6.6 at 0.9, and grows without bound as a approaches 1.")
        .explain(r"a^k = e^{k \ln(a)}", "One curve, two shutter speeds",
            "Rewriting the geometric weight as a continuous exponential: ln(a) is the per-period decay rate, and sampling the smooth curve once per period returns exactly the powers of a.")
        .explain(r"e^{k \ln(a)}", "The continuous version of a^k",
            "Euler's number raised to k times the log of a — the smooth exponential curve that passes through every power of a. For a under 1 the log is negative: decay.")
        .para(|p| p
            .text("Notice the sleight of hand that keeps this legal. Squaring a return is not a weigh-and-add operation — but the equation never squares anything it computes. It takes the squared returns as an input column, already made, and is perfectly linear in the columns it is handed. A great deal of practical modelling is this move: manufacture a new column first (squares, logs, dummies), then run linear machinery on the result."))
        .para(|p| p
            .text("One naming trap before the second dress, because it snares every reader eventually: the phrase \"moving average\" means two different things. The chartist's moving average smooths the observed prices — the ")
            .math("W")
            .text(" of this lesson, applied to data you can see. Timeseries papers also write of an \"MA process\","))
        .display(r"x_t = \varepsilon_t + \theta \varepsilon_{t-1}")
        .explain(r"\theta \varepsilon_{t-1}", "Yesterday's shock, partially retained",
            "The previous surprise, carried into today at weight theta — averaging the noise, not the data.")
        .para(|p| p
            .text("— a weighted average of the unobservable shocks. Same words, different column being averaged. The matrix dress makes the difference impossible to miss: the smoother is ")
            .math(r"Wx")
            .text(", a matrix times the data; the MA process is ")
            .math(r"(I + \theta L)\varepsilon")
            .text(", a matrix times the noise."))
        .explain(r"(I + \theta L)", "Keep today, add theta of yesterday",
            "Do nothing plus a theta-weighted slide: the MA process's recipe, applied to the shock column rather than the observed data.")
}

fn stack_the_recent_past(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("The second dress: stack the recent past")
        .para(|p| p
            .text("Everything so far wears the whole-history dress: columns as long as the sample, matrices with one row per date — a ")
            .math("T")
            .text("-day sample means ")
            .math("T")
            .text(" by ")
            .math("T")
            .text(" blocks, and every fresh date regrows them all. Papers that march forward through time — simulators, filters, forecasting systems — want equipment of one fixed size however long history gets, and wear a second, smaller dress. Meet it on the two-lag autoregression:"))
        .display(r"x_t = a_1 x_{t-1} + a_2 x_{t-2} + \varepsilon_t")
        .explain(r"a_1 x_{t-1}", "Yesterday, at its own weight",
            "Yesterday's value scaled by the first coefficient — in the worked numbers, 1.1 times yesterday.")
        .explain(r"a_2 x_{t-2}", "The day before, at its own weight",
            "The value from two days back scaled by the second coefficient — here minus 0.3 times it.")
        .para(|p| p
            .text("Concretely: a trading desk models a spread with ")
            .math(r"a_1")
            .text(" = 1.1 and ")
            .math(r"a_2")
            .text(" = minus 0.3. Yesterday the spread was 4 basis points, the day before 10. Setting today's shock to zero, today's model value is 1.1 times 4, minus 0.3 times 10: 1.4 basis points. The positive first weight continues the recent move; the negative second weight pulls back against where it came from — overshoot and correction in two coefficients."))
        .explain(r"a_1", "The one-day weight",
            "The coefficient on yesterday's value: how strongly the most recent past continues into today.")
        .explain(r"a_2", "The two-day weight",
            "The coefficient on the value two days back. A negative value pulls against the older level — the correction in an overshoot-and-correct dynamic.")
        .para(|p| p
            .text("A single cell cannot step this equation forward: tomorrow needs two memories, today's value and yesterday's. So carry both. Define the state as the pair ")
            .math(r"z_t = (x_t, x_{t-1})")
            .text(" — the short list of everything the future needs to know — and one day's step becomes a small matrix times the state:"))
        .explain(r"(x_t, x_{t-1})", "The state: what tomorrow needs",
            "Both memories bundled into one object: today's value and yesterday's. Knowing this pair today is enough to step the model forward.")
        .explain(r"z_t", "The state at time t",
            "The bundle of recent memories the recursion runs on — here a pair, for a two-lag model.")
        .figure(Figure::new(STATE_SVG,
            "One step of the AR(2), as cells. The top row of the 2-by-2 block is the model: 1.1 and minus 0.3, SUMPRODUCTed against yesterday's state (4, 10), give 1.4. The bottom row is pure bookkeeping: weights 1 and 0 copy the newer memory down into the older slot. Today's shock is set to zero to keep the arithmetic visible."))
        .para(|p| p
            .text("The top row is the model. The bottom row does no modelling at all — weights 1 and 0 simply copy the newer memory into the older slot, the bookkeeping that keeps the pair current. This 2-by-2 block is called the companion matrix of the recursion, and the construction generalises: a model that remembers three days gets a 3-by-3 companion, deeper memory a bigger one — coefficients across the top, then ones one step below the diagonal. Look at that lower block with this lesson's eyes: it is a small lag matrix. The same object, reused one scale down."))
        .para(|p| p
            .text("What the second dress buys is iteration. One day is one multiplication by the companion ")
            .math("A")
            .text("; ")
            .math("k")
            .text(" days is ")
            .math(r"A^k")
            .text(". Whether ")
            .math(r"A^k")
            .text(" dies away or blows up is decided by the matrix's eigenvalues — the growth factors the recursion can sustain by itself. Ask which pure paths the shock-free recursion can carry: a path that multiplies by a fixed factor ")
            .math(r"\lambda")
            .text(" every step stands at ")
            .math(r"\lambda^t")
            .text(" on date ")
            .math("t")
            .text(". Substitute it:"))
        .explain(r"A^k", "k days in one multiplication",
            "The companion matrix applied k times: the recursion run k steps forward. Its entries fade or explode with the eigenvalues' k-th powers.")
        .display(r"\lambda^t = a_1 \lambda^{t-1} + a_2 \lambda^{t-2}")
        .explain(r"\lambda^t", "The pure path on date t",
            "A path growing by the fixed factor lambda every step: by date t it stands at lambda multiplied by itself t times.")
        .explain(r"a_1 \lambda^{t-1}", "Yesterday's path value, through the one-day weight",
            "The pure path one step back, fed through the coefficient on yesterday — the recursion's first claim on today.")
        .explain(r"a_2 \lambda^{t-2}", "The path two steps back, through the two-day weight",
            "The pure path's value two steps earlier, fed through the coefficient on the day before yesterday.")
        .para(|p| p
            .text("Divide through by ")
            .math(r"\lambda^{t-2}")
            .text(", the path's value two steps back, and the date drops out entirely — a factor sustainable on one date is sustainable on every date — leaving"))
        .explain(r"\lambda^{t-2}", "The path's value two steps back",
            "Where the pure path stood two dates ago. Every term of the substituted equation carries it as a factor, so dividing it out is legal — and removes t.")
        .display(r"\lambda^2 = a_1 \lambda + a_2")
        .explain(r"\lambda^2", "The factor, applied twice",
            "A growth factor lambda sustained for two steps. The equation asks: which factors can the two-lag recursion sustain on its own? Feed the companion the state (lambda, 1) and its top row demands exactly this — the eigenvalue equation, written in the recursion's letters.")
        .explain(r"a_1 \lambda", "The factor through the one-day weight",
            "One step of growth at factor lambda, fed back through the coefficient on yesterday.")
        .para(|p| p
            .text("For 1.1 and minus 0.3 the two roots are 0.5 and 0.6 — both under 1 in size, so every shock fades and the spread mean-reverts. With two distinct roots the pure paths are the whole story: a shock-free path has exactly two free memories, today's value and yesterday's, and matching two numbers with two different ingredients is a school problem with one solution — precisely because 0.6 and 0.5 differ. The slower root outlives the other, so 0.6 sets the memory of the entire model. Any root beyond 1 in size and the model explodes; a root at exactly 1 plants a random walk inside it. But a quadratic has three moods, not one, and the discriminant — school algebra again — picks between them."))
        .para(|p| p
            .text("The roots can come as a complex pair, and then the decay swings as it fades. Watch it with weights built for mental arithmetic, 1 and minus 0.5, whose quadratic has no real roots: the pair has size about 0.71 and angle 45 degrees. Start the series at 10 and 10, switch the shocks off, and step it by hand: 5, 0, minus 2.5, minus 2.5, minus 1.25, 0, 0.625 — past the mean, back, past it again. Both numbers are sitting in that arithmetic. At 45 degrees a step, a full swing takes 360 over 45 — eight days: count the zeros, four steps apart, half a swing each. And each overshoot lands at a quarter of the one before, because the size to the fourth power is exactly one quarter. A complex pair always arrives as mirror twins whose sum is real — size to the power ")
            .math("t")
            .text(", times a cosine-and-sine mix: the size sets the fade, the angle the rhythm."))
        .para(|p| p
            .text("And the two roots can land on the same spot. Take weights 1 and minus 0.25: the quadratic is a perfect square — root 0.5, twice — one ingredient where the matching problem needs two. The recursion does not care; step it from memories 0 and 1: 0, 1, 1, 0.75, 0.5, 0.3125 — a shape no blend of fading geometrics can draw, because it rises before it falls. The missing second ingredient is ")
            .math(r"t \lambda^t")
            .text(", the pure path aging as it fades, and every double root manufactures it. So the honest sentence has three clauses: distinct roots, a blend of two fades; a complex pair, a swing that fades; a double root, a rise-then-fade. The desk's model sits one nudge from that boundary — move the second weight from minus 0.3 to minus 0.3025 and its two roots collide at 0.55."))
        .explain(r"t \lambda^t", "The aging path",
            "Fade by lambda each step while picking up one unit of age: the double root's second ingredient. Substitute it into the recursion — with weights 2 lambda and minus lambda squared, yesterday's and the day before's aging paths recombine into today's exactly. On the matrix side, a double root leaves the companion with one eigen-direction where two are needed; this path is the missing direction's visible trace.")
        .figure(Figure::new(ROOTS_SVG,
            "The three faces of a two-lag memory, decided by the roots. Left: the desk's model (1.1, minus 0.3), roots 0.6 and 0.5 — a unit shock's echo is 6 times 0.6-to-the-t minus 5 times 0.5-to-the-t: 1, 1.1, 0.91, 0.67 and on down, two geometrics blended. Middle: weights (1, minus 0.5) give a complex pair of size about 0.71 — the hand-stepped path 10, 10, 5, 0, minus 2.5, minus 2.5, minus 1.25, 0, 0.625, zeros four steps apart: half a swing every four days, period eight. Right: weights (1, minus 0.25) make the quadratic a perfect square, root 0.5 twice — the response 2t times 0.5-to-the-t climbs before it decays, a shape no single geometric can draw. All three fade, because every root sits inside 1."))
        .para(|p| p
            .text("That swing is why the first autoregression ever fitted to data was exactly this two-lag form — and it was built to win an argument. In 1927 the standard treatment of the eleven-year sunspot cycle was to fit sine waves, which treats the record as a perfect clock read through sloppy eyes: the true curve exact, the errors only in the reading. Udny Yule looked at nearly two centuries of counts and did not believe it — the swings wander too much in size and timing. His counter-image, from the paper itself: a pendulum swinging in a hall while boys pepper it with peas from peashooters. Each pea is one ")
            .math(r"\varepsilon_t")
            .text("; the pendulum still swings, but every hit jolts the amplitude and the phase, and each jolt is inherited by every later swing instead of averaging away. A recursion pushed along by yesterday and pulled back by the day before is a once-per-tick pendulum, and a pair of complex roots is its swing — the size sets the fade, the angle the rhythm. Where the image breaks: a real pendulum lives in continuous time and is linear only in small swings, and no physical pendulum can explode — the recursion will, the moment a root crosses 1. Ride the pendulum for the fading swing, not for the physics. Two lags is the smallest memory that can overshoot and swing back, and sunspots swing."))
        .plot(Plot::new(0.0..=16.0)
            .curve("the swing: shock-free AR(2) path",
                "pow(r, x) * (10*cos(theta*x) + 10*((1/r - cos(theta))/sin(theta))*sin(theta*x))")
            .curve("its envelope: the memory curve, size playing a",
                "pow(r, x) * sqrt(100 + pow(10*((1/r - cos(theta))/sin(theta)), 2))")
            .scatter("stepped by hand: weights 1, -0.5, start 10, 10",
                vec![[0.0, 10.0], [1.0, 10.0], [2.0, 5.0], [3.0, 0.0], [4.0, -2.5],
                     [5.0, -2.5], [6.0, -1.25], [7.0, 0.0], [8.0, 0.625]])
            .param("r", 0.30..=1.05, 0.7071)
            .param("theta", 0.15..=2.5, 0.7854)
            .hline(0.0)
            .x_label("days since the shocks were switched off")
            .y_label("distance from the mean")
            .height(280.0)
            .caption("The complex-pair case, live: the swing r^x (10 cos(theta x) + d sin(theta x)), with d fixed by the start 10, 10. At the defaults — size r = 0.7071, angle theta = 0.7854, 45 degrees — it threads every hand-stepped dot: period 8, each overshoot a quarter of the last, because r to the fourth is 1/4. The upper curve is the envelope — the memory curve again, with the size playing a. Drag r: the beat holds — zeros still four days apart — while the fade changes: at 1 the swing never dies, Yule's pendulum with the peas and the friction switched off; past 1 every swing outgrows the last. Drag theta: the beat changes — a full swing takes 2 pi over theta days — while the fade stays r per day; pushed toward 0.15 the swing flattens into a plain fade, the complex pair collapsing toward a repeated real root. The size sets the fade, the angle the rhythm."))
        .para(|p| p
            .text("Forecasting drops out of the same object for free. The best guess of any future shock is zero, so the best ")
            .math("k")
            .text("-day-ahead guess is just the model run ")
            .math("k")
            .text(" steps with the shocks removed: ")
            .math(r"A^k z_t")
            .text(". For the one-lag model that collapses to ")
            .math(r"a^k x_t")
            .text(" — the memory curve on the plot above is also the profile along which every forecast decays toward the mean. A matrix power is a forecast."))
        .explain(r"A^k z_t", "The state, run k days ahead",
            "Today's bundle of memories pushed k steps through the companion recipe with all future shocks set to zero: the k-day forecast of the whole state.")
        .explain(r"a^k x_t", "Today, faded k days forward",
            "The k-step-ahead forecast of the AR(1): today's value with k days of memory decay applied and no future shocks assumed.")
        .para(|p| p
            .text("The state dress has one more costume change in it, and it is the one that pays the rent. The companion's bottom row was bookkeeping — 1 and 0, copy a memory down. Give that row a model of its own and the state stops being one series' memories and becomes two series in conversation. Two spreads on one desk, each leaning on both:"))
        .display(r"s_t = 0.8 s_{t-1} + 0.1 f_{t-1}")
        .display(r"f_t = 0.2 s_{t-1} + 0.7 f_{t-1}")
        .explain(r"s_t", "The first spread, today",
            "One cell of the first series — the spread that is about to be shocked in the worked example.")
        .explain(r"0.8 s_{t-1}", "Own memory",
            "Eighty percent of the first spread's yesterday survives into its today — the diagonal entry of the step matrix.")
        .explain(r"0.1 f_{t-1}", "The leak in",
            "A tenth of the neighbour's yesterday seeps into the first spread's today — an off-diagonal entry, the channel between the books.")
        .explain(r"f_t", "The neighbouring spread, today",
            "One cell of the second series — the book that receives the leak.")
        .explain(r"0.2 s_{t-1}", "The leak out",
            "A fifth of the first spread's yesterday crosses into the neighbour's today — the other off-diagonal, and the channel is wider in this direction.")
        .explain(r"0.7 f_{t-1}", "The neighbour's own memory",
            "Seventy percent of the second spread's yesterday survives into its today — the other diagonal entry.")
        .para(|p| p
            .text("Stack the pair as the state and one day is again one small matrix: diagonal entries 0.8 and 0.7 are each series' own memory, off-diagonals 0.1 and 0.2 are the channel between them. This is a vector autoregression — a VAR — and the off-diagonals are the entire point: set both to zero and the pair falls apart into two AR(1)s that never speak. Hit the first spread with a 10 basis-point shock, leave the second untouched, switch the shocks off and step: 10 and 0 becomes 8 and 2, then 6.6 and 3.0, then 5.58 and 3.42. Two basis points cross the channel on day one, and the neighbour rises on no news of its own before fading — a sentence no equation in one series alone can write, because the crossing lives between the columns. The pure-path question transfers verbatim: this step sustains the factors of ")
            .math(r"\lambda^2 = 1.5 \lambda - 0.54")
            .text(", which are 0.9 and 0.6 — the opening photograph's own coefficient, returned as the pair's slow root. The spreads' common level fades at 0.9 a day; their disagreement is arbitraged away at 0.6; and the pair mean-reverts because, as ever, the slow root sits under 1. The channel reads like contagion, and the reading is safe only while everything fades: an epidemic compounds, this merely redistributes a dying shock — it earns the name the day a root crosses 1, the pendulum's boundary again."))
        .explain(r"1.5 \lambda", "The diagonal's pull",
            "One step of growth at factor lambda, weighted by the sum of the two own-memory entries, 0.8 plus 0.7.")
        .explain(r"0.54", "The block's determinant",
            "0.8 times 0.7 minus 0.1 times 0.2: what the step matrix scales area by, and the product of its two sustainable factors — 0.9 times 0.6.")
        .explain(r"\lambda^2 = 1.5 \lambda - 0.54", "The pair's sustainable factors",
            "The same eigenvalue question, asked of the 2-by-2 step: 1.5 is the diagonal's sum, 0.54 the block's determinant, and the roots are 0.9 and 0.6. Check them as directions: feed the step the state (1, 1) and out comes (0.9, 0.9); feed it (1, minus 2) and out comes (0.6, minus 1.2). Moving together is persistent; moving apart is arbitraged away fast.")
        .figure(Figure::new(LEAK_SVG,
            "The doorway, opened one step. Two series share one step matrix: the diagonal is each series' own memory, the off-diagonals are the leak. A 10bp shock lands on s alone, and one multiplication per day gives (10, 0), (8, 2), (6.6, 3.0), (5.58, 3.42), (4.81, 3.51): the 0.2 carries s's shock into f, the 0.1 feeds it back. Watch the bars: s fades from the first step, but f rises for four straight days and crests at 3.51 — one step past the frame it reads 3.42, fading at last, the sustainable factors 0.9 and 0.6 both inside 1. A VAR is this picture with more rows. Set both off-diagonals to zero and it falls apart into two AR(1)s that never speak."))
        .para(|p| p
            .text("One more pull-back of the camera and this lesson touches its neighbour. Everything above pushed a value forward — the state, times ")
            .math("A")
            .text(", ")
            .math("k")
            .text(" times. The Kalman filter lesson in this series pushes a belief forward: not a dot but a cloud of maybes, one dot per scenario the desk cannot rule out. It can, because of the boundary this lesson drew. What that lesson calls a linear map is this lesson's weigh-and-add recipe — one object, two dialects — and its load-bearing fact, a bell-shaped cloud pushed through a linear map comes out bell-shaped, is why the same companion that steps a value can step an entire belief: two numbers, centre and width, survive the trip where a whole cloud went in. A simulator's matrix is already a filter's matrix. Now hand the cloud to the option clerk instead: max slams every dot below zero into a wall, the bell crumples against it, and no centre-and-width pair describes what is left. The clerk that failed the netting test is the clerk that breaks a filter — the linearity boundary, met from the belief side. The image has its limit: the recipe does not carry the cloud rigidly — a fade shrinks it, blocks shear it; what survives is the family, bell in, bell out, not the size or the shape."))
        .para(|p| p
            .text("The two dresses are two stackings of the same scalar equations. Stack all dates and you get the operator form, built for solving and estimating in one shot. Stack the recent memories — or the neighbouring series — and you get the state form, built for stepping forward. When a paper writes ")
            .math(r"z_t = A z_{t-1} + \varepsilon_t")
            .text(", you now know what you are looking at. And an honest word about which dress actually runs: a twelve-year daily sample is three thousand rows, its whole-history blocks hold nine million cells each, and nobody builds them. The operator dress is the thinking dress — it is what made the undo expandable, the recipes commutable, and stability a root question. What executes is the recursion: one multiply-and-add per row, the sheet's own dragged formula, exactly the forward substitution that earned the inverse. Derive in the big dress; run in the small one — or in the spreadsheet you already had."))
        .explain(r"A z_{t-1}", "Yesterday's state, stepped forward",
            "The companion recipe applied to yesterday's bundle of memories: the model row produces the new value, the bookkeeping rows shuffle the memories down.")
        .explain(r"z_{t-1}", "The state one step back",
            "Yesterday's bundle of memories: everything the model needed to produce today.")
}

fn reading_one_in_the_wild(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("How to read one in the wild")
        .para(|p| p
            .text("The whole lesson compresses into five moves. When an equation with ")
            .math("t")
            .text(" subscripts stares at you:"))
        .para(|p| p
            .text("One — find the columns. Whatever carries the time subscript is a column pretending to be a number: name it as a vector."))
        .para(|p| p
            .text("Two — find the weights. Whatever multiplies the subscripted quantities becomes matrix entries: one output per row, one input per column."))
        .para(|p| p
            .text("Three — find the shifts. Every ")
            .math("t-1")
            .text(" or ")
            .math("t-2")
            .text(" is a power of the lag matrix in the whole-history dress, or one more slot in the state in the one-step dress. Choose the dress by the job: solve or estimate in one shot, stack the dates; simulate or filter forward, stack the memories."))
        .para(|p| p
            .text("Four — mind the top rows. Ask what the equation assumes about the days before the sample began; the matrix form will force the question anyway. And while your eye is on the block, glance at the diagonal: any ink above it, the recipe is reading tomorrow."))
        .para(|p| p
            .text("Five — verify. Expand one row of your matrix equation by SUMPRODUCT and check it lands on the scalar original. If it does not, the translation is wrong, whatever else looks elegant."))
        .para(|p| p
            .text("A one-line workout. The second difference — the acceleration of a price — is ")
            .math(r"y_t = x_t - 2 x_{t-1} + x_{t-2}")
            .text(". Shifts of 0, 1, 2 with weights 1, minus 2, 1 — and the weights factor: this is differencing done twice,"))
        .explain(r"2 x_{t-1}", "Yesterday, counted twice",
            "Yesterday's value at weight two: it is subtracted once by each of the two differencing passes.")
        .display(r"y = (I - L)^2 x")
        .explain(r"(I - L)^2", "Difference, then difference again",
            "The differencing recipe applied twice. Expand the square and the weights 1, minus 2, 1 fall out — move five confirms it row by row.")
        .para(|p| p
            .text("and move five confirms it: expand the square, SUMPRODUCT a row, recover the scalar line."))
        .para(|p| p
            .text("Two conversions for you, before the answers. First: the weekly change on a sheet of trading days, ")
            .math(r"y_t = x_t - x_{t-5}")
            .text(". Second, subtler: an autoregression with a drift, ")
            .math(r"x_t = a x_{t-1} + b + \varepsilon_t")
            .text(" — what does a lone constant become when the equation goes whole-column? Actually try both; the attempt is where the reading skill sets."))
        .explain(r"x_{t-5}", "The value five rows up",
            "Same day last week, on a sheet of trading days: the entry five time steps back.")
        .para(|p| p
            .text("The first is shifts 0 and 5 with weights 1 and minus 1:"))
        .display(r"y = (I - L^5) x")
        .explain(r"(I - L^5)", "The weekly-change recipe",
            "Do nothing minus slide-down-five: row t reads x at t minus x at t-5 — this week against last week, for all dates at once.")
        .para(|p| p
            .text("The second catches nearly everyone once: a lone number cannot be added to a column, so the constant must become a column first — ")
            .math("b")
            .text(" in every row, which is ")
            .math("b")
            .text(" times a column of ones. Econometrics has kept a letter for that column since the subject began, the Greek iota:"))
        .display(r"x = a L x + b \iota + \varepsilon")
        .explain(r"b \iota", "The drift, one copy per row",
            "The constant b times the column of all ones: how a lone number rides into a column equation. The intercept column of every regression you will ever run is this same object.")
        .explain_char('ι', "Greek small iota: the column of ones",
            "A 1 in every row. Multiplying it by a constant spreads that constant down the whole column.")
        .para(|p| p
            .text("That column of ones is not a pedantry. It is the intercept column silently prepended to every regression ever run, and once you have seen it here you will recognise it in every design matrix you meet."))
        .para(|p| p
            .text("One last dividend, because in practice you rarely know ")
            .math("a")
            .text(" — you estimate it. Lay the sample out twice: a column ")
            .math("u")
            .text(" of yesterday's values (the series slid down one row) beside a column ")
            .math("v")
            .text(" of today's. The least-squares estimate of the memory coefficient is"))
        .display(r"\hat a = \frac{u \cdot v}{u \cdot u}")
        .explain(r"\hat a", "The estimated memory",
            "a with a hat: the best guess of the AR(1) coefficient from the data, in the least-squares sense.")
        .explain(r"\frac{u \cdot v}{u \cdot u}", "SUMPRODUCT over SUMPRODUCT",
            "How much yesterday co-moves with today, divided by how much yesterday moves at all: the slope that makes a times u the best forecast of v. \"Best\" in the least-squares sense: the total squared miss is a parabola in the coefficient, and this ratio is its vertex.")
        .explain(r"=", "Equals",
            "The two sides are the same quantity, written two ways.")
        .para(|p| p
            .text("— =SUMPRODUCT(u, v) / SUMPRODUCT(u, u), two cells of spreadsheet. \"Best\" is a chosen word, and the choice does the work: least squares crowns the ")
            .math("a")
            .text(" with the smallest total of squared misses, and that total is a parabola in ")
            .math("a")
            .text(" opening upward — school algebra again — whose lowest point is exactly this ratio. Squares are the convention because a parabola's bottom is a formula two SUMPRODUCTs can fill; absolute misses give a kinked curve and no such cell. That ratio is least squares in its smallest form, and every regression in empirical finance is this same object with more columns: dot products, arranged by the recipes of this lesson."))
        .para(|p| p
            .text("The parabola names the winner; a second reading says why the winner looks like that — and it is the reading that scales. The winning miss is the one that carries no trace of the regressor: demand ")
            .math(r"u \cdot (v - \hat a u) = 0")
            .text(" — the miss perpendicular to the column you fitted with — and the ratio falls out by rearrangement. Nothing is left behind in the direction you were allowed to use; the workout below checks it, to the cell. Now widen. Stand several regressor columns side by side in a block ")
            .math("X")
            .text(", demand the miss be perpendicular to every column at once, and the demand reads"))
        .display(r"X' X \beta = X' y")
        .para(|p| p
            .text("— the normal equations, page one of every econometrics text, and nothing but the atom in a grid: ")
            .math(r"X'X")
            .text(" is the SUMPRODUCT of every regressor column against every other, ")
            .math(r"X'y")
            .text(" the SUMPRODUCTs against the target, and one lone column collapses the grid back to the ratio above. The prime finally earns its keep here: ")
            .math(r"u'v")
            .text(" is a tipped column times a standing one — a one-row block meeting a one-column block, one SUMPRODUCT — the dot product written as a matrix product. And the lesson's own warning about the slide returns with teeth. If one regressor column is a copy, or a sum, of others, the grid is a recipe that destroyed information: no undo exists, the division cannot be earned, and the regression dies with the words \"matrix is singular\". No recipe can bring back a cell that is simply gone."))
        .explain(r"u \cdot (v - \hat a u) = 0", "The perpendicularity demand",
            "The leftover miss must not co-move with the regressor at all. Rearranged, u dot v equals the estimate times u dot u — the ratio above, re-derived. On the workout's numbers the residuals are (2.1, minus 0.7), and against u = (1, 3): 2.1 minus 2.1 — zero, exactly.")
        .explain(r"v - \hat a u", "The miss column",
            "What the fit fails to explain: today's column minus the estimate times yesterday's, one leftover per row.")
        .explain(r"(v - \hat a u)", "The miss column",
            "What the fit fails to explain: today's column minus the estimate times yesterday's, one leftover per row.")
        .explain(r"X' X \beta", "The grid, applied to the coefficients",
            "The SUMPRODUCT grid times the coefficient column: what the fitted weights imply each regressor should co-move with the fit.")
        .explain(r"1/\lambda", "The factor, flipped",
            "One over a sustainable factor: a factor of 0.5 inside the unit circle flips to a polynomial root of 2 outside it.")
        .explain(r"\hat a u", "The fitted column",
            "Yesterday's column scaled by the estimated memory: the part of today the regression claims to explain.")
        .explain(r"X'X", "The grid of SUMPRODUCTs",
            "Entry i, j is column i of X dotted with column j: every regressor against every other. With the drift column and yesterday's column from the workouts it is the 2-by-2 table 2, 4 over 4, 10.")
        .explain(r"X'y", "The target SUMPRODUCTs",
            "Each regressor column dotted with the column being explained — 5 and 9 on the workout numbers.")
        .explain(r"X'", "The block, tipped",
            "Every regressor column laid on its side as a row, so that each row of X-prime meeting a column is one SUMPRODUCT.")
        .explain(r"\beta", "The coefficient column",
            "One weight per regressor, chosen so the miss is perpendicular to every column at once. On the workout numbers the grid solves to 3.5 and minus 0.5 — fitting both rows exactly: two coefficients, two rows, no slack. Inference begins only when rows outnumber columns.")
        .explain(r"u'v", "A row meets a column",
            "The transpose turns the dot product into a matrix product: tip u into a row, stand v as a column, and the rows-dot-columns rule performs exactly one SUMPRODUCT.")
        .para(|p| p
            .text("Finally, some field marks for the wild, where typography varies. Econometrics writes the transpose as a prime — x-prime — where statistics writes a superscript T or a top symbol; all mean the same flip over the diagonal, which you now read as content, not costume: on a date-indexed block, the flip reverses time. Bold lower-case letters are vectors, capitals are usually matrices — the very convention this lesson has set in bold throughout — and a capital sigma is sometimes a summation sign and sometimes the name of a covariance matrix — the presence of limits under it decides. A time subscript on a matrix means the recipe itself changes with the date."))
        .para(|p| p
            .text("The lag wears local costume too. Statistics and the forecasting literature write B — for backshift — where this lesson wrote ")
            .math("L")
            .text(": the identical slide. Differencing gets the shorthand ")
            .math(r"\Delta")
            .text(", so the second difference of the workout above appears as ")
            .math(r"\Delta^2")
            .text(". And a whole autoregression is routinely folded into a lag polynomial — the desk's model shows up as \"1 minus 1.1 L plus 0.3 L-squared, applied to the series, equals the shock\", the entire left side packed into one parenthesis. One reconciliation before it bites you in a footnote: such papers declare stability by \"the roots of the lag polynomial lie outside the unit circle\", while this lesson kept saying the sustainable factors sit inside 1. Both are true at once, because the polynomial's roots are the reciprocals of the factors: divide the factor equation through by the factor squared and ")
            .math(r"z = 1/\lambda")
            .text(" solves the polynomial. The desk's factors 0.5 and 0.6 sit inside; the polynomial's roots 2 and 5/3 sit outside; one fade, two dialects. None of these change the reading drill; they only change the costume."))
        .explain(r"\Delta", "The differencing shorthand",
            "Papers' one-letter name for I minus L: today minus yesterday, for every row at once.")
        .explain(r"\Delta^2", "Difference twice",
            "The differencing recipe applied twice — the acceleration, with weights 1, minus 2, 1, exactly as the workout expanded it.")
        .explain(r"z = 1/\lambda", "The reciprocal bridge",
            "Divide lambda-squared equals 1.1 lambda minus 0.3 through by lambda squared and the equation for z is the lag polynomial set to zero. Factors inside the unit circle and polynomial roots outside it are one statement, inverted.")
}

/// Retrieval practice: eight workouts — worked, faded, independent, diagnostic
/// — every number drawn from the lesson's own running examples.
fn practice(b: LessonBuilder) -> LessonBuilder {
    b.rule()
        .heading("Practice: run the five moves")
        .para(|p| p
            .text("Reading is a muscle. Eight workouts, every number taken from this lesson's own examples — the price column 100, 103, 106, 103, 100, 97, the desk's models, the printed undo. For each, commit to an answer before reading past the rule."))
        .para(|p| p
            .text("First, one worked in full, so the moves stay visible. Convert the half-of-two-day-change recipe:"))
        .display(r"y_t = \frac{x_t - x_{t-2}}{2}")
        .explain(r"\frac{x_t - x_{t-2}}{2}", "Half the two-day change",
            "Today minus two days ago, halved: a speed estimate over a two-day stride.")
        .para(|p| p
            .text("Columns: ")
            .math("x")
            .text(" in, ")
            .math("y")
            .text(" out. Shifts: 0 and 2. Weights: one half and minus one half. Stack the rows:"))
        .display(r"y = \frac{I - L^2}{2} x")
        .explain(r"\frac{I - L^2}{2} x", "The whole recipe, applied",
            "The half-of-two-day-change recipe applied to the entire history in one multiplication: every row's speed estimate computed simultaneously.")
        .explain(r"\frac{I - L^2}{2}", "Do nothing minus slide-twice, halved",
            "Row t reads half of x at t minus x at t-2: the two-day change over two, for every date at once.")
        .para(|p| p
            .text("A dividend for factoring: ")
            .math(r"I - L^2")
            .text(" is ")
            .math(r"(I - L)(I + L)")
            .text(", so half the two-day change is also the two-day average of the daily changes — equally the daily change of the two-day averages, the factors being polynomials in ")
            .math("L")
            .text(". Move five, on row 4 of the price column, all three routes: direct, 103 minus 103 over 2 is 0; average the changes, minus 3 and plus 3 average to 0; difference the averages, 104.5 minus 104.5 is 0. Three routes, one answer."))
        .explain(r"I - L^2", "Do nothing minus slide-twice",
            "The two-day change for every date at once: weight 1 on today, minus 1 on two days back.")
        .explain(r"(I - L)(I + L)", "Difference times two-day sum",
            "Multiply the factors out and I - L^2 returns. Two familiar recipes composing into a third — and they commute, being polynomials in L.")
        .explain(r"(I + L)", "Keep today, add yesterday",
            "Do nothing plus slide-once: the two-day sum; halved, the two-day average.")
        .rule()
        .para(|p| p
            .text("Now you supply both rows. The VAR of the second dress — own memories 0.8 and 0.7, leaks 0.1 and 0.2 — was shocked in its first spread and leaked 2 into the neighbour on day one. Run the mirror experiment: shock the second spread alone, 0 and 10. Day one is worked for you: 0.8 times 0 plus 0.1 times 10 is 1; 0.2 times 0 plus 0.7 times 10 is 7 — the state lands at 1 and 7. Supply day two — and say why this shock leaked only 1 where the other leaked 2."))
        .note("Two SUMPRODUCTs, four products — commit to both cells before reading on.")
        .para(|p| p
            .text("Top row: 0.8 times 1 plus 0.1 times 7 — 0.8 plus 0.7 is 1.5. Bottom row: 0.2 times 1 plus 0.7 times 7 — 0.2 plus 4.9 is 5.1. The state is 1.5 and 5.1. The asymmetry is two different cells doing two different jobs: the first spread's news travels through the 0.2, the second's through the 0.1 — the channel has a different width in each direction, and knowing which cell is which is what reading the matrix means. Both spreads then fade, the step's sustainable factors 0.9 and 0.6 sitting inside 1."))
        .rule()
        .para(|p| p
            .text("A charting package hands you its smoother in matrix dress:"))
        .display(r"y = \frac{L + I + L'}{3} x")
        .explain(r"\frac{L + I + L'}{3}", "Slide down, do nothing, slide up — averaged",
            "One third each of yesterday, today and tomorrow. Which everyday recipe this is, and whether a desk can use it, is the exercise.")
        .explain(r"\frac{L + I + L'}{3} x", "The smoother, applied",
            "The recipe run down the entire history in one multiplication — every row's three-day average, centred on itself.")
        .para(|p| p
            .text("Recover the scalar row, check it on row 4 of the price column — and then say whether a desk can trade the signal."))
        .note("Locate every nonzero weight relative to the diagonal before reading on. If you cannot recover the scalar row, you have only looked at the equation.")
        .para(|p| p
            .text("Row ")
            .math("t")
            .text(" reads a third of ")
            .math(r"x_{t-1}")
            .text(" plus ")
            .math(r"x_t")
            .text(" plus ")
            .math(r"x_{t+1}")
            .text(": the centred three-day average. Row 4 is (106 + 103 + 100) / 3 — 103, an average of the wrong three days. The tempting verdict is that nothing is wrong: the weights are the same three 1/3s as ")
            .math("W")
            .text(", the curve hugs the data beautifully, and every charting package draws it. That is exactly why it snares people — it is a legitimate average, of days a live desk does not have. The ")
            .math(r"L'")
            .text(" third sits above the diagonal: the smoothed day-4 value cannot exist until day 5's close does, and a backtest built on it buys yesterday's tomorrow. The tradable version is ")
            .math("W")
            .text(", the same numbers one row later. And the sheet's edges now bite at both ends — row 1 has no yesterday, row 6 no tomorrow."))
        .rule()
        .para(|p| p
            .text("A spread prints three days: 1, 3, 2 basis points. Lay out the yesterday column ")
            .math("u")
            .text(" against the today column ")
            .math("v")
            .text(", estimate the memory with the lesson's ratio — then compute the miss column, SUMPRODUCT it against ")
            .math("u")
            .text(", and predict that last number before doing its arithmetic:"))
        .display(r"\hat a = \frac{u \cdot v}{u \cdot u}")
        .note("Write the two columns out first — the alignment is the exercise.")
        .para(|p| p
            .text("Three days give only two aligned rows: ")
            .math("u")
            .text(" holds 1 and 3, ")
            .math("v")
            .text(" holds 3 and 2. SUMPRODUCT(u, v) is 3 plus 6, 9; SUMPRODUCT(u, u) is 1 plus 9, 10; the estimate is 0.9 — the opening photograph's coefficient, recovered from data in two cells. The miss column is 3 minus 0.9 and 2 minus 2.7 — 2.1 and minus 0.7 — and against ")
            .math("u")
            .text(" it SUMPRODUCTs to 2.1 minus 2.1: zero, exactly. You could have predicted the zero with no arithmetic at all, because perpendicularity is not a consequence of the formula — it is the formula, rearranged. With more columns it becomes one such zero per column, the normal equations; and two columns carrying the same information collapse them — the singular-matrix error of every stats package, live."))
        .rule()
        .para(|p| p
            .text("In the sheet figure, the matrix ")
            .math("A")
            .text(" occupies the block C1:E6. Which spreadsheet cell holds ")
            .math(r"A_{31}")
            .text("?"))
        .explain(r"A_{31}", "One entry of the block",
            "The whole question is which index runs first. Settle that before trusting your eyes.")
        .note("Commit to a cell before reading on — a reflex is being tested.")
        .para(|p| p
            .text("The tempting answer is E1, because sheet addresses read column-first — D4 means column D, row 4 — so a sheet-trained eye takes the 3 for a column count. Matrix subscripts read row-then-column: row 3, column 1, and the block's first column is C, so the entry lives in C3. Cross-check with the figure's own label, which put the block's row 4, column 2 entry at cell D4."))
        .rule()
        .para(|p| p
            .text("The undo at ")
            .math("a")
            .text(" = 0.9 is printed in full in the figure above. Feed it the column whose only nonzero cell is a 1 in row 3 — a lone unit shock on day 3 — and write all six output cells. Count the SUMPRODUCTs you actually perform."))
        .note("All six cells, in order, and the count — before reading on.")
        .para(|p| p
            .text("Zero SUMPRODUCTs. Grinding the rows would take six; the column reading takes none: an input with a lone 1 in slot 3 hands back column 3 of the block, unread — 0, 0, 1, 0.9, 0.81, 0.73. The zeros above the 1 are the future never reaching back; the fade below it is the memory curve, started on day 3. And the reading adds: shocks on days 3 and 4 return column 3 plus column 4 — 0, 0, 1, 1.9, 1.71, 1.54. Across a row, what went into today; down a column, where one day's shock goes."))
        .rule()
        .para(|p| p
            .text("Next, the edge. On the six-day sheet, a colleague refuses to write"))
        .display(r"x = (I - L)^{-1} \varepsilon")
        .para(|p| p
            .text("for a random walk — \"the geometric series only adds up when the ratio is under 1 in size, and here ")
            .math("a")
            .text(" = 1.\" Right or wrong?"))
        .note("Decide, and say why, before reading on.")
        .para(|p| p
            .text("Tempting to agree — this lesson sold the inverse through the geometric reflex, then leaned on ")
            .math(r"|a| < 1")
            .text(". But on the finite sheet the colleague is wrong: the expansion stops by itself because ")
            .math(r"L^6")
            .text(" is the zero recipe, the sum is exactly ")
            .math("S")
            .text(", the triangle of ones, and the random walk is an undiscounted running total of shocks — =SUM(news so far). The size condition bites only under the papers' idealisation of history running back forever. Knowing which world an equation lives in — finite sheet or infinite past — is part of reading it."))
        .rule()
        .para(|p| p
            .text("Last of all, the wild in its native dress. A paper on the desk's own model writes"))
        .display(r"(1 - 1.1 B + 0.3 B^2) x_t = \varepsilon_t")
        .explain(r"(1 - 1.1 B + 0.3 B^2)", "The lag polynomial, in costume",
            "The desk model's entire left-hand side folded into one parenthesis, with B — backshift — playing this lesson's L.")
        .explain(r"1.1 B", "Yesterday's weight, on the slide",
            "The coefficient on yesterday, riding the backshift: subtracting 1.1 times the slid-down series.")
        .explain(r"0.3 B^2", "The two-day weight, on the double slide",
            "The second coefficient riding two backshifts — the sign flipped by moving everything to the left side.")
        .para(|p| p
            .text("and declares it stationary \"because both roots of the lag polynomial lie outside the unit circle\". Two puzzles: what is B — and this lesson computed 0.5 and 0.6, comfortably inside. Is the paper wrong?"))
        .note("Name B, find the paper's two roots, and reconcile inside with outside — before reading on.")
        .para(|p| p
            .text("Tempting: the paper has it backwards — inside 1 meant stable at every stop of this lesson. But B is only the backshift, statistics' letter for ")
            .math("L")
            .text(", and the paper is right. Treat its parenthesis as school algebra in a number ")
            .math("z")
            .text(": the discriminant of 0.3 z-squared minus 1.1 z plus 1 is 1.21 minus 1.20 — 0.01 — and the roots are (1.1 plus or minus 0.1) over 0.6: 2 and 5/3. Outside, as claimed, and exact reciprocals of the factors 0.5 and 0.6, as the field marks said: divide the factor equation through by the factor squared. Same fade, two dialects; knowing which one a paper speaks is part of reading it."))
        .para(|p| p
            .text("The jump, restated once. Scalar algebra reads one cell's formula; linear algebra names whole columns and whole drag-down recipes, then does school algebra on the names. The two photographs are the same sheet. You now own the camera."))
}

/// Single-character meanings, set once for the whole lesson so that a bare
/// letter hovered anywhere says what this lesson means by it.
fn letter_overrides(b: LessonBuilder) -> LessonBuilder {
    b.explain_char('x', "The observed series",
        "One column of numbers, one per date, named as a single object. The subscript picks a row; bare x means the whole column.")
        .explain_char('y', "The output column",
            "What the recipe produces: one value per date, stacked into a column.")
        .explain_char('a', "The memory coefficient",
            "The fraction of yesterday's value that survives into today. Below 1 in size the past fades; at 1 it accumulates; above 1 it compounds.")
        .explain_char('L', "The lag matrix",
            "Ones one step below the diagonal: applying it slides a column down one row, so each cell holds the previous date's value.")
        .explain_char('I', "The identity matrix",
            "Ones down the diagonal: the recipe that returns a column unchanged — the matrix world's number 1.")
        .explain_char('W', "The moving-average recipe",
            "The block whose row t holds weight 1/3 on the three most recent days: the drag-down AVERAGE, written as a matrix.")
        .explain_char('S', "The running-total matrix",
            "A triangle of ones: row t holds 1 on every day up to t, so applying it computes every SUM-so-far at once.")
        .explain_char('A', "A matrix",
            "A named rectangular block of weights: one row per output, one column per input. In the second dress, the one-day step — a companion stacking one series' memories, or a VAR step stacking neighbouring series.")
        .explain_char('B', "A second matrix — or the backshift",
            "Another named block, kept distinct from A to show that order matters. In the final workout, B is the backshift: statistics' letter for the lag matrix L.")
        .explain_char('h', "The holdings column",
            "How many units of each asset the book carries, stacked into a column.")
        .explain_char('p', "The price column",
            "What one unit of each asset costs, stacked into a column aligned with the holdings.")
        .explain_char('u', "The yesterday column",
            "The series slid down one row, so each cell holds the previous date's value — the regressor.")
        .explain_char('v', "The today column",
            "The series as it stands, aligned cell for cell against the yesterday column.")
        .explain_char('z', "The state — or a stand-in number",
            "Usually the short list of recent memories that determines the model's next step. In the field marks and final workout, the ordinary number a lag polynomial is evaluated at.")
        .explain_char('q', "The geometric ratio",
            "The common ratio of a geometric series: the factor each successive term carries.")
        .explain_char('b', "The drift",
            "A constant added every period — the intercept. In column form it becomes b times the column of ones.")
        .explain_char('m', "The number of outputs",
            "How many rows the block has: one per output cell the recipe produces.")
        .explain_char('X', "The regressor block",
            "Regressor columns standing side by side — the design matrix. Its grid X-prime-X collects every column's SUMPRODUCT against every other.")
        .explain_char('β', "The coefficient column",
            "Greek beta: one weight per regressor column, chosen so the miss is perpendicular to every column at once.")
        .vector('x')
        .vector('y')
        .vector('h')
        .vector('p')
        .vector('u')
        .vector('v')
        .vector('z')
        .vector('ε')
        .vector('ι')
        .vector('β')
        .matrix('X')
        .matrix('L')
        .matrix('I')
        .matrix('W')
        .matrix('S')
        .matrix('A')
        .matrix('B')
}

/// [fig 1] A spreadsheet with a column named as a vector and a block named as
/// a matrix. Prices match the lesson's running example.
const SHEET_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 470 252" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="470" height="252" rx="8" fill="#f8fafc"/>
<!-- column headers -->
<g fill="#e2e8f0">
  <rect x="34" y="14" width="72" height="20"/><rect x="106" y="14" width="72" height="20"/>
  <rect x="178" y="14" width="72" height="20"/><rect x="250" y="14" width="72" height="20"/>
  <rect x="322" y="14" width="72" height="20"/>
</g>
<g fill="#64748b" text-anchor="middle">
  <text x="70" y="28">A</text><text x="142" y="28">B</text><text x="214" y="28">C</text>
  <text x="286" y="28">D</text><text x="358" y="28">E</text>
</g>
<!-- row headers -->
<g fill="#e2e8f0">
  <rect x="10" y="34" width="24" height="26"/><rect x="10" y="60" width="24" height="26"/>
  <rect x="10" y="86" width="24" height="26"/><rect x="10" y="112" width="24" height="26"/>
  <rect x="10" y="138" width="24" height="26"/><rect x="10" y="164" width="24" height="26"/>
</g>
<g fill="#64748b" text-anchor="middle">
  <text x="22" y="51">1</text><text x="22" y="77">2</text><text x="22" y="103">3</text>
  <text x="22" y="129">4</text><text x="22" y="155">5</text><text x="22" y="181">6</text>
</g>
<!-- cell fills: column B (vector) and block C1:E6 (matrix) -->
<rect x="106" y="34" width="72" height="156" fill="#dcfce7"/>
<rect x="178" y="34" width="216" height="156" fill="#dbeafe"/>
<!-- highlighted cells: B4 and D4 -->
<rect x="106" y="112" width="72" height="26" fill="none" stroke="#16a34a" stroke-width="2"/>
<rect x="250" y="112" width="72" height="26" fill="none" stroke="#2563eb" stroke-width="2"/>
<!-- grid lines -->
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M10 34H394M10 60H394M10 86H394M10 112H394M10 138H394M10 164H394M10 190H394"/>
  <path d="M10 14V190M34 14V190M106 14V190M178 14V190M250 14V190M322 14V190M394 14V190"/>
</g>
<!-- column A: dates -->
<g fill="#64748b" text-anchor="middle">
  <text x="70" y="51">day 1</text><text x="70" y="77">day 2</text><text x="70" y="103">day 3</text>
  <text x="70" y="129">day 4</text><text x="70" y="155">day 5</text><text x="70" y="181">day 6</text>
</g>
<!-- column B: prices -->
<g fill="#166534" text-anchor="middle">
  <text x="142" y="51">100</text><text x="142" y="77">103</text><text x="142" y="103">106</text>
  <text x="142" y="129">103</text><text x="142" y="155">100</text><text x="142" y="181">97</text>
</g>
<!-- block entry label at D4 = matrix row 4, column 2 -->
<text x="286" y="129" fill="#1d4ed8" text-anchor="middle">A<tspan font-size="8" dy="3">42</tspan></text>
<!-- annotations -->
<text x="142" y="212" fill="#166534" text-anchor="middle">one name for the column: x</text>
<text x="142" y="228" fill="#166534" text-anchor="middle">cell B4 is x<tspan font-size="8" dy="3">4</tspan><tspan dy="-3"> = 103</tspan></text>
<text x="286" y="212" fill="#1d4ed8" text-anchor="middle">one name for the block: A</text>
<text x="286" y="228" fill="#1d4ed8" text-anchor="middle">A<tspan font-size="8" dy="3">42</tspan><tspan dy="-3"> = block row 4, column 2</tspan></text>
</svg>"##;

/// [fig 2] The three-day moving average as a banded matrix W times the price
/// column x, with row 4's SUMPRODUCT worked in the annotation.
/// y_3 = (100+103+106)/3 = 103, y_4 = (103+106+103)/3 = 104,
/// y_5 = (106+103+100)/3 = 103, y_6 = (103+100+97)/3 = 100.
const BAND_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 280" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="640" height="280" rx="8" fill="#f8fafc"/>
<text x="149" y="24" fill="#64748b" text-anchor="middle">W (one row per date)</text>
<text x="418" y="24" fill="#64748b" text-anchor="middle">x</text>
<text x="524" y="24" fill="#64748b" text-anchor="middle">y = Wx</text>
<!-- W grid: 6x6, cells 38x28, origin (35,34) -->
<g fill="#f1f5f9"><rect x="35" y="34" width="228" height="56"/></g>
<g fill="#dcfce7">
  <rect x="35" y="90" width="114" height="28"/>
  <rect x="73" y="118" width="114" height="28"/>
  <rect x="111" y="146" width="114" height="28"/>
  <rect x="149" y="174" width="114" height="28"/>
</g>
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M35 34H263M35 62H263M35 90H263M35 118H263M35 146H263M35 174H263M35 202H263"/>
  <path d="M35 34V202M73 34V202M111 34V202M149 34V202M187 34V202M225 34V202M263 34V202"/>
</g>
<g fill="#166534" text-anchor="middle">
  <text x="54" y="108">1/3</text><text x="92" y="108">1/3</text><text x="130" y="108">1/3</text>
  <text x="92" y="136">1/3</text><text x="130" y="136">1/3</text><text x="168" y="136">1/3</text>
  <text x="130" y="164">1/3</text><text x="168" y="164">1/3</text><text x="206" y="164">1/3</text>
  <text x="168" y="192">1/3</text><text x="206" y="192">1/3</text><text x="244" y="192">1/3</text>
</g>
<!-- row labels -->
<g fill="#94a3b8" text-anchor="end">
  <text x="28" y="52">1</text><text x="28" y="80">2</text><text x="28" y="108">3</text>
  <text x="28" y="136">4</text><text x="28" y="164">5</text><text x="28" y="192">6</text>
</g>
<text x="270" y="56" fill="#b45309" font-size="10">boundary</text>
<text x="270" y="70" fill="#b45309" font-size="10">rows</text>
<!-- row 4 highlight across W, x and y -->
<rect x="35" y="118" width="228" height="28" fill="none" stroke="#16a34a" stroke-width="2"/>
<!-- times sign -->
<text x="352" y="122" fill="#64748b" font-size="16" text-anchor="middle">&#215;</text>
<!-- x column -->
<g fill="#dcfce7"><rect x="390" y="34" width="56" height="168"/></g>
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M390 34H446M390 62H446M390 90H446M390 118H446M390 146H446M390 174H446M390 202H446"/>
  <path d="M390 34V202M446 34V202"/>
</g>
<g fill="#166534" text-anchor="middle">
  <text x="418" y="52">100</text><text x="418" y="80">103</text><text x="418" y="108">106</text>
  <text x="418" y="136">103</text><text x="418" y="164">100</text><text x="418" y="192">97</text>
</g>
<!-- equals sign -->
<text x="470" y="122" fill="#64748b" font-size="16" text-anchor="middle">=</text>
<!-- y column -->
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M496 34H552M496 62H552M496 90H552M496 118H552M496 146H552M496 174H552M496 202H552"/>
  <path d="M496 34V202M552 34V202"/>
</g>
<g fill="#94a3b8" text-anchor="middle">
  <text x="524" y="52">&#8212;</text><text x="524" y="80">&#8212;</text>
</g>
<g fill="#166534" text-anchor="middle">
  <text x="524" y="108">103</text><text x="524" y="136">104</text>
  <text x="524" y="164">103</text><text x="524" y="192">100</text>
</g>
<rect x="496" y="118" width="56" height="28" fill="none" stroke="#16a34a" stroke-width="2"/>
<!-- annotation -->
<text x="320" y="234" fill="#166534" text-anchor="middle">row 4:  SUMPRODUCT(row 4 of W, x) = (103 + 106 + 103) / 3 = 104</text>
<text x="320" y="256" fill="#64748b" text-anchor="middle">empty cells are zeros; the same three weights slide one step right with each row down</text>
</svg>"##;

/// [fig 3] The lag matrix: ones one step below the diagonal slide the column
/// down one row; row 1 of the output has nothing above it.
const LAG_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 262" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="620" height="262" rx="8" fill="#f8fafc"/>
<text x="147" y="24" fill="#64748b" text-anchor="middle">L (ones one step below the diagonal)</text>
<text x="384" y="24" fill="#64748b" text-anchor="middle">x</text>
<text x="536" y="24" fill="#64748b" text-anchor="middle">Lx</text>
<!-- L grid: 6x6, cells 38x28, origin (33,34) -->
<g fill="#dcfce7">
  <rect x="33" y="62" width="38" height="28"/><rect x="71" y="90" width="38" height="28"/>
  <rect x="109" y="118" width="38" height="28"/><rect x="147" y="146" width="38" height="28"/>
  <rect x="185" y="174" width="38" height="28"/>
</g>
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M33 34H261M33 62H261M33 90H261M33 118H261M33 146H261M33 174H261M33 202H261"/>
  <path d="M33 34V202M71 34V202M109 34V202M147 34V202M185 34V202M223 34V202M261 34V202"/>
</g>
<g fill="#166534" text-anchor="middle">
  <text x="52" y="80">1</text><text x="90" y="108">1</text><text x="128" y="136">1</text>
  <text x="166" y="164">1</text><text x="204" y="192">1</text>
</g>
<!-- times sign -->
<text x="298" y="122" fill="#64748b" font-size="16" text-anchor="middle">&#215;</text>
<!-- x column -->
<g fill="#dcfce7"><rect x="356" y="34" width="56" height="168"/></g>
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M356 34H412M356 62H412M356 90H412M356 118H412M356 146H412M356 174H412M356 202H412"/>
  <path d="M356 34V202M412 34V202"/>
</g>
<g fill="#166534" text-anchor="middle">
  <text x="384" y="52">100</text><text x="384" y="80">103</text><text x="384" y="108">106</text>
  <text x="384" y="136">103</text><text x="384" y="164">100</text><text x="384" y="192">97</text>
</g>
<!-- equals sign -->
<text x="436" y="122" fill="#64748b" font-size="16" text-anchor="middle">=</text>
<!-- slide arrows from x row i to Lx row i+1 -->
<g stroke="#16a34a" stroke-width="1.4" fill="none">
  <path d="M416 46 L502 70"/><path d="M416 74 L502 98"/><path d="M416 102 L502 126"/>
  <path d="M416 130 L502 154"/><path d="M416 158 L502 182"/>
</g>
<g fill="#16a34a">
  <polygon points="504,71 495,70 499,63"/><polygon points="504,99 495,98 499,91"/>
  <polygon points="504,127 495,126 499,119"/><polygon points="504,155 495,154 499,147"/>
  <polygon points="504,183 495,182 499,175"/>
</g>
<!-- Lx column -->
<rect x="508" y="34" width="56" height="28" fill="#fef3c7"/>
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M508 34H564M508 62H564M508 90H564M508 118H564M508 146H564M508 174H564M508 202H564"/>
  <path d="M508 34V202M564 34V202"/>
</g>
<g fill="#166534" text-anchor="middle">
  <text x="536" y="80">100</text><text x="536" y="108">103</text><text x="536" y="136">106</text>
  <text x="536" y="164">103</text><text x="536" y="192">100</text>
</g>
<text x="536" y="52" fill="#b45309" text-anchor="middle">0</text>
<!-- annotation -->
<text x="310" y="232" fill="#166534" text-anchor="middle">every value slides down one row; 97 falls off the bottom of the sheet</text>
<text x="310" y="252" fill="#b45309" text-anchor="middle">row 1 has nothing above it &#8212; the convention: the world was quiet before day 1</text>
</svg>"##;

/// [fig 4] One step of the AR(2) in state form: companion matrix times
/// yesterday's state. Top row 1.1 * 4 - 0.3 * 10 = 1.4; bottom row copies 4.
const STATE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 224" font-family="sans-serif" font-size="12">
<rect x="0" y="0" width="640" height="224" rx="8" fill="#f8fafc"/>
<text x="87" y="30" fill="#64748b" text-anchor="middle">today's state z<tspan font-size="9" dy="3">t</tspan></text>
<text x="257" y="30" fill="#64748b" text-anchor="middle">companion A</text>
<text x="425" y="30" fill="#64748b" text-anchor="middle">yesterday's state z<tspan font-size="9" dy="3">t-1</tspan></text>
<text x="551" y="30" fill="#64748b" text-anchor="middle">shock</text>
<!-- z_t column -->
<g stroke="#cbd5e1" fill="#dcfce7"><rect x="42" y="44" width="90" height="30"/><rect x="42" y="74" width="90" height="30"/></g>
<g fill="#166534" text-anchor="middle">
  <text x="87" y="63">x<tspan font-size="9" dy="3">t</tspan></text>
  <text x="87" y="93">x<tspan font-size="9" dy="3">t-1</tspan></text>
</g>
<text x="150" y="90" fill="#64748b" font-size="15" text-anchor="middle">=</text>
<!-- companion matrix -->
<g stroke="#cbd5e1" fill="#dbeafe">
  <rect x="168" y="44" width="90" height="30"/><rect x="258" y="44" width="90" height="30"/>
  <rect x="168" y="74" width="90" height="30"/><rect x="258" y="74" width="90" height="30"/>
</g>
<g fill="#1d4ed8" text-anchor="middle">
  <text x="213" y="63">1.1</text><text x="303" y="63">&#8722;0.3</text>
  <text x="213" y="93">1</text><text x="303" y="93">0</text>
</g>
<text x="366" y="90" fill="#64748b" font-size="15" text-anchor="middle">&#215;</text>
<!-- z_{t-1} column -->
<g stroke="#cbd5e1" fill="#dcfce7"><rect x="384" y="44" width="90" height="30"/><rect x="384" y="74" width="90" height="30"/></g>
<g fill="#166534" text-anchor="middle">
  <text x="429" y="63">4</text><text x="429" y="93">10</text>
</g>
<text x="492" y="90" fill="#64748b" font-size="15" text-anchor="middle">+</text>
<!-- shock column -->
<g stroke="#cbd5e1" fill="#fef3c7"><rect x="510" y="44" width="82" height="30"/><rect x="510" y="74" width="82" height="30"/></g>
<g fill="#b45309" text-anchor="middle">
  <text x="551" y="63">0</text><text x="551" y="93">0</text>
</g>
<!-- result annotations -->
<text x="320" y="140" fill="#1d4ed8" text-anchor="middle">top row (the model):  1.1 &#215; 4  &#8722;  0.3 &#215; 10  =  1.4</text>
<text x="320" y="162" fill="#64748b" text-anchor="middle">bottom row (bookkeeping):  1 &#215; 4  +  0 &#215; 10  =  4  &#8212;  copy the newer memory down</text>
<text x="320" y="196" fill="#166534" text-anchor="middle">so z<tspan font-size="9" dy="3">t</tspan><tspan dy="-3"> = (1.4, 4): the spread overshoots down toward its mean, and the memories shuffle</tspan></text>
</svg>"##;

/// [fig 5] The dot product as PRODUCT then SUM: 10*100 = 1,000; 5*20 = 100;
/// 2*50 = 100; total 1,200 — the running portfolio example.
const DOT_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 200" font-family="sans-serif" font-size="11">
<!-- 10 * 100 = 1,000; 5 * 20 = 100; 2 * 50 = 100; 1,000 + 100 + 100 = 1,200 -->
<rect x="0" y="0" width="620" height="200" rx="8" fill="#f8fafc"/>
<text x="88" y="30" fill="#64748b" text-anchor="middle">h (holdings)</text>
<text x="204" y="30" fill="#64748b" text-anchor="middle">p (prices)</text>
<text x="320" y="30" fill="#64748b" text-anchor="middle">multiply, row by row</text>
<text x="465" y="30" fill="#64748b" text-anchor="middle">h &#183; p (one number)</text>
<g fill="#dcfce7"><rect x="60" y="44" width="56" height="28"/><rect x="60" y="72" width="56" height="28"/><rect x="60" y="100" width="56" height="28"/></g>
<g fill="#dcfce7"><rect x="176" y="44" width="56" height="28"/><rect x="176" y="72" width="56" height="28"/><rect x="176" y="100" width="56" height="28"/></g>
<g fill="#dbeafe"><rect x="292" y="44" width="56" height="28"/><rect x="292" y="72" width="56" height="28"/><rect x="292" y="100" width="56" height="28"/></g>
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M60 44H116M60 72H116M60 100H116M60 128H116M60 44V128M116 44V128"/>
  <path d="M176 44H232M176 72H232M176 100H232M176 128H232M176 44V128M232 44V128"/>
  <path d="M292 44H348M292 72H348M292 100H348M292 128H348M292 44V128M348 44V128"/>
</g>
<g fill="#166534" text-anchor="middle">
  <text x="88" y="62">10</text><text x="88" y="90">5</text><text x="88" y="118">2</text>
  <text x="204" y="62">100</text><text x="204" y="90">20</text><text x="204" y="118">50</text>
</g>
<g fill="#1d4ed8" text-anchor="middle">
  <text x="320" y="62">1,000</text><text x="320" y="90">100</text><text x="320" y="118">100</text>
</g>
<text x="146" y="90" fill="#64748b" font-size="15" text-anchor="middle">&#215;</text>
<text x="262" y="90" fill="#64748b" font-size="15" text-anchor="middle">=</text>
<g stroke="#b45309" stroke-width="1.4" fill="none">
  <path d="M352 58 L424 82"/><path d="M352 86 L424 88"/><path d="M352 114 L424 94"/>
</g>
<rect x="430" y="74" width="70" height="28" fill="#fef3c7" stroke="#cbd5e1"/>
<text x="465" y="93" fill="#b45309" text-anchor="middle">1,200</text>
<text x="310" y="158" fill="#166534" text-anchor="middle">=SUMPRODUCT(h, p):  10 &#215; 100  +  5 &#215; 20  +  2 &#215; 50  =  1,200 &#8212; the book's value</text>
<text x="310" y="180" fill="#64748b" text-anchor="middle">two columns in, one number out; the sheet's =h*p stops at the blue column &#8212; the dot adds it up</text>
</svg>"##;

/// [fig 6] The inverse (I - aL)^{-1} written out at a = 0.9: lower-triangular,
/// entry (t,s) = a^(t-s). Row 6 across is the scalar solution; column 1 down
/// is the memory curve. Exact: 1, 0.9, 0.81, 0.729, 0.6561, 0.59049.
const INVERSE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 520 300" font-family="sans-serif" font-size="11">
<!-- entry (t,s) = 0.9^(t-s) for s <= t, else 0. Exact values 1, 0.9, 0.81, 0.729, 0.6561, 0.59049;
     shown to 2 d.p., and each diagonal's fill-opacity equals its rounded entry, so shade = weight.
     Row 6 across: x6 = .59 e1 + .66 e2 + .73 e3 + .81 e4 + .9 e5 + 1 e6 (e = shock). -->
<rect x="0" y="0" width="520" height="300" rx="8" fill="#f8fafc"/>
<text x="260" y="20" fill="#64748b" text-anchor="middle">(I &#8722; aL)&#8315;&#185; at a = 0.9 &#8212; row t, column s holds a to the power t&#8722;s</text>
<g fill="#4ade80" fill-opacity="1.00"><rect x="116" y="40" width="48" height="30"/><rect x="164" y="70" width="48" height="30"/><rect x="212" y="100" width="48" height="30"/><rect x="260" y="130" width="48" height="30"/><rect x="308" y="160" width="48" height="30"/><rect x="356" y="190" width="48" height="30"/></g>
<g fill="#4ade80" fill-opacity="0.90"><rect x="116" y="70" width="48" height="30"/><rect x="164" y="100" width="48" height="30"/><rect x="212" y="130" width="48" height="30"/><rect x="260" y="160" width="48" height="30"/><rect x="308" y="190" width="48" height="30"/></g>
<g fill="#4ade80" fill-opacity="0.81"><rect x="116" y="100" width="48" height="30"/><rect x="164" y="130" width="48" height="30"/><rect x="212" y="160" width="48" height="30"/><rect x="260" y="190" width="48" height="30"/></g>
<g fill="#4ade80" fill-opacity="0.73"><rect x="116" y="130" width="48" height="30"/><rect x="164" y="160" width="48" height="30"/><rect x="212" y="190" width="48" height="30"/></g>
<g fill="#4ade80" fill-opacity="0.66"><rect x="116" y="160" width="48" height="30"/><rect x="164" y="190" width="48" height="30"/></g>
<g fill="#4ade80" fill-opacity="0.59"><rect x="116" y="190" width="48" height="30"/></g>
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M116 40H404M116 70H404M116 100H404M116 130H404M116 160H404M116 190H404M116 220H404"/>
  <path d="M116 40V220M164 40V220M212 40V220M260 40V220M308 40V220M356 40V220M404 40V220"/>
</g>
<g fill="#14532d" text-anchor="middle">
  <text x="140" y="59">1</text>
  <text x="140" y="89">.9</text><text x="188" y="89">1</text>
  <text x="140" y="119">.81</text><text x="188" y="119">.9</text><text x="236" y="119">1</text>
  <text x="140" y="149">.73</text><text x="188" y="149">.81</text><text x="236" y="149">.9</text><text x="284" y="149">1</text>
  <text x="140" y="179">.66</text><text x="188" y="179">.73</text><text x="236" y="179">.81</text><text x="284" y="179">.9</text><text x="332" y="179">1</text>
  <text x="140" y="209">.59</text><text x="188" y="209">.66</text><text x="236" y="209">.73</text><text x="284" y="209">.81</text><text x="332" y="209">.9</text><text x="380" y="209">1</text>
</g>
<g fill="#94a3b8" text-anchor="middle"><text x="140" y="34">1</text><text x="188" y="34">2</text><text x="236" y="34">3</text><text x="284" y="34">4</text><text x="332" y="34">5</text><text x="380" y="34">6</text></g>
<g fill="#94a3b8" text-anchor="end"><text x="108" y="59">1</text><text x="108" y="89">2</text><text x="108" y="119">3</text><text x="108" y="149">4</text><text x="108" y="179">5</text><text x="108" y="209">6</text></g>
<rect x="116" y="190" width="288" height="30" fill="none" stroke="#16a34a" stroke-width="2"/>
<rect x="116" y="40" width="48" height="180" fill="none" stroke="#b45309" stroke-width="2"/>
<text x="260" y="246" fill="#166534" text-anchor="middle">row 6, read across: x&#8326; = .59&#949;&#8321; + .66&#949;&#8322; + .73&#949;&#8323; + .81&#949;&#8324; + .9&#949;&#8325; + &#949;&#8326;</text>
<text x="260" y="266" fill="#b45309" text-anchor="middle">column 1, read down: day 1's shock fading through time &#8212; the plot's memory curve</text>
<text x="260" y="286" fill="#64748b" text-anchor="middle">empty cells are zeros &#8212; the future never reaches back; at a = 1 the triangle fills with ones: S</text>
</svg>"##;

/// [fig 7] Time as texture: L below the diagonal (past), its transpose above
/// (future), and the centred 3-day average straddling both. Centred row 4 =
/// (106 + 103 + 100)/3 = 103, using day 5's price.
const TIME_TEXTURE_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 252" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="620" height="252" rx="8" fill="#f8fafc"/>
<text x="100" y="34" fill="#64748b" text-anchor="middle">L &#8212; the lag</text>
<text x="310" y="34" fill="#64748b" text-anchor="middle">L&#8242; &#8212; the transpose</text>
<text x="520" y="34" fill="#64748b" text-anchor="middle">centred 3-day average</text>
<g fill="#dcfce7">
  <rect x="40" y="72" width="20" height="20"/><rect x="60" y="92" width="20" height="20"/><rect x="80" y="112" width="20" height="20"/><rect x="100" y="132" width="20" height="20"/><rect x="120" y="152" width="20" height="20"/>
  <rect x="460" y="72" width="20" height="20"/><rect x="480" y="92" width="20" height="20"/><rect x="500" y="112" width="20" height="20"/><rect x="520" y="132" width="20" height="20"/><rect x="540" y="152" width="20" height="20"/>
</g>
<g fill="#dbeafe">
  <rect x="460" y="52" width="20" height="20"/><rect x="480" y="72" width="20" height="20"/><rect x="500" y="92" width="20" height="20"/><rect x="520" y="112" width="20" height="20"/><rect x="540" y="132" width="20" height="20"/><rect x="560" y="152" width="20" height="20"/>
</g>
<g fill="#fef3c7">
  <rect x="270" y="52" width="20" height="20"/><rect x="290" y="72" width="20" height="20"/><rect x="310" y="92" width="20" height="20"/><rect x="330" y="112" width="20" height="20"/><rect x="350" y="132" width="20" height="20"/>
  <rect x="480" y="52" width="20" height="20"/><rect x="500" y="72" width="20" height="20"/><rect x="520" y="92" width="20" height="20"/><rect x="540" y="112" width="20" height="20"/><rect x="560" y="132" width="20" height="20"/>
</g>
<g stroke="#94a3b8" stroke-dasharray="3,3" fill="none"><path d="M40 52 L160 172"/><path d="M250 52 L370 172"/><path d="M460 52 L580 172"/></g>
<g stroke="#cbd5e1" stroke-width="1" fill="none">
  <path d="M40 52H160M40 72H160M40 92H160M40 112H160M40 132H160M40 152H160M40 172H160"/>
  <path d="M40 52V172M60 52V172M80 52V172M100 52V172M120 52V172M140 52V172M160 52V172"/>
  <path d="M250 52H370M250 72H370M250 92H370M250 112H370M250 132H370M250 152H370M250 172H370"/>
  <path d="M250 52V172M270 52V172M290 52V172M310 52V172M330 52V172M350 52V172M370 52V172"/>
  <path d="M460 52H580M460 72H580M460 92H580M460 112H580M460 132H580M460 152H580M460 172H580"/>
  <path d="M460 52V172M480 52V172M500 52V172M520 52V172M540 52V172M560 52V172M580 52V172"/>
</g>
<g fill="#166534" font-size="9" text-anchor="middle"><text x="50" y="85">1</text><text x="70" y="105">1</text><text x="90" y="125">1</text><text x="110" y="145">1</text><text x="130" y="165">1</text></g>
<g fill="#b45309" font-size="9" text-anchor="middle"><text x="280" y="65">1</text><text x="300" y="85">1</text><text x="320" y="105">1</text><text x="340" y="125">1</text><text x="360" y="145">1</text></g>
<g fill="#166534" font-size="8" text-anchor="middle"><text x="470" y="85">&#8531;</text><text x="490" y="105">&#8531;</text><text x="510" y="125">&#8531;</text><text x="530" y="145">&#8531;</text><text x="550" y="165">&#8531;</text></g>
<g fill="#1d4ed8" font-size="8" text-anchor="middle"><text x="470" y="65">&#8531;</text><text x="490" y="85">&#8531;</text><text x="510" y="105">&#8531;</text><text x="530" y="125">&#8531;</text><text x="550" y="145">&#8531;</text><text x="570" y="165">&#8531;</text></g>
<g fill="#b45309" font-size="8" text-anchor="middle"><text x="490" y="65">&#8531;</text><text x="510" y="85">&#8531;</text><text x="530" y="105">&#8531;</text><text x="550" y="125">&#8531;</text><text x="570" y="145">&#8531;</text></g>
<text x="100" y="192" fill="#166534" font-size="10" text-anchor="middle">only the past &#8212; slides down</text>
<text x="310" y="192" fill="#b45309" font-size="10" text-anchor="middle">the future &#8212; slides up</text>
<text x="520" y="192" fill="#64748b" font-size="10" text-anchor="middle">one foot in tomorrow</text>
<text x="310" y="220" fill="#64748b" text-anchor="middle">below the diagonal: the past &#183; on it: today &#183; above it: the future &#8212; triangularity is causality</text>
<text x="310" y="242" fill="#b45309" text-anchor="middle">centred row 4 = (106 + 103 + 100)/3 = 103 &#8212; it needs day 5's price on day 4: look-ahead, visible as texture</text>
</svg>"##;

/// [fig 8] Three root types, three shapes. Left: (1.1, -0.3), roots 0.6, 0.5,
/// unit-shock echo 6(0.6)^t - 5(0.5)^t = 1, 1.1, 0.91, 0.671, 0.4651, ...
/// Middle: (1, -0.5), complex pair size sqrt(0.5) = 0.7071, the prose's path
/// 10, 10, 5, 0, -2.5, -2.5, -1.25, 0, 0.625 (zeros 4 apart). Right:
/// (1, -0.25) = (lambda - 0.5)^2, response 2t(0.5)^t = 0, 1, 1, 0.75, 0.5, ...
const ROOTS_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 246" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="640" height="246" rx="8" fill="#f8fafc"/>
<g fill="#64748b" font-size="10" text-anchor="middle">
  <text x="116" y="36">two real roots: 0.5 and 0.6</text><text x="326" y="36">complex pair: size &#8776; 0.71</text><text x="536" y="36">repeated root: 0.5, twice</text>
</g>
<g fill="#94a3b8" font-size="9" text-anchor="middle">
  <text x="116" y="50">a&#8321; = 1.1, a&#8322; = &#8722;0.3</text><text x="326" y="50">a&#8321; = 1, a&#8322; = &#8722;0.5</text><text x="536" y="50">a&#8321; = 1, a&#8322; = &#8722;0.25</text>
</g>
<g stroke="#cbd5e1" fill="none"><path d="M34 185H198"/><path d="M244 150H408"/><path d="M454 185H618"/></g>
<g stroke="#16a34a" stroke-width="1.5">
  <path d="M40 185V130"/><path d="M59 185V124.5"/><path d="M78 185V135"/><path d="M97 185V148.1"/><path d="M116 185V159.4"/>
  <path d="M135 185V167.9"/><path d="M154 185V173.9"/><path d="M173 185V177.9"/><path d="M192 185V180.5"/>
</g>
<g fill="#16a34a"><circle cx="40" cy="130" r="2.5"/><circle cx="59" cy="124.5" r="2.5"/><circle cx="78" cy="135" r="2.5"/><circle cx="97" cy="148.1" r="2.5"/><circle cx="116" cy="159.4" r="2.5"/><circle cx="135" cy="167.9" r="2.5"/><circle cx="154" cy="173.9" r="2.5"/><circle cx="173" cy="177.9" r="2.5"/><circle cx="192" cy="180.5" r="2.5"/></g>
<g fill="#166534" font-size="8" text-anchor="middle"><text x="40" y="124">1</text><text x="59" y="118">1.1</text><text x="78" y="129">.91</text></g>
<g stroke="#b45309" stroke-width="1.5">
  <path d="M250 150V90"/><path d="M269 150V90"/><path d="M288 150V120"/><path d="M326 150V165"/><path d="M345 150V165"/><path d="M364 150V157.5"/><path d="M402 150V146.3"/>
</g>
<g fill="#b45309"><circle cx="250" cy="90" r="2.5"/><circle cx="269" cy="90" r="2.5"/><circle cx="288" cy="120" r="2.5"/><circle cx="326" cy="165" r="2.5"/><circle cx="345" cy="165" r="2.5"/><circle cx="364" cy="157.5" r="2.5"/><circle cx="402" cy="146.3" r="2.5"/></g>
<g stroke="#b45309" fill="none"><circle cx="307" cy="150" r="3"/><circle cx="383" cy="150" r="3"/></g>
<g fill="#b45309" font-size="8" text-anchor="middle"><text x="250" y="84">10</text><text x="269" y="84">10</text><text x="288" y="114">5</text><text x="307" y="142">0</text><text x="335" y="178">&#8722;2.5</text><text x="383" y="142">0</text></g>
<g stroke="#2563eb" stroke-width="1.5">
  <path d="M479 185V130"/><path d="M498 185V130"/><path d="M517 185V143.8"/><path d="M536 185V157.5"/><path d="M555 185V167.8"/><path d="M574 185V174.7"/><path d="M593 185V179"/><path d="M612 185V181.6"/>
</g>
<g fill="#2563eb"><circle cx="460" cy="185" r="2.5"/><circle cx="479" cy="130" r="2.5"/><circle cx="498" cy="130" r="2.5"/><circle cx="517" cy="143.8" r="2.5"/><circle cx="536" cy="157.5" r="2.5"/><circle cx="555" cy="167.8" r="2.5"/><circle cx="574" cy="174.7" r="2.5"/><circle cx="593" cy="179" r="2.5"/><circle cx="612" cy="181.6" r="2.5"/></g>
<g fill="#1d4ed8" font-size="8" text-anchor="middle"><text x="460" y="197">0</text><text x="479" y="124">1</text><text x="498" y="124">1</text><text x="517" y="138">.75</text></g>
<g font-size="10" text-anchor="middle">
  <text x="116" y="212" fill="#166534">a blend of two fades</text>
  <text x="326" y="212" fill="#b45309">swings as it fades &#8212; zeros four steps apart</text>
  <text x="536" y="212" fill="#1d4ed8">rises before it falls</text>
</g>
<text x="320" y="236" fill="#64748b" text-anchor="middle">one machine, two weights, three shapes of memory &#8212; the roots decide</text>
</svg>"##;

/// [fig 9] Two series, one step matrix: A = [[0.8, 0.1], [0.2, 0.7]].
/// Shock (10, 0) -> (8, 2) -> (6.6, 3.0) -> (5.58, 3.42) -> (4.81, 3.51).
/// Trace 1.5, det 0.54, roots 0.9 and 0.6. Bars: 12 px per bp, baseline 222.
const LEAK_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 620 272" font-family="sans-serif" font-size="11">
<rect x="0" y="0" width="620" height="272" rx="8" fill="#f8fafc"/>
<text x="96" y="26" fill="#64748b" text-anchor="middle">one day's step, A</text>
<rect x="232" y="34" width="12" height="12" fill="#dcfce7" stroke="#16a34a"/>
<text x="250" y="44" fill="#166534" font-size="10">s &#8212; the shocked spread</text>
<rect x="380" y="34" width="12" height="12" fill="#dbeafe" stroke="#2563eb"/>
<text x="398" y="44" fill="#1d4ed8" font-size="10">f &#8212; its neighbour</text>
<g fill="#64748b" text-anchor="middle"><text x="68" y="54">s</text><text x="124" y="54">f</text></g>
<g fill="#64748b" text-anchor="end"><text x="30" y="83">s</text><text x="30" y="115">f</text></g>
<rect x="40" y="62" width="56" height="32" fill="#dbeafe"/><rect x="96" y="94" width="56" height="32" fill="#dbeafe"/>
<rect x="96" y="62" width="56" height="32" fill="#fef3c7"/><rect x="40" y="94" width="56" height="32" fill="#fef3c7"/>
<g stroke="#cbd5e1" fill="none"><path d="M40 62H152M40 94H152M40 126H152M40 62V126M96 62V126M152 62V126"/></g>
<g text-anchor="middle"><text x="68" y="83" fill="#1d4ed8">0.8</text><text x="124" y="83" fill="#b45309">0.1</text>
<text x="68" y="115" fill="#b45309">0.2</text><text x="124" y="115" fill="#1d4ed8">0.7</text></g>
<text x="96" y="152" fill="#1d4ed8" font-size="10" text-anchor="middle">diagonal: own memory</text>
<text x="96" y="170" fill="#b45309" font-size="10" text-anchor="middle">off-diagonals: the leak</text>
<path d="M225 222H600" stroke="#cbd5e1"/>
<g fill="#dcfce7" stroke="#16a34a">
  <rect x="235" y="102" width="26" height="120"/><rect x="310" y="126" width="26" height="96"/>
  <rect x="385" y="142.8" width="26" height="79.2"/><rect x="460" y="155" width="26" height="67"/>
  <rect x="535" y="164.3" width="26" height="57.7"/>
</g>
<g fill="#dbeafe" stroke="#2563eb">
  <rect x="340" y="198" width="26" height="24"/><rect x="415" y="186" width="26" height="36"/>
  <rect x="490" y="181" width="26" height="41"/><rect x="565" y="179.9" width="26" height="42.1"/>
</g>
<g fill="#166534" font-size="9" text-anchor="middle"><text x="248" y="98">10</text><text x="323" y="122">8</text><text x="398" y="139">6.6</text><text x="473" y="151">5.58</text><text x="548" y="160">4.81</text></g>
<g fill="#1d4ed8" font-size="9" text-anchor="middle"><text x="278" y="218">0</text><text x="353" y="194">2</text><text x="428" y="182">3.0</text><text x="503" y="177">3.42</text><text x="578" y="176">3.51</text></g>
<g fill="#64748b" font-size="10" text-anchor="middle"><text x="263" y="238">day 0</text><text x="338" y="238">day 1</text><text x="413" y="238">day 2</text><text x="488" y="238">day 3</text><text x="563" y="238">day 4</text></g>
<text x="310" y="262" fill="#64748b" text-anchor="middle">day 1 check: s = 0.8&#215;10 + 0.1&#215;0 = 8 &#183; f = 0.2&#215;10 + 0.7&#215;0 = 2 &#8212; the shock crosses; s fades, f crests on day 4</text>
</svg>"##;
