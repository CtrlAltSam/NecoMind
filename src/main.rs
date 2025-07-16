use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Terminal,
    text::{Line, Span},
};
use std::{
    io, time::{Duration, Instant}
};
use sysinfo::{System};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    let mut sys = System::new_all();
    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();

    let ascii_art = r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⢀⡔⣻⠁⠀⢀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢀⣾⠳⢶⣦⠤⣀⠀⠀⠀⠀⠀⠀⠀⣾⢀⡇⡴⠋⣀⠴⣊⣩⣤⠶⠞⢹⣄⠀⠀⠀
⠀⠀⠀⠀⢸⠀⠀⢠⠈⠙⠢⣙⠲⢤⠤⠤⠀⠒⠳⡄⣿⢀⠾⠓⢋⠅⠛⠉⠉⠝⠀⠼⠀⠀⠀
⠀⠀⠀⠀⢸⠀⢰⡀⠁⠀⠀⠈⠑⠦⡀⠀⠀⠀⠀⠈⠺⢿⣂⠀⠉⠐⠲⡤⣄⢉⠝⢸⠀⠀⠀
⠀⠀⠀⠀⢸⠀⢀⡹⠆⠀⠀⠀⠀⡠⠃⠀⠀⠀⠀⠀⠀⠀⠉⠙⠲⣄⠀⠀⠙⣷⡄⢸⠀⠀⠀
⠀⠀⠀⠀⢸⡀⠙⠂⢠⠀⠀⡠⠊⠀⠀⠀⠀⢠⠀⠀⠀⠀⠘⠄⠀⠀⠑⢦⣔⠀⢡⡸⠀⠀⠀
⠀⠀⠀⠀⢀⣧⠀⢀⡧⣴⠯⡀⠀⠀⠀⠀⠀⡎⠀⠀⠀⠀⠀⢸⡠⠔⠈⠁⠙⡗⡤⣷⡀⠀⠀
⠀⠀⠀⠀⡜⠈⠚⠁⣬⠓⠒⢼⠅⠀⠀⠀⣠⡇⠀⠀⠀⠀⠀⠀⣧⠀⠀⠀⡀⢹⠀⠸⡄⠀⠀
⠀⠀⠀⡸⠀⠀⠀⠘⢸⢀⠐⢃⠀⠀⠀⡰⠋⡇⠀⠀⠀⢠⠀⠀⡿⣆⠀⠀⣧⡈⡇⠆⢻⠀⠀
⠀⠀⢰⠃⠀⠀⢀⡇⠼⠉⠀⢸⡤⠤⣶⡖⠒⠺⢄⡀⢀⠎⡆⣸⣥⠬⠧⢴⣿⠉⠸⡀⣇⠀⠀
⠀⠀⠇⠀⠀⠀⢸⠀⠀⠀⣰⠋⠀⢸⣿⣿⠀⠀⠀⠙⢧⡴⢹⣿⣿⠀⠀⠀⠈⣆⠀⠀⢧⢹⡄
⠀⣸⠀⢠⠀⠀⢸⡀⠀⠀⢻⡀⠀⢸⣿⣿⠀⠀⠀⠀⡼⣇⢸⣿⣿⠀⠀⠀⢀⠏⠀⠀⢸⠀⠇
⠀⠓⠈⢃⠀⠀⠀⡇⠀⠀⠀⣗⠦⣀⣿⡇⠀⣀⠤⠊⠀⠈⠺⢿⣃⣀⠤⠔⢸⠀⠀⠀⣼⠑⢼
⠀⠀⠀⢸⡀⣀⣾⣷⡀⠀⢸⣯⣦⡀⠀⠀⠀⢇⣀⣀⠐⠦⣀⠘⠀⠀⢀⣰⣿⣄⠀⠀⡟⠀⠀
⠀⠀⠀⠀⠛⠁⣿⣿⣧⠀⣿⣿⣿⣿⣦⣀⠀⠀⠀⠀⠀⠀⠀⣀⣠⣴⣿⣿⡿⠈⠢⣼⡇⠀⠀
⠀⠀⠀⠀⠀⠀⠈⠁⠈⠻⠈⢻⡿⠉⣿⠿⠛⡇⠒⠒⢲⠺⢿⣿⣿⠉⠻⡿⠁⠀⠀⠈⠁⠀⠀
"#;

    sys.refresh_cpu();

    let cpu_name = sys.cpus()
        .first().map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    let gpu_name = get_gpu_windows();
    let os_name = System::name().unwrap_or("Unknown".into());
    let host = System::host_name().unwrap_or("Unknown".into());
    let kernal = System::kernel_version().unwrap_or("Unknown".into());
    
    let uptime = System::uptime();
    let hours = uptime/3600;
    let minutes = (uptime % 3600) / 60;

    let (cols, rows) = crossterm::terminal::size().unwrap_or((0, 0));
    let resolution = format!("{}x{}", cols, rows);

    let stats_text = format!(
        "OS: {}\nHost: {}\nKernal: {}\nUptime: {} hours, {} mins\nResolution: {}\nCPU: {}\nGPU: {}",
        os_name, host, kernal, hours, minutes, resolution, cpu_name, gpu_name
    );

    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            sys.refresh_cpu();
            sys.refresh_memory();
            last_tick = Instant::now();
        }

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let total_mem = sys.total_memory() as f64;
        let used_mem = sys.used_memory() as f64;
        let mem_usage = ((used_mem / total_mem) * 100.0).round() as u16;
        let total_swap = sys.total_swap() as f64;
        let used_swap = sys.used_swap() as f64;
        let swap_usage = if total_swap > 0.0 {
            ((used_swap / total_swap) * 100.0).round() as u16
        } else {
            0
        };



        terminal.draw(|f| {
            let size = f.size();

            
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(size);

            
            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(9),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(chunks[1]);

            let art = Paragraph::new(ascii_art)
                .block(Block::default().title("NecoMind").borders(Borders::ALL))
                .wrap(Wrap { trim: false });

            let stats = Paragraph::new(stats_text.clone())
                .block(Block::default().title("Info").borders(Borders::ALL))
                .wrap(Wrap { trim: true });

            let cpu_bar = Gauge::default()
                .block(Block::default().title("CPU Usage").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::White))
                .percent(cpu_usage as u16);

            let mem_bar = Gauge::default()
                .block(Block::default().title("Memory Usage").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::White))
                .percent(mem_usage);

            let swap_bar = Gauge::default()
                .block(Block::default().title("Swap Usage").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::White))
                .percent(swap_usage);

            let palette = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("████", Style::default().fg(Color::Red)),
                    Span::styled("████", Style::default().fg(Color::Green)),
                    Span::styled("████", Style::default().fg(Color::Yellow)),
                    Span::styled("████", Style::default().fg(Color::Blue)),
                    Span::styled("████", Style::default().fg(Color::Magenta)),
                    Span::styled("████", Style::default().fg(Color::Cyan)),
                    Span::styled("████", Style::default().fg(Color::White)),
                ]),
            ])
            .block(Block::default().borders(Borders::ALL)).alignment(ratatui::layout::Alignment::Center);

            f.render_widget(art, chunks[0]);
            f.render_widget(stats, right_chunks[0]);
            f.render_widget(cpu_bar, right_chunks[1]);
            f.render_widget(mem_bar, right_chunks[2]);
            f.render_widget(swap_bar, right_chunks[3]);
            f.render_widget(palette, right_chunks[4]);
        })?;
    }
}

fn get_gpu_windows() -> String {
    let output = std::process::Command::new("powershell")
        .args(["-Command", "Get-WmiObject Win32_VideoController | Select-Object -ExpandProperty Name"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    output
        .lines()
        .next()
        .unwrap_or("GPU not found")
        .trim()
        .to_string()
}

