use snowflake::SnowflakeIdBucket;
use chrono::prelude::Local;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Default)]
enum TaskState {
    Abandon,
    Done,
    #[default]
    Todo,
}

enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
struct Task {
    task_id: i64,
    depth: u8,
    content: String,
    state: TaskState,
    comments: Option<String>,
    create_time: String,
    update_time: String,
    dead_time: Option<String>,
    prev_task: Option<i64>,
    next_task: Option<i64>
}

impl Task {
    fn new() -> Self {
        let mut id_generator_bucket = SnowflakeIdBucket::new(1, 1);
        let local_time = Local::now();
        Self {
            depth: 0,
            task_id: id_generator_bucket.get_id(),
            content: "".to_string(),
            state: TaskState::default(),
            comments: None,
            create_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            dead_time: None,
            prev_task: None,
            next_task: None
        }
    }
    fn todo_or_done(&mut self) {
        match &self.state {
            TaskState::Todo => {self.state = TaskState::Done},
            TaskState::Done => {self.state = TaskState::Todo},
            TaskState::Abandon => {self.state = TaskState::Todo},
        }
        
    }
    fn abandon(&mut self) {
        self.state = TaskState::Abandon;
    }
}

impl Default for Task {
    fn default() -> Self {
        Task::new()
    }
}

struct App {
    tasks: Vec<Task>,
    // Current selected value of the tasks
    index: usize,
    // Current input mode
    input_mode: InputMode,
}

impl App {
    fn new() -> App {
        let task1 = Task { content: "Have breakfast".to_string(), ..Default::default() };
        let task2 = Task { content: "Have lunch".to_string(), depth: 1, ..Default::default()};
        let task3 = Task { content: "Have dinner".to_string(), ..Default::default()};
        let task4 = Task { content: "Do homework".to_string(), ..Default::default()};
        App {
            tasks: vec![task1, task2, task3, task4],
            index: 0,
            input_mode: InputMode::Normal,
        }
    }

    fn edit_finished(&mut self) {
        self.input_mode = InputMode::Normal;
        if self.tasks[self.index].content == "".to_string() {
            self.edit_abandon();
        }
    }

    fn edit_abandon(&mut self) {
        self.tasks.remove(self.index);
        self.index -= 1;
        self.input_mode = InputMode::Normal;
    }

    fn add_brother_task(&mut self) {
        self.input_mode = InputMode::Editing;
        self.tasks.insert(self.index + 1, Task { depth: self.tasks[self.index].depth, ..Default::default() });
        self.index += 1;
    }

    fn next(&mut self) {
        self.index = (self.index + 1) % self.tasks.len();
    }

    fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tasks.len() - 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('h') | KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Char('l') | KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char(' ') => app.tasks[app.index].todo_or_done(),
                    KeyCode::Char('x') => app.tasks[app.index].abandon(),
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => app.add_brother_task(),
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => app.edit_finished(),
                    KeyCode::Esc => app.edit_abandon(),
                    KeyCode::Char(c) => {
                        if app.tasks[app.index].content.width() < 30 {
                            app.tasks[app.index].content.push(c)
                        }
                    },
                    KeyCode::Backspace => { app.tasks[app.index].content.pop(); },
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    match app.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        InputMode::Normal => {},
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                app.tasks[app.index].content.width() as u16 + (app.tasks[app.index].depth * 4) as u16 + format!("{:?} ", app.tasks[app.index].state).len() as u16 + 1,
                // Move one line down, from the border to the input line
                app.index as u16 + 1,
            )
        }
    }
    let texts: Vec<Spans> = app
        .tasks
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let mut text_style = Style::default().fg(Color::White).bg(Color::Reset);
            if app.index == index {
                text_style = Style::default().fg(Color::Black).bg(Color::White);
            }
            Spans::from(vec![
                Span::raw(format!("{:1$}", "", (task.depth * 4) as usize)),
                Span::raw(format!("{:?} ", task.state)),
                Span::styled(task.content.as_str(), text_style),
            ])
        })
        .collect();
    let size = f.size();

    // Surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Task tool designed by alonescar")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    let paragraph = Paragraph::new(texts.clone()).block(block);
    f.render_widget(paragraph, size);
}