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

pub fn latest_tag(path: &str) -> Option<Version> {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(_) => return None
    };

    let tags = match repo.tag_names(None) {
        Ok(tags) => tags,
        Err(_) => return None
    };

    tags.iter()
        .map(|tag| tag.unwrap())
        .filter_map(|tag| Version::parse(&tag[1..]).ok())
        .max()
}

pub fn version_bump_since_latest(path: &str) -> CommitType {
    match latest_tag(path) {
        Some(t) => {
            let tag = format!("v{}", t.to_string());
            version_bump_since_tag(path, &tag)
        },
        None => CommitType::Major
    }
}

pub fn version_bump_since_tag(path: &str, tag: &str) -> CommitType {
    let tag = range_to_head(tag);

    let repo = Repository::open(path).expect("Open repository failed");

    let mut walker = repo.revwalk().expect("Creating a revwalk failed");
    walker.push_range(&tag).expect("Adding a range failed");

    let tag = walker.map(|c| repo.find_commit(c).expect("No commit found"))
        .map(|c| format_commit(c))
        .map(|c| commit_analyzer::analyze_single(&c).expect("Analyzing commit failed"))
        .max().unwrap_or(CommitType::Unknown);

    tag
}
