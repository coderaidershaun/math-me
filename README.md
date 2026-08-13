# math-me

**Learn math fast.**

Run the `/lesson-builder` skill along with your prompt:

```
/lesson-builder teach me log returns
```

Go make a cup of coffee and come back to learn some math!

You'll have your own custom binary built, which you can run to start learning:

```sh
cargo run --release --bin lesson-log-returns
```

That's it.

---

Nothing built yet? `cargo run --release --bin template` runs the one that ships with the repo.

Already built: `cargo run --release --bin lesson-kalman-filter` and `cargo run --release --bin lesson-exponents`.

Lessons live in `lessons/`, each as a `.rs` binary beside a `.md` README of its own.
