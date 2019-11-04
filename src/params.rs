use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Params {
    /// Input field separator
    #[structopt(long = "ifc", default_value = "\t")]
    pub input_field_separator: char,

    /// Output field separator
    #[structopt(long = "ofc", default_value = ",")]
    pub output_field_separator: char,

    /// Output line break
    #[structopt(long = "olb", default_value = "\n")]
    pub output_line_break: char,

    /// Missing value substitution character
    #[structopt(long = "mvc", default_value = "")]
    pub output_missing_value: String,

    /// Print the output data dimentions
    #[structopt(short, long)]
    pub print_dims: bool,

    pub input_file: Option<PathBuf>,
    pub output_file: Option<PathBuf>,
}
