# PR Reviewer

Browse, search, and review GitHub pull requests with built-in review tracking.

**Live demo**: [Deploy to GitHub Pages + Render](./INFINITE.md) for infinite cached requests!

---

## Features

✅ **Infinite Requests** — Deploy backend + caching for unlimited API calls
✅ **Review Tracking** — Check/uncheck PRs you're reviewing (max 10 per browser)
✅ **Search & Filter** — Find PRs by title or number  
✅ **View Details** — See diffs, comments, reviews in a clean UI
✅ **Run PR** — Open in StackBlitz, CodeSandbox, Gitpod, or GitHub Web Editor
✅ **Pure Frontend** — Works directly with GitHub API (no backend needed)
✅ **Shared Cache** — Deploy backend for 100x faster loads across users
✅ **Private Repos** — Add a GitHub token to access private repositories

---

## Quick Start (Local)

Just open `index.html` in a browser. 

For best results, run a local server:
```bash
npx serve .
```

Then open `http://localhost:3000`.

---

## Deploy to GitHub Pages

See [DEPLOY.md](./DEPLOY.md) for step-by-step instructions.

TL;DR:
1. Push to GitHub
2. Enable Pages in Settings
3. Done! Your app is live at `https://YOUR-USERNAME.github.io/pr-reviewer/`

---

## How It Works

1. **Paste a repo URL** — `https://github.com/owner/repo`
2. **(Optional) Add GitHub token** — Higher rate limits (5,000/hr vs 60/hr)
3. **Load PRs** — Fetches all open + closed PRs
4. **Check to review** — Track up to 10 PRs per browser session
5. **Click to view details** — See diffs, comments, deployments
6. **Run anywhere** — StackBlitz, CodeSandbox, Gitpod, VS Code Web

---

## Review Tracking

Each browser gets a unique user ID. You can:
- Check/uncheck PRs to mark them as "reviewing"
- Review up to **10 PRs at a time**
- Your list persists across sessions (localStorage)
- Switch browsers or clear localStorage to reset

---

## Rate Limits

| Mode | Limit |
|------|-------|
| No token | 60 requests/hour |
| With token | 5,000 requests/hour |

Get a token at: https://github.com/settings/tokens/new (scope: `public_repo`)

---

## Files

| File | Purpose |
|------|---------|
| `index.html` | The complete frontend — deploy this to GitHub Pages |
| `DEPLOY.md` | Step-by-step GitHub Pages deployment |
| `server.js` | (Optional) Node backend for deployment testing locally |
| `package.json` | Dependencies for local Node server |
npm run dev
```

Server starts at `http://localhost:3001`.

### 4. Connect the frontend

Open `pr-reviewer.html` in your browser and paste `http://localhost:3001` in the **Backend Server URL** field.

The green "Backend connected" badge confirms it's working.

---

## Deploy to a VPS / cloud

```bash
# On your server:
git clone your-repo
cd your-repo
npm install
cp .env.example .env  # add your token

# Run with PM2 (keeps it alive after reboot):
npm install -g pm2
pm2 start server.js --name pr-reviewer
pm2 save
pm2 startup
```

Then in `pr-reviewer.html`, set Backend URL to `https://your-server.com`.

---

## GitHub Pages hosting (frontend only)

1. Push `pr-reviewer.html` to a repo (rename to `index.html`)
2. Settings → Pages → Deploy from main branch
3. Your tool is live at `https://yourusername.github.io/your-repo/`

Token is saved in `localStorage` — only stored in your browser.
