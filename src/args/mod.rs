use clap::{clap_derive::ArgEnum, Parser};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum SortBy {
    Top,
    Hot,
    Rising,
    New,
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(short = 's')]
    pub sub: String,
    #[clap(short = 'o', arg_enum, value_parser, default_value_t = SortBy::Hot)]
    pub sort: crate::args::SortBy,
    #[clap(short = 'n', default_value_t = 25)]
    pub lim: u32,
}
