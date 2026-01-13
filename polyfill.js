// ========== AudioWorklet Polyfill for TextDecoder/TextEncoder ==========
// This polyfill is required for WASM (rust_noise.js) to work in AudioWorkletGlobalScope
// Some browsers don't provide TextDecoder/TextEncoder in the AudioWorklet context

if (typeof TextDecoder === 'undefined') {
    globalThis.TextDecoder = class TextDecoder {
        constructor(encoding = 'utf-8') {
            this.encoding = encoding;
        }
        
        decode(input) {
            if (!input) return '';
            const bytes = new Uint8Array(input);
            let result = '';
            for (let i = 0; i < bytes.length; i++) {
                result += String.fromCharCode(bytes[i]);
            }
            return result;
        }
    };
}

if (typeof TextEncoder === 'undefined') {
    globalThis.TextEncoder = class TextEncoder {
        constructor() {
            this.encoding = 'utf-8';
        }
        
        encode(input) {
            if (!input) return new Uint8Array(0);
            const bytes = new Uint8Array(input.length);
            for (let i = 0; i < input.length; i++) {
                bytes[i] = input.charCodeAt(i);
            }
            return bytes;
        }
    };
}

console.log('[Polyfill] TextDecoder/TextEncoder loaded in AudioWorkletGlobalScope');
