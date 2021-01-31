//! Goal: Explode parquet file
extern crate parquet;
//extern crate s3;

use std::{fs, path::PathBuf};
use structopt::StructOpt;

use parquet::{
    column::reader::ColumnReader,
    errors::ParquetError,
    file::reader::{FileReader, SerializedFileReader},
};

/// Explode an ODBC data source at store the result in a Parquet file.
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    in_file: PathBuf,

    #[structopt(parse(from_os_str))]
    out_file: PathBuf,
}

fn explode(cfg: Cli) -> Result<isize, ParquetError> {
    let file = fs::File::open(cfg.in_file)?;
    let reader = SerializedFileReader::new(file)?;
    let metadata = reader.metadata();
    let schema = metadata.file_metadata().schema();
    let mut o: Vec<u8> = Vec::new();
    parquet::schema::printer::print_schema(&mut o, schema);

    println!("Schema is {}", std::str::from_utf8(o.as_slice())?);

    for i in 0..metadata.num_row_groups() {
        for j in 0..metadata.row_group(i).num_columns() {
            let mut column_reader = reader.get_row_group(i)?.get_column_reader(j)?;
            match column_reader {
                // You can also use `get_typed_column_reader` method to extract typed reader.
                ColumnReader::Int32ColumnReader(ref mut typed_reader) => {
                    let mut values = vec![0; 8];
                    let mut def_levels = vec![0; 8];
                    let mut rep_levels = vec![0; 8];
                    let (values_read, levels_read) = typed_reader.read_batch(
                        8, // batch size
                        Some(&mut def_levels),
                        Some(&mut rep_levels),
                        &mut values,
                    )?;

                    println!("{}\tValues Read {}", i, values_read);
                    println!("{}\tLevels Read {}", i, levels_read);
                    println!("Def {:?}", def_levels);
                    println!("Rep {:?}", def_levels);
                }
                ColumnReader::DoubleColumnReader(ref mut typed_reader) => {
                    let mut def_levels = vec![0; 16];
                    let mut rep_levels = vec![0; 16];
                    let mut values = vec![0.0; 8];

                    let (values_read, levels_read) = typed_reader.read_batch(
                        8, // batch size
                        Some(&mut def_levels),
                        Some(&mut rep_levels),
                        &mut values,
                    )?;
                    println!("{}\tValues Read {}", i, values_read);
                    println!("{}\tLevels Read {}", i, levels_read);

                    println!("Def {:?}", def_levels);
                    println!("Rep {:?}", def_levels);
                    println!("Values {:?}", values);
                }
                _ => {}
            }
        }
    }
    Ok(1)
}

pub fn main() {
    let opt = Cli::from_args();

    println!("In {:?}", opt.in_file);
    println!("Out {:?}", opt.out_file);

    explode(opt).unwrap();

    println!("Hello, world!");
}
