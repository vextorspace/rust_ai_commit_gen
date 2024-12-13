use ai_commit_message::ai::chat_gpt_ai::ChatGptAi;
use ai_commit_message::git::commit_generator::CommitGenerator;
use ai_commit_message::git::diff_provider::DiffProvider;
use ai_commit_message::git::git_diff::GitDiff;
use std::env;
use std::env::current_dir;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: PathBuf = if args.len() > 1 {
        PathBuf::from(args[1].as_str())
    } else {
        current_dir().expect("could not get current directory and no path specified")
    };


    let differ: Box<dyn DiffProvider> = Box::new(GitDiff::new());
    let mut ai = ChatGptAi::new();
    ai.load_env();

    let commit_generator = CommitGenerator::new()
        .with_differ(differ)
        .with_ai(Box::new(ai));
    let message = commit_generator
        .generate_commit_message(path.as_path());

    match message {
        Ok(msg) => {
            println!("{}", msg);
            std::process::exit(0);
        },
        Err(e) => {
            eprintln!("Error generating commit message: {}", e);
            std::process::exit(1);
        }
    }
}


