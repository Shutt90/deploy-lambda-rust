use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "zip main.zip main"])
            .output()
            .expect("failed to zip command")
    } else {
        Command::new("sh")
            .arg("zip main.zip main")
            .output()
            .expect("failed to zip process")
    };

    let _hello = output.stdout;
}