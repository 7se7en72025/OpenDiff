# Option 3: Token + Browser Caching = Infinite (No Backend)

**5,000 API requests/hour + infinite cached requests within your browser session**

---

## How It Works

### First Request to Repo
```
You load facebook/react
→ App calls GitHub API (uses 1 request)
→ Data stored in browser cache
→ Shows on screen
```

### Second Request to Same Repo
```
You load facebook/react again
→ App finds it in browser cache
→ ZERO API calls used
→ Instant load
```

### Why This Works
- **Same browser**: Cache persists entire session
- **Same data**: No API call needed
- **Unlimited repos**: Just keep loading different repos
- **Real infinite**: 5,000 API calls + unlimited cached revisits

---

## Setup (No Backend Needed!)

### 1. Deploy to GitHub Pages

```bash
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/YOUR-USERNAME/pr-reviewer.git
git branch -M main
git push -u origin main
```

Then:
1. Go to repo Settings → Pages
2. Source: `main` branch, `/` folder
3. Done! Live at `https://YOUR-USERNAME.github.io/pr-reviewer/index.html`

### 2. Get GitHub Token (Optional, But Recommended)

1. Go to https://github.com/settings/tokens/new
2. Create token with `public_repo` scope
3. Copy token
4. Paste in app under "GitHub Token"

Now you have **5,000 requests/hour** instead of 60!

---

## Usage

1. **First load**: `facebook/react` → Uses 1 API call (now cached)
2. **Revisit**: Click "Change repo", paste `facebook/react` again → Zero API calls, instant!
3. **View cache stats**: Click **📊 X cached** button in topbar
4. **Clear cache**: Go back home, click 🗑 **Clear Cache** button

---

## Cache Details

| Metric | Value |
|--------|-------|
| **Storage Location** | Browser localStorage |
| **Cache Time** | 1 hour per item |
| **Max Items** | ~5MB (browser limit) |
| **Persists** | Until browser closes or you clear cache |
| **Per Browser** | Each browser has its own cache |

---

## Example Flow

**Day 1:**
```
Load 50 repos = 50 API calls used
Close app
```

**Day 2 (same browser):**
```
Revisit 50 repos = 0 API calls used ✅
Load 50 new repos = 50 API calls used
Total = 50 calls (out of 5,000 daily limit)
```

---

## When to Use This

✅ **Best for**: 1-3 users, same repos repeatedly, offline fallback  
✅ **Cost**: $0 (GitHub Pages is free)  
✅ **Setup time**: 5 minutes  
✅ **API quota**: 5,000 req/hour (with token)  

❌ **Not good for**: 100+ concurrent users, brand new repos constantly

---

## Upgrade to Backend Caching

If you hit the 5,000 req/hour limit, upgrade to [backend caching](./INFINITE.md):

- Deploy `server.js` on Render (free)
- Shared cache across ALL users
- Truly unlimited requests
- Same simple setup, just 2 more minutes

---

## Tips

**Maximize cache hits:**
- Review repos repeatedly → reuse cache
- Same team repos → share cached data
- Offline mode → works even without internet!

**Cache status:**
- Click **📊 X cached** to see hits vs API calls
- Tip: More hits = fewer API calls used = better!

**Manual refresh:**
- To force fresh data, click 🗑 **Clear Cache** and reload

---

## Limitations

- **Per browser**: Cache doesn't sync across browsers
- **Expires after 1 hour**: Old entries auto-delete
- **Browser-only**: Clear cookies/cache and it's gone
- **5,000/hour ceiling**: Still have GitHub API limit (but caching delays hitting it)

---

**That's it!** You now have effectively infinite requests for $0. 🚀
