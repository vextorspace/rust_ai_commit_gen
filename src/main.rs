use std::path::Path;
use ai_commit_message::commit_generator::CommitGenerator;
use ai_commit_message::git::diff_provider::DiffProvider;
use ai_commit_message::git::git_diff::GitDiff;
use std::env;

mod git;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &Path = if args.len() > 1 {
        Path::new(args[1].as_str())
    } else {
        Path::new("")
    };

    let differ: Box<dyn DiffProvider> = Box::new(GitDiff::new());
    let commit_generator = CommitGenerator::new().with_differ(differ);

}


