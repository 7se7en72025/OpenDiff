# Deploy to GitHub Pages

This is a pure frontend app - just HTML, no backend needed!

---

## Step 1: Initialize Git & Push to GitHub

```bash
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/YOUR-USERNAME/pr-reviewer.git
git branch -M main
git push -u origin main
```

Replace `YOUR-USERNAME` with your GitHub username.

---

## Step 2: Enable GitHub Pages

1. Go to your repository: `https://github.com/YOUR-USERNAME/pr-reviewer`
2. Click **Settings** → **Pages**
3. Set:
   - **Source**: Deploy from a branch
   - **Branch**: `main`
   - **Folder**: `/ (root)`
4. Click **Save**

GitHub will deploy automatically. Your app is now live at:
```
https://YOUR-USERNAME.github.io/pr-reviewer/
```

---

## Step 3: Use the App

1. Open the URL above
2. Paste a GitHub repository URL (e.g., `https://github.com/facebook/react`)
3. (Optional) Add a GitHub Personal Access Token for higher rate limits:
   - Create one at: https://github.com/settings/tokens/new (scope: `public_repo`)
   - Paste it in the app
4. Click **Load PRs**

---

## Features

✅ **Review Tracking**: Check/uncheck PRs you're reviewing (max 10 per user)
✅ **File Diffs**: View all file changes line-by-line
✅ **Comments**: See all PR comments and reviews
✅ **Run PR**: Open the PR in StackBlitz, CodeSandbox, Gitpod, or GitHub Web Editor
✅ **No Backend Needed**: Works directly with GitHub API
✅ **Per-Browser**: Each browser/device tracks its own reviewing list

---

## How Review Tracking Works

- Each browser gets a unique user ID (stored in localStorage)
- You can review up to **10 PRs at a time**
- Check a PR to add it to your list
- Uncheck when you're done reviewing
- Your list persists even after closing the page

---

## Rate Limits

- **Without token**: 60 requests/hour (GitHub's public API limit)
- **With token**: 5,000 requests/hour (GitHub's authenticated limit)

---

## Updates

To update the app after it's deployed:

```bash
git add .
git commit -m "Update description"
git push origin main
```

GitHub Pages will automatically redeploy (usually within a minute).
