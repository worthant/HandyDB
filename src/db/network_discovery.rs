use std::net::{UdpSocket, SocketAddr};

pub struct NetworkDiscovery {
    socket: UdpSocket,
    broadcast_address: SocketAddr,
}

impl NetworkDiscovery {
    pub fn new(port: u16, broadcast_address: SocketAddr) -> Self {
        // Binding to "0.0.0.0" allows socket to receive messages sent to any of the network interfaces of our machine.
        let socket = UdpSocket::bind(("0.0.0.0", port)).expect("Unable to bind to UDP socket");
        socket.set_broadcast(true).expect("Unable to set broadcast for UDP socket");

        NetworkDiscovery {
            socket,
            broadcast_address,
        }
    }

    pub fn broadcast_presence(&self, message: &[u8]) {
        self.socket.send_to(message, self.broadcast_address).expect("Unable to send broadcast message");
    }

    pub fn listen_for_others(&self) -> Vec<u8> {
        let mut buffer = [0; 1024];
        match self.socket.recv_from(&mut buffer) {
            Ok((size, _)) => buffer[..size].to_vec(),
            Err(_) => vec![],
        }
    }
}
