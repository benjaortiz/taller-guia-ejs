use std::sync::mpsc::Receiver;
use std::{thread};
use std::sync::{Arc, Mutex, mpsc};



trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Task = Box<dyn FnBox + Send+ 'static>;

pub struct ThreadPool{
    thread_num:usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>
}


impl ThreadPool {
    /// Crea una nueva ThreadPool
    ///
    /// lanza panic si el 'thread_num' (la cantidad de 
    /// threads en la pool) es igual a 0
    pub fn new(thread_num:usize) -> ThreadPool {
        assert!(thread_num > 0);
        let (sender,receiver) = mpsc::channel::<Task>();

        let mut workers = Vec::with_capacity(thread_num);
        
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..thread_num{
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool{
            thread_num,
            workers,
            sender
        }
    }

    pub fn run<F>(&self, task:F)
        where F: FnOnce() + Send + 'static
    {
        let f = Box::new(task);

        self.sender.send(f).expect("no se pudo enviar la task");
    }

}


struct Worker{
    id:usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id:usize, task_list:Arc<Mutex<Receiver<Task>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop{
               let job = task_list.lock().unwrap().recv().unwrap();

               println!("worker {} recibió una task", id);

               job.call_box();
            }
        });
        
        Worker{
            id,
            thread,
        }
    }
}