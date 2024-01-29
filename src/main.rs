use clap::Parser;

use crate::{cpu::Cpu, keypad::Keypad, sound::Sound};

mod consts;
mod cpu;
mod display;
mod keypad;
mod sound;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    filename: String,

    #[arg(
        short,
        long,
        default_value_t = 8,
        help = "CPU speed (speed * 60 = Hz/TPS)"
    )]
    speed: u8,

    #[arg(short, long, default_value_t = false, help = "Enable fullscreen mode")]
    fullscreen: bool,

    #[arg(long, default_value_t = String::from("#0f380f"), help = "Background color")]
    bg: String,

    #[arg(long, default_value_t = String::from("#8bac0f"), help = "Foreground color")]
    fg: String,

    #[arg(
        long = "software",
        default_value_t = false,
        help = "Force software rendering"
    )]
    software_render: bool,

    #[arg(long, default_value_t = false, help = "Enable debug mode")]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let file_name = args.filename;

    let mut cpu = Cpu::new(args.speed, args.debug);
    cpu.load_rom(&file_name);

    let sdl_context = sdl2::init().expect("Failed to init SDL");
    let timer = sdl_context.timer().expect("SDL context timer failed");
    let mut keypad = Keypad::new(&sdl_context);
    let mut sound = Sound::new(&sdl_context);
    let mut display = display::Display::new(
        &sdl_context,
        display::Config {
            fullscreen: args.fullscreen,
            software_render: args.software_render,
            fg_hex: &args.fg,
            bg_hex: &args.bg,
        },
    )
    .unwrap_or_else(|e| {
        panic!("Failed to create display: {}", e);
    });

    let mut paused = false;
    let mut do_sound = false;

    // Frame timing
    const INTERVAL: u32 = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    'run: loop {
        // Input handling
        match keypad.key_press(&mut cpu.keypad) {
            keypad::State::Exit => break 'run,
            keypad::State::Continue => {}
            keypad::State::Increase => {
                cpu.speed = cpu.speed.wrapping_add(1);
                println!("Speed: {} ({} Hz)", cpu.speed, cpu.speed as u32 * 60);
            }
            keypad::State::Decrease => {
                cpu.speed = cpu.speed.wrapping_sub(1);
                println!("Speed: {} ({} Hz)", cpu.speed, cpu.speed as u32 * 60);
            }
            keypad::State::Reset => {
                cpu.reset();
            }
            keypad::State::Debug => {
                cpu.debug = !cpu.debug;
            }
            keypad::State::PauseToggle => {
                paused = !paused;
            }
            keypad::State::Pause => {
                paused = true;
            }
            keypad::State::Unpause => {
                paused = false;
            }
        }

        if !paused {
            for _ in 0..cpu.speed {
                cpu.tick(&mut display);
            }
        }

        // Frame timing
        let now = timer.ticks();
        let dt = now - before;

        if dt < INTERVAL {
            // timer.delay(INTERVAL - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1000 {
            println!(
                "FPS: {} | {}Hz ({}) | {}",
                fps,
                cpu.speed,
                (cpu.speed as u32 * 60),
                if paused { "Paused" } else { "Running" },
            );
            last_second = now;
            fps = 0;
        }

        let do_sound_now = cpu.sound_timer > 0;

        if do_sound != do_sound_now {
            if do_sound_now {
                sound.resume();
            } else {
                sound.pause();
            }

            do_sound = do_sound_now;
        }

        cpu.update_timers(dt as f32);
    }
}
