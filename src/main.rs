use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    style::{self, Color, Stylize},
    terminal::{self, ClearType},
};
use lazy_static::lazy_static;
use rand::Rng; // Re-add Rand
use regex::Regex;
use rodio::{
    OutputStreamBuilder,
    source::{Pink, Source},
};
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

    /// Enable Matrix Digital Rain mode
    #[arg(short, long)]
    matrix: bool,
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

    if args.matrix {
        run_matrix(args.sound)?;
    } else {
        // Pre-process content for highlighting and formatting
        let styled_content = highlight_code(&full_code);
        run_ui(&styled_content, args.speed, args.sound)?;
    }

    Ok(())
}

struct StyledChar {
    char: char,
    color: Color,
}

fn highlight_code(code: &str) -> Vec<StyledChar> {
    lazy_static! {
        static ref RE_COMMENT: Regex = Regex::new(r"//.*").unwrap();
        static ref RE_STRING: Regex = Regex::new(r#""[^"]*""#).unwrap();
        static ref RE_KEYWORD: Regex = Regex::new(r"\b(fn|let|mut|if|else|for|while|loop|match|return|use|mod|struct|impl|pub|unsafe|crate|self|super|where|break|continue|as|const|static|trait|enum|type)\b").unwrap();
        static ref RE_TYPE: Regex = Regex::new(r"\b[A-Z][a-zA-Z0-9_]*\b").unwrap();
        static ref RE_NUMBER: Regex = Regex::new(r"\b\d+(_\d+)*(\.\d+)?\b").unwrap();
    }

    let mut result = Vec::with_capacity(code.len());

    // We process the code line by line to safely handle comments and newlines
    for line in code.lines() {
        let line_len = line.len();

        // This is a naive 'painter' approach: buffer the chars with default color, then overwrite colors.
        let mut line_colors = vec![Color::Green; line_len];

        // 1. Strings
        for cap in RE_STRING.find_iter(line) {
            for i in cap.start()..cap.end() {
                line_colors[i] = Color::Yellow;
            }
        }

        // 2. Keywords
        for cap in RE_KEYWORD.find_iter(line) {
            for i in cap.start()..cap.end() {
                if line_colors[i] == Color::Green {
                    line_colors[i] = Color::Magenta;
                }
            }
        }

        for cap in RE_TYPE.find_iter(line) {
            for i in cap.start()..cap.end() {
                if line_colors[i] == Color::Green {
                    line_colors[i] = Color::Cyan;
                }
            }
        }

        for cap in RE_NUMBER.find_iter(line) {
            for i in cap.start()..cap.end() {
                if line_colors[i] == Color::Green {
                    line_colors[i] = Color::Red;
                }
            }
        }

        // 3. Comments (override everything)
        if let Some(mat) = RE_COMMENT.find(line) {
            for i in mat.start()..mat.end() {
                line_colors[i] = Color::DarkGrey;
            }
        }

        // Convert to StyledChar
        for (i, c) in line.chars().enumerate() {
            result.push(StyledChar {
                char: c,
                color: line_colors[i],
            });
        }

        // Handle Newline: In raw mode, we need \r\n
        result.push(StyledChar {
            char: '\r',
            color: Color::Reset,
        });
        result.push(StyledChar {
            char: '\n',
            color: Color::Reset,
        });
    }

    result
}

// Matrix Effect Implementation
struct Stream {
    x: u16,
    y: f32,
    speed: f32,
    len: usize,
    chars: Vec<char>,
}

fn run_matrix(enable_sound: bool) -> Result<()> {
    // Setup Audio
    let stream_opt = OutputStreamBuilder::open_default_stream().ok();

    // Setup Terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(ClearType::All))?;

    let (cols, rows) = terminal::size()?;
    let mut rng = rand::rng();

    let mut streams: Vec<Stream> = Vec::new();
    let standard_chars: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#$%^&*()"
            .chars()
            .collect();

    // Init streams, one per column logic (sparse)
    // Actually, matrix rain usually has one stream per column, but they are sparse.
    // We will manage active streams.
    // Let's create `cols` potential streams.
    for c in 0..cols {
        if rng.random_bool(0.1) {
            // 10% chance to start active
            streams.push(create_stream(c, rows, &mut rng, &standard_chars));
        }
    }

    loop {
        // Event Handling (Typing)
        if event::poll(Duration::from_millis(30))? {
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
                            // Typing adds more rain intensity?
                            if rng.random_bool(0.5) {
                                let c = rng.random_range(0..cols);
                                streams.push(create_stream(c, rows, &mut rng, &standard_chars));
                            }
                        }
                    }
                }
            }
        }

        // Update & Render
        // We only draw updates to minimize IO
        for stream in &mut streams {
            let old_y = stream.y.floor() as i32;
            stream.y += stream.speed;
            let new_y = stream.y.floor() as i32;

            if new_y > old_y {
                // Draw Head
                if new_y < rows as i32 && new_y >= 0 {
                    stdout.execute(cursor::MoveTo(stream.x, new_y as u16))?;
                    // Head is white/bright
                    let char_idx = (new_y as usize) % stream.chars.len();
                    print!("{}", stream.chars[char_idx].with(Color::White));
                }

                // Draw Tail (Green)
                if old_y < rows as i32 && old_y >= 0 {
                    stdout.execute(cursor::MoveTo(stream.x, old_y as u16))?;
                    let char_idx = (old_y as usize) % stream.chars.len();
                    print!("{}", stream.chars[char_idx].with(Color::Green));
                }

                // Erase (Black) at y - len
                let erase_y = new_y - stream.len as i32;
                if erase_y < rows as i32 && erase_y >= 0 {
                    stdout.execute(cursor::MoveTo(stream.x, erase_y as u16))?;
                    print!(" ");
                }
            }
        }

        // Prune finished streams and Respawn
        streams.retain(|s| (s.y - s.len as f32) < rows as f32);

        // Ensure population
        while streams.len() < (cols as usize / 2) {
            let c = rng.random_range(0..cols);
            streams.push(create_stream(c, rows, &mut rng, &standard_chars));
        }

        stdout.flush()?;
    }

    // Cleanup
    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}

fn create_stream(col: u16, _rows: u16, rng: &mut impl Rng, charset: &[char]) -> Stream {
    let len = rng.random_range(5..20);
    let speed = rng.random_range(0.2..0.8);
    let mut chars = Vec::with_capacity(40);
    for _ in 0..40 {
        chars.push(charset[rng.random_range(0..charset.len())]);
    }

    Stream {
        x: col,
        y: 0.0 - rng.random_range(0..10) as f32, // Start mostly above screen
        speed,
        len,
        chars,
    }
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
            eprintln!(
                "Failed to spawn terminal window: {}. Is '{}' installed?",
                e, terminal
            );
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

fn run_ui(content: &[StyledChar], chunk_size: usize, enable_sound: bool) -> Result<()> {
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
    let max_len = content.len();

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
                                for i in index..end {
                                    let sc = &content[i];
                                    if sc.char == '\r' || sc.char == '\n' {
                                        print!("{}", sc.char);
                                    } else {
                                        print!("{}", sc.char.with(sc.color));
                                    }
                                }
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
