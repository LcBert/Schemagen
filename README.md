# schemagen

A fast and flexible fake/mock data generator library for Rust with JSON, JSONL, and CSV export support.

## Features

- **Schema Builder**: Easily define schemas with chainable `.add_field()` methods.
- **Multiple Data Types**:
  - `range_int`: Uniform random integers within `[min, max]`.
  - `range_float`: Uniform random floats within `[min, max]`, with optional decimal rounding.
  - `pattern`: Template-based string generation (`#` for digits, `?` for uppercase letters).
  - `choice`: Random pick from a predefined collection of options.
  - `check`: Random boolean flag.
  - `sequential` / `sequential_float`: Deterministic counters with custom step increments.
- **Streaming & Exporting**:
  - In-memory batch generation (`generate_batch`, `generate_json`).
  - Streaming iterator (`iter`).
  - Direct file export (`write_jsonl`, `write_csv`).
- **Reproducibility**: Set seeds via `.with_seed(seed)` for deterministic output.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
