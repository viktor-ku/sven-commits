#[macro_use]
extern crate pest_derive;

mod additive;
mod analyze;
mod analyze_header;
mod at;
mod block;
mod conventional_commit;
mod domain;
mod footer_issue;
mod paper;
mod pencil;
mod report;
mod weak_commit;

use analyze::analyze;

fn main() {
    analyze("hello world").expect("invalid commit mate");
}
