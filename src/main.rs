#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] use std::os::windows::process;
use std::thread;

// hide console window on Windows in release
use eframe_template::{mdata, fakerinput};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc,RwLock};
use std::time::Duration;
use rand::Rng;
fn main() {
    // std::panic::catch_unwind(|| {
    //     bsod::bsod();
    // });
    // Log to stdout (if you run with `RUST_LOG=debug`).

    tracing_subscriber::fmt::init();

    let (mtx, mut mrx) = std::sync::mpsc::channel::<mdata>();
    
    let mut enabled = Arc::new(AtomicBool::new(false));
    let mut linear = Arc::new(AtomicBool::new(false));
    let mut lowValue = Arc::new(RwLock::new(1000.0));
    let mut hiValue = Arc::new(RwLock::new(1000.0));
    let en2 = enabled.clone();
    let hl = std::thread::spawn(move || {
        
        let mut hotket_listenr = hotkey::Listener::new();
        
        hotket_listenr.register_hotkey( hotkey::modifiers::SHIFT | hotkey::modifiers::CONTROL ,'L' as u32, move || {
            let last_known = en2.load(Ordering::Relaxed);
            println!("switching to {}", !last_known);
            en2.store(!last_known, Ordering::Relaxed)
        });
        hotket_listenr.register_hotkey( hotkey::modifiers::SHIFT | hotkey::modifiers::CONTROL ,hotkey::keys::CAPS_LOCK, move || {
            println!("BYE BYE");
            std::process::exit(0);
        });
        hotket_listenr.listen();
    });
    let lowval2 = lowValue.clone();
    let hival2 = hiValue.clone();
    let jh = std::thread::spawn(move ||  {
        let mut fi = fakerinput::FakerInput::new();
        fi.connect();
        println!("conn");
        loop {
            let e = {
                if linear.load(Ordering::Relaxed) {
                    println!("{}", *lowval2.clone().read().unwrap());
                    mrx.recv_timeout(Duration::from_millis((1000.0 / *lowval2.clone().read().unwrap()) as u64 ))
                } else {
                    let rand_range: f64;
                    if *lowval2.clone().read().unwrap() > *hival2.clone().read().unwrap() {
                        rand_range = 10.0;
                    } else {
                        rand_range = rand::thread_rng().gen_range(*lowval2.clone().read().unwrap()..=*hival2.clone().read().unwrap());
                    }
                    println!("{}", rand_range);
                    mrx.recv_timeout(Duration::from_millis((1000.0 / rand_range) as u64 ))
                }
            };
            if let Ok(x) = e {
                match x {
                    mdata::Enabled(x) => {
                        enabled.store(x, Ordering::Relaxed);
                    }
                    mdata::LowValue(v) => {
                        println!("opening write");
                        *lowval2.clone().write().unwrap() = v;
                        println!("closing write");
                    }
                    mdata::HiValue(v) => {
                        println!("opening write");
                        *hival2.clone().write().unwrap() = v;
                        println!("closing write");
                    }
                    mdata::Linear(x) => {
                        linear.store(x, Ordering::Relaxed);
                    }
                }        
            }
            let enabled = enabled.load(Ordering::Relaxed);
            if enabled {
                let mut mr = fakerinput::mouse_report::MouseReport::new();
                mr.button_down(fakerinput::mouse_report::MouseButtons::Left);
                fi.update_relative_mouse(&mr);
                mr.button_up(fakerinput::mouse_report::MouseButtons::Left);
                fi.update_relative_mouse(&mr);
            }
        }
    });

    let native_options = eframe::NativeOptions {
        initial_window_size : Some(egui::Vec2::new(500.0, 150.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "octo clicker",
        native_options,
        Box::new(|cc| Box::new(eframe_template::Octo::new(cc, mtx))),
    );
    std::process::exit(0);
    jh.join();
    hl.join();
    // jh.await.unwrap();
}
