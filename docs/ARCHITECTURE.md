# Dioxus Community Events App — Architecture & Deployment Diagrams

## System Architecture

### High-Level Overview
```
┌─────────────────────────────────────────────────────────────────┐
│                     END USER (Browser)                          │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  HTML Shell (index.html)                                │    │
│  │  ┌──────────────────────────────────────────────────┐   │    │
│  │  │  Dioxus WASM App (JavaScript Interop Layer)      │   │    │
│  │  │  ┌─────────────────────────────────────────────┐ │   │    │
│  │  │  │  Rust Components (Compiled to WASM)         │ │   │    │
│  │  │  │                                              │ │   │    │
│  │  │  │  [EventCard] [FilterBar] [EventList]        │ │   │    │
│  │  │  │        ↓           ↓           ↓             │ │   │    │
│  │  │  │  ┌──────────────────────────────────────┐   │ │   │    │
│  │  │  │  │  Event Data (Embedded JSON)          │   │ │   │    │
│  │  │  │  │  - 50 events (markets, workshops)    │   │ │   │    │
│  │  │  │  │  - Categories, dates, locations      │   │ │   │    │
│  │  │  │  └──────────────────────────────────────┘   │ │   │    │
│  │  │  │                                              │ │   │    │
│  │  │  │  ┌──────────────────────────────────────┐   │ │   │    │
│  │  │  │  │  Rate Limiter (Client-Side)          │   │ │   │    │
│  │  │  │  │  - Token bucket (10 req/sec)         │   │ │   │    │
│  │  │  │  │  - localStorage for state            │   │ │   │    │
│  │  │  │  └──────────────────────────────────────┘   │ │   │    │
│  │  │  │                                              │ │   │    │
│  │  │  └─────────────────────────────────────────────┘ │   │    │
│  │  │                                                   │   │    │
│  │  │  CSS (Vanilla CSS + Custom)                         │   │    │
│  │  │  Images (lazy-loaded from /public/images/)       │   │    │
│  │  └─────────────────────────────────────────────────┘   │    │
│  │                                                         │    │
│  │  Service Worker (optional, for offline caching)        │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                  │
│  HTTPS + Browser APIs (fetch, localStorage, geolocation)        │
└──────────────────────────┬──────────────────────────────────────┘
                           │ HTTPS (TLS 1.3)
                           ↓
        ┌──────────────────────────────────┐
        │  Cloudflare Global CDN Edge      │
        │  (300+ locations worldwide)      │
        │                                  │
        │  - HTTP/2 Push                   │
        │  - Brotli Compression            │
        │  - Geolocation-based caching     │
        │  - DDoS protection               │
        │  - WAF (optional)                │
        │                                  │
        │  Cache Layers:                   │
        │  ├─ Browser Cache (index.html)   │
        │  ├─ CDN Cache (wasm, css, images)│
        │  └─ Cloudflare Cache             │
        │                                  │
        │  Security Headers:               │
        │  ├─ X-Content-Type-Options       │
        │  ├─ Content-Security-Policy      │
        │  ├─ Strict-Transport-Security    │
        │  └─ X-Frame-Options              │
        └──────────────────────────────────┘
                           │
                           ↓
        ┌──────────────────────────────────┐
        │  GitHub Repository               │
        │  (Source of Truth)               │
        │                                  │
        │  - Rust source (src/)            │
        │  - Event data (data/)            │
        │  - Deployment config (wrangler)  │
        │  - Security headers (_headers)   │
        └──────────────────────────────────┘
```

---

## Deployment Flow

### Development → Production Pipeline
```
┌──────────────────────────────────────────────────────────────┐
│  DEVELOPER LOCAL MACHINE                                     │
│                                                              │
│  1. Edit Rust code / Update event data                       │
│  2. Run: dx serve --open                                     │
│  3. Test locally: http://localhost:8080                      │
│  4. Run: cargo clippy (lint)                                 │
│  5. Run: cargo test (unit tests)                             │
│  6. Run: dx build --release                                  │
│     └─→ Outputs: dist/                                       │
│         - index.html (1 KB)                                  │
│         - app.wasm (300 KB)                                  │
│         - style.css (50 KB)                                  │
│         - images/ (200 KB)                                   │
│                                                              │
│  7. Verify bundle size: ls -lh dist/app.wasm                │
│  8. Commit to Git                                            │
│  9. Push to GitHub: git push origin main                     │
└────────────┬─────────────────────────────────────────────────┘
             │
             │ GitHub Webhook (automatic)
             ↓
┌──────────────────────────────────────────────────────────────┐
│  CLOUDFLARE PAGES BUILD ENVIRONMENT                          │
│                                                              │
│  1. Webhook received from GitHub                             │
│  2. Clone repository                                         │
│  3. Install dependencies: cargo fetch                        │
│  4. Run build command: dx build --release                    │
│     └─→ Compiles Rust → WASM                                │
│     └─→ Optimizes: wasm-opt -O4                              │
│     └─→ Minifies CSS                                         │
│  5. Verify output: dist/ exists                              │
│  6. Upload to Cloudflare KV (metadata)                       │
│  7. Build complete ✓                                         │
└────────────┬─────────────────────────────────────────────────┘
             │
             ↓
┌──────────────────────────────────────────────────────────────┐
│  CLOUDFLARE GLOBAL CDN EDGE NETWORK                          │
│                                                              │
│  Replicate files to 300+ edge locations:                     │
│  - index.html (cached 5 min)                                 │
│  - app.wasm (cached 30 days, immutable)                      │
│  - style.css (cached 30 days, immutable)                     │
│  - images/ (cached 60 days)                                  │
│                                                              │
│  Apply security headers from _headers file                   │
│  Enable gzip/brotli compression                              │
│  Enable HTTP/2 server push                                   │
│                                                              │
│  Live URL: https://community-events.pages.dev                │
└────────────┬─────────────────────────────────────────────────┘
             │
             ↓
┌──────────────────────────────────────────────────────────────┐
│  CLOUDFLARE ANALYTICS DASHBOARD                              │
│                                                              │
│  Real-time metrics:                                          │
│  - Page views (by country, by date)                          │
│  - Unique visitors                                           │
│  - Cache hit rate (target >95%)                              │
│  - Response time (target <100ms p50)                         │
│  - Bandwidth used                                            │
│  - Security events (if WAF enabled)                          │
└──────────────────────────────────────────────────────────────┘
```

---

## Component Architecture

```
┌────────────────────────────────────────────────────────────┐
│                      App.rs (Root)                         │
│  Manages state, routing, global config                     │
└─────────────────────────────┬────────────────────────────────┘
                              │
            ┌─────────────────┼─────────────────┐
            │                 │                 │
            ↓                 ↓                 ↓
    ┌───────────────┐  ┌──────────────┐  ┌──────────┐
    │  FilterBar    │  │  EventList   │  │ EventMap │
    │               │  │              │  │(optional)│
    │ ├─ Categories │  │ - Sorted     │  │          │
    │ ├─ Date Range │  │ - Paginated  │  └──────────┘
    │ ├─ Search Box │  │ - Lazy load  │
    │ └─ Clear Btn  │  │              │
    └─────────────┬─┘  └──────────┬──┘
                  │               │
                  └───────┬───────┘
                          ↓
            ┌──────────────────────────────┐
            │    FilteredEventsList        │
            │ (Client-side filtering state)│
            │                              │
            │ Applies:                     │
            │ - Category filters           │
            │ - Date range filters         │
            │ - Keyword search             │
            │ - Sorting (date, distance)   │
            └──────────────┬───────────────┘
                           │
                           ↓
            ┌──────────────────────────────┐
            │      EventCard (List)        │
            │ ×50 cards rendered           │
            │                              │
            │ Shows:                       │
            │ - Image (lazy)               │
            │ - Name                       │
            │ - Date/Time                  │
            │ - Category badge             │
            │ - Distance (if location OK)  │
            │                              │
            │ On click: Open Modal ↓       │
            └──────────────┬───────────────┘
                           │
                           ↓
            ┌──────────────────────────────┐
            │   EventDetailModal           │
            │ (Full event information)     │
            │                              │
            │ Shows:                       │
            │ - Full image (hi-res)        │
            │ - Name, category             │
            │ - Date, start/end time       │
            │ - Full description           │
            │ - Location + map link        │
            │ - Contact info               │
            │ - Share/copy buttons         │
            │ - External website link      │
            └──────────────────────────────┘
```

---

## Data Flow

```
User Interaction
    │
    ├─ Type in search box
    │      ↓
    │  On input event
    │      ↓
    │  Check rate limiter
    │      ↓
    │  [NOT BLOCKED] → Proceed
    │  [BLOCKED] → Show "Too many searches" warning
    │      ↓
    │  Filter events in memory (Rust):
    │  - Convert input to lowercase
    │  - Validate input (length, chars)
    │  - Match against event names/descriptions
    │  - Update component state
    │      ↓
    │  Trigger re-render
    │      ↓
    │  Display filtered results (<100ms)
    │
    ├─ Click on event
    │      ↓
    │  Open modal
    │      ↓
    │  Display full details
    │      ↓
    │  User clicks "Open in Maps"
    │      ↓
    │  Navigate to Google Maps (external)
    │
    └─ Click "Share"
           ↓
       Copy link to clipboard with filters
           ↓
       User can share link: ?category=market&date=2025-08-15
```

---

## State Management

```
Global State (Dioxus Signals)
    │
    ├─ events: Vec<Event>
    │  └─ Immutable, loaded once on startup
    │
    ├─ search_query: String
    │  └─ User's search input (real-time)
    │
    ├─ selected_categories: Vec<EventCategory>
    │  └─ User's selected filters (multi-select)
    │
    ├─ date_range: (NaiveDate, NaiveDate)
    │  └─ User's date filter range
    │
    ├─ sort_by: SortOrder
    │  └─ Enum: Date | Distance | Name
    │
    ├─ rate_limiter: RateLimiter
    │  └─ Tracks search frequency
    │
    └─ show_modal: bool
       └─ Modal visibility toggle
```

---

## Security Layers

```
Browser Level
├─ Content Security Policy (CSP)
│  └─ Restricts which resources can load
│     └─ script-src: 'self' 'wasm-unsafe-eval'
│
├─ CORS (Cross-Origin Resource Sharing)
│  └─ Only resources from same origin
│
├─ X-Frame-Options
│  └─ Prevents clickjacking (DENY)
│
└─ X-Content-Type-Options
   └─ Prevents MIME type sniffing

Network Level (Cloudflare)
├─ TLS 1.3 (HTTPS only)
│  └─ Encrypts all data in transit
│
├─ DDoS Protection
│  └─ Automatic rate limiting at edge
│
├─ WAF (Optional, paid)
│  └─ Filter malicious requests
│
└─ Geo-blocking (Optional)
   └─ Restrict access by country

Application Level
├─ Input Validation (Rust)
│  └─ Character whitelist, length limits
│
├─ Client-Side Rate Limiting
│  └─ 10 searches/sec per browser
│
├─ No SQL/NoSQL (N/A)
│  └─ Static data only
│
└─ Memory Safety
   └─ Rust prevents buffer overflows
```

---

## Performance Optimization Pipeline

```
Source Code (Rust)
    ↓
Compilation Phase
├─ Strip debug symbols (strip = true)
├─ Link-time optimization (lto = true)
├─ Single codegen unit (slower build, smaller binary)
└─ Panic = abort (reduced overhead)
    ↓
WASM Optimization Phase
├─ wasm-opt -O4
│  ├─ Remove unused code
│  ├─ Inline functions
│  ├─ Optimize loops
│  └─ Merge duplicate logic
│
├─ Minify JavaScript wrapper
│
└─ Result: ~300 KB uncompressed
    ↓
Compression Phase (Cloudflare)
├─ Brotli compression (br)
│  └─ ~100 KB final size
│
└─ gzip fallback
    └─ ~150 KB final size
    ↓
Final Output
├─ index.html (1 KB)
├─ app.wasm (100-150 KB gzipped)
├─ style.css (15 KB gzipped)
├─ images/ (varies)
└─ Total: ~115 KB gzipped (on wire)
    ↓
Browser Loading
├─ Fetch index.html (1 KB, 5min cache)
├─ Parse HTML + initiate wasm load
├─ Fetch app.wasm (async, 100 KB, 30-day cache)
├─ Fetch style.css (async, 15 KB, 30-day cache)
├─ Instantiate WASM module (~200ms)
├─ Render UI
└─ First Contentful Paint: <1s (4G)
```

---

## Deployment Targets Comparison

```
┌─────────────────────────────────────────────────────────────┐
│  DEPLOYMENT TARGET ANALYSIS                                 │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  OPTION 1: Cloudflare Pages (CHOSEN ✓)                      │
│  ├─ Use Case: Static site hosting                           │
│  ├─ Cost: Free                                              │
│  ├─ Bandwidth: Unlimited                                    │
│  ├─ CDN: 300+ global locations                              │
│  ├─ Builds: 500/month free                                  │
│  ├─ Git Integration: Auto on push                           │
│  ├─ Latency: <50ms (average)                                │
│  ├─ Setup Time: 5 minutes                                   │
│  └─ Best For: Pure frontend (our case)                      │
│                                                              │
│  OPTION 2: Cloudflare Workers (Optional add-on)             │
│  ├─ Use Case: Edge compute, transforms                      │
│  ├─ Cost: Free first 100k req/day                           │
│  ├─ Latency: <1ms (edge compute)                            │
│  ├─ Use Only If: Need rate limiting at edge                 │
│  └─ Complexity: Higher (JS wrapper)                         │
│                                                              │
│  OPTION 3: Netlify (Not chosen)                             │
│  ├─ Similar to Pages                                        │
│  ├─ Cost: Free tier available                               │
│  ├─ Build time: ~3 minutes                                  │
│  └─ Less developer-friendly than Cloudflare                 │
│                                                              │
│  OPTION 4: GitHub Pages (Not chosen)                        │
│  ├─ Free hosting                                            │
│  ├─ Limited to static sites                                 │
│  ├─ No global CDN                                           │
│  └─ Slower response times                                   │
│                                                              │
│  OPTION 5: AWS S3 + CloudFront (Not chosen)                 │
│  ├─ More complex setup                                      │
│  ├─ Requires IAM configuration                              │
│  ├─ Pay per GB (but very cheap)                             │
│  └─ Overkill for MVP                                        │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Technology Stack Diagram

```
┌──────────────────────────────────────────────────────────────┐
│  PRESENTATION LAYER                                          │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Dioxus Components (Rust)                              │  │
│  │  ├─ Virtual DOM (JSX-like)                             │  │
│  │  ├─ Event handling (onclick, oninput)                  │  │
│  │  ├─ State management (Signals)                         │  │
│  │  └─ Reactivity (automatic re-render)                   │  │
│  └────────────────────────────────────────────────────────┘  │
│              ↓                                                │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Styling Layer                                          │  │
│  │  ├─ Vanilla CSS (utility classes)                      │  │
│  │  ├─ Custom CSS (global styles)                          │  │
│  │  └─ Responsive design (mobile-first)                    │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
                          ↓
┌──────────────────────────────────────────────────────────────┐
│  BUSINESS LOGIC LAYER                                        │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Rust Core (Type-Safe)                                 │  │
│  │  ├─ Event filtering (category, date, search)           │  │
│  │  ├─ Rate limiting (token bucket)                       │  │
│  │  ├─ Input validation (regex, length)                   │  │
│  │  ├─ Date/time handling (chrono)                        │  │
│  │  └─ Data serialization (serde)                         │  │
│  └────────────────────────────────────────────────────────┘  │
│              ↓                                                │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  WASM Runtime (JavaScript Interop)                     │  │
│  │  ├─ Browser APIs (fetch, localStorage, Date)           │  │
│  │  ├─ DOM manipulation (wasm-bindgen)                    │  │
│  │  ├─ Event listeners                                    │  │
│  │  └─ Console logging                                    │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
                          ↓
┌──────────────────────────────────────────────────────────────┐
│  DATA LAYER                                                  │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Embedded Static Data                                  │  │
│  │  ├─ 50 events (hardcoded JSON)                         │  │
│  │  ├─ Loaded once on startup                            │  │
│  │  └─ Cached in browser memory                          │  │
│  └────────────────────────────────────────────────────────┘  │
│              ↓                                                │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Browser Storage (Optional)                            │  │
│  │  ├─ localStorage (rate limiter state)                  │  │
│  │  ├─ sessionStorage (filter preferences)                │  │
│  │  └─ IndexedDB (future offline support)                 │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
                          ↓
┌──────────────────────────────────────────────────────────────┐
│  DEPLOYMENT LAYER                                            │
│  ├─ Cloudflare Pages (Static hosting)                        │
│  ├─ Global CDN (300+ edge locations)                         │
│  ├─ TLS 1.3 (HTTPS encryption)                               │
│  └─ Analytics (Web Vitals, traffic)                          │
└──────────────────────────────────────────────────────────────┘
```

---

## Testing Pyramid

```
                  ▲
                 ╱ ╲
                ╱   ╲  End-to-End Tests
               ╱     ╲  (Browser, manual)
              ╱───────╲  ~3-5 tests
             ╱ ╱───────╲╲
            ╱ ╱         ╲╲  Integration Tests
           ╱ ╱ Component  ╲╲  (event filtering, rendering)
          ╱ ╱   Tests     ╲╲  ~10-15 tests
         ╱ ╱────────────────╲╲
        ╱ ╱ Unit Tests       ╲╲  (validation, rate limiter)
       ╱ ╱ ~30-40 tests     ╲╲  Fastest, most specific
      ╱ ╱__________________╲╲
     ╱_______________________╲
```

---

**Diagram Version:** 1.0  
**Last Updated:** August 2025
