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

## 📋 System Requirements

This document lists the requirements for **Blaze House** to launch and run properly.

> **Note:** Blaze House comes bundled with an embedded `venv` — meaning PyQt6 and Python dependencies are included inside the package. You do not need to install additional Python packages on your system. The requirements listed below belong to the system/graphics stack that cannot be embedded into a virtual environment.

---

================================================================================
Blaze House — System Requirements
================================================================================

This document lists the prerequisites required to launch and run Blaze House.

Blaze House is distributed with an embedded virtual environment (venv) — meaning 
PyQt6 and Python dependencies are bundled within the package. You do NOT need 
to install additional Python packages on your system. 

The requirements listed below belong to the system/graphics stack and cannot 
be embedded into a Python venv.

--------------------------------------------------------------------------------
1. MANDATORY (Application will NOT launch without these)
--------------------------------------------------------------------------------

Component                      Reason Required              Debian/Ubuntu Package       Fedora Package
----------------------------   --------------------------   -------------------------   ---------------------------
glibc 2.31+                    Core runtime                 (Present in system)         (Present in system)
Qt6 platform plugin (xcb)      Window rendering on X11      libxcb-cursor0              xcb-util-cursor
                                                            libxkbcommon-x11-0          libxkbcommon-x11
Core X11/Wayland libraries     Graphics surface             libxcb1                     libxcb
                                                            libxkbcommon0               libxkbcommon
Active Desktop Session         To render GUI                X11 or Wayland session      X11 or Wayland session

* NOTE: Because Qt platform plugins (xcb/wayland) rely directly on the system's 
  graphics stack, they cannot be bundled inside a Python venv. Thus, they are 
  mandatory system dependencies. On any modern desktop installation 
  (GNOME, KDE, XFCE, etc.), these are already installed by default.

Quick Installation (if missing):

  # Debian / Ubuntu
  sudo apt install libxcb-cursor0 libxkbcommon-x11-0

  # Fedora
  sudo dnf install xcb-util-cursor libxkbcommon-x11


--------------------------------------------------------------------------------
2. OPTIONAL (If missing, the feature becomes "Unavailable"; app still runs)
--------------------------------------------------------------------------------

Blaze House dynamically probes for system tools at runtime using `shutil.which`. 
If a tool is missing, the app simply hides or disables the corresponding 
feature without crashing.

Tool                            Target Feature                             Behavior if Missing
----------------------------    ---------------------------------------    ---------------------------------------------
pkexec (Polkit)                 All elevated tasks (services, ZRAM, etc.)  Privileged operations fail; read-only works
gsettings (GNOME)               Privacy & performance tweaks, animations   These tweaks become "Unavailable"
systemctl (systemd)             Service management (Bluetooth, CUPS, etc.) Service cards are hidden
apt / apt-get OR dnf            Package cache cleaning, orphan cleanup     Related cleaning actions won't execute
journalctl (systemd)            System log shrinking                       Log cleaning action won't execute
sysctl                          swappiness / vfs_cache tuning              Performance tweaks will be disabled
powerprofilesctl                Power profile switching (Battery tab)      Power profile card shows "Not Found"
xdg-mime                        Default application detection              Defaults show as "Not Set"
lsblk, lspci                    Hardware inventory (System details)        Some hardware info lines remain blank
zramctl / /sys/block/zram0      ZRAM status and configuration              ZRAM page displays "Unavailable"


* Recommendation for BlazeOS Build:
  For the best out-of-the-box experience, include the following packages in the ISO:
  polkit (pkexec), systemd, power-profiles-daemon, util-linux (lsblk/zramctl),
  pciutils (lspci), xdg-utils (xdg-mime). If GNOME-based, gsettings is included by default.


--------------------------------------------------------------------------------
3. HARDWARE & ENVIRONMENT
--------------------------------------------------------------------------------

- Architecture: x86_64 (Packages are compiled specifically for this architecture)
- Storage: ~250 MB (Installed size including embedded venv + PyQt6)
- RAM: Minimum ~200 MB free (Required for Qt GUI); edition detection doesn't impact this
- Display: Graphical session required (Will NOT run in headless TTY — this is a GUI app)


--------------------------------------------------------------------------------
SUMMARY: MINIMAL RUNTIME CONDITION
--------------------------------------------------------------------------------

On any modern desktop Linux distribution (GNOME / KDE / XFCE on X11 or Wayland), 
Blaze House launches out-of-the-box without installing additional packages. 
Only ultra-minimal or headless base systems might lack the required Qt xcb libraries. 

As more optional tools are detected, more features automatically unlock. Even if 
NONE of the optional tools exist, the application will still open safely and 
display basic system and hardware diagnostics.
