# DIOXUS COMMUNITY EVENTS APP
## Technical Requirements Document (TRD) + Product Requirements Document (PRD)

---

## PART 1: PRODUCT REQUIREMENTS DOCUMENT (PRD)

### 1. EXECUTIVE SUMMARY

**Product:** Community Events Finder  
**Platform:** Web (Dioxus WASM)  
**Deployment:** Cloudflare Pages (static hosting) + optional Cloudflare Workers (edge functions)  
**Target Users:** Community members searching for local events  
**Free Tier:** Yes (Cloudflare free plan)  
**Go-Live Target:** ready tonight  

### 2. PROBLEM STATEMENT

Community members struggle to discover local events (farmers markets, workshops, festivals, meetups) because information is scattered across multiple platforms or not promoted effectively.

**Market Opportunity:**
- ~50,000 annual community events in Canada
- Event discovery is still driven by word-of-mouth
- No unified, offline-first event browsing solution for communities

### 3. SOLUTION OVERVIEW

A lightweight, fast web application displaying curated community events with client-side filtering by:
- **Category** (markets, workshops, festivals, meetups, sports)
- **Date range** (this week, this month, upcoming)
- **Distance** (optional: if location services enabled)
- **Search** (keyword matching on event name/description)

**Key Features:**
- Browse events without login
- Filter/search in real-time (client-side only)
- Share event links (URL with filter params)
- Responsive design (mobile-first)
- Fast load (<2s on 4G)
- Accessible (WCAG 2.1 AA)

**Non-Features (Intentionally Out of Scope):**
- User accounts
- Event submissions
- Email notifications
- Database persistence
- Real-time updates

### 4. TARGET USERS

**Primary:**
- Community members aged 25-65
- Low-tech to high-tech literacy
- Mobile + desktop browsing

**Secondary:**
- Event organizers (viewing their events listed)
- Community center staff (embedding event list on website)

### 5. SUCCESS METRICS

| Metric | Target | Measurement |
|--------|--------|------------|
| Page Load Time (4G) | <1.5s | Lighthouse, real-user analytics |
| Mobile UX Score | 90+ | Lighthouse mobile score |
| Lighthouse Performance | 90+ | Automated testing |
| CDN Cache Hit Rate | >95% | Cloudflare analytics |
| Uptime | 99.9% | Cloudflare health monitoring |
| First Contentful Paint | <1s | Web Vitals |

### 6. USER STORIES

**User Story 1: Browse Events**
```
As a community member,
I want to see upcoming events in my area,
So that I can decide what to attend this weekend.

Acceptance Criteria:
- Load main page in <1.5s
- See 20+ events with images, dates, descriptions
- Events display in chronological order by default
- Works offline after first load (service worker)
```

**User Story 2: Filter Events**
```
As a user browsing events,
I want to filter by category and date,
So that I can quickly find relevant events.

Acceptance Criteria:
- Filter updates in <100ms (client-side)
- Selected filters persist in URL
- Can share filtered view with others
- Clear filters with single button
```

**User Story 3: View Event Details**
```
As a user interested in an event,
I want to see full details (time, location, description, contact),
So that I can decide to attend.

Acceptance Criteria:
- Event details modal opens on click
- Shows address as text + Google Maps link
- Contact info clearly visible
- Can copy event details to clipboard
```

### 7. CONTENT STRATEGY

**Event Data Source:**
- Seed data: 50 fictitious events (farmers markets, workshops, festivals, meetups)
- Format: JSON embedded in WASM binary
- Update cadence: Manually edit JSON, rebuild, redeploy (weekly)

**Example Event Structure:**
```json
{
  "id": "001",
  "name": "Vancouver Farmers Market",
  "category": "market",
  "date": "2025-08-15",
  "start_time": "09:00",
  "end_time": "14:00",
  "location": "Granville Island, Vancouver, BC",
  "latitude": 49.2706,
  "longitude": -123.1506,
  "description": "Weekly farmers market featuring local produce, crafts, and prepared foods.",
  "image_url": "/images/farmers-market.jpg",
  "contact_email": "info@granvillemarkets.com",
  "website": "https://granvillemarkets.com"
}
```

### 8. MONETIZATION & BUSINESS MODEL

**Phase 1 (MVP):** Free, ad-free, community-supported  
**Phase 2:** Optional sponsorships from event organizers (not ads)  
**Phase 3:** Event listings API for regional tourism sites (B2B)  

**No user data collection. No tracking beyond Cloudflare analytics.**

### 9. COMPETITIVE LANDSCAPE

| Competitor | Strengths | Weaknesses |
|---|---|---|
| Eventbrite | Large DB, ticketing | Cluttered, ads, sign-up required |
| Facebook Events | Social integration | Privacy concerns, algorithm-driven |
| Meetup.com | Community focused | Paywall, slow, heavy UI |
| **Our App** | Fast, local, offline | Limited to curated events |

### 10. GO-TO-MARKET PLAN

1. **Hour 1-2:** Beta launch (50 events, Metro Vancouver focus)
2. **Hour 3:** Soft launch (community Discord/Reddit posts)
3. **Hour 4:** Local media outreach (community blogs)
4. **Hour 5-8:** Scale to other BC regions, gather feedback

**Channels:**
- Community Discord/Slack servers
- Local Reddit communities
- Neighborhood social media groups
- Community center bulletin boards (QR codes to site)

---

## PART 2: TECHNICAL REQUIREMENTS DOCUMENT (TRD)

### 1. TECH STACK

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| **Frontend** | Dioxus 0.6 (Rust) | Type-safe, compiles to WASM, fast |
| **Build Tool** | `dx` CLI | Official Dioxus build tool |
| **Compile Target** | WebAssembly (wasm32-unknown-unknown) | Runs in browser, instant load |
| **Styling** | Vanilla CSS + custom CSS | Responsive, zero-config |
| **Deployment** | Cloudflare Pages | Free, global CDN, git-based |
| **Optional Edge** | Cloudflare Workers | Rate limiting, redirects, security headers |
| **Data Format** | JSON (hardcoded/embedded) | No database needed |
| **Testing** | Cargo test + browser manual | Integration coverage only |
| **Analytics** | Cloudflare Web Analytics | Built-in, privacy-respecting |

### 2. ARCHITECTURE

```
┌─────────────────────────────────────────────────────┐
│         Browser (User's Device)                     │
│  ┌───────────────────────────────────────────────┐  │
│  │  Dioxus WASM App (community-events.wasm)      │  │
│  │  - Event rendering                            │  │
│  │  - Client-side filtering                      │  │
│  │  - Rate limiting (local)                      │  │
│  │  - Service Worker (offline caching)           │  │
│  └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
           ↓ HTTPS (immutable content)
┌─────────────────────────────────────────────────────┐
│    Cloudflare Edge Network (Global CDN)             │
│  ┌───────────────────────────────────────────────┐  │
│  │  HTTP/2 Push, Compression, Caching             │  │
│  │  - index.html (1KB, cached 5min)               │  │
│  │  - app.wasm (300KB gzipped, cached 30d)        │  │
│  │  - style.css (50KB, cached 30d)                │  │
│  │  - images/ (optimized, cached 60d)             │  │
│  │                                                │  │
│  │  Optional: Cloudflare Workers (edge compute)   │  │
│  │  - Add security headers                        │  │
│  │  - Rate limiting enforcement                   │  │
│  │  - Redirect rules                              │  │
│  └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
           ↓
┌─────────────────────────────────────────────────────┐
│  GitHub Repository (Source of Truth)                │
│  - Source code (Rust)                               │
│  - Event data (JSON)                                │
│  - GitHub Pages auto-deploy hook                    │
└─────────────────────────────────────────────────────┘
```

### 3. DETAILED REQUIREMENTS

#### 3.1 Frontend Application

**Technology:** Dioxus 0.6.x (Web)  
**Language:** Rust (compiles to WASM)  
**Target:** wasm32-unknown-unknown

**Requirements:**
- [ ] Load and render 50+ events from embedded JSON
- [ ] Search events by name/description in <100ms
- [ ] Filter by category, date range
- [ ] Sort by date (default), distance (optional)
- [ ] Responsive design (320px - 4K)
- [ ] Accessibility (WCAG 2.1 Level AA)
  - ARIA labels on buttons
  - Semantic HTML (nav, main, article)
  - Keyboard navigation (Tab, Enter, Escape)
  - Color contrast 4.5:1 minimum
- [ ] Client-side rate limiting (10 searches/sec per user)
- [ ] Share event link (copy to clipboard, prefilled filters)
- [ ] Open event location in Google Maps/Apple Maps
- [ ] No console errors or warnings in production
- [ ] Bundle size: <500KB uncompressed, <150KB gzipped

**Performance Budget:**
```
index.html:         1 KB
app.wasm:           ~300 KB (gzipped ~100 KB)
style.css:          50 KB (gzipped ~15 KB)
images/:            ~200 KB total (optimized)
fonts/:             0 (system fonts only)
─────────────────────────────
Total:              ~351 KB (gzipped ~115 KB)
```

**Browser Support:**
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Mobile Safari 14+
- No IE11 support

#### 3.2 Data Management

**Event Data:**
- Format: JSON (serde_json)
- Storage: Embedded in WASM binary as const
- Backup: Committed to GitHub
- Update Process: Edit JSON → `dx build` → deploy

**Example Data Schema:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub category: EventCategory,  // market, workshop, festival, meetup, sports
    pub date: String,             // YYYY-MM-DD
    pub start_time: String,       // HH:MM
    pub end_time: String,         // HH:MM
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    pub description: String,
    pub image_url: String,
    pub contact_email: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventCategory {
    Market,
    Workshop,
    Festival,
    Meetup,
    Sports,
}
```

#### 3.3 Build & Compilation

**Build Command (Development):**
```bash
dx serve --open
```
- Hot-reload on file change
- Watches Rust + CSS
- Serves on http://localhost:8080
- Auto-opens browser

**Build Command (Production):**
```bash
dx build --release
```
- Optimizes Rust code (LTO, strip, codegen-units=1)
- Minifies CSS
- Optimizes WASM (wasm-opt -O4)
- Output: `dist/` directory

**Cargo.toml (Optimization Flags):**
```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Slower compile, smaller binary
strip = true         # Remove debug symbols
panic = "abort"      # Reduces WASM overhead
```

**Environment:**
- Rust 1.70+
- wasm-pack 1.3+
- Dioxus CLI 0.6+
- Node.js 18+ (for build tooling)

#### 3.4 Deployment Target: Cloudflare Pages

**Why Cloudflare Pages (not Workers)?**
- <cite index="27-1">Pages ideal for static sites; free plan includes 300+ data centers, auto SSL, Git auto-deployment, unlimited bandwidth</cite>
- No serverless compute needed (app is fully client-side)
- Faster time-to-first-byte than Workers
- Automatic Git integration

**Deployment Workflow:**
```
GitHub Commit (Rust code)
     ↓
GitHub Webhook → Cloudflare Pages
     ↓
Cloudflare runs: `dx build --release`
     ↓
Output `dist/` → Global CDN (300+ edge locations)
     ↓
Live at: https://community-events.pages.dev
```

**Configuration: `wrangler.toml` (for Pages)**
```toml
[env.production]
name = "community-events"
compatibility_date = "2025-08-23"

# Build settings
build = { command = "dx build --release" }
build_watch_paths = ["src/**/*.rs", "public/**/*"]

# Output directory (where `dx build` writes)
build_output_dir = "dist"

# Triggers
env = "production"
vars = { APP_ENV = "production" }

# Observability
[observability]
enabled = true
```

**GitHub Actions (Optional CI/CD Workaround if Cloudflare direct doesn't work):**
```yaml
name: Deploy to Cloudflare Pages

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      
      - name: Install Dioxus CLI
        run: cargo install dioxus-cli
      
      - name: Build
        run: dx build --release
      
      - name: Deploy to Cloudflare Pages
        env:
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
        run: |
          npm install -g @cloudflare/wrangler
          wrangler pages deploy dist
```

#### 3.5 HTTP Headers & Security Configuration

**Via Cloudflare Pages `_headers` file:**

```
# Place in public/_headers (or public/ root)

/*
  X-Content-Type-Options: nosniff
  X-Frame-Options: DENY
  X-XSS-Protection: 1; mode=block
  Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
  Content-Security-Policy: default-src 'self'; script-src 'wasm-unsafe-eval' 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'
  Referrer-Policy: strict-origin-when-cross-origin
  Permissions-Policy: geolocation=(), microphone=(), camera=()
  Vary: Accept-Encoding

# Cache WASM binary for 30 days (content-hash in filename)
/app*.wasm
  Cache-Control: public, immutable, max-age=2592000

# Cache CSS for 30 days
/style*.css
  Cache-Control: public, immutable, max-age=2592000

# Cache images for 60 days
/images/*
  Cache-Control: public, max-age=5184000

# HTML: short cache + revalidate
/index.html
  Cache-Control: public, max-age=300, must-revalidate

# robots.txt: no indexing of dynamic filters
/robots.txt
  Cache-Control: public, max-age=86400
```

**Content of `public/robots.txt`:**
```
User-agent: *
Allow: /
Disallow: /?*  # Prevent indexing of filter URLs
```

#### 3.6 Rate Limiting

**Location:** Client-side (Rust/WASM)

**Implementation:**
- Token bucket algorithm
- Per-session (localStorage)
- Limit: 10 searches per second
- Shared budget across all filters/searches
- Server-side not needed (no backend)

**Code (src/utils/rate_limit.rs):**
```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct RateLimiter {
    requests: Rc<RefCell<Vec<i64>>>,
    max_requests: usize,
    window_ms: i64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_ms: i64) -> Self {
        RateLimiter {
            requests: Rc::new(RefCell::new(Vec::new())),
            max_requests,
            window_ms,
        }
    }

    pub fn is_allowed(&self) -> bool {
        let now = web_sys::js_sys::Date::now() as i64;
        let mut reqs = self.requests.borrow_mut();
        reqs.retain(|&t| now - t < self.window_ms);
        
        if reqs.len() < self.max_requests {
            reqs.push(now);
            true
        } else {
            false
        }
    }

    pub fn remaining(&self) -> usize {
        let now = web_sys::js_sys::Date::now() as i64;
        let reqs = self.requests.borrow();
        let active = reqs.iter()
            .filter(|&&t| now - t < self.window_ms)
            .count();
        self.max_requests.saturating_sub(active)
    }
}
```

**Usage in Component:**
```rust
use crate::utils::rate_limit::RateLimiter;
use dioxus::prelude::*;

#[component]
pub fn SearchBar() -> Element {
    let mut limiter = use_signal(|| RateLimiter::new(10, 1000));
    
    let handle_search = move |query: String| {
        if !limiter().is_allowed() {
            // Show toast/warning
            return;
        }
        // Proceed with search
    };
    
    rsx! {
        input {
            on_input: move |evt| handle_search(evt.value())
        }
        p { "Searches remaining this second: {limiter().remaining()}" }
    }
}
```

#### 3.7 Reverse Engineering Mitigation

**WASM Obfuscation:**
1. Use `wasm-opt` with aggressive optimization:
   ```bash
   wasm-opt -O4 --enable-mutable-globals app.wasm
   ```
2. Strip debug symbols (already in Cargo.toml with `strip = true`)
3. Minify + compress via Cloudflare (automatic gzip)

**Event Data Protection:**
```rust
// Optional: Encode event data at compile time (cosmetic only)
// Assume any client-side logic is inspectable

// Instead, rely on:
// - Event data is public anyway (no secrets)
// - Functionality is read-only (no mutations)
// - Inspection reveals only static information
```

**DevTools Mitigation (Cosmetic):**
```rust
// Detect and warn if DevTools is open (not enforceable)
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "console.log")]
    fn console_log(s: &str);
}

// In component init:
if is_devtools_open() {
    console_log("Development tools detected. This app is open source.");
}

fn is_devtools_open() -> bool {
    // Simple heuristic: timing difference in console operations
    let start = web_sys::js_sys::Date::now();
    web_sys::console::log_1(&"".into());
    let elapsed = web_sys::js_sys::Date::now() - start;
    elapsed > 100.0  // DevTools open = slower console
}
```

**Caveat:** WASM decompilation is theoretically possible. Assume any client-side logic is visible. Keep secrets on server (N/A here).

#### 3.8 Testing Strategy

**Unit Tests (Rust):**
```bash
cargo test
```

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_up_to_max() {
        let limiter = RateLimiter::new(3, 1000);
        assert!(limiter.is_allowed()); // 1st
        assert!(limiter.is_allowed()); // 2nd
        assert!(limiter.is_allowed()); // 3rd
        assert!(!limiter.is_allowed()); // Blocked
    }

    #[test]
    fn test_event_filter_by_category() {
        let events = vec![
            Event { category: EventCategory::Market, ... },
            Event { category: EventCategory::Workshop, ... },
        ];
        let filtered: Vec<_> = events.iter()
            .filter(|e| e.category == EventCategory::Market)
            .collect();
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_date_parsing() {
        let date = "2025-08-15";
        let parsed = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d");
        assert!(parsed.is_ok());
    }
}
```

**Browser Testing (Manual):**
1. Load site on 4G (Chrome DevTools throttle)
2. Search 20+ times in succession (test rate limiting)
3. Filter by each category
4. Click each event (modal opens)
5. Open DevTools (verify no errors)
6. Copy event details (clipboard works)
7. Open Maps link (external navigation works)
8. Disable JS → site gracefully degrades (shows message)
9. Test on iPhone/Android (touch interactions, viewport)

**Performance Testing:**
```bash
# After `dx build --release`:
# 1. Measure bundle size
ls -lh dist/app.wasm
gzip -k dist/app.wasm && ls -lh dist/app.wasm.gz

# 2. Use Lighthouse (Chrome DevTools)
# Performance, Accessibility, Best Practices scores
# Target: 90+ on all metrics

# 3. Test on slow network
# Chrome DevTools → Network tab → Throttle to "Slow 4G"
# Measure First Contentful Paint (target: <1.5s)
```

#### 3.9 Error Handling

**Compile-Time Errors:**
- Rust compiler rejects at build time
- All errors must be fixed before deploy
- No runtime type errors possible

**Runtime Errors:**
```rust
// In main.rs or error boundary component:

use dioxus::prelude::*;

#[component]
pub fn ErrorBoundary(children: Element) -> Element {
    rsx! {
        div {
            onerror: move |err| {
                web_sys::console::error_1(&format!("Error: {:?}", err).into());
                // User sees: "Something went wrong. Please refresh."
            },
            {children}
        }
    }
}
```

**User-Facing Errors:**
- Search failed → "No events found"
- Rate limited → "Please wait a moment before searching again"
- Location unavailable → "Couldn't load your location. Using default view."

#### 3.10 OWASP Top 10:2025 Compliance

| Risk | Status | Mitigation |
|------|--------|-----------|
| A01: Broken Access Control | N/A | No users or resources to control |
| A02: Security Misconfiguration | ✓ Required | CSP headers, HSTS, X-Frame-Options |
| A03: Software Supply Chain Failures | ✓ Required | `cargo audit`, Cargo.lock committed |
| A04: Cryptographic Failures | N/A | No sensitive data |
| A05: Injection | ✓ Required | Input validation, no SQL/NoSQL |
| A06: Vulnerable & Outdated Components | ✓ Required | Weekly `cargo outdated`, pin versions |
| A07: Identification & Authentication | N/A | No auth required |
| A08: Data Integrity Failures | ✓ Required | Subresource Integrity (SRI) for CDN |
| A09: Logging & Monitoring Failures | ✓ Required | Cloudflare Web Analytics, error tracking |
| A10: Server-Side Request Forgery | N/A | No server-side requests |

### 4. DEPLOYMENT CHECKLIST

#### Pre-Deploy (Local)
- [ ] `cargo clippy` passes with zero warnings
- [ ] `cargo test` passes all tests
- [ ] `dx build --release` completes without errors
- [ ] `dist/` directory exists with index.html, app.wasm, style.css
- [ ] No hardcoded API keys or secrets
- [ ] `.env.example` exists (if needed); `.env` not committed
- [ ] `_headers` file configured (security headers)
- [ ] `robots.txt` in place
- [ ] Event data JSON is valid
- [ ] Bundle size <500KB uncompressed

#### Deploy to Cloudflare Pages
1. **Setup:**
   ```bash
   # Install Wrangler CLI
   npm install -g @cloudflare/wrangler
   
   # Login to Cloudflare account
   wrangler login
   
   # Create Pages project (first time only)
   wrangler pages create community-events
   ```

2. **Connect GitHub:**
   - Cloudflare Dashboard → Pages → Connect Git
   - Select GitHub repository
   - Build command: `dx build --release`
   - Build output: `dist`
   - Deploy branch: `main`

3. **Manual Deploy (if needed):**
   ```bash
   wrangler pages deploy dist --project-name community-events
   ```

4. **Verify Deployment:**
   - [ ] Site loads at https://community-events.pages.dev
   - [ ] All events render (check console, no errors)
   - [ ] Filtering works (no latency)
   - [ ] Images load
   - [ ] Mobile responsive
   - [ ] CSP headers present (DevTools Network tab)
   - [ ] WASM module loads

#### Post-Deploy (Monitoring)
- [ ] Cloudflare analytics enabled
- [ ] Web Vitals metrics tracked
- [ ] Error logging active (if configured)
- [ ] CDN cache hit rate >95%
- [ ] Average response time <100ms (p50)
- [ ] Zero 5xx errors in first 24h

### 5. DOCUMENTATION STANDARDS

**Rust Code:**
```rust
/// Brief description (one line).
///
/// Longer explanation if needed.
///
/// # Examples
/// ```
/// let result = my_function("input");
/// assert_eq!(result, "output");
/// ```
pub fn my_function(input: &str) -> String {
    // Implementation
}
```

**Commit Messages:**
```
feat: Add event filtering by category

- Implement TokenBucket rate limiter
- Add category enum (Market, Workshop, etc)
- Update UI with filter buttons

Closes #123
```

### 6. MONITORING & ANALYTICS

**Cloudflare Web Analytics (Built-in):**
- Dashboard shows: Page views, unique visitors, bounce rate
- No cookie-based tracking (privacy-friendly)
- Visible in Cloudflare dashboard

**Custom Logging (Optional):**
```rust
use wasm_logger;

pub fn init_logging() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("App initialized");
}

// In components:
log::debug!("User searched for: {}", query);
log::warn!("Rate limited: {}", remaining);
```

---

## PART 3: CLOUDFLARE DEPLOYMENT BEST PRACTICES

### 1. Cloudflare Pages vs. Cloudflare Workers

| Aspect | Pages | Workers |
|--------|-------|---------|
| **Use Case** | Static sites (HTML, CSS, WASM) | Edge compute (API routes, transforms) |
| **For Our App** | Primary deployment ✓ | Optional (rate limiting at edge) |
| **Free Tier** | 500 builds/month, unlimited bandwidth | 100k requests/day free |
| **Build** | Runs `dx build` automatically | Deploy prebuilt binaries |
| **Latency** | <50ms (global CDN) | <1ms (computed on edge) |
| **Cost** | Free | Free (then $0.50/M requests) |

**Decision:** Use **Pages for main app**, optionally **Workers for edge security headers**.

### 2. Cloudflare Pages Setup (Step-by-Step)

#### Option A: Direct GitHub Integration (Easiest)

```bash
# 1. Push code to GitHub (main branch)
git push origin main

# 2. Go to Cloudflare Dashboard
# → Pages → Create a project

# 3. Select GitHub repo
# → Authorize Cloudflare on GitHub

# 4. Configure build settings:
#    Build command: dx build --release
#    Build output directory: dist
#    Environment variables: (none needed)

# 5. Click "Save and Deploy"
# → Automatic builds on every push to main

# 6. Get live URL: https://community-events.pages.dev
```

#### Option B: Manual Deployment via CLI

```bash
# Build locally
dx build --release

# Deploy built files
wrangler pages deploy dist --project-name community-events

# URL: https://community-events.pages.dev
```

### 3. Environment Variables (Cloudflare)

**Our App:** NO secrets needed. But if future versions need config:

```toml
# In wrangler.toml
[env.production]
vars = { APP_ENV = "production", LOG_LEVEL = "info" }

[env.staging]
vars = { APP_ENV = "staging", LOG_LEVEL = "debug" }
```

**Access in Rust:**
```rust
pub fn get_env(key: &str) -> Option<String> {
    // Read from compile-time env (safe)
    option_env!("APP_ENV").map(|s| s.to_string())
}
```

### 4. Custom Domains

1. **Add domain:**
   - Cloudflare Dashboard → Pages → Settings
   - Add custom domain (e.g., `events.example.com`)

2. **Point DNS:**
   - Add CNAME record: `events` → `community-events.pages.dev`
   - Or use Cloudflare Nameservers (preferred)

3. **SSL:** Automatic (Cloudflare manages certificate)

### 5. Caching Strategy

**Cloudflare Default Behavior:**
- HTML: Cache-Control applies (we set 5min via `_headers`)
- WASM: Cached 30d (content hash in filename)
- CSS/JS: Cached 30d
- Images: Cached 60d

**Override Cache with `_headers`:**
```
/app*.wasm
  Cache-Control: public, immutable, max-age=31536000

/index.html
  Cache-Control: public, max-age=300, must-revalidate
```

**Bypass Cache (if needed):**
```bash
# In Cloudflare Dashboard:
# Caching → Cache Rules → Create rule
# Path contains: /api/
# → Cache: Bypass
```

### 6. Optional: Cloudflare Workers (Edge Security)

**When to use:** If you want rate limiting or header injection at edge (faster than browser-side).

**Setup (`wrangler.toml`):**
```toml
name = "community-events-worker"
type = "service-worker"
compatibility_date = "2025-08-23"

# Point to Pages project for assets
routes = [{ pattern = "*", zone_name = "example.com" }]

[[r2_buckets]]
binding = "EVENTS_BUCKET"
bucket_name = "community-events"
```

**Example Worker (rate limiting at edge):**
```javascript
// src/index.js
export default {
  async fetch(request, env, ctx) {
    // Rate limiting via KV store
    const ip = request.headers.get('cf-connecting-ip');
    const key = `ratelimit:${ip}`;
    const count = (await env.RATE_LIMIT_KV.get(key)) || 0;

    if (count > 100) {
      return new Response('Too many requests', { status: 429 });
    }

    await env.RATE_LIMIT_KV.put(key, count + 1, { expirationTtl: 60 });

    // Fetch from Pages
    const response = await env.ASSETS.fetch(request);
    
    // Add security headers
    const headers = new Headers(response.headers);
    headers.set('X-Content-Type-Options', 'nosniff');
    headers.set('Strict-Transport-Security', 'max-age=31536000; includeSubDomains');
    
    return new Response(response.body, { ...response, headers });
  }
};
```

**Not needed for MVP** (client-side rate limiting sufficient).

### 7. Cloudflare Analytics & Observability

**Automatic Metrics:**
- Requests: Daily, by country, by status code
- Bandwidth: Total, cached vs. uncached
- Page Views: Per route

**Access Dashboard:**
```
Cloudflare Dashboard
  → Pages → community-events
    → Analytics
```

**Web Vitals (Optional):**
```html
<!-- Add to index.html head if desired -->
<script defer src="https://static.cloudflareinsights.com/beacon.min.js" 
  data-cf-beacon='{"token": "YOUR_TOKEN"}'></script>
```

### 8. Deployment Troubleshooting

| Problem | Solution |
|---------|----------|
| **Build fails** | Check build command, ensure `dx` CLI installed in build environment |
| **Site shows 404** | Verify `build_output_dir = "dist"` in `wrangler.toml` |
| **WASM doesn't load** | Check MIME type in Cloudflare → Content Type mapping |
| **Stale content** | Clear cache: Dashboard → Caching → Purge Cache |
| **Custom domain not working** | Ensure DNS records point to `pages.dev`, wait 24h for propagation |

### 9. OWASP Top 10 on Cloudflare

<cite index="27-1">Cloudflare Pages supports auto-deployment from GitHub, 300+ data centers, free SSL, and unlimited bandwidth on free plan</cite>

**Enforce on Cloudflare:**

1. **Via `_headers` file (Pages):**
   - Automatically injected on all responses
   - CSP, HSTS, X-Frame-Options, etc.

2. **Via Workers (optional, edge):**
   - Additional rate limiting
   - IP-based blocking
   - Bot protection

3. **Built-in Cloudflare features:**
   - DDoS protection (automatic)
   - SSL/TLS (automatic)
   - WAF rules (optional, paid)

### 10. Cost Breakdown (Free Tier)

| Service | Free Tier | Cost |
|---------|-----------|------|
| Cloudflare Pages | 500 builds/month, unlimited bandwidth | Free |
| Cloudflare Workers | 100k requests/day | Free (then $0.50/M) |
| Custom domain | Registration elsewhere | ~$10-15/year |
| Web Analytics | Built-in | Included free |
| **Total** | | **Free** (or $10-15 for domain) |

---

## APPENDIX A: PROJECT TIMELINE

| Hour | Milestone | Deliverable |
|------|-----------|------------|
| 1 | Project setup + Dioxus scaffolding | Basic page structure, event model |
| 2 | Event rendering + data pipeline | Display 50 events, images, details |
| 3 | Filtering + search | Category, date, keyword filters |
| 4 | Responsive design + accessibility | Mobile-first, WCAG 2.1 AA |
| 5 | Rate limiting + error handling | Client-side RateLimiter, validations |
| 6 | Security hardening | CSP headers, input sanitization, OWASP checks |
| 7 | Performance optimization | Bundle size <500KB, Lighthouse 90+ |
| 8 | Deployment + monitoring | Live on Cloudflare Pages, analytics active |

---

## APPENDIX B: USEFUL COMMANDS

```bash
# Development
dx serve --open                    # Hot reload on http://localhost:8080
cargo clippy                       # Lint for warnings
cargo test                         # Run unit tests
cargo audit                        # Check for security vulnerabilities

# Build
dx build --release                 # Optimized production build
ls -lh dist/app.wasm              # Check WASM size

# Deployment
wrangler login                     # Authenticate with Cloudflare
wrangler pages deploy dist         # Manual deploy to Pages
wrangler deployments list          # View deployment history

# Monitoring
curl -I https://community-events.pages.dev  # Check headers
lighthouse https://community-events.pages.dev  # Performance audit
```

---

## APPENDIX C: REFERENCES

- **Dioxus Docs:** https://dioxuslabs.com/learn/
- **Dioxus CLI:** `dx --help`
- **Cloudflare Pages Docs:** https://developers.cloudflare.com/pages/
- **Rust WASM:** https://rustwasm.org/
- **OWASP Top 10:2025:** https://owasp.org/Top10/
- **Web Vitals:** https://web.dev/vitals/
- **MDN Web Docs:** https://developer.mozilla.org/

---

**Document Version:** 1.0  
**Last Updated:** August 2025  
**Status:** Ready for Development
