use std::{fs, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if PathBuf::from("public/styles/main.css").exists() {
        fs::remove_file("public/styles/main.css")?;
    }

    // generate TailwindCSS classes
    Command::new("npx")
        .arg("tailwindcss")
        .arg("-i")
        .arg("tailwind.css")
        .arg("-o")
        .arg("public/styles/main.css")
        .output()?;

    Ok(())
}
