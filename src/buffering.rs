use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Configuration
    let encoder_port = 5004;
    let decoder_ip = "192.168.25.89";  // Decoder IP
    let decoder_port = 5004;           // Decoder Port
    let buffer_time = Duration::from_millis(1000); // Buffering time (1 second)
    let wait_time = Duration::from_millis(2000);   // Wait time (2 seconds)

    let listen_addr = format!("0.0.0.0:{}", encoder_port); // IP and port for receiving from Encoder
    let decoder_addr = format!("{}:{}", decoder_ip, decoder_port); // Decoder address

    // Bind to the encoder's IP and port to receive packets
    let socket = UdpSocket::bind(&listen_addr).await?;
    println!("Listening on: {}", listen_addr);

    let mut buffer = Vec::new();
    let mut packet_buf = vec![0u8; 1500]; // Buffer for incoming packets

    loop {
        // 1. WaitTime stage: Receiving and forwarding directly
        let wait_end = tokio::time::Instant::now() + wait_time;

        while tokio::time::Instant::now() < wait_end {
            match socket.recv_from(&mut packet_buf).await {
                Ok((len, addr)) => {
                    // Forward packet directly
                    let _ = socket.send_to(&packet_buf[..len], &decoder_addr).await?;
                }
                Err(e) => eprintln!("Error receiving packet: {}", e),
            }
        }

        // 2. Buffering stage: Collect packets for bufferTime duration
        println!("Buffering packets for {:?}", buffer_time);
        let buffer_end = tokio::time::Instant::now() + buffer_time;
        buffer.clear(); // Clear buffer from previous cycle

        while tokio::time::Instant::now() < buffer_end {
            match socket.recv_from(&mut packet_buf).await {
                Ok((len, addr)) => {
                    // Buffer packet
                    buffer.push(packet_buf[..len].to_vec());
                }
                Err(e) => eprintln!("Error receiving packet: {}", e),
            }
            
        }


        // Log the time when the last packet in the buffer is collected
        let end_buffer_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        println!("Buffering ended at: {:.3} seconds since UNIX epoch", end_buffer_time);

        // 3. Sending buffered packets in a burst to Decoder
        // Log the time when the first buffered packet is sent
        if let Some(first_packet) = buffer.first() {
            let start_sending_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
            println!("Sending first buffered packet to Decoder started at: {:.3} seconds since UNIX epoch", start_sending_time);

            // Calculate and display the time difference
            let time_difference = start_sending_time - end_buffer_time;
            println!("Time difference between end of buffering and start of sending first packet: {:.3} seconds", time_difference);

            for packet in &buffer {
                let _ = socket.send_to(packet, &decoder_addr).await?;
            }
        }
    }
}
