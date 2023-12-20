use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
