use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process;

/// A utility for managing and changing directories with shell integration.
#[derive(Parser)]
#[command(
    version = "0.1.0",
    author = "Grant Ramsay",
    about = "Set up the 'mcd' command to create and change directories in one step.",
    args_conflicts_with_subcommands = true,
    arg_required_else_help = true,
    override_usage = "mchdir [COMMAND]"
)]
struct Cli {
    /// Folder name to create and change into
    #[arg(short = 'd', value_name = "FOLDER_NAME", hide = true)]
    folder_name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints the shell integration code for reference
    Init,
    /// Installs the shell integration into your shell configuration
    Install,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Init => {
                print_shell_integration();
            }
            Commands::Install => {
                if let Err(e) = install_shell_integration() {
                    eprintln!("Error during installation: {}", e);
                    process::exit(1);
                }
            }
        }
    } else if let Some(folder_name) = cli.folder_name {
        let folder_path = Path::new(&folder_name);

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
# implements the 'mcd' function for changing directories
mcd() {{
    if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        echo "Usage: mcd <directory>"
        echo "Creates a new directory and changes into it."
        echo "If no directory is specified, changes to the home directory."
    elif [ -z "$1" ]; then
        cd
    else
        mchdir_target_dir=$("mchdir" -d "$1")

        if [ $? -eq 0 ]; then
            cd "$mchdir_target_dir"
        else
            echo "Failed to change directory."
        fi
    fi
}}
"#
    );
}

// Prints the integration function for Fish shell
fn print_fish_integration() {
    println!(
        r#"
# implements the 'mcd' function for changing directories
function mcd
    if test "$argv[1]" = "--help" -o "$argv[1]" = "-h"
        echo "Usage: mcd <directory>"
        echo "Creates a new directory and changes into it."
        echo "If no directory is specified, changes to the home directory."
    else if test (count $argv) -eq 0
        cd
    else
        set target_dir (mchdir -d $argv)

        if test $status -eq 0
            cd $target_dir
        else
            echo "Failed to change directory."
        end
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
    } else if shell.ends_with("/sh") || shell.contains("dash") {
        eprintln!("Automatic installation is not supported for your shell. Please add the shell integration code manually.");
        print_shell_integration(); // Outputs the POSIX-compliant shell script
        process::exit(1);
    } else {
        eprintln!("Unsupported shell. Only Bash, Zsh, and Fish are supported at this time.");
        process::exit(1);
    };

    // Check if the integration is already present in the config file
    if let Ok(file) = fs::File::open(&config_file) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if line_content.contains("mchdir init") {
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

    let comment = "\n# Integrate 'mcd' to create and change folder in one command\n";

    if shell.contains("fish") {
        writeln!(file, "{}eval (mchdir init)", comment)?;
    } else {
        writeln!(file, "{}eval \"$(mchdir init)\"", comment)?;
    }

    println!(
        "Successfully added 'mcd' integration to {}.",
        config_file.display()
    );
    println!("Please restart your shell to apply the changes.");
    Ok(())
}
