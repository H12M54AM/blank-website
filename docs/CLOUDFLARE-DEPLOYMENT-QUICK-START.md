# CLOUDFLARE PAGES DEPLOYMENT: QUICK START

## 5-Minute Setup

### Prerequisites
- GitHub account
- Cloudflare account (free)
- Dioxus project locally built once (`dx build --release`)

### Step 1: Prepare Repository
```bash
# Ensure these files exist in repo root:
# - src/main.rs (Dioxus app)
# - Cargo.toml (dependencies)
# - Dioxus.toml (Dioxus config)
# - public/_headers (security headers)
# - public/robots.txt (SEO)

# Commit everything
git add .
git commit -m "Ready for Cloudflare Pages deployment"
git push origin main
```

### Step 2: Create `_headers` File
```bash
# Place in public/_headers

/*
  X-Content-Type-Options: nosniff
  X-Frame-Options: DENY
  X-XSS-Protection: 1; mode=block
  Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
  Content-Security-Policy: default-src 'self'; script-src 'wasm-unsafe-eval' 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'
  Referrer-Policy: strict-origin-when-cross-origin
  Permissions-Policy: geolocation=(), microphone=(), camera=()
  Vary: Accept-Encoding

/app*.wasm
  Cache-Control: public, immutable, max-age=2592000

/style*.css
  Cache-Control: public, immutable, max-age=2592000

/images/*
  Cache-Control: public, max-age=5184000

/index.html
  Cache-Control: public, max-age=300, must-revalidate
```

### Step 3: Create Cloudflare Pages Project

**Option A: Via Dashboard (Easiest)**
1. Go to https://dash.cloudflare.com
2. **Pages** → **Create project** → **Connect to Git**
3. Authorize GitHub, select your repository
4. Fill in build settings:
   - **Build command:** `dx build --release`
   - **Build output directory:** `dist`
5. Click **Save and Deploy**
6. Wait 2-3 minutes for initial build

**Option B: Via CLI**
```bash
npm install -g wrangler
wrangler login
wrangler pages create community-events
# Follow prompts, select your GitHub repo
```

### Step 4: Verify Deployment
```bash
# Check build status in Cloudflare Dashboard
# When complete, visit:
https://community-events.pages.dev

# Verify in browser:
# 1. Page loads (no 404)
# 2. Events render
# 3. Search works
# 4. No console errors (F12)
# 5. WASM loads (DevTools → Application → Wasm)
```

### Step 5: Add Custom Domain (Optional)
1. Dashboard → **Pages** → **community-events** → **Settings**
2. **Custom domains** → **Add custom domain**
3. Enter domain (e.g., `events.example.com`)
4. Update DNS records (instructions shown by Cloudflare)
5. Wait up to 24 hours for DNS propagation
6. SSL certificate automatic ✓

---

## Monitoring After Deploy

### Dashboard Metrics
```
Cloudflare Dashboard
  → Pages → community-events
    → Analytics

Shows:
- Page views (daily)
- Unique visitors
- Countries accessing site
- Bounce rate
- Response times (p50, p95, p99)
```

### Performance Checks
```bash
# 1. Lighthouse score (target 90+)
# Use: https://pagespeed.web.dev
# Enter your site URL

# 2. Check cache hit rate (target >95%)
# Dashboard → Analytics → Cache

# 3. Monitor response time (target <100ms p50)
# Dashboard → Analytics → Performance
```

---

## Redeploy on Code Changes

**Automatic (Recommended):**
- Push to `main` branch on GitHub
- Cloudflare automatically rebuilds
- Deploys within 2-3 minutes

**Manual (If Needed):**
```bash
dx build --release
wrangler pages deploy dist --project-name community-events
```

---

## Troubleshooting

### Build Fails
```bash
# Check Cloudflare build logs:
# Dashboard → Pages → Deployments → Failed build → View logs

# Common issues:
# - Missing Rust toolchain target: wasm32-unknown-unknown
# - Old Dioxus CLI version (update: cargo install dioxus-cli)
# - Cargo.lock missing or outdated

# Fix:
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --force
cargo update
dx build --release
git push
```

### Site Shows 404
```bash
# Verify dist/ is created with index.html
ls -la dist/

# Check wrangler.toml has correct output directory:
# build_output_dir = "dist"

# Ensure public/_headers is in repo (not dist/)
ls -la public/
```

### WASM Module Fails to Load
```bash
# Check console (F12 → Console tab) for WASM load error

# Verify:
# 1. WASM file exists: dist/app*.wasm
# 2. MIME type correct (Content-Type: application/wasm)
# 3. CSP allows wasm-unsafe-eval

# If CSP issue:
# Edit public/_headers:
Content-Security-Policy: default-src 'self'; script-src 'wasm-unsafe-eval' 'self'
```

### Stale Content / Cache Issues
```bash
# Clear Cloudflare cache:
# Dashboard → Caching → Purge Cache → Purge Everything

# Or for specific files:
# Dashboard → Caching → Cache Rules → Create rule
# Purge by URL pattern
```

---

## Post-Launch Checklist

- [ ] Site loads on desktop + mobile
- [ ] Events render correctly
- [ ] Filtering/search works
- [ ] No console errors
- [ ] Lighthouse score ≥90
- [ ] WASM binary optimized (<500KB uncompressed)
- [ ] CSP headers present (DevTools → Network → Response Headers)
- [ ] Cache hit rate >95% (Dashboard → Analytics)
- [ ] Custom domain active (if applicable)
- [ ] Analytics dashboard shows traffic
- [ ] Automated rebuilds working (GitHub → Cloudflare webhook)

---

## Cost Summary

| Service | Free Tier | Status |
|---------|-----------|--------|
| Cloudflare Pages | 500 builds/month, unlimited bandwidth | ✓ Free |
| Custom domain | Registration required | $10-15/year |
| SSL/TLS | Included | ✓ Free |
| Web Analytics | Included | ✓ Free |
| DDoS protection | Included | ✓ Free |
| **Total** | | **Free** (or $10-15 for domain) |

---

## Next Steps

1. **Celebrate** 🎉 Site is live on Cloudflare Pages
2. **Monitor** Check analytics weekly
3. **Iterate** Update event data, gather feedback
4. **Scale** Add more regions/events as traffic grows
5. **Monetize** (Future) Partner with event organizers for sponsorships

---

## Support & Docs

- **Cloudflare Pages Docs:** https://developers.cloudflare.com/pages/
- **Cloudflare Community:** https://community.cloudflare.com
- **Dioxus Docs:** https://dioxuslabs.com/learn/
- **Rust WASM:** https://rustwasm.org/
