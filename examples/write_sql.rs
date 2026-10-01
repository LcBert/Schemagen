use schemagen::{field::Field, r#gen::uuid_gen::UuidVersion, schema::Schema};

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

    let uuid_v4 = Field::uuid(UuidVersion::V4);

    let uuid_v7 = Field::uuid(UuidVersion::V7);

    let datetime = Field::datetime(
        "2024-01-01 00:00:00",
        "2024-12-31 23:59:59",
        "%Y-%m-%d %H:%M:%S",
    );

    let date = Field::date("2024-01-01", "2024-12-31");

    let time = Field::time("00:00:00", "23:59:59");

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
        .add_field("uuid_v4", uuid_v4)
        .add_field("uuid_v7", uuid_v7)
        .add_field("datetime", datetime)
        .add_field("date", date)
        .add_field("time", time);

    // Generate csv file
    schema.write_sql("table", "products", 100)?;
    print!("File products.sql generated");

    Ok(())
}
