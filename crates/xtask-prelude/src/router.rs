use std::env::Args;

use super::{error, exit, info, Result};

pub type Task = fn(std::env::Args) -> Result<()>;

#[derive(Default)]
pub struct TaskRouter {
    tasks: Vec<(String, Task)>,
}

impl TaskRouter {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn insert(mut self, name: impl AsRef<str>, task: Task) -> Self {
        let name = name.as_ref().to_owned();
        self.tasks.push((name, task));
        self
    }

    pub fn run(self) {
        let mut args = std::env::args();
        let _bin_name = args.next();
        self.run_with_args(args);
    }

    pub fn run_with_args(self, mut args: Args) {
        let task = match args.next().as_deref() {
            Some(task_name) => match self.tasks.iter().find(|(name, _)| name == task_name) {
                None => {
                    error!("task not found: {task_name:?}");
                    exit(1);
                }
                Some((name, task)) => {
                    info!("execute task: {name:?}");
                    task
                }
            },
            None => {
                info!("available tasks:");
                for task_name in self.tasks.iter().map(|item| item.0.as_str()) {
                    println!("{task_name}");
                }
                std::process::exit(0);
            }
        };

        if let Err(err) = task(args) {
            error!("Error: {err:?}");
            exit(1);
        }
    }
}
