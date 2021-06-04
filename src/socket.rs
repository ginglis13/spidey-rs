// socket.rs
// create and return socket file descriptor

use std::net::{TcpListener, SocketAddr};

/**
 * Allocate socket, bind it, and listen to specified port.
 * All this is handled by TcpListener::bind
 *
 * @param   port        Port number to bind to and listen on.
 * @return  listener    TcpListener struct for accepting connection.
 **/
pub fn socket_listen(port: u16) -> std::io::Result<std::net::TcpListener> {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port)))?;

    Ok(listener)
}
