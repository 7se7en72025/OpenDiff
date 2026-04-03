# Plain HTML PR Reviewer - Complete Guide

## All Possible Approaches (Ranked)

### ✅ RECOMMENDED: Option 3 (Current Setup)
**Token + Browser Caching**
- **5,000 API calls/hour** (with GitHub token)
- **∞ cached requests** (browser localStorage)
- **Cost**: $0
- **Setup**: 5 minutes
- **Works**: Instantly, no backend
- **Limitation**: Single browser, 1-hour cache expiry

**How it works:**
```
Load repo → 1 API call → Cache data locally
Revisit repo → 0 API calls → Use cached data ✅
```

---

### ⭐ BEST: Option 1 (Backend Caching)
**Deploy backend + browser cache**
- **∞ true infinite** (shared server cache)
- **Cost**: $0 (free Render tier)
- **Setup**: 10 minutes
- **Works**: 100+ concurrent users
- **Limitation**: Requires deploying Node.js server

**How it works:**
```
User A loads repo → API call to GitHub → Cache on server
User B loads same repo → 0 API calls → Use server cache ✅
```

---

### ⚠️ NOT RECOMMENDED: Option 2 (Token Only)
**Just GitHub token, no caching**
- **5,000 API calls/hour**
- **Each load uses API** (no caching)
- **Cost**: $0
- **Setup**: 2 minutes
- **Works**: Light use only
- **Limitation**: Hit 5,000 limit quickly if browsing lots

**Problem:**
```
Load facebook/react → 10 API calls
Load tensorflow/tensorflow → 10 API calls
Load pytorch/pytorch → 10 API calls
= 30 calls used, still 4,970 left for 97th repo
```

---

### ❌ NOT VIABLE: Option 4 (No Token, No Cache)
**Plain HTML, nothing else**
- **60 API calls/hour limit** (public GitHub API)
- **Each load uses API**
- **Cost**: $0
- **Setup**: 0 minutes
- **Works**: Testing only
- **Limitation**: Ultra-limited, unusable

---

## Restrictions with Plain HTML ♾️

### JavaScript-Only Frontend Limitations

| Restriction | Why | Workaround |
|---|---|---|
| **CORS blocks direct API** | Browsers won't let JS call GitHub API directly | Need token in frontend (risky) OR use backend proxy |
| **No server-side storage** | Can't store millions of PRs anywhere | Use browser localStorage (5-10MB limit) |
| **No token hiding** | Token visible in network requests if in frontend | Move token to backend (but we don't have one in pure HTML) |
| **Can't batch requests** | Each PR detail = multiple API calls | Bundle requests in backend OR accept the cost |
| **No request deduplication** | If 10 browsers load same PR, 10 API calls | Backend cache deduplicates (we don't have it) |
| **Cross-tab sharing impossible** | Cache only works in current browser tab | Multi-tab = multiple caches = wasted API calls |
| **No background jobs** | Can't cache in background | Everything is synchronous/on-demand |
| **localStorage full quickly** | Can store ~5-10MB per domain | Cache expires/rotates, only stores recent repos |
| **No persistent cache** | Browser cache cleared = all gone | Rebuild on next load |
| **Single origin only** | All requests from client browser | Can't aggregate data from multiple origins |

---

## What You CAN Do (Plain HTML)

✅ **Browser-side caching** (localStorage, IndexedDB)
✅ **Token-based auth** (5,000 req/hr GitHub limit)
✅ **Pagination** (load PRs in chunks)
✅ **Search/filter locally** (don't re-query)
✅ **Lazy load details** (fetch PR details on demand)
✅ **Session persistence** (reload page keeps cache)
✅ **IndexedDB** (larger cache than localStorage ~50MB)
✅ **Service Workers** (advanced caching)

---

## What You CAN'T Do (Plain HTML)

❌ **Hide GitHub token** (visible in network tab)
❌ **Share cache between browsers/devices**
❌ **Request deduplication** (if 2 browsers load same PR = 2 API calls)
❌ **Batch queries** (Max ~200 API calls at once, then wait)
❌ **Server-side filtering** (must load all then filter)
❌ **Persistent storage** (only for current session)
❌ **Rate limit pooling** (can't combine quotas)
❌ **Custom authentication** (stuck with GitHub token)
❌ **Background processing** (everything blocking)

---

## API Call Breakdown

### Loading 50 PRs from facebook/react (Plain HTML + Token)

**Per PR you need:**
1. PR info = 1 call
2. Comments = 1 call
3. File changes = 1 call
4. Reviews = 1 call
5. Deployments = 1 call
= **5 calls per PR**

**Total for 50 PRs:**
- 50 PRs × 5 calls = 250 API calls
- With 5,000/hr limit = Can load 20 repos like this before limit
- Then you're done for the hour

**With Browser Cache (Current Setup):**
- First load: 250 calls
- Revisit: 0 calls ✅
- Load different repo: 250 calls
- Revisit that repo: 0 calls ✅
- Can revisit unlimited repos instantly

---

## Real-World Limits

### Scenario 1: Solo Developer (Current Setup)
```
Option 3: Token + Browser Cache
Personal use, same repos daily

Day 1: Load 5 repos = 1,250 calls (250 each)
Day 2: Revisit 5 repos = 0 calls ✅ (cached)
      Load 5 new repos = 1,250 calls
Total: 2,500 calls (50% of quota), everything cached for future
Status: ✅ Perfectly fine
```

### Scenario 2: Small Team (3 Developers)
```
Option 3: Token + Browser Cache
Each browser has separate cache

Dev 1 loads repo = 250 calls cached locally
Dev 2 loads same repo = 250 calls (no sharing!)
Dev 3 loads same repo = 250 calls (no sharing!)
Total: 750 calls for 1 repo across 3 devs
Status: ⚠️ Works but wasteful (should use backend)
```

### Scenario 3: Large Team (10+ People)
```
Option 3: Token + Browser Cache
Each person independently hits rate limit!

Person 1: 250 calls per repo × 4 repos = 1,000
Person 2: 250 calls per repo × 4 repos = 1,000
...
Person 10: 250 calls per repo × 4 repos = 1,000
= 10,000 calls TOTAL, rate limit hit
Status: ❌ Doesn't work (need backend Option 1)
```

### Scenario 4: Large Team with Backend (Option 1)
```
Backend + Shared Cache
All users share server cache

Person 1 loads repo = 250 calls (cached on server)
Person 2 loads repo = 0 calls (server cache) ✅
Person 3 loads repo = 0 calls (server cache) ✅
...
Person 100 loads repo = 0 calls (server cache) ✅
= 250 total calls for 100 people!
Status: ✅ Perfect scaling
```

---

## Performance Comparison

| Metric | No Token | Token Only | Token + Cache | Backend + Cache |
|--------|----------|-----------|---------------|-----------------|
| First load | 20 sec (60/hr limit) | 5 sec | 5 sec | 2 sec |
| Revisit same repo | 20 sec | 5 sec | Instant (cache) | Instant (cache) |
| 10 concurrent users | Broken | Broken | Works (separate caches) | Perfect (shared cache) |
| Monthly cost | $0 | $0 | $0 | $0 (free Render) |
| Setup time | 2 min | 2 min | 5 min | 10 min |

---

## Storage Limits (Plain HTML)

### Browser localStorage
```
Limit: ~5-10MB per domain
PR list example:
- 1000 PRs = ~2-3MB
- Can store ~2-3 repos worth of data
- Old entries auto-expire (1 hour TTL)
```

### IndexedDB (Advanced)
```
Limit: ~50MB per domain (can request more)
PR list example:
- 1000 PRs = ~2-3MB
- Can store ~10-15 repos worth of data
- Persists longer (manual expiry)
```

### Service Workers + Cache API (Very Advanced)
```
Limit: ~50-100MB (browser dependent)
Best for: App shells, static assets
Not ideal for: Dynamic PR data (better with backend)
```

---

## Current Setup Analysis

### You Have Now (Option 3)
✅ **5,000 API calls/hour** (token)
✅ **Browser caching** (localStorage)
✅ **$0 cost**
✅ **Works offline** (cached data)
✅ **Instant revisits** (cache hits)

❌ **Not sharable** (each browser independent)
❌ **No team scaling** (each user hits own limit)
❌ **Cache cleared on browser reset**

### When to Upgrade to Option 1 (Backend)
- **Team > 3 people**
- **Heavy daily usage** (hitting 5,000 limit)
- **Want shared cache** (save API calls)
- **Need persistent storage**
- **Want cross-device access**

### Cost-Benefit
```
Option 3 (Current): Free but limited to ~1 person
Option 1 (Upgrade): Free but unlimited team
Time to upgrade: 5 minutes
```

---

## Technical Decisions Made

### ✅ What We Did Right
1. **Added browser caching** (localStorage with TTL)
2. **Token support** (5x GitHub limit)
3. **Cache stats** (show hits vs misses)
4. **Fallback option** (backend if available)
5. **Lazy loading** (fetch details on demand)

### ⚠️ Trade-offs of Plain HTML
| Decision | Pro | Con |
|----------|-----|-----|
| No backend | Simple deploy | Can't share cache |
| Browser cache only | Instant revisits | Per-device only |
| Token in frontend | No server needed | Visible in network |
| Direct API calls | No proxy needed | Subject to CORS |
| localStorage | Works offline | Only 5-10MB |

---

## Summary Table

```
┌─────────────────┬──────────┬────────┬─────────┬──────────┐
│ Approach        │ Cost     │ Setup  │ Limit   │ Sharing  │
├─────────────────┼──────────┼────────┼─────────┼──────────┤
│ Plain HTML      │ Free     │ 2 min  │ 60/hr   │ None     │
│ Token Only      │ Free     │ 2 min  │ 5K/hr   │ None     │
│ Token + Cache   │ Free     │ 5 min  │ 5K/hr   │ Per user │
│ Backend Cache   │ Free     │ 10 min │ ∞       │ All team │
└─────────────────┴──────────┴────────┴─────────┴──────────┘
```

---

## Recommendation

**For you right now:**
→ Stay with **Option 3** (Token + Cache) — perfect for solo dev

**If team grows:**
→ Upgrade to **Option 1** (Backend) in 5 minutes

**Files:**
- `CACHING.md` — How to use current setup
- `INFINITE.md` — How to upgrade to backend
- `GITHUB.md` — How to deploy to GitHub Pages
