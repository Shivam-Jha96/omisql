const fs = require('fs');
const path = require('path');
const os = require('os');
const { execSync } = require('child_process');
const https = require('https');

// Config
const REPO = 'Shivam-Jha96/omnisql';
const VERSION = process.env.OMNISQL_VERSION || 'v0.1.0';
const BIN_NAME = os.platform() === 'win32' ? 'omnisql.exe' : 'omnisql';

// Map Node.js os/arch to GitHub Release asset names
const getPlatform = () => {
    switch (os.platform()) {
        case 'win32': return 'windows';
        case 'darwin': return 'macos';
        case 'linux': return 'linux';
        default: throw new Error(`Unsupported platform: ${os.platform()}`);
    }
};

const getArch = () => {
    switch (os.arch()) {
        case 'x64': return 'x86_64';
        case 'arm64': return 'aarch64';
        default: throw new Error(`Unsupported architecture: ${os.arch()}`);
    }
};

const getAssetSuffix = (platform) => {
    if (platform === 'windows') return 'zip';
    return 'tar.gz';
};

async function downloadBinary() {
    const platform = getPlatform();
    const arch = getArch();
    const suffix = getAssetSuffix(platform);
    const assetName = `omnisql-${VERSION}-${arch}-${platform}.${suffix}`;
    const url = `https://github.com/${REPO}/releases/download/${VERSION}/${assetName}`;

    const binDir = path.join(__dirname, 'bin');
    if (!fs.existsSync(binDir)) {
        fs.mkdirSync(binDir);
    }
    
    const binPath = path.join(binDir, BIN_NAME);
    const tempFile = path.join(binDir, `temp.${suffix}`);
    
    console.log(`Downloading OmniSQL for ${platform}-${arch} from ${url}...`);

    try {
        const axios = require('axios');
        const response = await axios({
            method: 'get',
            url: url,
            responseType: 'stream',
            maxRedirects: 5
        });

        const writer = fs.createWriteStream(tempFile);
        response.data.pipe(writer);

        await new Promise((resolve, reject) => {
            writer.on('finish', resolve);
            writer.on('error', reject);
        });

        console.log('Download complete. Extracting...');

        if (suffix === 'zip') {
            const unzipper = require('unzipper');
            await fs.createReadStream(tempFile)
                .pipe(unzipper.Parse())
                .on('entry', function (entry) {
                    if (entry.path === 'omnisql.exe') {
                        entry.pipe(fs.createWriteStream(binPath));
                    } else {
                        entry.autodrain();
                    }
                })
                .promise();
        } else {
            const tar = require('tar');
            await tar.x({
                file: tempFile,
                cwd: binDir,
                filter: (path) => path === 'omnisql'
            });
            fs.chmodSync(binPath, 0o755);
        }

        fs.unlinkSync(tempFile);
        console.log(`Successfully installed OmniSQL to ${binPath}`);
    } catch (e) {
        console.error("Failed to install OmniSQL:", e);
        if (fs.existsSync(tempFile)) fs.unlinkSync(tempFile);
        process.exit(1);
    }
}

downloadBinary();
