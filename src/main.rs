use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};
use clap::Parser;
use mainstage::{Concert, Patch};
use keylab::{KeyboardLcd, KeyboardPad, Keyboard, KLE_61, Color};

#[derive(Parser, Debug)]
struct Args {
    #[clap(index = 1)]
    file: String
}

fn print_patch_names(patch: &Patch, prefix: String) {
    match patch {
        Patch::Program { name, .. } => {
            println!("{}- Patch: {}", &prefix, name);
        }
        Patch::Bank { name, programs, .. } => {
            println!("{}- Set: {}", &prefix, name);
            for prog in programs {
                print_patch_names(prog, format!("{}    ", &prefix))
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let now = Instant::now();
    let concert = Concert::from_file(&args.file);

    if concert.is_ok() {
        let concert = concert.unwrap();
        println!("Loaded Concert: '{}' in {}ms", &concert.name, now.elapsed().as_secs_f64() * 1000.);

        println!("Patches in '{}'", &concert.name);

        for bank in &concert.banks {
            print_patch_names(&bank, "".to_string());
        }
    } else {
        eprintln!("{}", concert.unwrap_err());
    }

    let keyboard = KLE_61::new();

    if keyboard.is_err() {
        eprintln!("Could not find KeyLab Essential 61 plugged in");
        exit(1);
    }
    let mut keyboard = keyboard.unwrap();

    println!("Opening Keyboard Connection");
    let _ = &keyboard.open().unwrap();
    println!("Updating text to 'test1', 'test2'");
    let _ = &keyboard.update_text(Some("test1".to_string()), Some("test2".to_string())).unwrap();
    println!("Vegas Pad Time!!");
    vegas_pad_all(&mut keyboard, 1000, 50);
    println!("Closing Keyboard Connection");
    let _ = &keyboard.close().unwrap();
}

fn vegas_pad_all(keyboard: &mut KLE_61, iterations: u32, delay: u32) {
    let mut rgb: [Color; 8] = [
        Color {
            r: 255,
            g: 0,
            b: 0
        },
        Color {
            r: 225,
            g: 0,
            b: 0
        },
        Color {
            r: 195,
            g: 0,
            b: 0
        },
        Color {
            r: 165,
            g: 0,
            b: 0
        },
        Color {
            r: 135,
            g: 0,
            b: 0
        },
        Color {
            r: 105,
            g: 0,
            b: 0
        },
        Color {
            r: 75,
            g: 0,
            b: 0
        },
        Color {
            r: 45,
            g: 0,
            b: 0
        }
    ];

    for _ in 0..iterations {
        for i in 0..8 {
            let color = &mut rgb[i];

            if color.r > 0 && color.b == 0 {
                color.r -= 5;
                color.g += 5;
            }
            if color.g > 0 && color.r == 0 {
                color.g -= 5;
                color.b += 5;
            }
            if color.b > 0 && color.g == 0 {
                color.r += 5;
                color.b -= 5;
            }

            let _ = &keyboard.pad_light_up((i + 1) as u8, Some(color));
        }

        sleep(Duration::from_millis(delay as u64));
    }
}
