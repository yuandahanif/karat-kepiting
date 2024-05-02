use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

pub mod listener {
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread::{self, JoinHandle},
    };

    struct Worker {
        id: usize,
        thread: Option<JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
            let builder = thread::Builder::new();
            let thread = builder
                .spawn(move || loop {
                    let message = receiver.lock().expect("can't lock mutex").recv();

                    match message {
                        Ok(job) => {
                            println!("Worker {id} got a job; executing.");
                            job();
                        }
                        Err(err) => {
                            eprintln!("no more sender!, {}", err);
                            break;
                        }
                    }
                })
                .unwrap();

            Worker {
                id,
                thread: Some(thread),
            }
        }
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }

    impl ThreadPool {
        pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
            if size <= 0 {
                return Err("can' create 0 thread");
            }

            let mut workers = Vec::with_capacity(size);
            let (sender, receiver) = mpsc::channel::<Job>();
            let receiver = Arc::new(Mutex::new(receiver));

            for index in 0..size {
                workers.push(Worker::new(index, receiver.clone()));
            }

            Ok(ThreadPool {
                workers,
                sender: Some(sender),
            })
        }

        pub fn execute<F>(&self, job: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(job);
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.sender.take());

            for worker in &mut self.workers {
                println!("Shutting down worker {}", worker.id);
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }
}

pub fn init(addr: impl ToSocketAddrs) -> TcpListener {
    let listener = TcpListener::bind(addr).expect("can't bind port");
    listener
}

pub fn html_content_parser(path: &str) -> (String, usize) {
    let contents = fs::read_to_string(path).unwrap();
    let content_length = contents.len();
    (contents, content_length)
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);

    if let Some(request_line) = http_request.get(0) {
        if request_line == "GET / HTTP/1.1" {
            let response = "HTTP/1.1 200 OK";
            let (contents, content_length) = html_content_parser("html/hello.html");
            let response = format!("{response}\r\nContent-Length: {content_length}\r\nContent-type: text/html\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let (contents, content_length) = html_content_parser("html/404.html");
            let response = format!("{status_line}\r\nContent-Length: {content_length}\r\nContent-type: text/html\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
