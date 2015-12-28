extern crate git2;
extern crate semver;
extern crate commit_analyzer;

use git2::{Repository, Commit};
use semver::Version;
use commit_analyzer::CommitType;

fn range_to_head(commit: &str) -> String {
    format!("{}..HEAD", commit)
}

fn format_commit(commit: Commit) -> String {
    format!("{}\n{}", commit.id(), commit.message().unwrap_or(""))
}

pub fn latest_tag(path: &str) -> Version {
    let repo = Repository::open(path).unwrap();
    let mut biggest_tag = Version::parse("0.0.0").unwrap();

    let tags = repo.tag_names(None).unwrap();
    for tag in tags.iter() {
        let tag = tag.unwrap();
        let tag = &tag[1..];
        let v = Version::parse(tag).unwrap();

        if v > biggest_tag {
            biggest_tag = v;
        }
    }

    biggest_tag
}

pub fn version_bump_since_latest(path: &str) -> CommitType {
    let tag = latest_tag(path);

    let tag = format!("v{}", tag.to_string());
    version_bump_since_tag(path, &tag)
}

pub fn version_bump_since_tag(path: &str, tag: &str) -> CommitType {
    let tag = range_to_head(tag);

    let repo = Repository::open(path).unwrap();

    let mut walker = repo.revwalk().unwrap();
    walker.push_range(&tag).unwrap();

    let tag = walker.map(|c| repo.find_commit(c).unwrap())
        .map(|c| format_commit(c))
        .map(|c| commit_analyzer::analyze_single(&c).unwrap())
        .max().unwrap();

    tag
}
