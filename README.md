# Blaze House 🚀

**Blaze House** is a modern, feature-rich system control and optimization center designed primarily for **BlazeOS (Debian & Fedora series)** and other Linux distributions. It offers an intuitive GUI to monitor hardware, optimize system performance, manage storage, and configure advanced Linux features like ZRAM and power profiles effortlessly.
<img width="1234" height="683" alt="image" src="https://github.com/user-attachments/assets/36a43878-c27e-44ae-b5ea-a29b3aa674aa" />


---

## ✨ Features

* **📊 Live System Status & Diagnostics:** Real-time hardware monitoring including CPU usage, RAM, NVMe/SSD storage, GPU details, temperatures, and uptime.
<img width="1234" height="683" alt="image" src="https://github.com/user-attachments/assets/77771a5e-24c6-4c6f-b55d-425676a72fbe" />

* **⚡ System Optimization & Tweaks:**
  * **Privacy Controls:** Disable file history, app usage collection, and location services.
  * **Performance Boosts:** Toggle UI animations, tune `vm.swappiness` (e.g., swappiness=10), and optimize directory caching (`vm.vfs_cache_pressure`).
* **🧠 ZRAM Management:** Configure compressed memory swap dynamically. Adjust ZRAM swap size and select compression algorithms (e.g., `zstd`, `lz4`) with `pkexec` integration.
<img width="1280" height="808" alt="image" src="https://github.com/user-attachments/assets/bcb69eb6-7032-4b49-8eb3-7ad8cb4f9a0c" />

* **🧹 Storage & Junk Cleaner:** Analyze disk usage, inspect mount points, and clean APT package caches, thumbnail caches, and user application caches (`~/.cache`).
* **🔋 Power & Battery Health:** Monitor battery wear/health, view power supply status, and switch dynamically between Power Saver, Balanced, and Performance power profiles.
* **🌐 Multi-Language Support:** Instant switching between English, Turkish, German, French, and Spanish.
* **🔄 System & Ecosystem Updates:** Integrated version checking for BlazeOS, FireHub, and system repositories.

---

## 🖥️ System Requirements & Support

* **Target OS:** BlazeOS (Debian & Fedora editions)
* **Compatible Distros:** Ubuntu, Debian, Fedora, and Debian/RPM-based Linux systems.
* **Dependencies:** `apt` / `dnf` package managers, `pkexec` (for root privileges), ZRAM kernel module.
