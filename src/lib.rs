use std::thread::{self, JoinHandle};
use std::sync::{mpsc, Arc, Mutex};

pub struct Executor {
    tasks: Vec<fn()>,
    handles: Vec<JoinHandle<()>>,
    tx: mpsc::Sender<fn()>,
    n: u8,
}

impl Executor {
    pub fn new(n: u8) -> Self {
        let (tx, rx) = mpsc::channel::<fn()>();
        let rx = Arc::new(Mutex::new(rx));

        // let mut tasks = vec![];
        let mut handles = vec![];

        for _ in 0..n {
            let rx_cloned = Arc::clone(&rx);
            let handle = thread::spawn(move || loop {
                let task = rx_cloned.lock().unwrap().recv().unwrap();
                (task)();
            });
            handles.push(handle);
        }

        Executor { n, tasks: vec![], tx, handles }
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

    fn f() -> i32 { 5 }

    #[test]
    fn tt() {
        let mut e = Executor::new(3);
        e.add_task(|| { for _ in 0..5 { println!("1 task"); } });
        e.add_task(|| { for _ in 0..5 { println!("2 task"); } });
        e.add_task(|| { for _ in 0..5 { println!("3 task"); } });
        
        let _ff = f;
        // e.add_task(ff);

        e.run();
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
