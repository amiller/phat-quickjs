'use strict'; (function (g) {
    function m(b) { var a = b.charCodeAt(0) | 0; if (55296 <= a) if (56320 > a) if (b = b.charCodeAt(1) | 0, 56320 <= b && 57343 >= b) { if (a = (a << 10) + b - 56613888 | 0, 65535 < a) return k(240 | a >>> 18, 128 | a >>> 12 & 63, 128 | a >>> 6 & 63, 128 | a & 63) } else a = 65533; else 57343 >= a && (a = 65533); return 2047 >= a ? k(192 | a >>> 6, 128 | a & 63) : k(224 | a >>> 12, 128 | a >>> 6 & 63, 128 | a & 63) } function n() { } function p(b, a) {
        var f = void 0 === b ? "" : ("" + b).replace(/[\x80-\uD7ff\uDC00-\uFFFF]|[\uD800-\uDBFF][\uDC00-\uDFFF]?/g, m), d = f.length | 0, c = 0, e = 0, h = a.length | 0, q = b.length |
            0; h < d && (d = h); a: for (; c < d; c = c + 1 | 0) { b = f.charCodeAt(c) | 0; switch (b >>> 4) { case 0: case 1: case 2: case 3: case 4: case 5: case 6: case 7: e = e + 1 | 0; case 8: case 9: case 10: case 11: break; case 12: case 13: if ((c + 1 | 0) < h) { e = e + 1 | 0; break } case 14: if ((c + 2 | 0) < h) { e = e + 1 | 0; break } case 15: if ((c + 3 | 0) < h) { e = e + 1 | 0; break } default: break a }a[c] = b } return { written: c, read: q < e ? q : e }
    } var k = String.fromCharCode, t = g.Uint8Array || Array, r = n.prototype, l = g.TextEncoder; r.encode = function (b) {
        b = void 0 === b ? "" : ("" + b).replace(/[\x80-\uD7ff\uDC00-\uFFFF]|[\uD800-\uDBFF][\uDC00-\uDFFF]?/g,
            m); for (var a = b.length | 0, f = new t(a), d = 0; d < a; d = d + 1 | 0)f[d] = b.charCodeAt(d) | 0; return f
    }; r.encodeInto = p; if (!l) g.TextEncoder = n; else if (!(g = l.prototype).encodeInto) { var u = new l; g.encodeInto = function (b, a) { var f = b.length | 0, d = a.length | 0; if (f < ((d >> 1) + 3 | 0)) { var c = u.encode(b); if ((c.length | 0) < d) return a.set(c), { read: f, written: c.length | 0 } } return p(b, a) } }
})(globalThis);
(function (g) {
    class TextDecoder {
        constructor(encoding = 'utf-8') {
            const normalizedEncoding = encoding.toLowerCase();
            if (normalizedEncoding !== 'utf-8' && normalizedEncoding !== 'utf8') {
                throw new TypeError('Unsupported text encoding: ' + encoding);
            }
        }
        decode(bytes) {
            if (bytes == null) {
                return '';
            }
            let result = '';
            let i = 0;
            while (i < bytes.length) {
                let codePoint = bytes[i++];
                if (codePoint >= 0xC0) {
                    if (codePoint < 0xE0) {
                        codePoint = ((codePoint & 0x1F) << 6) | (bytes[i++] & 0x3F);
                    } else if (codePoint < 0xF0) {
                        codePoint = ((codePoint & 0x0F) << 12) | ((bytes[i++] & 0x3F) << 6) | (bytes[i++] & 0x3F);
                    } else {
                        codePoint = ((codePoint & 0x07) << 18) | ((bytes[i++] & 0x3F) << 12) | ((bytes[i++] & 0x3F) << 6) | (bytes[i++] & 0x3F);
                    }
                }
                result += String.fromCharCode(codePoint);
            }
            return result;
        }
    }
    g.TextDecoder = TextDecoder;
}
)(globalThis);
export default {};