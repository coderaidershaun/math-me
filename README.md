# math-me

**Learn math fast.**

Run the `/lesson-builder` skill along with your prompt:

```
/lesson-builder teach me log returns
```

Go do something fun for an hour or two and come back to learn some math!

You'll have your own custom binary built, which you can run to start learning:

```sh
cargo run --release --bin lesson-algebra-to-linear
cargo run --release --bin lesson-exponents
cargo run --release --bin lesson-kalman-filter
cargo run --release --bin lesson-limits
cargo run --release --bin lesson-probability
cargo run --release --bin lesson-trig
```

That's it.

---

Nothing built yet? `cargo run --release --bin template` runs the one that ships with the repo.

Already built: `cargo run --release --bin lesson-kalman-filter` and `cargo run --release --bin lesson-exponents`.

Lessons live in `lessons/`, each as a single `.rs` binary whose `//!` module comment carries its prerequisites and run command.
