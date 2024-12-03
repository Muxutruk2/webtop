Webtop
======
A lightweight, Rust-powered web server for real-time system monitoring.

![Preview](https://i.imgur.com/ZODGxOl.png)

## Features
- **Memory**: Available, usage, and swap statistics.
- **CPU**: Core count, usage percentage, load averages, and processor name.
- **GPU**: Graphics card name, memory usage, and temperature.
- **System**: Hostname, kernel version, operating system, and uptime.
- **Networks**: Active interfaces with received and transmitted data stats.
- **Processes**: Details like PID, name, CPU/RAM usage, virtual memory, status, and runtime.---

## Setup

Make sure you have a valid OS installed (not windows) and rust.

---

## Usage
After you clone this repo to your desktop, go to its root directory and run `cargo run` to run the server. The default port is 3000

---

## Contributing

We welcome contributions of all kinds, from bug fixes to new features!  

### Steps to Contribute

1. **Fork the Repository**  

   ```bash
   git clone https://github.com/Muxutruk2/webtop.git
   cd webtop
   ```

2. **Create a Branch**  

Use a descriptive name for your branch:  
```bash
git checkout -b feature/your-feature-name
```

3. **Make Changes**  
Implement your changes and ensure they align with Rust best practices.  

4. **Test Your Changes**  

Ensure all functionality is working and the code is ready to merge by using `cargo clippy`

5. **Submit a Pull Request (PR)**  
Push your branch and open a PR to the main repository. Clearly describe your changes and link any related issues.  

---

### Contribution Guidelines

- Write clear, concise commit messages.  
- Update documentation if your changes affect usage.  
- Be respectful and constructive in discussions.

---

## License

This project is licensed under the terms of the **MIT License**.  
See the full license [here](https://github.com/Muxutruk2/webtop/blob/master/LICENSE).
