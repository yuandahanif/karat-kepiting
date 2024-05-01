mod handler;

pub mod hello_server {
    use crate::handler;

    pub fn start() {
        let listener = handler::init("127.0.0.1:7878");

        let pool = handler::listener::ThreadPool::new(10).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(|| handler::handle_connection(stream));
        }
    }
}
