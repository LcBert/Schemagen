use schemagen::{field::Field, schema::Schema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define all Fields for the generator
    let sequential = Field::sequential(1, 1);

    let sequential_float = Field::sequential_float(0.5, 0.5, Some(2));

    let range_int = Field::range_int(1, 100);

    let range_float = Field::range_float(1.0, 100.0, Some(2));

    let pattern = Field::pattern("PRD-####-????")
        .add_entry('#', vec!['1'..='9'])?
        .add_entry('?', vec!['a'..='z', 'A'..='Z'])?;

    let choice = Field::choice(vec!["Prod1", "Prod2", "Prod3", "Prod4", "Prod5"]);

    let check = Field::check();

    let uuid_v4 = Field::uuid_v4();

    // Fill Schema with desired Fields
    let mut schema = Schema::new()
        .with_seed(12345)
        .add_field("sequential", sequential)
        .add_field("sequential_float", sequential_float)
        .add_field("int", range_int)
        .add_field("float", range_float)
        .add_field("pattern", pattern)
        .add_field("choice", choice)
        .add_field("check", check)
        .add_field("uuid", uuid_v4);

    // Generate json
    let generated = schema.generate_json(100)?;
    print!("{generated}");

    Ok(())
}
