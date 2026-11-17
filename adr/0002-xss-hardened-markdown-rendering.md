# ADR 0002: XSS-Hardened and Nonce-Protected Markdown Rendering

## Status
Accepted

## Context
DTN bundles originate from remote actors on the amateur radio mesh network. Displaying untrusted content in the web UI (Preact/Vite) exposes the application to Cross-Site Scripting (XSS) attacks.

## Decision
1. Strict HTML sanitization (`escapeHtml`, `escapeAttribute`).
2. Replacement of code blocks and links with random tokens based on a cryptographic nonce (`crypto.getRandomValues`) and `\x00` control delimiters to prevent any collision or spoofing.
3. Default blocking of remote images and dangerous URI schemes (`javascript:`, `vbscript:`, non-raster `data:`).
4. Linear regex validation without nested quantifiers to prevent ReDoS attacks.

## Consequences
- Highest-level OWASP Top 10 A03:2021 (Injection) protection.
- Immunity against session cookie theft and sensitive information leakage in the browser.
