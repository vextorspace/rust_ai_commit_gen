mod git;

fn main() {
    use std::env;

    fn main() {
        let args: Vec<String> = env::args().collect();
        let path = if args.len() > 1 {
            &args[1];
        } else {
            "";
        };


        use std::process::Command;

        let output = Command::new("git")
            .arg("diff")
            .output()
            .expect("Failed to execute git diff");

        if output.status.success() {
            let diff = String::from_utf8_lossy(&output.stdout);
            println!("Git diff output:\n{}", diff);
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            println!("Git diff failed with error:\n{}", error_message);
        }
    }
}

