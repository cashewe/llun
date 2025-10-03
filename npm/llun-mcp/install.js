// install.js - download the correct GitHub release binary for this platform
const https = require('https');
const fs = require('fs');
const path = require('path');

function repoFromPackageJson() {
  try {
    const pkg = require('./package.json');
    const url = (pkg.repository && pkg.repository.url) || '';
    // expect either https://github.com/owner/repo.git or https://github.com/owner/repo
    const m = url.match(/github.com[:\/]([^\/]+)\/([^\/]+)(?:\.git)?$/i);
    if (m) return { owner: m[1], repo: m[2] };
  } catch (e) {}
  return null;
}

const repo = repoFromPackageJson();
if (!repo) {
  console.error('Cannot determine GitHub repo from package.json repository.url.');
  process.exit(1);
}

const { owner, repo: repoName } = repo;
const pkg = require('./package.json');
let tag = pkg.version || '';
if (!tag) tag = 'latest';
if (!/^v/.test(tag) && tag !== 'latest') tag = 'v'+tag; // releases use vX.Y.Z

const map = {
  'linux-x64': 'x86_64-unknown-linux-musl',
  'linux-arm64': 'aarch64-unknown-linux-musl',
  'darwin-x64': 'x86_64-apple-darwin',
  'darwin-arm64': 'aarch64-apple-darwin',
  'win32-x64': 'x86_64-pc-windows-msvc'
};

const plat = process.platform;
const arch = process.arch;
const key = `${plat}-${arch}`;
const target = map[key];

if (!target) {
  console.error(`Unsupported platform/arch: ${plat}/${arch}`);
  process.exit(1);
}

const ext = plat === 'win32' ? '.exe' : '';
const assetName = `llun-mcp-${tag}-${target}${ext}`;
const downloadUrl = `https://github.com/${owner}/${repoName}/releases/download/${tag}/${assetName}`;

const outDir = path.join(__dirname, 'bin');
if (!fs.existsSync(outDir)) fs.mkdirSync(outDir, { recursive: true });
const outPath = path.join(outDir, `llun-mcp${ext}`);

console.log('Downloading', downloadUrl, '→', outPath);

https.get(downloadUrl, (res) => {
  if (res.statusCode !== 200) {
    console.error(`Failed to download ${downloadUrl}: ${res.statusCode}`);
    res.pipe(process.stdout);
    process.exit(1);
  }
  const file = fs.createWriteStream(outPath, { mode: 0o755 });
  res.pipe(file);
  file.on('finish', () => {
    file.close();
    if (plat !== 'win32') fs.chmodSync(outPath, 0o755);
    console.log('Installed llun-mcp to', outPath);
  });
}).on('error', (err) => {
  console.error('Download error:', err);
  process.exit(1);
});
