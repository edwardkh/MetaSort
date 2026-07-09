![macOS](https://img.shields.io/badge/macOS-blue?style=flat-square&logo=apple&logoColor=white)
&nbsp;
![Windows](https://img.shields.io/badge/Windows-blue?style=flat-square&logo=microsoft&logoColor=white)
&nbsp;
![Linux](https://img.shields.io/badge/Linux-blue?style=flat-square&logo=linux&logoColor=white)
&nbsp;

[![Donate via UPI](https://img.shields.io/badge/Donate-UPI-blue?logo=googlepay&style=for-the-badge)](https://upier.vercel.app/pay/sanmith@superyes)
[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue?logo=paypal&style=for-the-badge)](https://www.paypal.com/paypalme/iamsanmith)
[![Donate via Ko-fi](https://img.shields.io/badge/Donate-Ko--fi-blue?logo=kofi&style=for-the-badge)](https://ko-fi.com/L3L01JGIUW)


<div align="center">
  <img src="assets/logo.png" alt="MetaSort Logo" width="120"/>
  <h1>MetaSort v1.1.0</h1>
  <h3>🚀 Google Photos Takeout Organizer</h3>
  <p><em>Transform your messy Google Photos Takeout into beautifully organized media libraries!</em></p>
</div>

---

## 🎯 What is MetaSort?

**MetaSort** is your all-in-one solution for organizing Google Photos Takeout exports (or any messy media folder). It's lightning-fast, user-friendly, and works on both macOS and Windows.

### ✨ What MetaSort Does:
- 🧹 **Cleans up filenames** and removes .json clutter
- 📅 **Extracts dates** from filenames, JSON metadata, or file timestamps
- 🏷️ **Embeds metadata** (date, camera, GPS) directly into your photos/videos
- 📦 **Sorts everything** into organized folders by year/month/type
- 💬 **Separates WhatsApp & Screenshots** (optional)
- 📊 **Generates beautiful reports** (CSV + HTML)
- 🎨 **Beautiful UI** with progress bars and emoji-rich feedback
- ⚡ **Multithreaded processing** for blazing-fast performance via Rayon
- 📱 **iOS Live Photo pairing** — keeps .HEIC/.JPG + .MP4/.MOV pairs together
- 🔗 **Cross-media extension claiming** — proactively reserves paired filenames to prevent collisions

---

## 🚀 Quick Start (5 Minutes)

### For Non-Technical Users:

#### macOS:
1. **Download MetaSort** from GitHub
2. **Open Terminal** and run:
   ```bash
   ./scripts/build_macos.sh
   ```
3. **Double-click** `Run_MetaSort.command` to start!

#### Windows:
1. **Download MetaSort** from GitHub
2. **Right-click** `scripts/install_windows.bat` → "Run as administrator"
3. **Follow the prompts** - it will install everything automatically!

### For Developers:
```bash
# Clone and build
git clone https://github.com/edwardkh/MetaSort.git
cd MetaSort
cargo build --release
cargo run --release
```

---

## 💙 Support & Donations

<div align="center">

  <!-- UPI Donation -->
  <a href="https://upier.vercel.app/pay/sanmith@superyes" target="_blank">
    <img 
      src="https://img.shields.io/badge/Donate%20via-UPI-green?style=for-the-badge&logo=google-pay" 
      alt="Donate via UPI"
    />
  </a>
  <br/><br/>

  <!-- PayPal Donation -->
  <a href="https://www.paypal.com/paypalme/iamsanmith" target="_blank">
    <img 
      src="https://raw.githubusercontent.com/stefan-niedermann/paypal-donate-button/master/paypal-donate-button.png" 
      alt="Donate with PayPal" 
      width="200"
    />
  </a>
  <br/><br/>

  <!-- Ko-fi Donation -->
  <a href="https://ko-fi.com/L3L01JGIUW" target="_blank">
    <img 
      height="36" 
      style="border:0px;height:36px;" 
      src="https://storage.ko-fi.com/cdn/kofi6.png?v=6" 
      border="0" 
      alt="Buy Me a Coffee at ko-fi.com"
    />
  </a>

  <br/><br/>
  <em>
    If MetaSort saved you hours, please consider supporting the project!  
    Every contribution, no matter how small, makes a difference and helps keep MetaSort free and actively maintained.
  </em>

</div>



---

## 📋 Requirements

### System Requirements:
- **macOS 10.13+**, **Windows 10+**, or **Linux**
- **4GB RAM** (recommended)
- **500MB free space** for the application

### Dependencies:
- **ExifTool** - For metadata extraction and embedding
- **Rust** - For building the application

> 💡 **Don't worry!** Our installation scripts handle all dependencies automatically.

---

## 🛠️ Detailed Installation

### macOS Installation

#### Option 1: Automated (Recommended)
```bash
# Download and extract MetaSort
git clone hhttps://github.com/edwardkh/MetaSort.git
cd MetaSort

# Build and create launchers
./scripts/build_macos.sh
```

#### Option 2: Manual Installation
```bash
# 1. Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Press 1 when prompted, then restart Terminal

# 3. Install ExifTool
brew install exiftool

# 4. Build MetaSort
git clone hhttps://github.com/edwardkh/MetaSort.git
cd MetaSort
cargo build --release
```

### Windows Installation

#### Option 1: Automated (Recommended)
```cmd
# Run PowerShell installer
powershell -ExecutionPolicy Bypass -File scripts/install_windows.ps1

# Or run batch installer
scripts/install_windows.bat
```

#### Option 2: Manual Installation
```cmd
# 1. Install Rust from https://rustup.rs/

# 2. Install ExifTool
winget install ExifTool.ExifTool

# 3. Build MetaSort
git clone hhttps://github.com/edwardkh/MetaSort.git
cd MetaSort
cargo build --release
```

---

## 🎮 How to Use MetaSort

### Step 1: Launch MetaSort
- **macOS**: Double-click `Run_MetaSort.command`
- **Windows**: Run `cargo run --release` or use the generated executable

### Step 2: Select Your Folder
- **Drag and drop** your Google Photos Takeout folder
- Or **type the path** to your media folder
- MetaSort works with any folder containing photos/videos!

### Step 3: Choose Options
- **Separate WhatsApp/Screenshots?** (Recommended: Yes)
- **Metadata embedding method** (Recommended: Auto-detect)
- **Output directory** (Default: `MetaSort_Output`)

### Step 4: Watch the Magic! ✨
MetaSort will:
1. 🔍 Scan your files
2. 🧹 Clean up filenames
3. 📅 Extract dates
4. 🏷️ Embed metadata
5. 📦 Sort into folders
6. 📊 Generate reports

### Step 5: Enjoy Your Organized Media! 🎉
- **Photos/Videos**: `MetaSort_Output/Media Files/`
- **Reports**: `MetaSort_Output/Technical Files/`
- **HTML Summary**: Open `MetaSort_Output/Technical Files/report.html`

---

## 📁 Output Structure

After processing, you'll find:

```
MetaSort_Output/
├── Media Files/
│   ├── 2023/
│   │   ├── 01_January/
│   │   │   ├── Photos/
│   │   │   ├── Videos/
│   │   │   └── Screenshots/
│   │   └── 02_February/
│   └── 2024/
├── Technical Files/
│   ├── report.html          # Beautiful summary report
│   ├── processing_log.csv   # Detailed processing log
│   ├── metadata_summary.csv # Metadata statistics
│   └── error_log.txt        # Any issues encountered
└── Original Files/          # Backup of original structure
```

---

## 🎯 Supported File Types

### Media Files:
- **Photos**: JPG, JPEG, PNG, WEBP, HEIC, HEIF, BMP, TIFF, GIF, AVIF, JXL, JFIF
- **Raw Formats**: RAW, CR2, NEF, ORF, SR2, ARW, DNG, PEF, RAF, RW2, SRW, 3FR, ERF, K25, KDC, MEF, MOS, MRW, NRW, SRF, X3F
- **Design Files**: SVG, ICO, PSD, AI, EPS
- **Videos**: MP4, MOV, MKV, AVI, WEBM, 3GP, M4V, MPG, MPEG, MTS, M2TS, TS, FLV, F4V, WMV, ASF, RM, RMVB, VOB, OGV, MXF, DV, DIVX, XVID

### Metadata Sources:
- **JSON files** (Google Photos metadata)
- **Filename patterns** (WhatsApp, Screenshots, etc.)
- **EXIF data** (embedded in files — `EXIF:DateTimeOriginal` set on all supported formats)
- **File timestamps** (fallback)

### iOS Live Photo Pairing:
MetaSort intelligently detects iOS Live Photo pairs (`.HEIC`/`.JPG`/`.JPEG` + `.MP4`/`.MOV`) and ensures:
- Both components are sorted into the **same destination folder**
- Both receive the **same base filename** if a naming collision occurs
- The video component is never orphaned into the "Unknown Time" folder
- Cross-media extensions are proactively claimed to prevent collisions from other years

---

## 📅 Smart Date Detection

MetaSort can extract dates from countless filename patterns:

### 📱 Mobile Apps:
- **WhatsApp**: `IMG-20220101-WA0001.jpg` → `2022:01:01 00:00:00`
- **Screenshots**: `Screenshot_2023-01-01-12-00-00.png` → `2023:01:01 12:00:00`
- **Telegram**: `photo_2023-01-01 12.00.00.jpg` → `2023:01:01 12:00:00`

### 📷 Cameras & Phones:
- **Samsung/Android**: `20230101_123456.jpg` → `2023:01:01 12:34:56`
- **Google Photos**: `PXL_20230101_123456789.jpg` → `2023:01:01 12:34:56`
- **Sony Camera**: `DSC01234_20230101_123456.JPG` → `2023:01:01 12:34:56`
- **MIUI**: `IMG_20230101_120000.jpg` → `2023:01:01 12:00:00`

### 🎯 Custom Patterns:
- `wallpaper - IMG_20240113_143213Jan 13 2024` → `2024:01:13 14:32:13`
- `San-1 Oct 2024.jxl` → `2024:10:01 00:00:00`
- `RMLmc20250531_115820_RMlmc.7` → `2025:05:31 11:58:20`

> 💡 **MetaSort is smart!** If your filename contains a date, it will likely find it!

---

## 🛠️ Advanced Features

### ⚡ Multithreaded Processing
MetaSort uses [Rayon](https://github.com/rayon-rs/rayon) for parallel processing, enabling:
- **Parallel file cleaning** — JSON filename cleanup runs across all CPU cores
- **Parallel metadata embedding** — ExifTool operations run concurrently
- **Parallel file sorting** — folder creation and file moves are parallelized
- Automatic thread-pool sizing based on available CPU cores

### 🔧 Command Line Options
```bash
# Run with specific options
cargo run --release -- --help

# Process specific folder
cargo run --release -- --input "/path/to/folder"

# Custom output directory
cargo run --release -- --output "/path/to/output"
```

### 📊 Report Customization
- **HTML Report**: Beautiful web-based summary with statistics
- **CSV Reports**: Detailed logs for spreadsheet analysis
- **Error Logs**: Track any issues during processing

### 🔄 Batch Processing
- Process multiple folders
- Resume interrupted operations
- Skip already processed files

### 🛡️ Robust ExifTool Integration
- Minor ExifTool warnings are automatically suppressed to prevent false failures
- `EXIF:DateTimeOriginal` is set on **all** supported file formats (not just photos)
- Graceful error handling for unsupported or corrupted files

---

## 🆘 Troubleshooting

### Common Issues:

#### "ExifTool not found"
**macOS:**
```bash
brew install exiftool
```

**Windows:**
```cmd
winget install ExifTool.ExifTool
```

#### "Permission denied"
**macOS:**
```bash
chmod +x scripts/build_macos.sh
```

**Windows:**
- Right-click script → "Run as administrator"

#### "Rust not found"
**macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
- Download from https://rustup.rs/

#### "App won't open" (macOS)
- Use `Run_MetaSort.command` instead of app bundles
- Right-click → Open (if needed)

### Getting Help:
1. Check the **error logs** in `MetaSort_Output/Technical Files/`
2. Ensure **ExifTool is installed** and accessible
3. Try **running in terminal** for detailed error messages
4. **Open an issue** on GitHub with error details

---

## 🏗️ Project Structure

```
MetaSort/
├── 📁 src/                    # Source code
│   ├── main.rs               # Main application & multithreaded orchestration
│   ├── platform.rs           # Cross-platform compatibility
│   ├── ui.rs                 # User interface & progress bars
│   ├── media_cleaning.rs     # File cleaning & organization (multithreaded)
│   ├── metadata_extraction.rs # JSON metadata extraction
│   ├── metadata_embed.rs     # Metadata embedding (EXIF:DateTimeOriginal)
│   ├── sort_to_folders.rs    # File sorting, folder creation & Live Photo pairing
│   ├── csv_report.rs         # CSV report generation
│   ├── html_report.rs        # HTML report generation
│   ├── filename_date_guess.rs # Date extraction from filenames
│   └── utils.rs              # Utility functions
├── 📁 scripts/               # Build & installation scripts
│   ├── build_macos.sh        # macOS build script
│   ├── build_windows.bat     # Windows build script
│   ├── install_windows.ps1   # Windows installer (PowerShell)
│   └── install_windows.bat   # Windows installer (Batch)
├── 📁 docs/                  # Documentation
│   ├── SIMPLE_INSTALL.md     # Non-technical user guide
│   └── CROSS_PLATFORM_CHANGES.md # Technical details
├── 📁 assets/                # Resources
│   ├── logo.png              # MetaSort logo
│   └── upi.png               # UPI QR code
├── 🚀 Run_MetaSort.command   # Easy launcher (macOS)
├── 🚀 MetaSort.command       # Advanced launcher (macOS)
├── 📄 README.md              # This file
├── 📄 LICENSE.txt            # Apache 2.0 License
└── 📄 Cargo.toml             # Rust project configuration (includes rayon)
```

---

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### 🐛 Report Bugs
1. Check existing issues first
2. Provide detailed error messages
3. Include your OS and MetaSort version

### 💡 Suggest Features
1. Describe the feature clearly
2. Explain why it would be useful
3. Consider implementation complexity

### 🔧 Submit Code
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

### 📝 Documentation
- Improve README sections
- Add examples
- Fix typos or unclear instructions

---

## 📄 License

MetaSort is licensed under the **Apache License 2.0** - see the [LICENSE.txt](LICENSE.txt) file for details.

This means you can:
- ✅ Use MetaSort for personal or commercial projects
- ✅ Modify and distribute MetaSort
- ✅ Use MetaSort in proprietary software
- ✅ Distribute modified versions

**Requirements:**
- Include the original license and copyright notice
- State any changes you made

---

## 🏆 Acknowledgments

- **ExifTool** - For powerful metadata handling
- **Rayon** - For effortless data parallelism in Rust
- **Rust Community** - For the amazing ecosystem
- **All Contributors** - For making MetaSort better
- **You** - For using and supporting MetaSort!

---

## 📝 Changelog (since v1.0.0)

### v1.1.0 — Performance & Live Photo Improvements
- ⚡ **Multithreading**: Added Rayon-based parallel processing for file cleaning, metadata embedding, and sorting
- 📱 **iOS Live Photo pairing**: `.HEIC`/`.JPG`/`.JPEG` + `.MP4`/`.MOV` pairs are kept together during sorting
- 🔗 **Cross-media extension claiming**: Proactively reserves paired filenames to prevent collisions across years
- 🏷️ **Universal EXIF:DateTimeOriginal**: All supported file formats now receive `EXIF:DateTimeOriginal` metadata
- 🛡️ **ExifTool robustness**: Minor warnings are now suppressed to prevent false failures
- 📁 **Unknown timestamp folder**: Re-added fallback folder for files with no determinable date
- 🔧 **Filename collision fix**: `.HEIC` and `.MP4` components from the same source folder receive identical base filenames on collision
- 🧹 **Multithreaded JSON cleanup**: `clean_json_filenames` now runs in parallel for faster processing

---

<div align="center">
  <h3>🎉 Ready to organize your photos?</h3>
  <p><strong>Get started with MetaSort today!</strong></p>
  <a href="#-quick-start-5-minutes">
    <strong>🚀 Quick Start Guide</strong>
  </a>
</div>

---

<div align="center">
  <sub>
    Made with ❤️ by <a href="https://github.com/iamsanmith">Sanmith S</a>
    <br/>
    <em>Transform your digital memories into organized treasures!</em>
  </sub>
</div> 
