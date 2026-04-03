# Deploy Backend for Large Teams (10+ People)

## The Problem

**Current setup (Token + Cache):**
```
Person 1 loads facebook/react → 250 API calls (uses GitHub quota)
Person 2 loads facebook/react → 250 API calls (uses GitHub quota)
Person 3 loads facebook/react → 250 API calls (uses GitHub quota)
...
= 250 calls × 10 people = 2,500 calls WASTED on same data
= Everyone hits rate limit quickly
= Broken for teams ❌
```

**With Backend Cache:**
```
Person 1 loads facebook/react → 250 API calls → Cached on server
Person 2 loads facebook/react → 0 API calls (uses server cache) ✅
Person 3 loads facebook/react → 0 API calls (uses server cache) ✅
...
= 250 calls total across 100 people
= Never hits rate limit
= Perfect for teams ✅
```

---

## Deploy in 3 Steps (5 minutes)

### Step 1: Push Code to GitHub

```bash
cd "C:\Users\RAIYYAN\Desktop\pr reivew"
git init
git add .
git commit -m "PR Reviewer with backend cache"
git remote add origin https://github.com/YOUR-USERNAME/pr-reviewer.git
git branch -M main
git push -u origin main
```

Replace `YOUR-USERNAME` with your actual GitHub username.

---

### Step 2: Deploy Backend to Render

**2a. Go to [render.com](https://render.com)**
- Sign up with GitHub (takes 30 seconds)

**2b. Create Web Service**
1. Click **New +** → **Web Service**
2. Select your `pr-reviewer` repository
3. Fill in these settings:
   ```
   Name: pr-reviewer-backend
   Environment: Node
   Region: (default)
   Build Command: npm install
   Start Command: npm start
   Plan: Free
   ```
4. Scroll down, click **Create Web Service**

**Render will deploy automatically.** Takes ~2-3 minutes.

When done, you'll see:
```
✅ pr-reviewer-backend-xxxx.onrender.com
```

Copy this URL. ← **You need this in Step 3**

---

### Step 3: Connect Frontend to Backend

1. **Open your app** (on GitHub Pages or local):
   ```
   https://YOUR-USERNAME.github.io/pr-reviewer/index.html
   ```
   OR
   ```
   http://localhost:3000/index.html
   ```

2. **Go back to home** (click logo)

3. **Paste your GitHub token** (optional but recommended):
   - Get one: https://github.com/settings/tokens/new
   - Scope: `public_repo`

4. **That's it!** 
   - The app auto-detects the backend
   - All teams use the same cache
   - No more wasted API calls ✅

---

## How Backend Cache Works

### Server-Side Caching (Render)
```
cache = {
  "/repos/facebook/react/pulls?...": { data, timestamp },
  "/repos/torvalds/linux/pulls?...": { data, timestamp },
  ... (1000+ repos)
}
Cache expires after 1 hour
All users share the same cache
```

### Example: 50 Developers Loading Same Repo
```
Dev 1: Loads facebook/react → API call → Result cached
Dev 2: Loads facebook/react → Server cache (0 API calls) ✅
Dev 3: Loads facebook/react → Server cache (0 API calls) ✅
...
Dev 50: Loads facebook/react → Server cache (0 API calls) ✅

Total: 1 API call for 50 developers 🎉
```

---

## Performance Impact

### Before (Everyone Using Token + Cache)
```
Load time: 5-10 seconds per unique repo
API calls: 250 per repo per person
Team waste: 250 × number of people

Example (10 developers):
- Load 10 different repos = 25,000 API calls 😱
- Everyone hitting own 5,000/hr limit
- Unusable after 1-2 hours
```

### After (With Backend Cache)
```
Load time: 1-3 seconds
API calls: 250 per repo TOTAL (all people combined)
Team waste: None

Example (10 developers):
- Load 10 different repos = 2,500 API calls ✅
- Can keep using all day
- 20x fewer API calls
```

---

## What Gets Cached on Backend

✅ **Cached (reused across all users):**
- PR lists (open, closed, merged)
- PR details (comments, reviews, files)
- Repository info
- User profiles

❌ **Not cached (per-user):**
- Review status (each person's personal tracking)
- Search results (filtered locally)
- Bookmarks (localStorage, per browser)

---

## Render Deployment Details

### Free Tier Limits
- **Uptime**: 750 hours/month (~24/7)
- **Memory**: 512MB
- **Spin-down**: After 15 min inactivity (wakes instantly)
- **Cache size**: ~100MB (plenty for 1000+ repos)
- **Cost**: $0/month

### First Request After Sleep
```
Request comes in → Render wakes server → Takes ~5-10 sec
Next requests → Instant (server is awake)
```

### Typical Server Usage
```
Hour 1: All requests cached → Server uses 5% CPU
Hour 1:30: Cache expires → 1-2 refreshes → 10% CPU spike
Hour 2: All cached again → 5% CPU
= Very efficient, costs $0
```

---

## Optional: Add GitHub Token to Backend

For better rate limits on the backend server itself:

1. **Generate GitHub token** (if you haven't):
   - Go to https://github.com/settings/tokens/new
   - Scope: `repo` (for private repos too)
   - Copy token

2. **Add to Render backend**:
   - Go to Render dashboard
   - Click your `pr-reviewer-backend` service
   - Click **Environment**
   - Add new variable:
     ```
     Key: GITHUB_TOKEN
     Value: ghp_xxxxxxxxxxxxx (your token)
     ```
   - Click **Save** (auto-redeploys)

3. **Backend now has**:
   - 15,000 API calls/hour (instead of 5,000)
   - Can access private repos
   - No more rate limit issues

---

## Testing It's Working

### 1. Check Backend Health
```
Open: https://pr-reviewer-backend-xxxx.onrender.com/api/status

Should show:
{
  "ok": true,
  "rateLimit": { "remaining": 4999, ... },
  "cache": { "size": 42, "ttl": "3600s" }
}
```

### 2. Load a Repo Multiple Times
1. Open app: `https://YOUR-USERNAME.github.io/pr-reviewer/index.html`
2. Load `facebook/react` → Takes 5 seconds (API call)
3. Go back home (click logo)
4. Load `facebook/react` again → Takes <1 second (cached!) ✅
5. Open in different browser tab → <1 second ✅
6. Have teammate load it → <1 second ✅

---

## Team Onboarding

Share with your team:
```
🎉 PR Reviewer is now live!

URL: https://YOUR-USERNAME.github.io/pr-reviewer/index.html

Features:
✅ Search all PRs
✅ Review tracking (check 10 at a time)
✅ File diffs
✅ Run in StackBlitz/CodeSandbox

Token (optional): Add for offline support
Cache: Shared across all team members
```

---

## Troubleshooting

**Q: App still says "offline"**
A: Wait 2-3 minutes for Render to fully deploy. Refresh page.

**Q: Backend URL not working**
A: Check Render dashboard, copy exact URL from Settings → URL

**Q: Still getting rate limited**
A: Make sure nobody is hammering API directly. Backend should handle it.

**Q: Cache has stale data**
A: Cache expires every 1 hour. Manual refresh: Redeploy on Render (Settings → Manual Deploy)

**Q: Server goes to sleep**
A: Normal on free tier. First request wakes it (~5-10 sec). Subsequent requests are instant.

---

## Cost Breakdown

| Item | Cost |
|------|------|
| GitHub Pages (frontend) | $0 |
| Render (backend) | $0 |
| GitHub API (with token) | $0 |
| **Total Monthly** | **$0** |

---

## Next Steps

1. ✅ Deploy backend to Render (this page)
2. ✅ Share URL with team
3. ✅ Watch API usage drop 90%
4. ✅ Everyone's happy

**You're done!** 🚀 Your PR Reviewer now scales to 100+ people for free.

---

## Still Have Questions?

- **CACHING.md** — How caching works
- **RESTRICTIONS.md** — Technical limits
- **INFINITE.md** — Detailed backend setup
- **GITHUB.md** — Frontend deployment
