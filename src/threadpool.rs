use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Task = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Crea una nueva ThreadPool
    ///
    /// lanza panic si el 'thread_num' (la cantidad de
    /// threads en la pool) es igual a 0
    pub fn new(thread_num: usize) -> ThreadPool {
        assert!(thread_num > 0);
        let (sender, receiver) = mpsc::channel::<Message>();

        let mut workers = Vec::with_capacity(thread_num);

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..thread_num {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn run<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let f = Box::new(task);

        self.sender
            .send(Message::NewJob(f))
            .expect("no se pudo enviar la task");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Time to terminate the working class");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }

            println!("Worker[{}]: *se muere*", worker.id);
        }
    }
}

enum Message {
    NewJob(Task),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, task_list: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = task_list.lock().unwrap().recv().unwrap();

            match msg {
                Message::NewJob(job) => {
                    println!("worker {} recibió una task", id);

                    job.call_box();
                }
                Message::Terminate => {
                    println!("despidieron al worker {} ", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
