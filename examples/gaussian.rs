//! To understand how define all Fields and Schema look at "all_generators.rs" example

use std::collections::BTreeMap;

use schemagen::{field::Field, schema::Schema, value::Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut schema = Schema::new().add_field("value", Field::gaussian_int(5.0, 1.0));

    let mut finded_numbers: BTreeMap<i64, u64> = BTreeMap::new();

    schema
        .iter()
        .take(100)
        .map(|mut p| match p.shift_remove("value") {
            Some(Value::Int(value)) => value,
            _ => -1,
        })
        .filter(|v| *v != -1)
        .for_each(|val| {
            if let Some(count) = finded_numbers.get(&val) {
                finded_numbers.insert(val, count + 1);
            } else {
                finded_numbers.insert(val, 1);
            }
        });

    for (num, count) in finded_numbers {
        println!("{num}-{count}");
    }
    Ok(())
}
