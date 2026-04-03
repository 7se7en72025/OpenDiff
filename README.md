# 🔀 OpenDiff - Team PR Review Tool

A **production-ready** PR review tool built with **Rust + Actix-web** backend and **Apple/Uber-style** modern frontend. Deploy in minutes, sync across 10+ team members in real-time.

## ✨ Features

- ✅ **Real-time Sync** - WebSocket-powered live updates across all devices
- 🚀 **Ultra-Fast** - Rust backend with zero-copy performance
- 📱 **Apple Design** - Glassmorphic UI with smooth animations
- 🔄 **Team Collaboration** - Multiple reviewers, exclusive PR assignment
- 💾 **MongoDB** - Cloud-ready database with instant backup
- 📊 **PR Analytics** - Track changes, approvals, and team velocity
- 🌐 **Fully Deployable** - Docker + Railway/Render ready
- ⚡ **No Build Tools** - Frontend is pure HTML/JS

## 🚀 Quick Start

### Prerequisites
- **Rust** 1.70+ → [Install](https://rustup.rs/)
- **Docker** (optional, for containerization)
- **MongoDB Atlas** (free cloud DB)

### 1️⃣ Setup Backend (Rust)

```bash
# Navigate to project
cd "pr reivew"

# Edit backend/.env with your MongoDB connection
# MONGODB_URI=mongodb+srv://username:password@cluster.mongodb.net/opendiff

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build backend
cd backend
cargo build --release

# Run backend
cargo run --release
# ✅ Server running on http://localhost:5000
```

### 2️⃣ Open Frontend

Frontend lives at: `http://localhost:5000`

### 3️⃣ Get MongoDB Atlas (Free)

1. Go to [mongodb.com/cloud/atlas](https://mongodb.com/cloud/atlas)
2. Create account (free tier = 512MB)
3. Create cluster
4. Get connection string
5. Add to `.env` file

## 🐳 Docker Deployment (Recommended)

```bash
# Build and run with Docker Compose
docker-compose up --build

# ✅ App running on http://localhost:5000
```

## 🚀 Production Deployment

### Render.com (Recommended)

```bash
# 1. Push to GitHub
git push -u origin main

# 2. Go to render.com → New → Web Service
# 3. Connect GitHub repo
# 4. Build: cargo build --release
# 5. Start: ./target/release/opendiff-backend
# 6. Add MONGODB_URI env var
# 7. Deploy!
```

## 🔧 Technology Stack

- **Backend**: Rust + Actix-web ⚡
- **Database**: MongoDB 📊
- **Frontend**: Vanilla JS + HTML 🚀
- **Real-time**: WebSockets 🔄
- **Deployment**: Docker 📦

## 🔌 API Endpoints

- `POST /api/auth/login` - Login
- `GET /api/prs` - Get PRs
- `POST /api/reviews` - Create review
- `WS /ws` - Real-time sync

---

**Built with ❤️ for Team Collaboration** 🚀
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
