# Security Policy — `template-dtnpm-mod`

The `template-dtnpm-mod` project enforces European **Cyber Resilience Act (CRA)** requirements and the **OWASP Top 10** standard.

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.1.x   | ✅ Yes    |

## Security Principles
- **Authentication & BPSec (RFC 9172)**: Strict mode by default. Any unauthenticated or forged bundle is rejected at the module boundary.
- **Secret Management**: Bearer tokens must be passed via file using `--token-file` or `DTN_HDY_TOKEN_FILE` to avoid any leakage in `/proc/<pid>/cmdline`.
- **XSS / DOM Mitigation**: All Markdown content originating from remote peers is rendered via the hardened `renderMarkdown` utility using ephemeral cryptographic nonces.
- **No Panics**: Production code guaranteed free of `unwrap()` and `panic!`.

## Reporting a Vulnerability

To report a security vulnerability, please contact the security team at:
`security@dtn-postmaster.org` or open a private Security Advisory (GitHub Security Advisory).
