use std::{
    error::Error,
    io,
    sync::mpsc::channel,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::{self, new_frame, Drawable, Frame},
    invaders::Invaders,
    player::Player,
    render,
};

fn main() -> Result<(), Box<dyn Error>> {
    // terminal
    let mut stdout = io::stdout();

    // raw mode
    enable_raw_mode().expect("Failed to enable raw mode");
    stdout
        .execute(EnterAlternateScreen)
        .expect("Failed to enter alternate screen");
    stdout.execute(Hide).expect("Failed to hide cursor");

    // Render loop in a seperate thread
    let (render_tx, render_rx) = channel::<Frame>();

    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(new_frame) => new_frame,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        // per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input handling
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left | KeyCode::Char('a') => player.move_left(),
                    KeyCode::Right | KeyCode::Char('d') => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => player.shoot(),

                    KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,
                    _ => {}
                }
            }
        }

        // updates
        player.update(delta);
        invaders.update(delta);
        player.detect_hit(&mut invaders);

        // draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // win or loose
        if invaders.all_killed() {
            println!("You win!");
            break;
        } else if invaders.reached_bottom() {
            println!("You loose!");
            break;
        }
    }

    // cleanup
    drop(render_tx);
    render_handle.join().expect("Render thread panicked");
    stdout.execute(Show).expect("Failed to show cursor");
    stdout
        .execute(LeaveAlternateScreen)
        .expect("Failed to leave alternate screen");
    disable_raw_mode().expect("Failed to disable raw mode");
    Ok(())
}
