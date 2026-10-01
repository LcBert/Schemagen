set quiet

example:
    cargo run --example generate_json
    cargo run --example write_jsonl
    cargo run --example write_csv
    cargo run --example write_sql

package:
    cargo package

publish-check:
    cargo publish --dry-run