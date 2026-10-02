//! Schema builder and data generator engine.

use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use indexmap::IndexMap;
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::value::{Generator, Value};

/// Streaming iterator yielding generated rows as [`IndexMap<String, Value>`].
///
/// This iterator is returned by [`Schema::iter`] and produces an infinite sequence
/// of generated rows. Use `.take(n)` to limit the number of rows generated.
///
/// # Example
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("id", Field::sequential(1, 1))
///     .add_field("value", Field::range_int(1, 100));
///
/// // Generate exactly 10 rows
/// for row in schema.iter().take(10) {
///     println!("{:?}", row);
/// }
/// ```
pub struct SchemaIter<'a> {
    fields: &'a mut [(String, Box<dyn Generator>)],
    rng: Box<dyn Rng>,
}

impl<'a> Iterator for SchemaIter<'a> {
    type Item = IndexMap<String, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut row = IndexMap::with_capacity(self.fields.len());
        for (name, generator) in self.fields.iter_mut() {
            let val = generator.next_value(&mut self.rng);
            row.insert(name.clone(), val);
        }

        Some(row)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

/// Schema definition containing fields and generation configuration.
///
/// A [`Schema`] defines the structure of generated data by specifying field names
/// and their corresponding generators. It supports batch generation, streaming,
/// and direct export to various formats (JSON, JSONL, CSV, SQL).
///
/// # Example
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .with_seed(12345)
///     .add_field("id", Field::sequential(1, 1))
///     .add_field("name", Field::pattern("USR-####").add_entry('#', vec!['0'..='9']).unwrap())
///     .add_field("price", Field::range_float(1.0, 100.0, Some(2)));
///
/// // Generate JSON
/// let json = schema.generate_json(10)?;
///
/// // Or export directly to files
/// schema.write_jsonl("output", 100)?;
/// schema.write_csv("output", 100, true)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Schema {
    fields: Vec<(String, Box<dyn Generator>)>,
    seed: Option<u64>,
}

impl Schema {
    /// Creates a new, empty [`Schema`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::schema::Schema;
    ///
    /// let schema = Schema::new();
    /// ```
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            seed: None,
        }
    }

    /// Sets a fixed random seed for reproducible data generation.
    ///
    /// When a seed is set, the schema will produce the same sequence of values
    /// across multiple runs, which is useful for testing and debugging.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .with_seed(42)
    ///     .add_field("id", Field::range_int(1, 100));
    ///
    /// // This will always produce the same sequence
    /// let rows = schema.generate_batch(10);
    /// ```
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Adds a named field with its corresponding [`Generator`].
    ///
    /// # Arguments
    ///
    /// * `name` - The field name to use in generated output.
    /// * `generator` - The generator to use for this field.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let schema = Schema::new()
    ///     .add_field("id", Field::sequential(1, 1))
    ///     .add_field("name", Field::pattern("USR-####").add_entry('#', vec!['0'..='9']).unwrap());
    /// ```
    pub fn add_field<G: Generator + 'static>(
        mut self,
        name: impl Into<String>,
        generator: G,
    ) -> Self {
        self.fields.push((name.into(), Box::new(generator)));
        self
    }

    /// Generates a batch of `count` rows in memory as a `Vec<IndexMap<String, Value>>`.
    ///
    /// This method collects all generated rows into memory. For large datasets,
    /// consider using [`Schema::iter`] or the streaming export methods instead.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of rows to generate.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("id", Field::sequential(1, 1));
    ///
    /// let rows = schema.generate_batch(100);
    /// ```
    pub fn generate_batch(&mut self, count: usize) -> Vec<IndexMap<String, Value>> {
        let mut rows = Vec::with_capacity(count);

        if let Some(seed) = self.seed {
            let mut rng = StdRng::seed_from_u64(seed);
            self.fill_rows(&mut rng, count, &mut rows)
        } else {
            let mut rng = rand::rng();
            self.fill_rows(&mut rng, count, &mut rows)
        }
        rows
    }

    fn fill_rows(
        &mut self,
        rng: &mut dyn Rng,
        count: usize,
        rows: &mut Vec<IndexMap<String, Value>>,
    ) {
        for _ in 0..count {
            let mut row = IndexMap::new();
            for (name, generator) in self.fields.iter_mut() {
                let val = generator.next_value(rng);
                row.insert(name.clone(), val);
            }
            rows.push(row);
        }
    }

    /// Returns a streaming [`SchemaIter`] that lazily produces rows without collecting them all in memory.
    ///
    /// The iterator produces an infinite sequence. Use `.take(n)` to limit the number of rows.
    /// This is memory-efficient for generating large datasets.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("id", Field::sequential(1, 1));
    ///
    /// // Generate exactly 10 rows
    /// for row in schema.iter().take(10) {
    ///     println!("{:?}", row);
    /// }
    /// ```
    pub fn iter(&mut self) -> SchemaIter<'_> {
        let rng: Box<dyn Rng> = match self.seed {
            Some(seed) => Box::new(StdRng::seed_from_u64(seed)),
            None => Box::new(rand::rng()),
        };

        SchemaIter {
            fields: &mut self.fields,
            rng,
        }
    }

    /// Generates a pretty-formatted JSON string array containing `count` records.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of records to generate.
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("id", Field::sequential(1, 1));
    ///
    /// let json = schema.generate_json(10)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn generate_json(&mut self, count: usize) -> Result<String, serde_json::Error> {
        let rows = self.generate_batch(count);
        serde_json::to_string_pretty(&rows)
    }

    /// Streams `count` records directly into a JSON Lines (`.jsonl`) file at `path`.
    ///
    /// Each record is written as a separate JSON object on its own line.
    /// This format is memory-efficient and suitable for large datasets.
    ///
    /// # Arguments
    ///
    /// * `path` - The output file path. Automatically appends `.jsonl` extension if not present.
    /// * `count` - The number of records to write.
    ///
    /// # Errors
    ///
    /// Returns an error if file creation or writing fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("id", Field::sequential(1, 1));
    ///
    /// schema.write_jsonl("output", 100)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn write_jsonl<P: AsRef<Path>>(&mut self, path: P, count: usize) -> io::Result<()> {
        let final_path = ensure_extension(path, "jsonl");
        let file = File::create(final_path)?;
        let mut writer = BufWriter::new(file);

        for row in self.iter().take(count) {
            serde_json::to_writer(&mut writer, &row)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            writer.write_all(b"\n")?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Streams `count` records directly into a CSV file at `path`.
    ///
    /// If `export_headers` is `true`, writes the field names as the first line.
    /// Automatically appends the `.csv` extension if not present.
    ///
    /// # Arguments
    ///
    /// * `path` - The output file path. Automatically appends `.csv` extension if not present.
    /// * `count` - The number of records to write.
    /// * `export_headers` - Whether to include field names as the first line.
    ///
    /// # Errors
    ///
    /// Returns an error if file creation or writing fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("id", Field::sequential(1, 1))
    ///     .add_field("name", Field::pattern("USR-####").add_entry('#', vec!['0'..='9']).unwrap());
    ///
    /// schema.write_csv("output", 100, true)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn write_csv<P: AsRef<Path>>(
        &mut self,
        path: P,
        count: usize,
        export_headers: bool,
    ) -> io::Result<()> {
        let final_path = ensure_extension(path, "csv");
        let file = File::create(final_path)?;
        let mut writer = BufWriter::new(file);

        let headers: Vec<String> = self.fields.iter().map(|(name, _)| name.clone()).collect();
        if export_headers {
            writer.write_all(headers.join(",").as_bytes())?;
            writer.write_all(b"\n")?;
        }

        for row in self.iter().take(count) {
            let mut first = true;
            for header in &headers {
                if !first {
                    writer.write_all(b",")?;
                }
                first = false;

                if let Some(val) = row.get(header) {
                    write!(writer, "{val}")?;
                }
            }
            writer.write_all(b"\n")?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Streams `count` records directly into an SQL file at `path` as `INSERT INTO` statements.
    ///
    /// Automatically appends the `.sql` extension if not present.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the table to insert into.
    /// * `path` - The output file path. Automatically appends `.sql` extension if not present.
    /// * `count` - The number of records to write.
    ///
    /// # Errors
    ///
    /// Returns an error if file creation or writing fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("id", Field::sequential(1, 1))
    ///     .add_field("name", Field::pattern("USR-####").add_entry('#', vec!['0'..='9']).unwrap());
    ///
    /// schema.write_sql("users", "output", 100)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn write_sql<P: AsRef<Path>>(
        &mut self,
        table_name: &str,
        path: P,
        count: usize,
    ) -> io::Result<()> {
        let final_path = ensure_extension(path, "sql");
        let file = File::create(final_path)?;
        let mut writer = BufWriter::new(file);

        let headers: Vec<String> = self.fields.iter().map(|(name, _)| name.clone()).collect();

        let columns = headers.join(",");

        for row in self.iter().take(count) {
            let mut values: Vec<String> = Vec::new();
            for header in &headers {
                if let Some(val) = row.get(header) {
                    values.push(val.to_sql());
                }
            }

            writeln!(
                writer,
                "INSERT INTO {table_name} ({columns}) VALUES ({});",
                values.join(",")
            )?;
        }

        writer.flush()?;
        Ok(())
    }
}

fn ensure_extension<P: AsRef<Path>>(path: P, expected_ext: &str) -> PathBuf {
    let path = path.as_ref();
    match path.extension() {
        Some(ext) if ext.eq_ignore_ascii_case(expected_ext) => path.to_path_buf(),
        _ => path.with_extension(expected_ext),
    }
}
