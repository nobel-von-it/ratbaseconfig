/*
really useful (without git clone and refactor) only start and stop functions,
maybe State enum or App struct
 */



mod main {
    use std::time::Duration;
    use anyhow::Result;
    use crossterm::{
        event, execute,
        terminal::{
            disable_raw_mode,
            enable_raw_mode,
            EnterAlternateScreen,
            LeaveAlternateScreen
        }
    };
    use ratatui::{
        Frame, Terminal,
        backend::CrosstermBackend,
        layout::Alignment,
        prelude::{Color, Style},
        widgets::{
            Block, Borders, BorderType, Paragraph
        }
    };

    #[derive(PartialEq)]
    /* 2 base state for me */
    enum State {
        Write,
        Read,
    }

    /* useful things */
    struct App {
        date: String,
        secret: String,
        state: State,
        quit: bool,
    }

    impl App {
        /* sugar */
        fn new() -> Self {
            App {
                date: String::new(),
                secret: String::new(),
                state: State::Read,
                quit: false
            }
        }
    }

    /* i like this architecture :) */
    pub fn ui_run() -> Result<()> {
        start()?;
        let status = run();
        stop()?;
        status?;
        Ok(())
    }

    pub fn start() -> Result<()> {
        enable_raw_mode()?;
        execute!(std::io::stderr(), EnterAlternateScreen)?;
        Ok(())
    }
    pub fn stop() -> Result<()> {
        disable_raw_mode()?;
        execute!(std::io::stderr(), LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn render(app: &App, f: &mut Frame) {
        /* use your colors */
        let color = if app.state == State::Write {
            Color::Green
        } else {
            Color::Blue
        };
        /* super base config */
        f.render_widget(
            Paragraph::new(format!("enter date (dd.mm.yyyy):\n{}\n{}", app.date, app.secret))
                .block(
                    Block::default()
                        .title("date life counter")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ).style(Style::default().fg(color)).alignment(Alignment::Center),
            f.size())
    }
    fn update(app: &mut App) -> Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match app.state {
                        State::Write => match key.code {
                            event::KeyCode::Char(c) => app.date.push(c),
                            event::KeyCode::Backspace => {app.date.pop();},
                            event::KeyCode::Tab => app.state = State::Read,
                            _ => {}
                        }
                        State::Read => match key.code {
                            event::KeyCode::Char('q') => app.quit = true,
                            event::KeyCode::Tab => app.state = State::Write,
                            event::KeyCode::Enter => app.date = String::from("done"),
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub fn run() -> Result<()> {
        let mut term = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
        let mut app = App::new();

        loop {
            term.draw(|f| {
                render(&app, f);
            })?;
            update(&mut app)?;

            if app.quit {
                break;
            }
        }

        Ok(())
    }
}