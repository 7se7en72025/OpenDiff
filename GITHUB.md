# GitHub Setup & Deployment Guide

## Step-by-Step: From Zero to Live

### 1️⃣ Create a GitHub Repository

1. Go to [github.com/new](https://github.com/new)
2. Create a repository called `pr-reviewer`
3. Leave "Add a README" unchecked
4. Click **Create repository**

### 2️⃣ Push Your Code

Run these commands in this folder:

```bash
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/YOUR-USERNAME/pr-reviewer.git
git branch -M main
git push -u origin main
```

Replace `YOUR-USERNAME` with your GitHub username.

### 3️⃣ Enable GitHub Pages

1. Go to your repo: `https://github.com/YOUR-USERNAME/pr-reviewer`
2. Click **Settings** (top right)
3. Click **Pages** (left sidebar)
4. Under "Build and deployment":
   - **Source**: Select "Deploy from a branch"
   - **Branch**: Select `main`
   - **Folder**: Select `/ (root)`
5. Click **Save**

GitHub will show: `Your site is live at https://YOUR-USERNAME.github.io/pr-reviewer/`

### 4️⃣ Test Your App

Open: `https://YOUR-USERNAME.github.io/pr-reviewer/index.html`

---

## Using the App

### 1. Load a Repository

- Paste any GitHub repo URL: `https://github.com/owner/repo`
- Click **Load PRs**

Example repos to try:
- `https://github.com/facebook/react`
- `https://github.com/nodejs/node`
- `https://github.com/torvalds/linux`

### 2. Review PRs

- **Check the checkbox** next to a PR to mark it as "reviewing"
- You can review **up to 10 PRs at a time**
- **Uncheck** to mark as done
- Your list persists even after closing the page

### 3. View Details

- **Click the PR** to see:
  - 💬 All comments and reviews
  - 📄 File changes and diffs
  - ▶ Run options (StackBlitz, CodeSandbox, etc.)

### 4. Run the PR

- Go to the **▶ Run** tab
- Click StackBlitz, CodeSandbox, Gitpod, or GitHub Web Editor
- Code opens instantly in the browser (no installation needed!)

---

## Optional: Add GitHub Token

For higher rate limits (5,000 req/hr instead of 60):

1. Go to [github.com/settings/tokens/new](https://github.com/settings/tokens/new)
2. Give it a name: `PR Reviewer`
3. Check only `public_repo` scope
4. Click **Generate token**
5. Copy the token
6. Paste in the app under "GitHub Token"

---

## Updates & Redeploy

To update the app:

```bash
git add .
git commit -m "Your changes"
git push origin main
```

GitHub Pages redeploys automatically (usually within 1 minute).

---

## File Structure (After Deployment)

Inside your GitHub repo:
```
pr-reviewer/
├── index.html          ← The app (served at /index.html)
├── DEPLOY.md           ← Detailed deployment guide
├── GITHUB.md           ← This file
├── README.md           ← Project info
├── server.js           ← (Optional) Test backend locally
├── package.json        ← Dependencies
└── .gitignore          ← Files to ignore
```

Your GitHub Pages site serves `index.html` directly.

---

## Troubleshooting

**Q: App shows "Invalid GitHub URL"**  
A: Paste the full URL: `https://github.com/owner/repo`

**Q: "Rate limit hit" error**  
A: Add a GitHub token (see above section)

**Q: Changes not showing up?**  
A: Git changes take 1-2 minutes to appear on GitHub Pages. Try:
- Hard refresh: `Ctrl+Shift+R` (Windows) or `Cmd+Shift+R` (Mac)
- Clear browser cache
- Check the page deploy status in Settings > Pages

**Q: Where's my review list?**  
A: It's stored locally in your browser. Clear browser data and it resets.

---

## You're Done! 🎉

Your PR reviewer is live. Share the URL:

```
https://YOUR-USERNAME.github.io/pr-reviewer/index.html
```

Or make a custom domain if you want (GitHub Pages supports this).