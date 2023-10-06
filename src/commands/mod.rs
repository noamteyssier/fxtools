pub mod extract;
pub mod filter;
pub mod fix;
pub mod io;
pub mod reverse;
pub mod sgrna_table;
pub mod sort;
pub mod t2g;
pub mod trim;
pub mod unique;
pub mod upper;

pub use io::{
    match_output_stream, write_mut_output, write_mut_output_with_invalid, write_output,
    write_output_with_invalid,
};
