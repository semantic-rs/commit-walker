extern crate git2;
extern crate semver;
extern crate commit_analyzer;

use git2::{Repository, Commit};
use semver::Version;

use std::env;

fn range_to_head(commit: &str) -> String {
    format!("{}..HEAD", commit)
}

fn format_commit(commit: Commit) -> String {
    format!("{}\n{}", commit.id(), commit.message().unwrap_or(""))
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let path = args[0].clone();
    // let tag = args[1].clone();

    let repo = Repository::open(path).unwrap();
    let mut biggest_tag = Version::parse("0.0.0").unwrap();

    if let Some(tag) = args.get(1) {
        biggest_tag = Version::parse(tag).unwrap();
    } else {
        println!("Tags!");
        let tags = repo.tag_names(None).unwrap();
        for tag in tags.iter() {
            let tag = tag.unwrap();
            let tag = &tag[1..];
            let v = Version::parse(tag).unwrap();

            if v > biggest_tag {
                biggest_tag = v;
            }
        }
    }

    println!("Biggest tag: {:?}", biggest_tag.to_string());

    let mut walker = repo.revwalk().unwrap();
    // walker.push_head();
    //let oid = Oid::from_str("8a4fb449a5db41a539f2738e5f5ce9d3804d555a").unwrap();
    //walker.set_sorting(git2::SORT_TIME);
    let start_tag = format!("v{}", biggest_tag.to_string());
    let start_tag = range_to_head(&start_tag);
    walker.push_range(&start_tag).unwrap();

    println!("Walker!");
    let commits = walker.map(|c| repo.find_commit(c).unwrap())
        .map(|c| format_commit(c))
        .map(|c| commit_analyzer::analyze_single(&c).unwrap())
        .max();
    println!("Commits: {:?}", commits);
}
