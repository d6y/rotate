# Rotate a delimited text file

Reads in _r x c_ rows and columns,
and outputs _c x r_ rows and columns.

Input is assumed to be tab separated.
Output is comma separated.

```
$ cargo build --release

$ ./target/release/rotate --help

USAGE:
    rotate [FLAGS] [OPTIONS] [ARGS]

FLAGS:
    -h, --help          Prints help information
    -p, --print_dims    Print the output data dimentions
    -V, --version       Prints version information

OPTIONS:
        --ifc <input_field_separator>     Input field separator [default: 	]
        --ofc <output_field_separator>    Output field separator [default: ,]
        --olb <output_line_break>         Output line break [default:
                                          ]
        --mvc <output_missing_value>      Missing value substitution character [default: ]

ARGS:
    <input_file>
    <output_file>

$ cat example.input
Sample	SubjectA	SubjectB	SubjectC	SubjectD
Probe1	1	2	3	4
Probe2	5	6	7	8
Probe3	9			12

$ ./target/release/rotate --mvc ? example.input
Sample,Probe1,Probe2,Probe3
SubjectA,1,5,9
SubjectB,2,6,?
SubjectC,3,7,?
SubjectD,4,8,12
```
