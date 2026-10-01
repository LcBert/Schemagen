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
pub struct SchemaIter<'a> {
    fields: &'a mut [(String, Box<dyn Generator>)],
    rng: Box<dyn Rng>,
    remaining: usize,
}

impl<'a> Iterator for SchemaIter<'a> {
    type Item = IndexMap<String, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        self.remaining -= 1;
        let mut row = IndexMap::new();
        for (name, generator) in self.fields.iter_mut() {
            let val = generator.next_value(&mut self.rng);
            row.insert(name.clone(), val);
        }

        Some(row)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for SchemaIter<'_> {}

/// Schema definition containing fields and generation configuration.
pub struct Schema {
    fields: Vec<(String, Box<dyn Generator>)>,
    seed: Option<u64>,
}

impl Schema {
    /// Creates a new, empty [`Schema`].
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            seed: None,
        }
    }

    /// Sets a fixed random seed for reproducible data generation.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Adds a named field with its corresponding [`Generator`].
    pub fn add_field<G: Generator + 'static>(
        mut self,
        name: impl Into<String>,
        generator: G,
    ) -> Self {
        self.fields.push((name.into(), Box::new(generator)));
        self
    }

    /// Generates a batch of `count` rows in memory as a `Vec<IndexMap<String, Value>>`.
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

    /// Returns a streaming [`SchemaIter`] that lazily produces `count` rows without collecting them all in memory.
    pub fn iter(&mut self, count: usize) -> SchemaIter<'_> {
        let rng: Box<dyn Rng> = match self.seed {
            Some(seed) => Box::new(StdRng::seed_from_u64(seed)),
            None => Box::new(rand::rng()),
        };

        SchemaIter {
            fields: &mut self.fields,
            rng,
            remaining: count,
        }
    }

    /// Generates a pretty-formatted JSON string array containing `count` records.
    pub fn generate_json(&mut self, count: usize) -> Result<String, serde_json::Error> {
        let rows = self.generate_batch(count);
        serde_json::to_string_pretty(&rows)
    }

    /// Streams `count` records directly into a JSON Lines (`.jsonl`) file at `path`.
    ///
    /// Automatically appends the `.jsonl` extension if not present.
    pub fn write_jsonl<P: AsRef<Path>>(&mut self, path: P, count: usize) -> io::Result<()> {
        let final_path = ensure_extension(path, "jsonl");
        let file = File::create(final_path)?;
        let mut writer = BufWriter::new(file);

        for row in self.iter(count) {
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

        for row in self.iter(count) {
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

        for row in self.iter(count) {
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
