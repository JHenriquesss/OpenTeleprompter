# Installing OpenPrompter RS

Thank you for downloading OpenPrompter RS!

This guide will help you install and run the app on your computer.

OpenPrompter RS is now at **v1.0.0** (first stable release). All builds are **unsigned**, which means your operating system may show a warning when you first run the app. This is expected for open-source software that hasn't paid for code-signing certificates. We explain how to handle it below.

The app **updates itself**: when a newer version is published, it shows an "Update available" banner you can install with one click (no reinstall). See [Automatic updates](#automatic-updates).

---

## Which file should I download?

Each release on GitHub includes several files. Here is which one to pick:

| You are using... | Download this file | Why |
|------------------|-------------------|-----|
| **Windows** (most users) | `*_x64-setup.exe` | The NSIS installer. Double-click and follow the prompts. Handles upgrades. |
| **Windows** (no install) | `*_portable.zip` | Extract and run — no installer, no registry changes. Good for USB drives. |
| **Windows** (enterprise) | `*_x64_en-US.msi` | The MSI package. Works with Group Policy. Note: reinstall may fail — see notes below. |
| **Linux** (any distro) | `*.AppImage` | Works on every Linux distribution. Make executable and run. No install needed. |
| **Linux** (Debian/Ubuntu) | `*.deb` | Installs system-wide. Run `sudo dpkg -i *.deb`. |
| **Linux** (Fedora/RHEL/openSUSE) | `*.rpm` | Installs system-wide. Run `sudo rpm -i *.rpm` (or `dnf install`). |
| **macOS** (Apple Silicon) | `*_aarch64.dmg` | For M1/M2/M3/M4 Macs. Open the DMG and drag to Applications. |

> **Intel macOS:** a native Intel (`x86_64`) DMG is added in a follow-up release (v1.0.1). On Apple Silicon, use the `aarch64` DMG.

**Not sure?** Download the `*_x64-setup.exe` (Windows) or `*.AppImage` (Linux). They work for most people.

---

## Verifying file integrity (optional)

Each release now includes **SHA256 checksum files**. A checksum is a unique digital fingerprint of a file. You can use it to confirm the file you downloaded is exactly the same as the one we published — no corruption, no tampering.

The release contains per-platform checksum files:
- `SHA256SUMS-windows.txt` for Windows files
- `SHA256SUMS-linux.txt` for Linux files (AppImage, deb, RPM)
- `SHA256SUMS-macos-aarch64.txt` (and `-macos-x64.txt` once Intel ships) for macOS files

**How to verify (optional):**

Download the artifact and the matching `SHA256SUMS-*` file into the same folder first.

**Windows (PowerShell):**
```powershell
Get-FileHash .\OpenPrompter.RS_*.msi
```
Compare the output hash with the one in `SHA256SUMS-windows.txt`.

**Linux / macOS:**
```bash
# Download the artifact and checksum file, place them in the same directory
sha256sum --check SHA256SUMS-linux.txt
```

Both commands check every file listed in the checksum file and print "OK" for each match. If a file is corrupted or missing, `sha256sum --check` will warn you.

> Checksum verification is entirely optional. If you just want to use the app, you can skip this step.

---

## Which operating system are you using?

- [Windows](#windows)
- [Linux](#linux)
- [macOS](#macos)

---

## Windows

### Option 1: Setup Installer (recommended for most users)

1. Go to the [GitHub Releases page](https://github.com/JHenriquesss/OpenTeleprompter/releases).
2. Find the latest release (`v1.0.0` or newer).
3. Download the file ending in `-setup.exe` (for example, `OpenPrompter.RS_1.0.0_x64-setup.exe`).
4. Double-click the downloaded file to run the installer.
5. Follow the installer prompts.

**About the SmartScreen warning:**

When you run the installer, Windows may show a blue warning that says "Windows protected your PC" or "Microsoft Defender SmartScreen prevented an unrecognized app from starting."

This happens because the installer is **unsigned** (we haven't paid for a code signing certificate). It does **not** mean the app is unsafe.

To continue:
- Click **More info** (or "More information").
- Click **Run anyway**.
- The installer will proceed normally.

The app is 100% open-source. You can [inspect the source code](https://github.com/JHenriquesss/OpenTeleprompter) yourself.

### Option 2: MSI Installer (for system administrators)

If you need to deploy the app across multiple machines, download the `.msi` file instead. This is the Windows Installer package.

- Download `OpenPrompter.RS_1.0.0_x64_en-US.msi`.
- Double-click to install, or deploy via Group Policy.

### Option 3: Portable ZIP (no installation required)

If you prefer not to install, or want to run the app from a USB drive:

1. Download the `*_portable.zip` file.
2. Extract the ZIP to any folder.
3. Double-click `OpenPrompter RS.exe`.

Your scripts and settings are saved in the same folder. Nothing is written to the registry or system folders.

---

## Linux

### Option 1: AppImage (recommended — works on any Linux distribution)

1. Go to the [GitHub Releases page](https://github.com/JHenriquesss/OpenTeleprompter/releases).
2. Find the latest release.
3. Download the `.AppImage` file (for example, `OpenPrompter.RS_1.0.0_amd64.AppImage`).
4. Open a terminal in the download folder.
5. Make the file executable:

   ```bash
   chmod +x OpenPrompter.RS_*.AppImage
   ```

6. Run it:

   ```bash
   ./OpenPrompter.RS_*.AppImage
   ```

That's it! No installation needed. The AppImage contains everything the app needs.

**Tip:** You can move the AppImage to any folder (like `~/Applications`) and create a shortcut to it.

### Option 2: Debian/Ubuntu package (.deb)

If you use Debian, Ubuntu, or a derivative (like Mint, Pop!_OS, etc.):

1. Download the `.deb` file (for example, `OpenPrompter.RS_1.0.0_amd64.deb`).
2. Open a terminal in the download folder.
3. Install the package:

   ```bash
   sudo dpkg -i OpenPrompter.RS_*.deb
   ```

4. Launch OpenPrompter RS from your app menu.

**Note:** Linux packages are not signed or added to official repositories. Install them with the commands above.

---

## macOS

1. Go to the [GitHub Releases page](https://github.com/JHenriquesss/OpenTeleprompter/releases).
2. Find the latest release.
3. Download the `.dmg` file (for example, `OpenPrompter.RS_1.0.0_aarch64.dmg`).
4. Double-click the `.dmg` file to open it.
5. Drag the `OpenPrompter RS` app into your `Applications` folder.

**About the Gatekeeper warning:**

The first time you open the app, macOS may show a warning: _"OpenPrompter RS cannot be opened because the developer cannot be verified."_

This happens because the app is **unsigned** (we haven't enrolled in the Apple Developer Program for notarization). It does **not** mean the app is unsafe.

To open it anyway:

- **Option A:** Right-click (or Control+click) the app in the Applications folder, then click **Open**. Select **Open** in the dialog that appears. You only need to do this once.
- **Option B:** If the above doesn't work, open Terminal and run:

  ```bash
  xattr -dr com.apple.quarantine /Applications/OpenPrompter\ RS.app
  ```

After that, the app will open normally.

---

## Automatic updates

OpenPrompter RS checks for updates on launch. When a newer version is available,
an **"Update available"** banner appears at the top of the window with **Install**
and **Dismiss** buttons. Installing downloads the update, applies it, and
relaunches — no manual reinstall. Updates are **never** installed silently; you
always choose. If you're offline or already up to date, nothing happens.

Updates are cryptographically verified (minisign) before they install.

## System tray

The app lives in your system tray:

- **Closing the window (X)** hides it to the tray — the app keeps running. A
  one-time hint reminds you it's still there.
- **Left-click** the tray icon to show/hide the window.
- **Right-click** the tray icon for **Show / Hide / Quit**. Use **Quit** to exit
  the app completely.

---

## Is it safe?

Yes. Here is why you can trust OpenPrompter RS:

- **Open source** — The entire source code is public on [GitHub](https://github.com/JHenriquesss/OpenTeleprompter). Anyone can review it.
- **No cloud** — The app runs entirely offline. Your scripts never leave your computer.
- **No accounts** — No registration, no login, no account creation.
- **No telemetry** — The app does not track you, collect data, or phone home.
- **No payment** — Free forever. No in-app purchases, no subscriptions.
- **No JavaScript** — The frontend is built in Rust, compiled to WebAssembly, not JavaScript.

The unsigned warning from your operating system is simply because we haven't paid for code signing certificates — a common situation for open-source projects. The safety of this app is backed by its open-source nature, not by a paid certificate.

---

## System requirements

| Platform | Minimum |
|----------|---------|
| Windows | Windows 10 or newer (x64) |
| Linux | Any distribution with WebKit2GTK 4.1 (Ubuntu 22.04+, Fedora 38+, etc.) |
| macOS | macOS 11 (Big Sur) or newer, Apple Silicon or Intel |

## Still have questions?

Open an issue on [GitHub](https://github.com/JHenriquesss/OpenTeleprompter/issues) — we are happy to help.
