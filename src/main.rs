use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the 'init' command is provided
    if args.len() > 1 && args[1] == "init" {
        print_zsh_integration();
        return;
    }

    // Check if the 'install' command is provided
    if args.len() > 1 && args[1] == "install" {
        if let Err(e) = install_zsh_integration() {
            eprintln!("Error during installation: {}", e);
            process::exit(1);
        }
        return;
    }

    // Check for the '-d' flag and ensure a folder name is provided
    if args.len() < 3 || args[1] != "-d" {
        eprintln!("Usage: mchdir -d <folder_name>");
        process::exit(1);
    }

    let folder_name = &args[2];
    let folder_path = Path::new(folder_name);

    // Check if the folder exists; if not, create it
    if !folder_path.exists() {
        if let Err(e) = fs::create_dir(folder_path) {
            eprintln!("Error creating folder '{}': {}", folder_name, e);
            process::exit(1);
        }
    }

    // Get the absolute path and output it
    match fs::canonicalize(folder_path) {
        Ok(absolute_path) => println!("{}", absolute_path.display()),
        Err(e) => {
            eprintln!("Error obtaining absolute path for '{}': {}", folder_name, e);
            process::exit(1);
        }
    }
}

fn print_zsh_integration() {
    println!(
        r#"
# Add this function to your .zshrc for integration with folder_utility
mcd() {{
    local target_dir=$("mchdir" -d "$1")

    if [[ $? -eq 0 ]]; then
        cd "$target_dir"
    else
        echo "Failed to create or change directory."
    fi
}}
"#
    );
}

fn install_zsh_integration() -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Could not find home directory");
    let zshrc_path = Path::new(&home_dir).join(".zshrc");

    // Check if the integration is already present in .zshrc
    if let Ok(file) = fs::File::open(&zshrc_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if line_content.contains("eval \"$(mchdir init)\"") {
                    println!("Integration already exists in .zshrc.");
                    return Ok(());
                }
            }
        }
    }

    // Append integration if not already present
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&zshrc_path)?;

    writeln!(
        file,
        "\n# The following line integrates the folder_utility into your shell\n\
        eval \"$(mchdir init)\""
    )?;

    println!("Successfully added folder_utility integration to .zshrc.");
    println!("Please restart your shell to apply the changes.");
    Ok(())
}
