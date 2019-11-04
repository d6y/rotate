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
    output_missing_val: char,
) -> io::Result<Dims> {
    let input = BufReader::new(raw_input);
    let mut output = BufWriter::new(raw_output);

    let mut columns = Vec::new();

    for maybe_line in input.lines() {
        // A line of input becomes a column of output
        let column: Vec<String> = maybe_line?
            .split(input_separator)
            .map(|s| s.to_owned())
            .collect();
        columns.push(column);
    }

    // Output dimentions will be:
    let num_cols = columns.len();
    let num_rows = columns[0].len();

    let sep_string = output_separator.to_string();
    let separator = sep_string.as_bytes();

    let break_string = output_line_break.to_string();
    let break_bytes = break_string.as_bytes();

    let missing_string = output_missing_val.to_string();
    let missing_bytes = missing_string.as_bytes();

    for row_num in 0..num_rows {
        let mut write_separator = false;
        for col in columns.iter() {
            if !write_separator {
                write_separator = true;
            } else {
                output.write_all(separator)?;
            }

            let cell = col[row_num].as_bytes(); // Will panic if there are not enough rows
            let output_cell = if cell.is_empty() { missing_bytes } else { cell };
            output.write_all(output_cell)?;
        }
        output.write_all(break_bytes)?;
    }

    Ok(Dims {
        rows: num_rows,
        cols: num_cols,
    })
}
