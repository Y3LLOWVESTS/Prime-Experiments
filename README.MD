# Prime-Experiments

Prime-Experiments is a Rust-based experimental mathematics lab for studying prime numbers, exact rational transforms, floor-function behavior, jump patterns, residue classes, and other structures that may or may not reveal something genuinely prime-specific.

This repo is not built on the assumption that every visible pattern is profound. It is built on the opposite assumption: most apparent patterns are artifacts until they survive exact arithmetic, larger ranges, baseline comparisons, and changes of transform.

The goal is to build a serious framework for testing ideas honestly.

---

## Core idea

A lot of number patterns look mysterious when written in decimal form, but become much clearer when converted to exact fractions.

For example:

- `1.6 = 8/5`
- `0.625 = 5/8`

So:

- dividing by `0.625` is the same as multiplying by `8/5`
- dividing by `1.6` is the same as multiplying by `5/8`

That matters because many “patterns” come from the denominator structure of a rational transform, especially when a floor or truncation is involved.

This repo is designed to test those effects exactly.

---

## Current experiment

The first experiment studies transforms of the form:

- `floor(n * a/b)`
- or equivalently `floor(n / r)` where `r` is a rational number

and then inspects the **local jump behavior** around prime inputs.

For a given transformed sequence `f(n)`, we define a prime jump pair at prime `p` as:

- `jump_in = f(p) - f(p - 1)`
- `jump_out = f(p + 1) - f(p)`

So a prime can be tagged with a local pair such as:

- `2 1`
- `1 2`
- `2 2`
- `1 0`
- `0 1`
- etc.

These are not arbitrary decorations. They encode the local step structure of the transformed sequence at prime positions.

---

## What we have already learned

So far, the repo confirms a very important equivalence:

- `divide by 0.625`
- `multiply by 1.6`

produce the same floor experiment, because both are exactly multiplication by `8/5`.

That means if the outputs match, that is not a coincidence. It is a structural identity.

For the range `1..=115`, the `floor(8n/5)` experiment over prime positions produces the pair counts:

- `1 2 -> 15`
- `2 1 -> 9`
- `2 2 -> 6`

Meanwhile, the `floor(5n/8)` experiment produces a different family of local pairs:

- `0 1`
- `1 0`
- `1 1`

This is already a strong signal that the visible pattern is being shaped heavily by the rational transform itself.

That is valuable. It means the project is already separating:

- transform-driven structure
- from prime-specific structure

---

## Why this project matters

There are two ways an experiment like this can fail, and both failures are useful.

### Failure type 1: the pattern is fake
This happens when a pattern disappears after:
- using exact fractions instead of decimals
- increasing the range
- changing the factor
- comparing against all integers instead of only primes

That kind of failure is good because it eliminates false leads.

### Failure type 2: the pattern is real, but not prime-specific
This happens when the pattern is produced mainly by:
- floor behavior
- denominator periodicity
- residue classes
- modular cycles
- truncation artifacts

That is also useful, because it tells us what *not* to mistake for a deep prime law.

The real target is a third possibility:

### Success condition
A pattern that:
- survives exact arithmetic
- survives large ranges
- survives changes in scale
- cannot be explained away by denominator periodicity alone
- differs meaningfully between primes and non-primes
- connects to known prime statistics in a nontrivial way

That is the kind of thing worth taking seriously.

---

## Could this help prove the Riemann Hypothesis?

Maybe indirectly, but not in the naive sense.

### The honest answer
A computational pattern found in a transform like `floor(8n/5)` is **not** by itself a proof of the Riemann Hypothesis.

The Riemann Hypothesis is a global analytic statement about the zeros of the Riemann zeta function. To prove it, one would need a rigorous argument that reaches far beyond a finite computation.

### But experiments can still matter
Experiments like these can still be useful if they do one or more of the following:

- reveal a previously unnoticed invariant of prime distribution
- identify a transform that isolates prime irregularity unusually well
- uncover a new exact relationship tied to residue classes, gaps, or counting functions
- produce a conjecture that can later be translated into rigorous mathematics
- help distinguish “surface patterns” from truly structural phenomena

### Most likely outcome of the current line
The present floor-jump experiments are more likely to teach us about:
- Beatty-style sequences
- rational step patterns
- modular periodicity
- how primes sample those periodic backgrounds

That is still worthwhile, but it is probably **RH-adjacent at best**, not a direct road to proof yet.

### The deeper research strategy
If this project is ever going to touch RH meaningfully, it will probably happen in stages:

1. Build tools that identify exact, reproducible prime-sensitive patterns.
2. Learn to subtract off the “background periodicity” coming from rational transforms.
3. Compare what remains against classical prime-distribution quantities.
4. Connect the strongest surviving patterns to known RH-adjacent objects such as:
   - `pi(x)`
   - `li(x)`
   - Chebyshev functions
   - Möbius and Mertens behavior
   - prime gaps
   - explicit formulas involving zeta zeros

That is the serious path.

---

## Project philosophy

This repo follows a strict research discipline.

### Rule 1: exact arithmetic first
Decimals can mislead. Use exact fractions whenever possible.

### Rule 2: larger ranges matter
Small-range patterns are often noise, or periodic artifacts that only look deep because the sample is tiny.

### Rule 3: compare against baselines
A pattern is not interesting unless it beats obvious explanations.

### Rule 4: transform effects must be separated from prime effects
If a result comes mainly from the denominator of the transform, that is important to know.

### Rule 5: no hype
Interesting is not the same as profound.
Reproducible is not the same as new.
Numerical evidence is not the same as proof.

---

## Current program

The current Rust program supports:

- exact rational parsing
- multiply and divide modes
- floor and exact modes
- prime-only output
- jump-pair summaries

Examples of accepted factors:

- `0.625`
- `1.6`
- `5/8`
- `8/5`

This lets us test the same transform in different but mathematically equivalent forms.

---

## Usage

### Basic run
```bash
cargo run -- --start 1 --end 115 --op divide --factor 0.625 --mode floor
```

### Equivalent run using multiplication

```bash
cargo run -- --start 1 --end 115 --op multiply --factor 1.6 --mode floor
```

### Multiply by 0.625 instead

```bash
cargo run -- --start 1 --end 115 --op multiply --factor 0.625 --mode floor
```

### Show only prime rows

```bash
cargo run -- --start 1 --end 115 --op divide --factor 0.625 --mode floor --primes-only
```

### Show exact rational values instead of floor values

```bash
cargo run -- --start 1 --end 30 --op divide --factor 0.625 --mode exact --primes-only
```

### Run on a larger range

```bash
cargo run --release -- --start 1 --end 100000 --op divide --factor 0.625 --mode floor --no-table
```

For larger runs, prefer:

* `--release`
* `--no-table`

because printing every row becomes the bottleneck long before the arithmetic does.

---

## Recommended larger-range workflow

Start with a ladder of increasingly large tests instead of jumping immediately to huge values.

### Suggested progression

```bash
cargo run --release -- --start 1 --end 1000 --op divide --factor 0.625 --mode floor --no-table
cargo run --release -- --start 1 --end 10000 --op divide --factor 0.625 --mode floor --no-table
cargo run --release -- --start 1 --end 100000 --op divide --factor 0.625 --mode floor --no-table
cargo run --release -- --start 1 --end 1000000 --op divide --factor 0.625 --mode floor --no-table
```

At each step, ask:

* Do the pair frequencies stabilize?
* Do new pair types appear?
* Does the relative frequency drift?
* Does the same structure appear for non-primes?
* Does changing the factor preserve or destroy the pattern?

That is how we separate a genuine structural effect from a small-sample illusion.

---

## Research roadmap

### Phase 1: catalog floor-jump phenomena

Study many rational transforms:

* `8/5`
* `5/8`
* `3/2`
* `5/3`
* `7/4`
* `11/7`
* `13/8`

Track:

* jump pairs
* pair counts
* prime-only distributions
* all-integer distributions
* residue classes

### Phase 2: residue-class analysis

For each rational `a/b`, analyze the jump pattern modulo `b` or a related natural modulus.

Goal:

* determine how much of the pattern is caused by the transform cycle itself
* determine how primes are distributed across that cycle

### Phase 3: baseline subtraction

Compare:

* all integers
* odd integers
* composites
* primes

This matters because a prime pattern is only meaningful if it differs from the background pattern in a statistically coherent way.

### Phase 4: larger statistical probes

Add:

* frequency drift across growing windows
* pair entropy
* transition matrices
* windowed counts
* normalized deviations from expected residue frequencies

### Phase 5: export and visualization

Add:

* CSV export
* histogram output
* residue summaries
* rolling windows
* charts and plotting pipeline

### Phase 6: RH-adjacent experiments

Once the transform experiments are mature, expand into more classical territory:

* `pi(x)` versus approximations
* prime gap studies
* Chebyshev-style functions
* Möbius and Mertens experiments
* transforms of prime indicator sequences
* explicit-formula-inspired numerical probes

That is where the project starts to move from “interesting arithmetic mechanics” toward “serious prime-distribution investigation.”

---

## What success would look like

A valuable result does **not** have to prove RH.

This repo would already be successful if it accomplishes any of the following:

* builds a trustworthy exact-arithmetic framework for prime experiments
* identifies which visible patterns are transform artifacts
* finds a family of patterns that survives large ranges and baseline comparisons
* produces a conjecture precise enough to state mathematically
* discovers a transform that isolates prime behavior better than naive counting alone

That would already be a real contribution to understanding.

---

## What would count as a red flag

Be skeptical if a pattern:

* only appears in a tiny range
* disappears when using exact fractions
* depends completely on one denominator
* is equally strong for all integers and primes
* vanishes when changing the factor slightly
* cannot be described cleanly in modular terms

Those are classic signs of artifact rather than deep structure.

---

## Development priorities

Immediate next steps for the codebase:

1. Add residue-class summaries.
2. Add CSV export for large runs.
3. Add an option to compare primes vs all integers directly.
4. Add pair frequencies by window.
5. Add support for batches of rational factors.
6. Add experiment notes and saved result files.

---

## Repo structure

Planned direction:

```text
Prime-Experiments/
├── Cargo.toml
├── README.MD
├── docs/
│   └── NOTES.MD
├── src/
│   └── main.rs
├── data/
└── results/
```

Over time, this will likely evolve into:

```text
src/
├── lib.rs
├── primes.rs
├── rationals.rs
├── experiments/
└── bin/
```

That will make it easier to keep the code modular as the project grows.

---

## Notes discipline

Use `docs/NOTES.MD` as a research log, not a scratchpad.

Each experiment entry should record:

* date
* factor
* operation
* mode
* range
* whether primes-only was used
* pair counts
* interpretation
* likely artifact explanation
* next test to run

That turns the repo into a genuine research notebook.

---

### Credits
This project was created by Stevan White, with research and development assistance from ChatGPT and Grok.