// single.rs
// for handling single HTTP connection

use std::net::TcpListener;
use crate::request::Request;

pub fn single_server(stream: &TcpListener, root_path: String) {
    /* Accept and handle HTTP request */
    loop {
    	/* Accept request */
        let mut r = Request::new();
        r.accept_request(stream);

	    /* Handle request */
        r.handle_request(root_path.to_string());
    }

    /* Close server socket */
	// this will happen automatically as Rust will drop the TcpListener once it is
    // out of scope

    //return EXIT_SUCCESS;
    // return a result most likely
}
