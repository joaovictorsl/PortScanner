# PortScanner

PortScanner allows you to scan open ports on a given host. It provides a simple and straightforward way to identify open ports.

## Features

- Scans a single host for open ports.
- Supports scanning multiple ports simultaneously.
- Allows customization of the number of threads for concurrent scanning.
- Reports open ports.

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/joaovictorsl/PortScanner.git
   ```

2. Change into the project directory:

   ```
   cd PortScanner
   ```

## Usage

To run the PortScanner, use the following command:

```
cargon run -- [options] <host>
```

Replace `<host>` with the target host. The following options are available:

- `-t`: Set the number of threads to use for concurrent scanning. Default is 4 threads.
- `-h, --help`: Display the help message and exit.

## Examples

1. Scan a host for open TCP ports:

   ```
   cargo run -- 192.168.0.1
   ```

2. Choose number of threads when scanning a host for open TCP ports:

   ```
   cargo run -- -t 1000 192.168.0.1
   ```
