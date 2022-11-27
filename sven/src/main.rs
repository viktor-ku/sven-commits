use find_issues::find_issues;

#[macro_use]
extern crate pest_derive;

mod additive;
mod at;
mod block;
mod conventional_commit;
mod find_header_issues;
mod find_issues;
mod footer_issue;
mod header_issue;
mod paper;
mod pencil;
mod report;
mod subject;
mod weak_commit;

fn main() {
    find_issues("hello world").expect("invalid commit mate");
}
