use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    style::{self, Color, Stylize},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use rodio::{source::{Pink, Source}, OutputStreamBuilder};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

/// Hacker Typer: Ultimate Pranking Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Custom file to type out (defaults to built-in code)
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Typing speed (chars per keystroke)
    #[arg(short, long, default_value_t = 3)]
    speed: usize,

    /// Enable mechanical keyboard sounds
    #[arg(long, default_value_t = true)]
    sound: bool,

    /// Spawn multiple windows for "Hollywood Hacker" effect
    #[arg(long)]
    multi_window: bool,

    /// Number of extra windows to spawn
    #[arg(long, default_value_t = 2)]
    window_count: usize,
}


const DEFAULT_CODE: &str = r#"
// MATRIX CORE SYSTEM - SECURITY LEVEL 9
// ACCESSING MAINFRAME...

use std::matrix::render;
use system::kernel::{inject, bypass};

fn main() -> SystemResult<()> {
    let mut cyberspace = Matrix::connect("192.168.0.1");
    
    // Breaking encryption layer...
    for attempt in 0..u64::MAX {
        if cyberspace.crack_node(attempt) {
            println!("ACCESS GRANTED: NODE_{}", attempt);
            break;
        }
    }

    // Injecting payload
    let payload = vec![0xDE, 0xAD, 0xBE, 0xEF];
    unsafe {
        system::memory::write(0x0000_FFFF, payload);
    }

    // Downloading secret data...
    let secrets = cyberspace.download("/root/passwords.db");
    
    // Covering tracks
    system::logs::wipe_all();
    
    Ok(())
}

// END OF TRANSMISSION
"#;

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle Multi-Window Mode
    if args.multi_window {
        spawn_windows(args.window_count)?;
        // After spawning others, run in the current window too
    }

    let code = match args.file {
        Some(path) => fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {:?}", path))
            .unwrap_or_else(|_| DEFAULT_CODE.to_string()),
        None => DEFAULT_CODE.to_string(),
    };

    // Infinite loop of the code logic to prevent running out
    let full_code = code.repeat(100);

    run_ui(&full_code, args.speed, args.sound)?;

    Ok(())
}

fn spawn_windows(count: usize) -> Result<()> {
    let current_exe = std::env::current_exe()?;
    let terminal = identify_terminal();

    for _ in 0..count {
        // We carefully construct the command to run THIS executable again, 
        // but WITHOUT the --multi-window flag to prevent infinite recursion.
        let child = Command::new(&terminal)
            // Most terminals use -e to run a command
            .arg("-e")
            .arg(&current_exe) 
            .spawn();

        if let Err(e) = child {
            eprintln!("Failed to spawn terminal window: {}. Is '{}' installed?", e, terminal);
        }
    }
    Ok(())
}


fn identify_terminal() -> String {
    // A heuristic generic list of terminals to try on Linux
    let candidates = [
        "x-terminal-emulator", // Debian/Ubuntu alias
        "gnome-terminal",
        "konsole",
        "alacritty",
        "xterm",
    ];

    for term in candidates {
        if which::which(term).is_ok() {
            return term.to_string();
        }
    }
    
    // Fallback
    "x-terminal-emulator".to_string()
}

fn run_ui(content: &str, chunk_size: usize, enable_sound: bool) -> Result<()> {
    // Setup Audio
    // Initialize output stream but allow failure (e.g. no audio device)
    // We keep stream alive.
    let stream_opt = OutputStreamBuilder::open_default_stream().ok();
    
    // Setup Terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    let mut index = 0;
    let chars: Vec<char> = content.chars().collect();
    let max_len = chars.len();

    loop {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        _ => {
                            if enable_sound {
                                if let Some(ref stream) = stream_opt {
                                     play_type_sound(stream.mixer());
                                }
                            }

                            let end = (index + chunk_size).min(max_len);
                            if index < max_len {
                                let chunk: String = chars[index..end].iter().collect();
                                
                                // Print the chunk in "Hacker Green"
                                print!("{}", chunk.with(Color::Green));
                                stdout.flush()?;
                                
                                index = end;
                            } else {
                                // Loop back to start if finished
                                index = 0; 
                                stdout.execute(terminal::Clear(ClearType::All))?;
                                stdout.execute(cursor::MoveTo(0, 0))?;
                            }
                        }
                    }
                }
            }
        }
    }

    // Cleanup
    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}

fn play_type_sound(mixer: &rodio::mixer::Mixer) {
    // Generate a short burst of noise to simulate a mechanical switch
    // Pink noise is a good approximation of a keypress thud/click
    let source = Pink::new(48000)
        .take_duration(Duration::from_millis(30))
        .amplify(0.20); 

    mixer.add(source);
}
