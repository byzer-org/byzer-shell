use std::sync::mpsc::{channel, Sender};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::{self, sleep};
use std::time::Duration;

/// a progress bar to indicate the progress of starting the engine
/// or executing a single task.
pub struct ExecutingProgressBar {
    finish_signal: Option<Sender<bool>>,
}

impl ExecutingProgressBar {
    pub fn new() -> Self {
        Self {
            finish_signal: None,
        }
    }

    /// start a child thread to monitor the execution, waiting for the finish signal.
    pub fn start_monitor(&mut self, prefix: String) -> thread::JoinHandle<()> {
        let (tx, rx) = channel::<bool>();
        self.finish_signal = Some(tx);
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner} {wide_msg}")
        );
        let h = thread::spawn(move || {
            let mut count: u64 = 0;
            loop {
                pb.set_message(format!("{} {}s", prefix, count));
                match rx.try_recv() {
                    Ok(val) => {
                        pb.finish_and_clear();
                        if val {
                            println!("✅ {}s.", count);
                        } else {
                            println!("❌ {}s.", count);
                        }
                        break;
                    },
                    Err(_) => {
                        count += 1;
                    }
                }
                sleep(Duration::from_secs(1));
            }
        });
        
        h
    }

    /// send finish signal to monitor thread, send false if finished with error
    /// , vice versa.
    pub fn send_finish_signal(&self, val: bool) {
        if let Some(f) = &self.finish_signal {
            f.send(val).unwrap();
        }
    }
}