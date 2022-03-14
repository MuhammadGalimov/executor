use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct Executor {
    tasks: Vec<fn()>,
    tx: mpsc::Sender<fn()>
}

impl Executor {
    pub fn new(n: u8) -> Self {
        let (tx, rx) = mpsc::channel::<fn()>();
        let rx = Arc::new(Mutex::new(rx));

        for _ in 0..n {
            let rx_cloned = Arc::clone(&rx);
            thread::spawn(move || loop {
                let task = rx_cloned.lock().unwrap().recv();

                match task {
                    Ok(rx) => { (rx)(); }
                    Err(_) => { break; }
                }
            });
        }

        Executor { tasks: vec![], tx }
    }

    pub fn add_task(&mut self, fp: fn()) {
        self.tasks.push(fp);
    }

    pub fn run(&self) {
        for task in &self.tasks {
            self.tx.send(*task).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tt() {
        let mut e = Executor::new(3);
        e.add_task(|| { for _ in 0..2 { println!("1 task"); } });
        e.add_task(|| { for _ in 0..2 { println!("2 task"); } });
        e.add_task(|| { for _ in 0..2 { println!("3 task"); } });
        e.add_task(|| { for _ in 0..2 { println!("4 task"); } });
        
        e.run();
    }
}
