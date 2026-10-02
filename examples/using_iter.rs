//! To understand how define all Fields and Schema look at "all_generators.rs" example

use schemagen::{field::Field, schema::Schema, value::Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let range_float_field = Field::range_float(1.0, 100.0, Some(2));
    let mut schema = Schema::new().add_field("price", range_float_field);

    // Use iter
    let sum = schema
        .iter() // Generate infinite records
        .take(50) // Stop after 50 generations
        .filter_map(|p| match p.get("price") {
            Some(Value::Float(price)) => Some(*price),
            _ => None,
        })
        .sum::<f64>();

    println!("{sum:.2}");

    let pattern_field = Field::pattern("###").add_entry('#', vec!['0'..='9']);
    let mut schema = Schema::new().add_field("id", pattern_field);

    let iterator = schema
        .iter()
        .filter_map(|mut p| match p.shift_remove("id") {
            Some(Value::Text(id)) => Some(id.clone()),
            _ => None,
        })
        .take_while(|id| id != "123");

    for r in iterator {
        println!("{r}");
    }
    Ok(())
}
