use minifb::{Key, Window, WindowOptions};

use std::error::Error;
use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::buffer::BVECBuffer;

#[derive(Debug)]
pub struct BVEC {
    //buffer: Arc<BVECBuffer>,
    running: Arc<AtomicBool>,
    render_handle: Option<std::thread::JoinHandle<()>>,
}

impl BVEC {
    pub fn new() -> BVEC {
        Self{
            running: Arc::new(AtomicBool::new(true)),
            render_handle: None,
        }
    }

    pub fn run(&mut self) {
        let running = self.running.clone();

        self.render_handle = Some(std::thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                println!("Joe");
            }
        }));
    }

    pub fn end(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        self.render_handle.take().unwrap().join().unwrap();
        println!("bama");
    }
}
