use std::io::BufRead;
use subprocess::{Popen, PopenConfig, Redirection};

fn main() {
    let git_url = std::env::args().nth(1).expect("no argument provided");

    let git_repo = git_url
        .trim_start_matches("https://")
        .trim_start_matches("github.com/")
        .trim_end_matches(".git");

    let mut git_paths: Vec<&str> = git_repo.split("/").collect();
    if git_paths.len() < 2 {
        println!("invalid git url");
        return;
    }
    if git_paths.len() == 2 {
        git_paths.push("main");
    }

    let username = git_paths[0];
    let mut repos: Vec<&str> = git_paths[1].split(':').collect();
    if repos.len() > 2 {
        println!("invalid repo name");
        return;
    }

    if repos.len() == 1 {
        repos.push(repos[0]);
    }

    let branch = git_paths[2];

    let config = PopenConfig {
        stdout: Redirection::Pipe,
        stderr: Redirection::Merge,
        ..Default::default()
    };
    let mut p = Popen::create(
        &[
            "git",
            "clone",
            &format!("git@github.com:{}/{}.git", username, repos[0]),
            repos[1],
            "-b",
            branch,
            "--progress",
        ],
        config,
    )
    .expect("failed to execute process");

    let mut stdout = std::io::BufReader::new(p.stdout.take().expect("failed to open stdout"));
    // let mut stderr = std::io::BufReader::new(p.stderr.take().unwrap());

    let mut line_buffer = String::new();
    loop {
        let mut stdout_output = String::new();

        match stdout.read_line(&mut line_buffer) {
            Ok(0) => break,
            Ok(_) => {
                stdout_output.push_str(&line_buffer);
                print!("{}", line_buffer);
            }
            Err(e) => panic!("failed to read stdout: {}", e),
        }
        line_buffer.clear();
    }
}
