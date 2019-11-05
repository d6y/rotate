mod params;

extern crate structopt;
use params::Params;
use structopt::StructOpt;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() -> io::Result<()> {
    let params = Params::from_args();

    let input: Box<dyn Read> = match params.input_file {
        Some(ref file) => Box::new(File::open(file)?),
        None => Box::new(io::stdin()),
    };

    let output: Box<dyn Write> = match params.output_file {
        Some(ref file) => Box::new(File::create(file)?),
        None => Box::new(io::stdout()),
    };

    let dims = rotate(
        input,
        output,
        params.input_field_separator,
        params.output_field_separator,
        params.output_line_break,
        params.output_missing_value,
    )?;

    if params.print_dims {
        println!("rows {} x {} cols", dims.rows, dims.cols);
    }

    Ok(())
}

struct Dims {
    rows: usize,
    cols: usize,
}

fn rotate(
    raw_input: Box<dyn Read>,
    raw_output: Box<dyn Write>,
    input_separator: char,
    output_separator: char,
    output_line_break: char,
    output_missing_val: String,
) -> io::Result<Dims> {

    let columns = read_columns(raw_input, input_separator)?;

    // Output dimentions will be:
    let num_cols = columns.len();
    let num_rows = columns[0].len();

    // Byte slices we need to write:
    let sep_string = output_separator.to_string();
    let separator = sep_string.as_bytes();

    let break_string = output_line_break.to_string();
    let break_bytes = break_string.as_bytes();

    let missing_bytes = output_missing_val.as_bytes();

    // Write
    let estimated_cell_size = 20; // TODO: Sample an average from cells?
    let estimated_row_length = columns.len() * estimated_cell_size;
    let mut output = BufWriter::with_capacity(estimated_row_length, raw_output);

    for row_num in 0..num_rows {
        let mut write_separator = false;
        for (col_num, col) in columns.iter().enumerate() {
            if !write_separator {
                write_separator = true;
            } else {
                output.write_all(separator)?;
            }

            if let Some(cell) = col.get(row_num) {
                let output_cell = if cell.is_empty() {
                    missing_bytes
                } else {
                    cell.as_bytes()
                };
                output.write_all(output_cell)?;
            } else {
                panic!(
                    "expected value at input row {} column {}",
                    1 + col_num,
                    1 + row_num
                );
            }
        }
        output.write_all(break_bytes)?;
        output.flush()?;
    }

    Ok(Dims {
        rows: num_rows,
        cols: num_cols,
    })
}

fn read_columns(raw_input: Box<dyn Read>, input_separator: char) -> io::Result<Vec<Vec<String>>> {

    let input = BufReader::new(raw_input);

    let mut columns = Vec::new();

    for maybe_line in input.lines() {
        // A line of input becomes a column of output
        let line = maybe_line?;
        if !line.is_empty() {
            let column: Vec<String> = line.split(input_separator).map(|s| s.to_owned()).collect();
            columns.push(column);
        }
    }

    Ok(columns)
}