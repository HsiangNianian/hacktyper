use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    style::{Color, Stylize},
    terminal::{self, ClearType},
};
use rand::Rng; // Re-add Rand
use rodio::{
    OutputStreamBuilder,
    source::{Pink, SineWave, Source},
};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::LinesWithEndings;

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

    let (code, extension) = match args.file {
        Some(path) => {
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_string());
            let content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read file: {:?}", path))
                .unwrap_or_else(|_| DEFAULT_CODE.to_string());
            (content, ext)
        },
        None => (DEFAULT_CODE.to_string(), Some("rs".to_string())),
    };

    // Infinite loop of the code logic to prevent running out
    let full_code = code.repeat(100);

    if args.matrix {
        run_matrix(args.sound)?;
    } else {
        // Pre-process content for highlighting and formatting
        let styled_content = highlight_code(&full_code, extension.as_deref());
        run_ui(&styled_content, args.speed, args.sound)?;
    }

    Ok(())
}

struct StyledChar {
    char: char,
    color: Color,
}

fn highlight_code(code: &str, extension: Option<&str>) -> Vec<StyledChar> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // Try to find a nice dark theme
    let theme = ts.themes.get("base16-ocean.dark")
        .or_else(|| ts.themes.get("base16-mocha.dark"))
        .unwrap_or_else(|| ts.themes.values().next().unwrap());

    let mut syntax = ps.find_syntax_plain_text();
    
    if let Some(ext) = extension {
        if let Some(s) = ps.find_syntax_by_extension(ext) {
            syntax = s;
        }
    }
    
    // Fallback to first line detection if extension failed or was not provided
    if syntax.name == "Plain Text" {
         if let Some(s) = ps.find_syntax_by_first_line(code) {
             syntax = s;
         }
    }

    let mut h = HighlightLines::new(syntax, theme);
    let mut result = Vec::with_capacity(code.len());

    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap_or_default();
        
        for (style, text) in ranges {
            let fg = style.foreground;
            let color = Color::Rgb { r: fg.r, g: fg.g, b: fg.b };
            
            for c in text.chars() {
                match c {
                    '\n' => {
                        result.push(StyledChar { char: '\r', color: Color::Reset });
                        result.push(StyledChar { char: '\n', color: Color::Reset });
                    }
                    '\r' => { /* Skip, handled by \n */ }
                    '\t' => {
                        for _ in 0..4 {
                            result.push(StyledChar { char: ' ', color });
                        }
                    }
                    _ => {
                        result.push(StyledChar { char: c, color });
                    }
                }
            }
        }
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
        if event::poll(Duration::from_millis(30))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Esc => break,
                _ => {
                    if enable_sound && let Some(ref stream) = stream_opt {
                        play_type_sound(stream.mixer());
                    }
                    // Typing adds more rain intensity?
                    if rng.random_bool(0.5) {
                        let c = rng.random_range(0..cols);
                        streams.push(create_stream(c, rows, &mut rng, &standard_chars));
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

fn run_ui(content: &[StyledChar], start_chunk_size: usize, enable_sound: bool) -> Result<()> {
    // Setup Audio
    // Initialize output stream but allow failure (e.g. no audio device)
    // We keep stream alive.
    let stream_opt = OutputStreamBuilder::open_default_stream().ok();

    // Setup Terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::Show)?; // Show cursor AFTER clear
    stdout.execute(cursor::SetCursorStyle::BlinkingBlock)?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    let mut index = 0;
    let mut chunk_size = start_chunk_size;
    let max_len = content.len();
    let mut freestyle_mode = false;
    let mut agent_mode = false;

    loop {
        let mut key_code = None;
        // Agent mode types faster/more consistently
        let poll_interval = if agent_mode { Duration::from_millis(30) } else { Duration::from_millis(50) };

        if event::poll(poll_interval)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    key_code = Some(key.code);
                }
            }
        } else if agent_mode {
            // Auto-type tick (simulated keypress)
            key_code = Some(KeyCode::Null);
        }

        if let Some(code) = key_code {
            match code {
                KeyCode::Esc => break,
                KeyCode::F(4) => {
                    agent_mode = !agent_mode;
                }
                KeyCode::Up => {
                    chunk_size = chunk_size.saturating_add(1);
                }
                KeyCode::Down => {
                    chunk_size = chunk_size.saturating_sub(1).max(1);
                }
                KeyCode::F(1) => {
                    if let Some(ref stream) = stream_opt {
                        if enable_sound {
                            play_result_sound(true, stream.mixer());
                        }
                    }
                    show_result_popup(&mut stdout, true)?;
                    index = 0; // Reset code
                }
                KeyCode::F(2) => {
                    if let Some(ref stream) = stream_opt {
                        if enable_sound {
                            play_result_sound(false, stream.mixer());
                        }
                    }
                    show_result_popup(&mut stdout, false)?;
                    index = 0; // Reset code
                }
                KeyCode::F(3) => {
                    freestyle_mode = !freestyle_mode;
                    // Optional: You could show a swift popup "FREESTYLE ENABLED", but let's keep it subtle for now.
                }
                code => {
                    if freestyle_mode {
                        match code {
                            KeyCode::Char(c) => {
                                if enable_sound && let Some(ref stream) = stream_opt {
                                    play_type_sound(stream.mixer());
                                }
                                print!("{}", c.with(Color::Green));
                                stdout.flush()?;
                            }
                            KeyCode::Enter => {
                                if enable_sound && let Some(ref stream) = stream_opt {
                                    play_type_sound(stream.mixer());
                                }
                                print!("\r\n");
                                stdout.flush()?;
                            }
                            KeyCode::Backspace => {
                                if enable_sound && let Some(ref stream) = stream_opt {
                                    play_type_sound(stream.mixer());
                                }
                                stdout.execute(cursor::MoveLeft(1))?;
                                print!(" ");
                                stdout.execute(cursor::MoveLeft(1))?;
                                stdout.flush()?;
                            }
                            KeyCode::Tab => {
                                if enable_sound && let Some(ref stream) = stream_opt {
                                    play_type_sound(stream.mixer());
                                }
                                print!("    ");
                                stdout.flush()?;
                            }
                            KeyCode::BackTab => {
                                // Handle Shift+Tab if supported (Go back 4 spaces?)
                                // Simple implementation: perform backspace 4 times
                                if enable_sound && let Some(ref stream) = stream_opt {
                                    play_type_sound(stream.mixer());
                                }
                                for _ in 0..4 {
                                    stdout.execute(cursor::MoveLeft(1))?;
                                    print!(" ");
                                    stdout.execute(cursor::MoveLeft(1))?;
                                }
                                stdout.flush()?;
                            }
                            _ => {}
                        }
                    } else {
                        // Standard Hacker Typer Mode
                        if enable_sound && let Some(ref stream) = stream_opt {
                            play_type_sound(stream.mixer());
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

    // Cleanup
    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}

fn play_type_sound(mixer: &rodio::mixer::Mixer) {
    // "Thock" component (Body) - Low sine wave
    let body = SineWave::new(150.0)
        .take_duration(Duration::from_millis(50))
        .amplify(0.10);
    
    // "Click" component (Transient) - Short pink noise
    let click = Pink::new(48000)
        .take_duration(Duration::from_millis(15))
        .amplify(0.15);

    mixer.add(body);
    mixer.add(click);
}

fn play_result_sound(success: bool, mixer: &rodio::mixer::Mixer) {
    if success {
        // Major chord arpeggio for success
        let vol = 0.15;
        mixer.add(SineWave::new(523.25).take_duration(Duration::from_millis(200)).amplify(vol)); // C5
        mixer.add(SineWave::new(659.25).take_duration(Duration::from_millis(200)).amplify(vol)); // E5
        mixer.add(SineWave::new(783.99).take_duration(Duration::from_millis(200)).amplify(vol)); // G5
    } else {
        // Low dissonance for failure
        let vol = 0.30;
        mixer.add(SineWave::new(110.0).take_duration(Duration::from_millis(600)).amplify(vol)); // A2
        mixer.add(SineWave::new(116.54).take_duration(Duration::from_millis(500)).amplify(vol)); // A#2 (Clash)
    }
}

fn show_result_popup(stdout: &mut io::Stdout, success: bool) -> Result<()> {
    let (cols, rows) = terminal::size()?;
    let text = if success { " ACCESS GRANTED " } else { " ACCESS DENIED " };
    let color = if success { Color::Green } else { Color::Red };
    
    let text_len = text.len() as u16;
    let start_x = (cols.saturating_sub(text_len)) / 2;
    let start_y = rows / 2;
    
    // Simple flashing effect
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::Hide)?; // Temporarily hide during animation
    
    for _ in 0..6 {
        stdout.execute(cursor::MoveTo(start_x, start_y))?;
        // Draw with reversed colors for "box" feel
        print!("{}", text.with(Color::Black).on(color).bold());
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(150));
        
        stdout.execute(terminal::Clear(ClearType::All))?;
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(100));
    }
    
    // Finally clear and prepare to respawn code
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(cursor::Show)?; // Restore cursor
    stdout.execute(cursor::SetCursorStyle::BlinkingBlock)?;
    Ok(())
}
 