# IP Sniffer (Rust) 

<p align="left">  <img src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExMGF5Y3lyZWgweWd5eHVndDF2a3ZhYXZsMDB2dWtnd2tjcjlxYmhoeCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/1yMvhR4M47Okw4n8tt/giphy.gif" alt="sniffer" /> 
   </p>
This Rust application provides a simple yet effective tool for scanning ports on a given IP address. With multithreading support, it efficiently checks for open ports within the specified range.

## Features

- **Port Scanning**: Utilizes TCP connection attempts to scan for open ports.
- **Multithreading**: Distributes scanning tasks across multiple threads for faster execution.
- **Command-line Interface**: Accepts command-line arguments for specifying IP addresses, threads, and help messages.

## Usage

### Installation

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/your_username/ip-sniffer-rust.git
   
2. Navigate to the project directory:

   ```bash
   cd ip-sniffer-rust
   
3. Build the Application:

     ```bash
     cargo build

### Running
The application accepts the following command-line arguments:

- `-j <num_threads>`: Specify the number of threads for concurrent scanning.
- `-h` or `-help`: Display help message.
Example uses:
- Display help messages:-
  
   ```bash
   cargo run --h
- Scan ports on a specific IP address with default settings (4 threads):-

   ```bash
   cargo run 192.168.1.1  
- Scan ports on a specific IP address with a custom number of threads:

   ```bash
   cargo run 192.168.1.1 
### Output
The application provides a list of open ports found during the scan.

## Contributions
Contributions are welcome! Feel free to open issues or pull requests.
