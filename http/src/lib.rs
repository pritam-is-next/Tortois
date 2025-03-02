
mod server_handler;

pub struct Server {
    pub port: i16,
    pub hostname: &'static str,
}

impl Server {
    
    pub fn start(&self)  {
        server_handler::init(&self.hostname, self.port);
    }
}