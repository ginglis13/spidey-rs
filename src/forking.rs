// forking.rs
// for handling multiple incoming HTTP connections
// fork each connection into new process

use std::net::TcpListener;
use crate::request::Request;
use nix::unistd::{fork, ForkResult};

pub fn forking_server(stream: &TcpListener, root_path: String) {
    /* Accept and handle HTTP request */
    loop {
        
    	/* Accept request */
        let mut r = Request::new();
        r.accept_request(stream);

	    /* Handle request by forking into new process */
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                // In the original project, parent process frees the request struct
                // Not needed since Rust will drop values once out of scope
            }
            Ok(ForkResult::Child) => {
                log::debug!("Handling request in child process...");
                r.handle_request(root_path.to_string());
            }
            Err(_) => log::error!("Fork failed."),
         }      
    }

    /* Close server socket */
	// this will happen automatically as Rust will drop the TcpListener once it is
    // out of scope

    //return EXIT_SUCCESS;
    // return a result most likely? still tbd
}
