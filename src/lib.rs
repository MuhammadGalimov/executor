pub struct Executor {
    tasks: Vec<fn()>
}

impl Executor {
    pub fn new() -> Self {
        Executor { tasks: vec![] }
    }

    pub fn add_task(&mut self, f: fn()) {
        self.tasks.push(f);
    }

    pub fn exec(&self) {
        for task in &self.tasks {
            (task)();
        }
    }
}

pub struct ExecutorBuilder {}

#[cfg(test)]
mod tests {
    use crate::*;

    fn f() -> i32 { 5 }

    #[test]
    fn tt() {
        let mut e = Executor::new();
        e.add_task(|| { println!("1 task"); });
        e.add_task(|| { println!("2 task"); });
        e.add_task(|| { println!("3 task"); });
        
        let _ff = f;
        // e.add_task(ff);

        e.exec();
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
