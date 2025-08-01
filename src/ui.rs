use crossterm::{
    cursor, execute, queue,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
    },
};
use std::io::{self, Write};

pub struct Plane {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    child: Option<Box<Plane>>,
}

impl Plane {
    pub fn new(x: u16, y: u16, width: u16, height: u16, child: Option<Box<Plane>>) -> Self {
        Self {
            x,
            y,
            width,
            height,
            child,
        }
    }

    pub fn add_child(&mut self, child: Plane) {
        self.child = Some(Box::new(child));
    }
}

pub fn init_terminal() -> io::Result<Plane> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;

    let (rows, cols) = size()?;

    execute!(stdout, EnterAlternateScreen)?;

    queue!(stdout, cursor::MoveTo(0, 0))?;

    stdout.flush()?;

    Ok(Plane::new(0, 0, cols, rows, None))
}

pub fn deinit_terminal() -> io::Result<()> {
    execute!(io::stdout(), LeaveAlternateScreen)?;

    disable_raw_mode()?;

    Ok(())
}

// pub fn render
