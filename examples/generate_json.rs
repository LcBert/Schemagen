use schemagen::{field::Field, schema::Schema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut schema = Schema::new()
        // .with_seed(12345)
        .add_field("sequential", Field::sequential(1, 1))
        .add_field(
            "sequential_float",
            Field::sequential_float(0.5, 0.5, Some(2)),
        )
        .add_field("int", Field::range_int(1, 100))
        .add_field("float", Field::range_float(1.0, 100.0, Some(2)))
        .add_field("pattern", Field::pattern("PRD-####-??"))
        .add_field("choice", Field::choice(vec!["Prod1", "Prod2", "Prod3"]))
        .add_field("check", Field::check());

    let generated = schema.generate_json(100)?;
    print!("{generated}");

    Ok(())
}
