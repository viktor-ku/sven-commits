#[macro_use]
extern crate pest_derive;

mod additive;
mod analyze;
mod analyze_header;
mod block;
mod block_factory;
mod bytes;
mod config;
mod conventional_commit;
mod domain;
mod footer_issue;
mod report;
use analyze::analyze;
mod weak_commit;

fn main() {
    analyze("hello world").expect("invalid commit mate");
}
