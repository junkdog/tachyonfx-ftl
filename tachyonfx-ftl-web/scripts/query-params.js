function base64UrlEncode(bytes) {
    const bin = Array.from(bytes, b => String.fromCharCode(b)).join('');
    return btoa(bin)
        .replace(/\+/g, '-')
        .replace(/\//g, '_')
        .replace(/=+$/, ''); // strip padding
}

function base64Decode(b64) {
    const standardB64 = b64
        .replace(/-/g, '+')
        .replace(/_/g, '/')
        .padEnd(b64.length + (4 - b64.length % 4) % 4, '=');

    const bin = atob(standardB64);
    return Uint8Array.from([...bin].map(c => c.charCodeAt(0)));
}

function deflateAndEncode(str) {
    const inputBytes = new TextEncoder().encode(str);
    const compressed = pako.deflate(inputBytes, { raw: true });
    return base64UrlEncode(compressed);
}

function decodeAndInflate(b64) {
    const compressed = base64Decode(b64);
    const decompressed = pako.inflate(compressed, { raw: true });
    return new TextDecoder().decode(decompressed);
}

function updateCodeAndCanvas(editor, canvasInput) {
    const params = new URLSearchParams(window.location.search);
    const codeRaw = params.get("code_raw");
    const code = params.get("code");
    const canvas = params.get("canvas");
    const canvasId = params.get("canvas_id");

    if (codeRaw) {
        // Convert raw code to compressed format and update URL
        try {
            const compressed = deflateAndEncode(codeRaw);
            editor.setValue(codeRaw, -1);

            // Update URL: remove code_raw, set code
            const url = new URL(window.location);
            url.searchParams.delete("code_raw");
            url.searchParams.set("code", compressed);
            history.replaceState(null, "", url);
        } catch (e) {
            console.warn("Failed to encode raw code", e);
        }
    } else if (code) {
        try {
            let dsl = decodeAndInflate(code);
            editor.setValue(dsl, -1);
        } catch (e) {
            console.warn("Invalid compressed code in ?code", e);
        }
    }

    if (canvasId === "tfxdocs") {
        try {
            canvasInput.value = `\x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m┌\x1b[0m\x1b[38;5;142m\x1b[48;2;29;32;33m\x1b[1m tachyonfx live demo \x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m─────────────────┐\x1b[0m
\x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m│\x1b[0m\x1b[48;2;29;32;33m      \x1b[0m\x1b[38;5;214m\x1b[48;2;29;32;33mTerminal effects\x1b[0m\x1b[48;2;29;32;33m rendered       \x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m│\x1b[0m
\x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m│\x1b[0m\x1b[48;2;29;32;33m     in your \x1b[0m\x1b[38;5;109m\x1b[48;2;29;32;33mbrowser\x1b[0m\x1b[48;2;29;32;33m with \x1b[0m\x1b[38;5;167m\x1b[48;2;29;32;33mWebGL2\x1b[0m\x1b[48;2;29;32;33m!     \x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m│\x1b[0m
\x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m│\x1b[0m\x1b[48;2;29;32;33m                                      \x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m│\x1b[0m
\x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m│\x1b[0m\x1b[48;2;29;32;33m      ▁▂▃▄▅▆▇█  \x1b[0m\x1b[38;5;142m\x1b[48;2;29;32;33m● \x1b[0m\x1b[38;5;214m\x1b[48;2;29;32;33m● \x1b[0m\x1b[38;5;167m\x1b[48;2;29;32;33m●\x1b[0m\x1b[48;2;29;32;33m  █▇▆▅▄▃▂▁       \x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m│\x1b[0m
\x1b[0m\x1b[38;5;208m\x1b[48;2;29;32;33m└──────────────────────────────────────┘\x1b[0m
`;
        } catch (e) {
            console.warn("Invalid compressed buffer in ?canvas", e);
        }
    } else if (canvas) {
        try {
            canvasInput.value = decodeAndInflate(canvas);
        } catch (e) {
            console.warn("Invalid compressed buffer in ?canvas", e);
        }
    }
}


// Query param utils
function updateQueryParam(key, value) {
    const url = new URL(window.location);

    if (value === null) {
        url.searchParams.delete(key);
    } else {
        url.searchParams.set(key, value);
    }

    history.replaceState(null, "", url);
}