# schemagen

A fast and flexible fake/mock data generator library for Rust with JSON, JSONL, CSV, and SQL export support.

## Features

- **Schema Builder**: Easily define schemas with chainable `.add_field()` methods.
- **Multiple Data Types**:
  - `range_int`: Uniform random integers within `[min, max]`.
  - `range_float`: Uniform random floats within `[min, max]`, with optional decimal rounding.
  - `pattern`: Template-based string generation with configurable placeholder characters.
  - `choice`: Random pick from a predefined collection of options.
  - `check`: Random boolean flag with 50/50 distribution.
  - `sequential` / `sequential_float`: Deterministic counters with custom step increments.
  - `uuid` / `uuid_v4` / `uuid_v7`: UUID generation (random v4 or time-ordered v7).
  - `datetime` / `date` / `time`: Random date, time, or datetime with custom formats.
  - `optional_value`: Wrap any generator to produce NULL values with configurable probability.
  - `array`: Generate arrays of values.
  - `object`: Generate nested objects.
  - `color_rgb`: Generate random RGB colors.
  - `color_hex`: Generate random hex colors.
- **Streaming & Exporting**:
  - In-memory batch generation (`generate_batch`, `generate_json`).
  - Streaming iterator (`iter`).
  - Direct file export (`write_jsonl`, `write_csv`, `write_sql`).
- **Reproducibility**: Set seeds via `.with_seed(seed)` for deterministic output.

## Quick Start

```rust
use schemagen::{field::Field, schema::Schema};

let mut schema = Schema::new()
    .with_seed(12345)
    .add_field("id", Field::sequential(1, 1))
    .add_field("name", Field::pattern("USR-####").add_entry('#', vec!['0'..='9']))
    .add_field("price", Field::range_float(1.0, 100.0, Some(2)))
    .add_field("is_active", Field::check())
    .add_field("uuid", Field::uuid_v4())
    .add_field("created_at", Field::date("2024-01-01", "2024-12-31"))
    .add_field("optional_field", Field::optional_value(Field::range_int(1, 100), 0.8));

// Generate JSON
let json = schema.generate_json(10)?;

// Or export directly to files
schema.write_jsonl("output", 100)?;
schema.write_csv("output", 100, true)?;
schema.write_sql("users", "output", 100)?;
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
