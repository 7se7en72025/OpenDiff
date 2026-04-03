/**
 * PR Reviewer — Caching Backend Server
 * Caches all GitHub API responses → Infinite requests!
 * Multiple users share the cache, so only 1 API call per unique PR.
 *
 * Deploy: Render.com (free tier)
 * Setup:
 *   npm install
 *   npm start
 *   (optional) export GITHUB_TOKEN=ghp_xxx for 5000 req/hr quota
 */

require('dotenv').config();
const express = require('express');
const cors    = require('cors');

const app  = express();
const PORT = process.env.PORT || 3001;
const TOKEN = process.env.GITHUB_TOKEN || '';

app.use(cors());
app.use(express.json());
app.use(express.static(__dirname));

// ── CACHE (in-memory) ────────────────────────────────────────────────────────
const cache = new Map();
const CACHE_TTL = 3600000; // 1 hour

function cacheKey(path) { return 'gh:' + path; }

function getCached(path) {
  const key = cacheKey(path);
  const item = cache.get(key);
  if (item && Date.now() - item.time < CACHE_TTL) return item.data;
  cache.delete(key);
  return null;
}

function setCached(path, data) {
  cache.set(cacheKey(path), { data, time: Date.now() });
}

// ── helpers ──────────────────────────────────────────────────────────────────
async function gh(path) {
  // Check cache first
  const cached = getCached(path);
  if (cached) return cached;

  // Fetch from GitHub
  const headers = { 'Accept': 'application/vnd.github+json', 'X-GitHub-Api-Version': '2022-11-28' };
  if (TOKEN) headers['Authorization'] = `Bearer ${TOKEN}`;
  const r = await fetch('https://api.github.com' + path, { headers });
  const data = await r.json();
  if (!r.ok) throw Object.assign(new Error(data.message || 'GitHub error'), { status: r.status });
  
  // Cache successful responses
  setCached(path, data);
  return data;
}

function wrap(fn) {
  return async (req, res) => {
    try { res.json(await fn(req, res)); }
    catch (e) { res.status(e.status || 500).json({ error: e.message }); }
  };
}

// ── routes ────────────────────────────────────────────────────────────────────

// Health + rate limit + cache info
app.get('/api/status', wrap(async () => {
  const r = await gh('/rate_limit');
  return { 
    ok: true, 
    rateLimit: r.resources?.core,
    cache: { size: cache.size, ttl: CACHE_TTL / 1000 + 's' },
    token: TOKEN ? '✓ ' + TOKEN.slice(0,8) + '…' : '✗ not set (5000 req/hr without token)'
  };
}));

// Repo info (cached)
app.get('/api/repos/:owner/:repo', wrap(async req => {
  const { owner, repo } = req.params;
  return gh(`/repos/${owner}/${repo}`);
}));

// PR list (cached)
app.get('/api/repos/:owner/:repo/pulls', wrap(async req => {
  const { owner, repo } = req.params;
  const state = req.query.state || 'open';
  const per_page = Math.min(parseInt(req.query.per_page) || 100, 100);
  const page = parseInt(req.query.page) || 1;
  return gh(`/repos/${owner}/${repo}/pulls?state=${state}&per_page=${per_page}&page=${page}&sort=updated`);
}));

// PR detail bundled (cached separately for each component)
app.get('/api/repos/:owner/:repo/pulls/:num/detail', wrap(async req => {
  const { owner, repo, num } = req.params;
  const base = `/repos/${owner}/${repo}`;

  const [comments, reviewComments, files, reviews, deployments] = await Promise.all([
    gh(`${base}/issues/${num}/comments?per_page=100`).catch(() => []),
    gh(`${base}/pulls/${num}/comments?per_page=100`).catch(() => []),
    gh(`${base}/pulls/${num}/files?per_page=100`).catch(() => []),
    gh(`${base}/pulls/${num}/reviews?per_page=100`).catch(() => []),
    gh(`${base}/deployments?per_page=10`).catch(() => []),
  ]);

  // Resolve deployment URLs
  const deplsWithUrls = await Promise.all(
    (deployments || []).slice(0, 5).map(async d => {
      try {
        const statuses = await gh(`${base}/deployments/${d.id}/statuses?per_page=1`);
        return { ...d, envUrl: statuses?.[0]?.environment_url || statuses?.[0]?.target_url || '' };
      } catch { return d; }
    })
  );

  return { comments, reviewComments, files, reviews, deployments: deplsWithUrls };
}));

app.listen(PORT, () => {
  console.log(`\n✅ PR Reviewer backend running at http://localhost:${PORT}`);
  console.log(`   GitHub token: ${TOKEN ? '✓ loaded (' + TOKEN.slice(0,8) + '…)' : '✗ not set (add to .env)'}`);
  console.log(`   Test: http://localhost:${PORT}/api/status\n`);
});
