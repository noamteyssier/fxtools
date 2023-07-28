use std::{
    borrow::Borrow,
    io::{stdin, Write},
    str::from_utf8,
};

use anyhow::Result;
use fxread::{initialize_reader, initialize_stdin_reader, Record};
use memchr::memmem;
use spinoff::{Color, Spinner, Spinners, Streams};

use super::match_output_stream;

struct Trimmer {
    adapter: String,
    trim_adapter: bool,
    num_records: usize,
    num_trimmed: usize,
}
impl Trimmer {
    pub fn new(adapter: String, trim_adapter: bool) -> Self {
        Self {
            adapter,
            trim_adapter,
            num_records: 0,
            num_trimmed: 0,
        }
    }

    pub fn trim<'a>(&mut self, record: &'a Record) -> Option<String> {
        self.num_records += 1;
        if let Some(idx) = memmem::find(record.seq(), self.adapter.as_bytes()) {
            self.num_trimmed += 1;
            Some(self.prepare_record(record, idx))
        } else {
            None
        }
    }

    fn prepare_record(&self, record: &Record, index: usize) -> String {
        if let Some(_) = record.qual() {
            format!(
                "@{}\n{}\n{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(self.trim_sequence(record, index)).expect("invalid utf8"),
                from_utf8(record.plus().unwrap()).expect("invalid utf8"),
                from_utf8(self.trim_qual(record, index)).expect("invalid utf8"),
            )
        } else {
            format!(
                ">{}\n{}\n",
                from_utf8(record.id()).expect("invalid utf8"),
                from_utf8(self.trim_sequence(record, index)).expect("invalid utf8"),
            )
        }
    }

    fn trim_sequence<'a>(&self, record: &'a Record, index: usize) -> &'a [u8] {
        if self.trim_adapter {
            &record.seq()[index + self.adapter.len()..]
        } else {
            &record.seq()[index..]
        }
    }

    fn trim_qual<'a>(&self, record: &'a Record, index: usize) -> &'a [u8] {
        if self.trim_adapter {
            &record
                .qual()
                .expect("Missing Quality - called from trim_qual")[index + self.adapter.len()..]
        } else {
            &record
                .qual()
                .expect("Missing Quality - called from trim_qual")[index..]
        }
    }
}

pub fn write_conditional_output_string<W, I, R>(
    writer: &mut W,
    reader: I,
    f: &mut dyn FnMut(&Record) -> Option<String>,
) where
    W: Write,
    I: Iterator<Item = R>,
    R: Borrow<Record>,
{
    reader.for_each(|x| {
        assert!(
            x.borrow().valid(),
            "Invalid Nucleotides in record: {}",
            from_utf8(x.borrow().id()).expect("invalid utf8")
        );
        if let Some(s) = f(x.borrow()) {
            write!(writer, "{}", s).expect("Error Writing to File");
        }
    });
}

pub fn run(
    input: Option<String>,
    adapter: &str,
    output: Option<String>,
    trim_adapter: bool,
    num_threads: Option<usize>,
    compression_level: Option<usize>,
) -> Result<()> {
    let reader = if let Some(path) = input {
        initialize_reader(&path)
    } else {
        initialize_stdin_reader(stdin().lock())
    }?;
    let mut trimmer = Trimmer::new(adapter.to_string(), trim_adapter);
    let mut writer = match_output_stream(output, num_threads, compression_level)?;

    let spinner = Spinner::new_with_stream(
        Spinners::Dots12,
        format!("Trimming records with adapter: {}", adapter),
        Color::Green,
        Streams::Stderr,
    );

    write_conditional_output_string(&mut writer, reader, &mut |x| trimmer.trim(x));

    spinner.stop_with_message(&format!(
        "Trimmed {} out of {} records ( {:.2}% )",
        trimmer.num_trimmed,
        trimmer.num_records,
        100.0 * trimmer.num_trimmed as f64 / trimmer.num_records as f64
    ));

    Ok(())
}
