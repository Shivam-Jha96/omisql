#!/usr/bin/env node

const { spawnSync } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

const BIN_NAME = os.platform() === 'win32' ? 'omnisql.exe' : 'omnisql';
const binPath = path.join(__dirname, 'bin', BIN_NAME);

if (!fs.existsSync(binPath)) {
    console.error(`OmniSQL binary not found at ${binPath}.`);
    console.error('Please run "npm install" to download the binary.');
    process.exit(1);
}

const args = process.argv.slice(2);
const result = spawnSync(binPath, args, { stdio: 'inherit' });

process.exit(result.status || 0);
