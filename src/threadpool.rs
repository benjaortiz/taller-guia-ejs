use std::{thread};



pub struct ThreadPool{
    thread_num:usize,
    workers: Vec<Worker>
}


impl ThreadPool {
    /// Crea una nueva ThreadPool
    ///
    /// lanza panic si el 'thread_num' (la cantidad de 
    /// threads en la pool) es igual a 0
    pub fn new(thread_num:usize) -> ThreadPool {
        assert!(thread_num > 0);
        
        let mut workers = Vec::with_capacity(thread_num);

        for i in 0..thread_num{
            workers.push(Worker::new(i));
        }
        ThreadPool{
            thread_num,
            workers
        }
    }

    pub fn run<F>(&self, task:F)
        where F: FnOnce() + Send + 'static
    {

    }

}

struct Worker{
    id:usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id:usize) -> Worker {
        let thread = thread::spawn(|| {});
        
        Worker{
            id,
            thread
        }
    }
}



