# t-meter Documentation

Welcome to the official documentation for **t-meter**, a terminal-based day progress bar.

## Table of Contents

1.  [Introduction](#introduction)
2.  [Installation](#installation)
3.  [Usage](#usage)
4.  [Configuration](#configuration)
5.  [Themes](#themes)
6.  [Troubleshooting](#troubleshooting)

---

## Introduction

**t-meter** is a simple, calm progress bar for your day, running right in your terminal. It helps you visualize the passage of time, stay grounded, and manage your day better.

### Key Features

-   **Visual Day Progress**: A progress bar that fills up as the day goes by.
-   **Markers**: Indicators for Wake Up time, Noon, and Bed Time.
-   **Zen Quotes**: Hourly quotes to help you pause and reflect.
-   **Themes**: Multiple beautiful themes with light and dark modes.
-   **Customizable**: Configurable wake/bed times and styles.

---

## Installation

### macOS / Linux (Homebrew)

```bash
brew tap jordan-jakisa/tap
brew install t-meter
```

### Standalone Binary

Download the latest release for your platform from the [GitHub Releases](https://github.com/jordan-jakisa/t-meter/releases) page.

### From Source

Requirements: Rust and Cargo installed.

```bash
git clone https://github.com/jordan-jakisa/t-meter.git
cd t-meter
cargo install --path t-meter
```

---

## Usage

Run the application from your terminal:

```bash
t-meter
```

### Keybindings

| Key             | Action                                                       |
| :-------------- | :----------------------------------------------------------- |
| `q` or `Ctrl+c` | Quit the application                                         |
| `t`             | Cycle through available themes                               |
| `d`             | Toggle between light and dark mode                           |
| `s`             | Cycle through progress bar styles (Gradient, Grainy, Analog) |
| `w`             | Edit Wake Up time                                            |
| `b`             | Edit Bed Time                                                |
| `h`             | Show Help screen                                             |
| `?`             | Open documentation                                           |

### Editing Times

1.  Press `w` to edit Wake Up time or `b` to edit Bed Time.
2.  Type the new time in `HH:MM` format (24-hour).
3.  Press `Enter` to save or `Esc` to cancel.

---

## Configuration

**t-meter** can be configured via a TOML file.

### File Location

-   **Linux/macOS**: `~/.config/t-meter/config.toml`
-   **macOS (alternative)**: `~/Library/Application Support/t-meter/config.toml`

If the file doesn't exist, **t-meter** will generate a default one for you upon first run.

### Configuration Options

| Option               | Type   | Default   | Description                                        |
| :------------------- | :----- | :-------- | :------------------------------------------------- |
| `theme_name`         | String | "default" | The active theme name.                             |
| `theme_mode`         | String | "light"   | The active mode ("light" or "dark").               |
| `progress_bar_style` | String | "Analog"  | Style of the bar ("Gradient", "Grainy", "Analog"). |
| `wake_up_time`       | String | "07:00"   | Your wake up time in HH:MM.                        |
| `bed_time`           | String | "23:00"   | Your bed time in HH:MM.                            |

### Example Config

```toml
theme_name = "ocean"
theme_mode = "dark"
progress_bar_style = "Gradient"
wake_up_time = "06:30"
bed_time = "22:30"
```

---

## Themes

**t-meter** includes several built-in themes:

-   **default**: Clean monochrome design.
-   **ocean**: Calming blue and teal tones.
-   **forest**: Natural green and earth colors.
-   **sunset**: Warm orange and pink hues.
-   **monochrome**: Pure grayscale aesthetic.
-   **contrast**: High contrast (Black/White/Blue/Red/Yellow).

Cycle through them using the `t` key.

---

## Troubleshooting

### Config file not found

If **t-meter** cannot find your config file, it will use default values. Check the [Configuration](#configuration) section for the expected file paths.

### Colors look wrong

Ensure your terminal supports TrueColor (24-bit color). Most modern terminals (iTerm2, Alacritty, Kitty, VS Code) support this out of the box.

### "Invalid format" when editing time

Ensure you are entering the time strictly in `HH:MM` 24-hour format (e.g., `09:05`, `14:30`).
