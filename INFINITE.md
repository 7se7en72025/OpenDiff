# Deploy for Infinite Requests ♾️

This guide deploys the **caching backend** to Render (free) for truly unlimited requests.

---

## How It Works

**Multiple users → Shared cache → Infinite requests effectively**

- Backend caches all GitHub API responses for 1 hour
- Multiple users accessing the same PR? Only 1 API call to GitHub
- 100 users viewing facebook/react? Still just a few hundred API calls total
- Fallback: If backend is down, use direct GitHub API + token (5,000 req/hr)

---

## Deploy Backend to Render (Free!)

### 1. Sign up on Render

1. Go to [render.com](https://render.com)
2. Sign up with GitHub (instant, free)

### 2. Deploy the Node Backend

1. Click **New +** → **Web Service**
2. Connect your GitHub repo (`pr-reviewer`)
3. Fill in:
   - **Name**: `pr-reviewer-backend`
   - **Environment**: `Node`
   - **Build Command**: `npm install`
   - **Start Command**: `npm start`
   - **Plan**: `Free`
4. Click **Create Web Service**

Render will deploy and show you a URL like:
```
https://pr-reviewer-backend-xxxx.onrender.com
```

### 3. Copy Backend URL

Your backend URL is something like: `https://pr-reviewer-backend-xxxx.onrender.com`

---

## Update Frontend (GitHub Pages)

1. Open the app: `https://YOUR-USERNAME.github.io/pr-reviewer/index.html`
2. In the **Caching Backend** field, paste your Render URL
3. Done! The app auto-detects and uses the cache

---

## Optional: Add GitHub Token to Backend

For higher GitHub API quota:

1. Go to [github.com/settings/tokens/new](https://github.com/settings/tokens/new)
2. Create token with `public_repo` scope
3. Copy the token
4. On Render dashboard:
   - Go to your service
   - Click **Environment**
   - Add: `GITHUB_TOKEN` = your token
5. Deploy redeploys automatically

---

## How to Use

1. **Open**: `https://YOUR-USERNAME.github.io/pr-reviewer/index.html`
2. **Paste repo**: `https://github.com/facebook/react`
3. **Paste backend URL**: `https://pr-reviewer-backend-xxxx.onrender.com`
4. **Click Load PRs** → All cached, super fast!

---

## What You Get

✅ **Infinite requests** (shared cache)  
✅ **Super fast** (1-5 second loads, cached data)  
✅ **Free** (GitHub Pages + Render free tier)  
✅ **No rate limits** (backend caches everything)  
✅ **Share with team** (same cache for everyone)  

---

## Cache Details

- **TTL**: 1 hour (data refreshes hourly)
- **Storage**: In-memory (clears on Render redeploy)
- **Size**: Unlimited (limited by Render's memory)

To force a refresh, redeploy on Render (takes ~2 min).

---

## Cost

**Completely free!**

- GitHub Pages: ✅ Free
- Render free tier: ✅ Free  
- GitHub API: ✅ Free (with token)

Render's free tier:
- Includes 750 hours/month (enough for 24/7 app)
- Sleeps after 15 min inactivity (wakes instantly on request)
- Plenty fast for PR browsing

---

## Troubleshooting

**Q: Backend URL not working?**  
A: Check the URL in Render dashboard (Settings → URL)

**Q: Still getting rate limited?**  
A: Make sure backend URL is pasted correctly in the app

**Q: Data not updating?**  
A: Redeploy on Render to clear cache (Settings → Redeployments)

**Q: Backend is slow?**  
A: It's probably waking up from sleep. Give it 5-10 seconds first request.
