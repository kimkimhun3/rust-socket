use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    // Define the IP and port to listen for incoming packets (from Encoder)
    let encoder_ip = "0.0.0.0"; // Replace with specific IP if needed
    let encoder_port = 5004;    // Replace with actual Encoder port
    let listen_addr = format!("{}:{}", encoder_ip, encoder_port);

    // Define the IP and port to forward packets to (Decoder)
    let decoder_ip = "192.168.25.89"; // Replace with Decoder IP
    let decoder_port = 5004;          // Replace with Decoder port
    let decoder_addr = format!("{}:{}", decoder_ip, decoder_port);

    // Bind to the encoder's IP and port to receive packets
    let socket = UdpSocket::bind(&listen_addr)?;
    println!("Listening on: {}", listen_addr);

    let mut buf = [0u8; 1500]; // Buffer to hold incoming packets

    loop {
        // Receive packet from the encoder
        let (len, src) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", len, src);

        // Forward the packet to the decoder
        let bytes_sent = socket.send_to(&buf[..len], &decoder_addr)?;
        println!("Forwarded {} bytes to {}", bytes_sent, decoder_addr);
    }
}
