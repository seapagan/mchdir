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
        print_shell_integration();
        return;
    }

    // Check if the 'install' command is provided
    if args.len() > 1 && args[1] == "install" {
        if let Err(e) = install_shell_integration() {
            eprintln!("Error during installation: {}", e);
            process::exit(1);
        }
        return;
    }

    // Check for the '-d' flag and ensure a folder name is provided
    if args.len() < 3 || args[1] != "-d" {
        eprintln!("Usage: mchdir init\n       mchdir install\n       mchdir -d <folder_name>");
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

// Prints the appropriate integration code for the user's shell
fn print_shell_integration() {
    let shell = env::var("SHELL").unwrap_or_default();

    if shell.contains("fish") {
        print_fish_integration();
    } else {
        print_bash_zsh_integration();
    }
}

// Prints the integration function for Bash/Zsh
fn print_bash_zsh_integration() {
    println!(
        r#"
# Add this function to your shell configuration for integration with folder_utility
mcd() {{
    local target_dir=$("mchdir" -d "$1")

    if [[ $? -eq 0 ]]; then
        cd "$target_dir"
    else
        echo "Failed to change directory."
    fi
}}
"#
    );
}

// Prints the integration function for Fish shell
fn print_fish_integration() {
    println!(
        r#"
# Add this function to your fish configuration for integration with folder_utility
function mcd
    set target_dir (mchdir -d $argv)

    if test $status -eq 0
        cd $target_dir
    else
        echo "Failed to change directory."
    end
end
"#
    );
}

fn install_shell_integration() -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Could not find home directory");
    let shell = env::var("SHELL").unwrap_or_default();

    // Determine the correct config file based on the shell
    let config_file = if shell.contains("fish") {
        Path::new(&home_dir).join(".config/fish/config.fish")
    } else if shell.contains("zsh") {
        Path::new(&home_dir).join(".zshrc")
    } else if shell.contains("bash") {
        Path::new(&home_dir).join(".bashrc")
    } else {
        eprintln!("Unsupported shell. Only Bash, Zsh, and Fish are supported at this time.");
        process::exit(1);
    };

    // Check if the integration is already present in the config file
    if let Ok(file) = fs::File::open(&config_file) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if line_content.contains("folder_utility init") {
                    println!("Integration already exists in the config file.");
                    return Ok(());
                }
            }
        }
    }

    // Append integration if not already present
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&config_file)?;

    if shell.contains("fish") {
        writeln!(
            file,
            "\n# The following function integrates mchdir into your shell (fish)\n\
            eval (mchdir init)"
        )?;
    } else {
        writeln!(
            file,
            "\n# Integrate 'mcd' into your shell (bash/zsh)\n\
            eval \"$(mchdir init)\""
        )?;
    }

    println!(
        "Successfully added 'mcd' integration to {}.",
        config_file.display()
    );
    println!("Please restart your shell to apply the changes.");
    Ok(())
}
