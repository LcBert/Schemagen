set quiet

example:
    cargo run --example all_generators

package:
    cargo package

publish-check:
    cargo publish --dry-run