#!/usr/bin/env node

/**
 * Wasm-pack post-build script
 * rust_noise.jsにTextDecoder/TextEncoderのpolyfillを注入
 */

const fs = require('fs');
const path = require('path');

const POLYFILL = `// AudioWorklet Polyfill for Chrome (Auto-injected by patch-wasm.js)
// DO NOT REMOVE - Required for AudioWorklet compatibility
if (typeof TextDecoder === 'undefined') {
    globalThis.TextDecoder = class {
        decode(input) {
            if (!input || input.length === 0) return '';
            let result = '';
            const uint8Array = input instanceof Uint8Array ? input : new Uint8Array(input);
            for (let i = 0; i < uint8Array.length; i++) {
                result += String.fromCharCode(uint8Array[i]);
            }
            return result;
        }
    };
}

if (typeof TextEncoder === 'undefined') {
    globalThis.TextEncoder = class {
        encode(str) {
            if (!str) return new Uint8Array(0);
            const arr = new Uint8Array(str.length);
            for (let i = 0; i < str.length; i++) {
                arr[i] = str.charCodeAt(i) & 0xFF;
            }
            return arr;
        }
    };
}

`;

const TARGET_FILE = path.join(__dirname, 'pkg', 'rust_noise.js');

console.log('========================================');
console.log('Patching rust_noise.js for AudioWorklet');
console.log('========================================\n');

try {
    const pkgDir = path.join(__dirname, 'pkg');
    if (!fs.existsSync(pkgDir)) {
        console.error('❌ ERROR: pkg/ directory not found');
        console.error('   → Run: wasm-pack build --target web\n');
        process.exit(1);
    }

    if (!fs.existsSync(TARGET_FILE)) {
        console.error('❌ ERROR: pkg/rust_noise.js not found');
        console.error('   → Run: wasm-pack build --target web\n');
        process.exit(1);
    }

    console.log(`Target: ${TARGET_FILE}`);

    let content = fs.readFileSync(TARGET_FILE, 'utf8');
    const originalSize = content.length;

    // 既存のpolyfillを削除
    if (content.includes('AudioWorklet Polyfill')) {
        console.log('✓ Removing old polyfill...');
        const lines = content.split('\n');
        let firstCodeLine = 0;
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i].trim();
            if (line && 
                !line.includes('AudioWorklet Polyfill') && 
                !line.includes('DO NOT REMOVE') && 
                !line.includes('TextDecoder') && 
                !line.includes('TextEncoder') && 
                !line.includes('globalThis') &&
                !line.startsWith('//') &&
                !line.startsWith('if (typeof') &&
                !line.includes('decode') &&
                !line.includes('encode') &&
                !line.includes('input') &&
                !line.includes('uint8Array') &&
                !line.includes('result') &&
                !line.includes('String.fromCharCode') &&
                line !== '{' &&
                line !== '}' &&
                line !== '};') {
                firstCodeLine = i;
                break;
            }
        }
        content = lines.slice(firstCodeLine).join('\n');
    }

    content = POLYFILL + content;

    fs.writeFileSync(TARGET_FILE, content, 'utf8');

    const newSize = content.length;
    const polyfillSize = newSize - originalSize;

    console.log('✓ Successfully patched pkg/rust_noise.js');
    console.log(`  Original size: ${originalSize} bytes`);
    console.log(`  Polyfill size: ${polyfillSize} bytes`);
    console.log(`  New size: ${newSize} bytes\n`);
    console.log('========================================');
    console.log('Patch complete!');
    console.log('========================================\n');

} catch (err) {
    console.error('❌ ERROR: Failed to patch rust_noise.js');
    console.error(`   ${err.message}\n`);
    process.exit(1);
}
