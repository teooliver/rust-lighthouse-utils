use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub url: String,
    pub dir: String,
    pub runs: i32,
    pub out_file_name: String,
}

pub fn run_lighthouse_tests(config: Config) {
    let mut runs = 0;

    let pb = indicatif::ProgressBar::new(config.runs as u64);
    pb.inc(0);

    while runs < config.runs {
        let mut control_output = Command::new("lighthouse");
        control_output.arg(format!("{}", config.url));
        control_output.arg("--quiet");
        control_output.arg("--chrome-flags=\"--headless\"");
        control_output.arg("--only-categories=\"performance\"");
        control_output.arg("--output=\"json\"");
        control_output.arg("--output=\"html\"");
        control_output.arg(format!(
            "--output-path=./{}/{}-control-v{}",
            config.dir.as_str(),
            config.out_file_name.as_str(),
            runs
        ));
        control_output.status().expect("failed to execute process");

        pb.inc(1);
        pb.println(format!("[+] finished test #{}", runs + 1));
        runs += 1;
    }
    pb.finish_with_message("done");
}
