use std::io::{self};
use std::{thread::sleep, time::Duration};

use crate::ui::Plane;

mod ui;

fn main() -> io::Result<()> {
    let mut stdplane = match ui::init_terminal() {
        Ok(it) => it,
        Err(err) => {
            println!("Unable to setup terminal :(");
            return Err(err);
        }
    };

    stdplane.add_child(Plane::new(10, 10, 10, 10, None));

    print!("Hello raw terminal!");

    sleep(Duration::new(2, 0));

    ui::deinit_terminal()?;

    Ok(())
}
