#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::Deserialize;
use chrono::Utc;
use std::collections::HashSet;
use gloo_timers::future::sleep;
use std::time::Duration;

// ----------------------------------------------------------------------------
// ASSETS & STATIC FILES
// ----------------------------------------------------------------------------
// We load our CSS and images here so WebAssembly knows where to find them.
const MAIN_CSS: Asset = asset!("/assets/main.css");
const OG_IMAGE: Asset = asset!("/assets/og-image.jpg");

// All 7 Wallpapers
const WP_1: Asset = asset!("/assets/pexels-alex-blokstra-300443-1327354.jpg");
const WP_2: Asset = asset!("/assets/pexels-andres-figueroa-3326883-38048122.jpg");
const WP_3: Asset = asset!("/assets/pexels-egojane-9049273.jpg");
const WP_4: Asset = asset!("/assets/pexels-josh-fields-2290258-3964379.jpg");
const WP_5: Asset = asset!("/assets/pexels-oskar-gross-1074333632-38912437.jpg");
const WP_6: Asset = asset!("/assets/pexels-vladalex94-1461370.jpg");
const WP_7: Asset = asset!("/assets/pexels-vraj-shah-115200-924831.jpg");

// ----------------------------------------------------------------------------
// DATA STRUCTURES
// ----------------------------------------------------------------------------
// This defines what an "Event" looks like when we download it from the NYC database.
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct NycEvent {
    event_name: Option<String>,
    start_date_time: Option<String>,
    event_location: Option<String>,
    event_borough: Option<String>,
    event_type: Option<String>, // Added to pull category data
}

// This tracks which "Folder" or "App" is currently open on the screen.
#[derive(Clone, Copy, PartialEq)]
enum WindowState {
    None,
    Home,
    Trending,
    New,
    Classic,
    Settings,
}

// A helper function to give a clean title to the window based on what's open.
impl WindowState {
    fn title(&self) -> &'static str {
        match self {
            WindowState::None => "",
            WindowState::Home => "Welcome Home",
            WindowState::Trending => "Trending Events",
            WindowState::New => "New Events",
            WindowState::Classic => "Classic Events",
            WindowState::Settings => "Settings",
        }
    }
}

// ----------------------------------------------------------------------------
// APP BOOTSTRAP
// ----------------------------------------------------------------------------
// This is the starting point of the entire application.
fn main() {
    dioxus::launch(App);
}

// ----------------------------------------------------------------------------
// MAIN APPLICATION COMPONENT
// ----------------------------------------------------------------------------
#[component]
fn App() -> Element {
    // --- Global State ---
    
    // Window Management
    let mut active_window = use_signal(|| WindowState::Home);
    let mut is_maximized = use_signal(|| false);
    
    // Theme & Appearance (Default wallpaper is WP_5 / Oskar Gross)
    let theme = use_signal(|| "light".to_string());
    let wallpaper = use_signal(|| WP_5);

    // Drag-and-Drop Physics
    let mut window_offset = use_signal(|| (0.0, 0.0));
    let mut is_dragging = use_signal(|| false);
    let mut last_mouse = use_signal(|| (0.0, 0.0));

    // The event currently selected by the user to view in the popup modal
    let selected_event: Signal<Option<NycEvent>> = use_signal(|| None);

    // --- Data Fetching ---
    // Connects to the official NYC Parks database and downloads the latest 500 events.
    let events_resource = use_resource(move || async move {
        let url = "https://data.cityofnewyork.us/resource/tvpp-9vvx.json?$limit=500&$order=start_date_time%20DESC";
        match reqwest::get(url).await {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<Vec<NycEvent>>().await {
                        Ok(data) => {
                            // Filter out duplicates so the user only sees unique events
                            let mut unique_events = Vec::new();
                            let mut seen_names = HashSet::new();
                            for event in data {
                                let name = event.event_name.clone().unwrap_or_default();
                                if !name.is_empty() && !seen_names.contains(&name) {
                                    seen_names.insert(name);
                                    unique_events.push(event);
                                }
                            }
                            Ok(unique_events)
                        },
                        Err(_) => Err("Failed to parse the database information.".to_string()),
                    }
                } else {
                    Err("The NYC database is currently down. Try again later.".to_string())
                }
            }
            Err(_) => Err("No internet connection.".to_string()),
        }
    });

    // --- User Interface Rendering ---
    rsx! {
        // Website Meta Tags (for SEO and sharing links)
        document::Title { "NYC Parks Community Events | Free Activities & Outdoor Markets" }
        document::Meta { name: "description", content: "Discover free community events, outdoor markets, and weekend family activities across the NYC Parks system." }
        document::Meta { property: "og:image", content: OG_IMAGE }
        document::Link { rel: "icon", href: OG_IMAGE, r#type: "image/jpeg" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        // The "Master Root" wraps the whole screen. It tracks the mouse so you can drag windows smoothly.
        div { 
            class: "master-root",
            "data-theme": theme(),
            style: "background-image: url({wallpaper()});",
            onmousemove: move |evt| {
                if is_dragging() {
                    let current_x = evt.client_coordinates().x;
                    let current_y = evt.client_coordinates().y;
                    
                    let dx = current_x - last_mouse().0;
                    let dy = current_y - last_mouse().1;
                    
                    let mut offset = window_offset();
                    offset.0 += dx;
                    offset.1 += dy;
                    window_offset.set(offset);
                    
                    last_mouse.set((current_x, current_y));
                }
            },
            onmouseup: move |_| { is_dragging.set(false); }, // Stop dragging when the mouse is let go
            onmouseleave: move |_| { is_dragging.set(false); },

            // --- The Desktop Grid ---
            nav { class: "desktop-grid", aria_label: "Desktop Folders",
                FolderIconHome { title: "Home", tint: "#e0f2e9", state: WindowState::Home, active_window }
                FolderIcon { title: "Trending", tint: "#e3f2fd", state: WindowState::Trending, active_window }
                FolderIcon { title: "New", tint: "#fff3e0", state: WindowState::New, active_window }
                FolderIcon { title: "Classic", tint: "#f5f5dc", state: WindowState::Classic, active_window }
                
                // Settings App Icon
                button { 
                    class: "folder-container",
                    onclick: move |_| active_window.set(WindowState::Settings),
                    div { class: "folder-icon", style: "background-color: #f0f0f0;",
                        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24",
                            path { d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" }
                            path { d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z" }
                        }
                    }
                    div { class: "folder-label", "Settings" }
                }

                // External Link: LinkedIn
                a {
                    class: "folder-container external-link",
                    href: "https://linkedin.com/in/edwardnaidoo",
                    target: "_blank",
                    div { class: "folder-icon", style: "background-color: #e8f5e9;",
                        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24",
                            path { d: "M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" }
                        }
                    }
                    div { class: "folder-label", "LinkedIn" }
                }

                // External Link: Portfolio Website
                a {
                    class: "folder-container external-link",
                    href: "https://edwardcreates.ca",
                    target: "_blank",
                    div { class: "folder-icon", style: "background-color: #f3e5f5;",
                        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24",
                            path { d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z" }
                        }
                    }
                    div { class: "folder-label", "Browser" }
                }
            }

            // --- The Glass Window ---
            if active_window() != WindowState::None {
                div { 
                    class: if is_maximized() { "os-window glass maximized" } else { "os-window glass" },
                    style: if is_maximized() { "".to_string() } else { format!("transform: translate(calc(-50% + {}px), calc(-50% + {}px));", window_offset().0, window_offset().1) },
                    
                    div { 
                        class: "window-header",
                        onmousedown: move |evt| {
                            if !is_maximized() {
                                is_dragging.set(true); 
                                last_mouse.set((evt.client_coordinates().x, evt.client_coordinates().y));
                            }
                        },
                        div { class: "window-controls",
                            button { 
                                class: "win-btn close", 
                                onclick: move |evt| { 
                                    evt.stop_propagation(); 
                                    active_window.set(WindowState::None); 
                                    is_maximized.set(false); 
                                },
                                aria_label: "Close Window"
                            }
                            button { 
                                class: "win-btn max", 
                                onclick: move |evt| {
                                    evt.stop_propagation(); 
                                    is_maximized.set(!is_maximized());
                                },
                                aria_label: "Maximize Window"
                            }
                        }
                        div { class: "window-title", "{active_window().title()}" }
                    }

                    main { 
                        class: "window-content",
                        role: "main",
                        // Swap out what goes inside the window based on what app is open
                        match active_window() {
                            WindowState::Home => rsx! {
                                div { class: "content-text", style: "text-align: center; margin-top: 2rem;",
                                    h1 { "NYC Community Explorer" }
                                    p { "Welcome! This tool helps you discover the best local events, outdoor markets, and community gatherings happening across New York City today." }
                                    
                                    h2 { style: "margin-top: 3rem; font-size: 1.8rem;", "How to use this tool:" }
                                    div { style: "text-align: left; max-width: 600px; margin: 0 auto; line-height: 1.8; font-size: 1.15rem;",
                                        p { "Think of this website like a computer desktop. You have folders on your screen, and when you click them, they open up into windows just like this one." }
                                        p { strong { "1. Moving Windows: " } "If this window is in your way, you can click and hold the top bar (where it says 'Welcome Home') and drag it around your screen." }
                                        p { strong { "2. Making it Bigger: " } "On a computer or tablet, click the green circle in the top left corner to make this window take up the whole screen. On a phone, it will automatically fit perfectly!" }
                                        p { strong { "3. Closing a Window: " } "When you are done reading, click the red circle in the top left corner to close this window." }
                                        p { strong { "4. Exploring Events: " } "Click the 'Trending' or 'New' folders on your desktop to see what's happening around New York City. You can also click the quick links below to jump straight to them." }
                                    }
                                    
                                    div { style: "display: flex; flex-direction: row; flex-wrap: wrap; justify-content: center; gap: 1.5rem; margin-top: 2rem;",
                                        a { 
                                            href: "#",
                                            style: "font-size: 1.2rem; font-weight: bold; cursor: pointer;",
                                            onclick: move |evt| { evt.prevent_default(); active_window.set(WindowState::Trending); },
                                            "Explore Trending Events"
                                        }
                                        a { 
                                            href: "#",
                                            style: "font-size: 1.2rem; font-weight: bold; cursor: pointer;",
                                            onclick: move |evt| { evt.prevent_default(); active_window.set(WindowState::New); },
                                            "Explore New Events"
                                        }
                                        a { 
                                            href: "#",
                                            style: "font-size: 1.2rem; font-weight: bold; cursor: pointer;",
                                            onclick: move |evt| { evt.prevent_default(); active_window.set(WindowState::Classic); },
                                            "Explore Classic Events"
                                        }
                                        a { 
                                            href: "#",
                                            style: "font-size: 1.2rem; font-weight: bold; cursor: pointer;",
                                            onclick: move |evt| { evt.prevent_default(); active_window.set(WindowState::Settings); },
                                            "Open Settings"
                                        }
                                    }
                                }
                            },
                            WindowState::Settings => rsx! {
                                SettingsApp { theme, wallpaper }
                            },
                            WindowState::Trending | WindowState::New | WindowState::Classic => rsx! { 
                                EventViewer { 
                                    events_resource, 
                                    category: active_window(),
                                    selected_event 
                                } 
                            },
                            _ => rsx! {}
                        }
                    }
                }
            }

            // --- The Event Details Popup ---
            if let Some(event) = selected_event() {
                EventModal { event, selected_event }
            }

            div { class: "desktop-footer",
                "© 2026 edwardcreates.ca"
            }
        }
    }
}

// ----------------------------------------------------------------------------
// HELPER COMPONENTS
// ----------------------------------------------------------------------------

#[component]
fn FolderIcon(title: &'static str, tint: &'static str, state: WindowState, active_window: Signal<WindowState>) -> Element {
    rsx! {
        button { 
            class: "folder-container",
            aria_label: "{title} Folder",
            onclick: move |_| active_window.set(state),
            div { class: "folder-icon", aria_hidden: "true", style: "background-color: {tint};",
                svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24",
                    path { d: "M2 6a2 2 0 012-2h5.586a1 1 0 01.707.293l2.414 2.414a1 1 0 00.707.293h6.586a2 2 0 012 2v9a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" }
                }
            }
            div { class: "folder-label", "{title}" }
        }
    }
}

#[component]
fn FolderIconHome(title: &'static str, tint: &'static str, state: WindowState, active_window: Signal<WindowState>) -> Element {
    rsx! {
        button { 
            class: "folder-container",
            aria_label: "Home Folder",
            onclick: move |_| active_window.set(state),
            div { class: "folder-icon", aria_hidden: "true", style: "background-color: {tint};",
                svg { xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 24 24",
                    path { d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6", stroke: "currentColor", stroke_width: "2", fill: "none", stroke_linecap: "round", stroke_linejoin: "round" }
                }
            }
            div { class: "folder-label", "{title}" }
        }
    }
}

// ----------------------------------------------------------------------------
// SETTINGS APP
// ----------------------------------------------------------------------------
#[component]
fn SettingsApp(theme: Signal<String>, wallpaper: Signal<Asset>) -> Element {
    let mut active_tab = use_signal(|| "appearance");

    rsx! {
        div { class: "settings-layout",
            div { class: "settings-sidebar",
                button { 
                    class: if active_tab() == "appearance" { "settings-tab active" } else { "settings-tab" },
                    onclick: move |_| active_tab.set("appearance"),
                    "Appearance"
                }
                button { 
                    class: if active_tab() == "world_clock" { "settings-tab active" } else { "settings-tab" },
                    onclick: move |_| active_tab.set("world_clock"),
                    "World Clock"
                }
                button { 
                    class: if active_tab() == "tos" { "settings-tab active" } else { "settings-tab" },
                    onclick: move |_| active_tab.set("tos"),
                    "TOS & Privacy"
                }
            }
            
            div { class: "settings-body",
                match active_tab() {
                    "appearance" => rsx! {
                        div { class: "content-text", style: "padding: 0;",
                            h2 { "Appearance" }
                            p { "Customize your OS desktop." }
                            
                            h3 { "Theme" }
                            button {
                                class: "theme-toggle",
                                onclick: move |_| {
                                    if theme() == "light" { theme.set("dark".to_string()) } else { theme.set("light".to_string()) }
                                },
                                if theme() == "light" { "Switch to Dark Mode 🌙" } else { "Switch to Light Mode ☀️" }
                            }

                            h3 { style: "margin-top: 2rem;", "Wallpaper" }
                            div { class: "wallpaper-grid",
                                button { class: if wallpaper() == WP_1 { "wallpaper-thumb active" } else { "wallpaper-thumb" }, style: "background-image: url({WP_1});", onclick: move |_| wallpaper.set(WP_1) }
                                button { class: if wallpaper() == WP_2 { "wallpaper-thumb active" } else { "wallpaper-thumb" }, style: "background-image: url({WP_2});", onclick: move |_| wallpaper.set(WP_2) }
                                button { class: if wallpaper() == WP_3 { "wallpaper-thumb active" } else { "wallpaper-thumb" }, style: "background-image: url({WP_3});", onclick: move |_| wallpaper.set(WP_3) }
                                button { class: if wallpaper() == WP_4 { "wallpaper-thumb active" } else { "wallpaper-thumb" }, style: "background-image: url({WP_4});", onclick: move |_| wallpaper.set(WP_4) }
                                button { class: if wallpaper() == WP_5 { "wallpaper-thumb active" } else { "wallpaper-thumb" }, style: "background-image: url({WP_5});", onclick: move |_| wallpaper.set(WP_5) }
                                button { class: if wallpaper() == WP_6 { "wallpaper-thumb active" } else { "wallpaper-thumb" }, style: "background-image: url({WP_6});", onclick: move |_| wallpaper.set(WP_6) }
                                button { class: if wallpaper() == WP_7 { "wallpaper-thumb active" } else { "wallpaper-thumb" }, style: "background-image: url({WP_7});", onclick: move |_| wallpaper.set(WP_7) }
                            }
                        }
                    },
                    "world_clock" => rsx! {
                        WorldClockTab {}
                    },
                    "tos" => rsx! {
                        div { class: "content-text", style: "padding: 0;",
                            h2 { "Terms of Service & Privacy Policy" }
                            p { strong { "Jurisdiction: British Columbia, Canada" } }
                            p { "Last Updated: August 2026" }
                            
                            h3 { "1. Scope of Service & Data Accuracy" }
                            p { "This application serves as an aggregator and visualization tool for public events sourced directly from the NYC Parks Open Data API. We do not manually curate, verify, or alter the contents of these events." }
                            p { "Users must exercise their own discretion. Event times, locations, and safety protocols are determined by third parties and the City of New York. We accept no liability for cancellations, injuries, inaccuracies, or incidents resulting from attempting to attend these events. Use this application purely as an exploratory guide." }

                            h3 { "2. Zero Tracking & No PII" }
                            p { "Your privacy is an absolute priority. We do not store any Personally Identifiable Information (PII) or geolocation data. All filtering and sorting operations occur locally on your device within the browser memory." }

                            h3 { "3. Security First Design" }
                            p { "This platform was engineered by a Digital Forensics & Cybersecurity graduate. It operates entirely on secure protocols (HTTPS), utilizes memory-safe Rust compilation to prevent common web vulnerabilities, and enforces strict Content Security Policies. There are no user accounts, meaning there are no passwords or sensitive data to breach." }
                            
                            h3 { "4. Copyright & Fair Use" }
                            p { "This software interface and OS design are copyright © 2026 edwardcreates.ca. However, the event data remains the property of its respective owners via the NYC Open Data initiative. By using this software, you agree to comply with Canadian copyright laws operating under the jurisdiction of British Columbia." }
                        }
                    },
                    _ => rsx! {}
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// WORLD CLOCK WIDGET
// ----------------------------------------------------------------------------
#[component]
fn WorldClockTab() -> Element {
    let mut current_utc = use_signal(|| Utc::now());
    
    use_future(move || async move {
        loop {
            sleep(Duration::from_secs(1)).await;
            current_utc.set(Utc::now());
        }
    });

    let now = current_utc();
    
    let tz_van = chrono_tz::America::Vancouver;
    let tz_ny = chrono_tz::America::New_York;
    let tz_lon = chrono_tz::Europe::London;
    let tz_tok = chrono_tz::Asia::Tokyo;
    let tz_syd = chrono_tz::Australia::Sydney;

    let van_time = now.with_timezone(&tz_van).format("%I:%M:%S %p").to_string();
    let van_date = now.with_timezone(&tz_van).format("%b %e, %Y").to_string();
    
    let ny_time = now.with_timezone(&tz_ny).format("%I:%M:%S %p").to_string();
    let ny_date = now.with_timezone(&tz_ny).format("%b %e, %Y").to_string();
    
    let lon_time = now.with_timezone(&tz_lon).format("%I:%M:%S %p").to_string();
    let lon_date = now.with_timezone(&tz_lon).format("%b %e, %Y").to_string();
    
    let tok_time = now.with_timezone(&tz_tok).format("%I:%M:%S %p").to_string();
    let tok_date = now.with_timezone(&tz_tok).format("%b %e, %Y").to_string();
    
    let syd_time = now.with_timezone(&tz_syd).format("%I:%M:%S %p").to_string();
    let syd_date = now.with_timezone(&tz_syd).format("%b %e, %Y").to_string();

    rsx! {
        div { class: "content-text", style: "padding: 0;",
            h2 { "World Clock" }
            p { "Live functional tracking of major global cities." }
            
            div { class: "clock-grid",
                div { class: "clock-card glass", div { "Vancouver" } div { class: "clock-time", "{van_time}" } div { class: "clock-date", "{van_date}" } }
                div { class: "clock-card glass", div { "New York" } div { class: "clock-time", "{ny_time}" } div { class: "clock-date", "{ny_date}" } }
                div { class: "clock-card glass", div { "London" } div { class: "clock-time", "{lon_time}" } div { class: "clock-date", "{lon_date}" } }
                div { class: "clock-card glass", div { "Tokyo" } div { class: "clock-time", "{tok_time}" } div { class: "clock-date", "{tok_date}" } }
                div { class: "clock-card glass", div { "Sydney" } div { class: "clock-time", "{syd_time}" } div { class: "clock-date", "{syd_date}" } }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// EVENT VIEWER APP (Trending, New, Classic Folders)
// ----------------------------------------------------------------------------
#[component]
fn EventViewer(
    events_resource: Resource<Result<Vec<NycEvent>, String>>, 
    category: WindowState,
    selected_event: Signal<Option<NycEvent>>
) -> Element {
    let mut current_page = use_signal(|| 0);
    let mut sort_order = use_signal(|| "date_desc".to_string());
    let mut last_category = use_signal(|| category);
    
    if last_category() != category {
        current_page.set(0);
        last_category.set(category);
    }

    match &*events_resource.read_unchecked() {
        Some(Ok(raw_events)) => {
            if raw_events.is_empty() {
                rsx! { div { class: "loading", "No events available at this time." } }
            } else {
                let mut processed = raw_events.clone();

                match category {
                    WindowState::New => {},
                    WindowState::Classic => { processed.reverse(); }, 
                    WindowState::Trending => {
                        processed.sort_by(|a, b| {
                            let len_a = a.event_name.as_ref().map(|s| s.len()).unwrap_or(0);
                            let len_b = b.event_name.as_ref().map(|s| s.len()).unwrap_or(0);
                            len_b.cmp(&len_a)
                        });
                    },
                    _ => {}
                }

                if sort_order() == "name_asc" {
                    processed.sort_by(|a, b| a.event_name.cmp(&b.event_name));
                } else if sort_order() == "name_desc" {
                    processed.sort_by(|a, b| b.event_name.cmp(&a.event_name));
                }

                let items_per_page = 12;
                let total_pages = (processed.len() as f64 / items_per_page as f64).ceil() as usize;
                let start_idx = current_page() * items_per_page;
                let end_idx = usize::min(start_idx + items_per_page, processed.len());
                let page_items = &processed[start_idx..end_idx];

                rsx! {
                    div { class: "command-bar",
                        div { class: "command-group",
                            span { "Sort by:" }
                            select { 
                                class: "command-select",
                                onchange: move |evt| sort_order.set(evt.value()),
                                option { value: "date_desc", "Default" }
                                option { value: "name_asc", "Name (A-Z)" }
                                option { value: "name_desc", "Name (Z-A)" }
                            }
                        }
                        div { class: "pagination-controls",
                            button { 
                                class: "page-btn", 
                                disabled: current_page() == 0,
                                onclick: move |_| current_page.set(current_page() - 1),
                                "Prev"
                            }
                            span { "Page {current_page() + 1} of {total_pages}" }
                            button { 
                                class: "page-btn", 
                                disabled: current_page() + 1 >= total_pages,
                                onclick: move |_| current_page.set(current_page() + 1),
                                "Next"
                            }
                        }
                    }

                    div { class: "event-grid",
                        for event in page_items {
                            EventCard { 
                                event: event.clone(),
                                on_click: move |e| selected_event.set(Some(e)) 
                            }
                        }
                    }
                }
            }
        }
        Some(Err(e)) => {
            rsx! { div { class: "error-view", h2 { "Unable to Load Events" } p { "{e}" } } }
        }
        None => {
            rsx! { div { class: "loading", "Fetching unique community events..." } }
        }
    }
}

// ----------------------------------------------------------------------------
// SINGLE EVENT CARD
// ----------------------------------------------------------------------------
#[component]
fn EventCard(event: NycEvent, on_click: EventHandler<NycEvent>) -> Element {
    let title = event.event_name.clone().unwrap_or_else(|| "Untitled Event".to_string());
    
    let start_dt = event.start_date_time.clone().unwrap_or_default();
    let display_date = if let Ok(parsed_date) = chrono::NaiveDateTime::parse_from_str(&start_dt, "%Y-%m-%dT%H:%M:%S%.f") {
        parsed_date.format("%b %e, %Y %l:%M %p").to_string() 
    } else {
        if start_dt.is_empty() { "Date TBD".to_string() } else { start_dt }
    };
    
    // Truncate location data so it doesn't spill over
    let mut location = event.event_location.clone().unwrap_or_else(|| "Various Locations".to_string());
    if location.len() > 40 {
        location.truncate(37);
        location.push_str("...");
    }
    
    let borough = event.event_borough.clone().unwrap_or_default();

    rsx! {
        article { 
            class: "event-card glass", 
            aria_label: "Event: {title}",
            onclick: move |_| on_click.call(event.clone()),
            
            div { class: "event-title", "{title}" }
            div { class: "event-date", "📅 {display_date}" }
            div { class: "event-location", "📍 {location}" }
            if !borough.is_empty() {
                div { class: "event-badge", "{borough}" }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// POPUP EVENT MODAL
// ----------------------------------------------------------------------------
#[component]
fn EventModal(event: NycEvent, selected_event: Signal<Option<NycEvent>>) -> Element {
    let title = event.event_name.unwrap_or_else(|| "Untitled Event".to_string());
    let start_dt = event.start_date_time.unwrap_or_default();
    
    let display_date = if let Ok(parsed_date) = chrono::NaiveDateTime::parse_from_str(&start_dt, "%Y-%m-%dT%H:%M:%S%.f") {
        parsed_date.format("%A, %B %e, %Y at %l:%M %p").to_string() 
    } else {
        if start_dt.is_empty() { "Date TBD".to_string() } else { start_dt }
    };
    
    let location = event.event_location.unwrap_or_else(|| "Various Locations".to_string());
    let borough = event.event_borough.unwrap_or_else(|| "NYC".to_string()); // Sub-city is now shown here!
    let category = event.event_type.unwrap_or_else(|| "General".to_string()); // Category shown here too!

    let search_query = title.replace(" ", "+");
    let parks_url = format!("https://www.nycgovparks.org/events/keyword_search?keyword={}", search_query);
    let google_url = format!("https://www.google.com/search?q={}+nyc+parks+event", search_query);

    rsx! {
        div { class: "modal-overlay", onclick: move |_| selected_event.set(None),
            
            div { class: "modal-content", onclick: move |evt| evt.stop_propagation(),
                button { 
                    class: "modal-close", 
                    onclick: move |_| selected_event.set(None),
                    "×"
                }
                
                h2 { style: "font-size: 2rem; color: var(--color-text-primary); margin-right: 2rem;", "{title}" }
                
                div { style: "display: flex; flex-direction: column; gap: 0.5rem; font-size: 1.1rem; color: var(--color-text-secondary);",
                    div { "📌 " strong { "Category: " } "{category}" }
                    div { "📅 " strong { "When: " } "{display_date}" }
                    div { "📍 " strong { "Where: " } "{location}, {borough}" } // Borough is placed here
                }
                
                p { style: "line-height: 1.6; margin-top: 1rem; color: var(--color-text-secondary);",
                    "Join your community at this incredible NYC Parks event! Free and open to the public."
                }
                
                a { 
                    class: "modal-cta", 
                    href: "{parks_url}",
                    target: "_blank",
                    "Search Tickets on NYC Parks" 
                }
                a { 
                    class: "modal-cta", 
                    style: "background: var(--color-glass-surface); color: var(--color-text-primary); border: 1px solid var(--color-glass-border); margin-top: -1rem;",
                    href: "{google_url}",
                    target: "_blank",
                    "Search on Google" 
                }
            }
        }
    }
}
