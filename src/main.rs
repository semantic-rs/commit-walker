extern crate commit_walker;

use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let path = args[0].clone();

    let bump = match args.get(1) {
        Some(tag) => commit_walker::version_bump_since_tag(&path, &tag),
        None => commit_walker::version_bump_since_latest(&path)
    };

    println!("Next version: {:?}", bump);
}
