use http::{self, Server};


fn main() {
   let server: Server =  Server{ port: 8080, hostname: "localhost"};

   server.start();
   // s::init(, 8080);
}
