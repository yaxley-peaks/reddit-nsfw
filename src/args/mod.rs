use clap::{Parser, clap_derive::ArgEnum};
use crate::args::SortBy as OtherSortBy;


#[derive(Debug,Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum SortBy {
    Top,
    Hot,
    Rising,
    Best,
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(short = 's')]
    pub sub: String,
    #[clap(short = 'o', arg_enum, value_parser, default_value_t = SortBy::Hot)]
    pub sort: OtherSortBy,
}