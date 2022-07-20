use anyhow::Result;
use spinners::{Spinner, Spinners};
use std::{collections::HashMap, fs::File, io::Write};
use fxread::{FastxRead, Record};

struct Table {
    map: HashMap<String, String>
}
impl Table {
    pub fn from_reader(reader: Box<dyn FastxRead<Item = Record>>) -> Self  
    {
        let map = Self::build(reader);
        Self { map }
    }

    pub fn num_records(&self) -> usize 
    {
        self.map.len()
    }

    fn build(reader: Box<dyn FastxRead<Item = Record>>) -> HashMap<String, String>
    {
        reader
            .map(|record| (record.id().to_string(), Self::parse_header(&record)))
            .collect()
    }

    fn parse_header(record: &Record) -> String 
    {
        record.id().split('_').next().unwrap().to_string()
    }

    fn iter(&self) -> impl Iterator<Item = (&String, &String)>{
        self.map.iter()
    }
}

fn write_to_stdout(table: Table, delim: &str)
{
    table
        .iter()
        .for_each(|(k, v)| println!("{}{}{}", k, delim, v));
}

fn write_to_file(table: Table, path: &str, delim: &str) -> Result<()>
{
    let mut file = File::create(path)?;
    table
        .iter()
        .for_each(
            |(k, v)| 
            writeln!(file, "{}{}{}", k, delim, v)
                .expect("Writing Error")
        );
    Ok(())
}

pub fn run(
    input: String,
    output: Option<String>,
    delim: Option<char>) -> Result<()> 
{
    let delim = match delim {
        Some(c) => c.to_string(),
        None => "\t".to_string()
    };
    let reader = fxread::initialize_reader(&input)?;

    let mut spinner = Spinner::new(Spinners::Dots12, "Determining Unique Records".to_string());
    let table = Table::from_reader(reader);
    spinner.stop_and_persist("✔", format!("Mapped {} Records", table.num_records()));
    match output {
        Some(f) => write_to_file(table, &f, &delim)?,
        None => write_to_stdout(table, &delim)
    };

    Ok(())
}
