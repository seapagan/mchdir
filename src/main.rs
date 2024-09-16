use clap::{Parser, Subcommand};
use rand::{distributions::Alphanumeric, Rng};
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

    /// Create a folder in the temporary directory
    #[arg(short = 't', hide = true)]
    temp_folder: bool,

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

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("This project is only compatible with Linux or macOS.");

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
    } else {
        let folder_path = if cli.temp_folder {
            // Handle '-t' flag: create a folder in the system temporary directory
            let mut temp_path = std::env::temp_dir();
            if let Some(folder_name) = cli.folder_name {
                temp_path.push(folder_name);
            } else {
                let random_name: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect();
                temp_path.push(format!("mct-{}", random_name));
            }
            temp_path
        } else if let Some(folder_name) = cli.folder_name {
            // Handle '-d' flag: create a folder in the specified path
            Path::new(&folder_name).to_path_buf()
        } else {
            eprintln!("Error: No folder name provided.");
            process::exit(1);
        };

        // Create the folder if it doesn't exist
        if !folder_path.exists() {
            if let Err(e) = fs::create_dir(&folder_path) {
                eprintln!("Error creating folder '{}': {}", folder_path.display(), e);
                process::exit(1);
            }
        }

        // Get the absolute path and output it
        match fs::canonicalize(&folder_path) {
            Ok(absolute_path) => println!("{}", absolute_path.display()),
            Err(e) => {
                eprintln!(
                    "Error obtaining absolute path for '{}': {}",
                    folder_path.display(),
                    e
                );
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
# mcd() function - create and change directories
mcd() {{
    if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        echo "Usage: mcd <directory>"
        echo "  Creates a new directory and changes into it."
        echo "  If no directory is specified, changes to the home directory."
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

# mct() function - create a directory in the system temp folder
mct() {{
    if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        echo "Usage: mct <directory>"
        echo "  Creates a new directory in the system temporary directory and changes into it."
        echo "  If no directory is specified, creates a random directory in the temp folder."
    elif [ -z "$1" ]; then
        mchdir_target_dir=$("mchdir" -t)

        if [ $? -eq 0 ]; then
            cd "$mchdir_target_dir"
        else
            echo "Failed to change directory."
        fi
    else
        mchdir_target_dir=$("mchdir" -t -d "$1")

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
# mcd() function - create and change directories
function mcd
    if test "$argv[1]" = "--help" -o "$argv[1]" = "-h"
        echo "Usage: mcd <directory>"
        echo "  Creates a new directory and changes into it."
        echo "  If no directory is specified, changes to the home directory."
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

# mct() function - create a directory in the system temp folder
function mct
    if test "$argv[1]" = "--help" -o "$argv[1]" = "-h"
        echo "Usage: mct <directory>"
        echo "  Creates a new directory in the system temporary directory and changes into it."
        echo "  If no directory is specified, creates a random directory in the temp folder."
    else if test (count $argv) -eq 0
        set target_dir (mchdir -t)

        if test $status -eq 0
            cd $target_dir
        else
            echo "Failed to change directory."
        end
    else
        set target_dir (mchdir -t -d $argv)

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
