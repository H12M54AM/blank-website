# NYC Community Explorer OS 🗽

Welcome to **NYC Community Explorer**, a highly interactive, desktop-OS style web application built entirely in Rust and WebAssembly using the [Dioxus](https://dioxuslabs.com/) framework.

This project pulls live, public event data directly from the **NYC Parks Open Data API**, allowing users to explore free activities, outdoor markets, and community gatherings across all boroughs in an incredibly unique, window-based interface.

## ✨ Features

*   **Desktop OS Paradigm:** Features drag-and-drop draggable windows, maximizing/minimizing controls, and a fully functional task dock on mobile.
*   **True Glassmorphism:** CSS-driven frosted glass UI (`backdrop-filter`) that beautifully refracts dynamic backgrounds.
*   **Settings Engine:** Fully functional Settings app featuring:
    *   **Theme Toggling:** Seamless switching between Light and Dark mode.
    *   **Wallpaper Picker:** Instantly swap between 7 stunning, curated Pexels nature wallpapers.
    *   **Live World Clock:** Real-time ticking clocks tracking global timezones (Vancouver, NYC, London, Tokyo, Sydney) using async WASM timers.
*   **Data Aggregation:** Parses live JSON from the NYC Parks API, sorts by name/date, removes duplicates, and paginates into clean 12-item chunks.
*   **Smart Routing Fallbacks:** Automatically generates keyword searches on both the NYC Parks website and Google to help users secure tickets, avoiding broken API links.
*   **Mobile Responsive:** Intelligently drops the desktop paradigm on narrow screens (< 600px) in favor of auto-maximized windows and a bottom navigation dock.

## 🛠 Tech Stack

*   **Language:** Rust 🦀
*   **Framework:** Dioxus (WebAssembly / Frontend)
*   **Styling:** Vanilla CSS3 (Custom Properties, Grid, Flexbox, Media Queries)
*   **Dependencies:**
    *   `reqwest` (API Fetching)
    *   `serde` & `serde_json` (JSON Parsing)
    *   `chrono` & `chrono-tz` (Timezone Management)
    *   `gloo-timers` (Async WebAssembly intervals)

## 🚀 Getting Started

### Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed, along with the Dioxus CLI:

```bash
cargo install dioxus-cli
```

### Running Locally

Clone the repository and serve it locally:

```bash
git clone https://github.com/yourusername/nyc-community-explorer.git
cd nyc-community-explorer
dx serve --open
```

The Dioxus CLI will compile the Rust code to WebAssembly and open your default browser to `http://localhost:8080`.

## 🔒 Security & Privacy

This application was engineered with a "Security First" philosophy by a Digital Forensics & Cybersecurity graduate. 
*   **Memory Safety:** Built in Rust to natively prevent common memory-based web vulnerabilities.
*   **Zero Tracking:** Absolutely no Personally Identifiable Information (PII) or geolocation data is collected, stored, or processed.
*   **Client-Side Processing:** All API sorting, filtering, and pagination occurs entirely within the client's browser memory.

## ⚖️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

*Disclaimer: The event data visualised in this app belongs to the City of New York via the NYC Open Data initiative. This app serves solely as an exploratory interface.*
