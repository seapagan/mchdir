use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process;

/// A utility for managing and changing directories with shell integration.
#[derive(Parser)]
#[command(
    name = "mchdir",
    version = "0.1.0",
    author = "Grant Ramsay",
    about = "Create a new folder and change into it in one command.",
    args_conflicts_with_subcommands = true,
    arg_required_else_help = true
)]
struct Cli {
    /// Folder name to create and change into
    #[arg(short = 'd', long, value_name = "FOLDER_NAME", hide = true)]
    folder_name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints the shell integration code
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
        // } else {
        //     // No arguments provided
        //     eprintln!("Usage: mchdir init\n       mchdir install\n       mchdir -d <folder_name>");
        //     process::exit(1);
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
# Add this function to your shell configuration for integration with mchdir
mcd() {{
    if [ -z "$1" ]; then
        cd
    else
        local target_dir=$("mchdir" -d "$1")

        if [[ $? -eq 0 ]]; then
            cd "$target_dir"
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
# Add this function to your fish configuration for integration with mchdir
function mcd
    if test (count $argv) -eq 0
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
