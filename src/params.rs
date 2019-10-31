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

    /// Missing value substitution character
    #[structopt(long = "mvc", default_value = "?")]
    pub output_missing_value: char,

    pub input_file: Option<PathBuf>,
    pub output_file: Option<PathBuf>,
}
