# Running raworc-mcp in Production

This guide summarizes hardening, deployment, and operations practices for a production-ready MCP server connected to Raworc.

## Key Settings

- `RAWORC_API_URL` — API base URL (e.g., `https://api.remoteagent.com/api/v0`)
- `RAWORC_AUTH_TOKEN` — Bearer token (recommended), or use `RAWORC_USERNAME` + `RAWORC_PASSWORD`
- `RAWORC_DEFAULT_SPACE` — Default space name for space-scoped routes
- `RAWORC_TIMEOUT` — Request timeout in seconds (default 30)
- `LOG_LEVEL` or `RUST_LOG` — Log level (e.g., `info`, `debug`, `trace`)
- `LOG_FORMAT` — `json` for machine-readable logs; omit for human-readable

## Reliability

- Automatic retries: transient HTTP errors (408/425/429/5xx) and timeouts/connect failures are retried with exponential backoff.
- 401 re-auth: if `RAWORC_USERNAME` and `RAWORC_PASSWORD` are provided, the client re-authenticates once and retries the call.
- Timeouts: HTTP client sets connect and request timeouts; tune via `RAWORC_TIMEOUT`.
- Graceful shutdown: the server listens for Ctrl+C (SIGINT) and exits cleanly.

## Logging

- Set `LOG_FORMAT=json` for structured logs and `LOG_LEVEL=info` (or `RUST_LOG=raworc_mcp=info,raworc_mcp::client=info`).
- All logs are emitted to stderr; JSON logs are suitable for log shipping.

## Example: systemd (Linux)

Create `/etc/systemd/system/raworc-mcp.service`:

```
[Unit]
Description=Raworc MCP Server
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
Environment=RAWORC_API_URL=https://api.remoteagent.com/api/v0
Environment=RAWORC_AUTH_TOKEN=YOUR_TOKEN
Environment=RAWORC_DEFAULT_SPACE=production
Environment=RAWORC_TIMEOUT=30
Environment=LOG_LEVEL=info
Environment=LOG_FORMAT=json
ExecStart=/usr/local/bin/raworc-mcp
Restart=on-failure
RestartSec=2

[Install]
WantedBy=multi-user.target
```

Then run:

```
sudo systemctl daemon-reload
sudo systemctl enable --now raworc-mcp
journalctl -u raworc-mcp -f
```

## Example: Windows (NSSM)

Using NSSM (Non-Sucking Service Manager):

- Download nssm and install: `nssm install raworc-mcp "C:\\path\\to\\raworc-mcp.exe"`
- In the NSSM UI, add environment variables:
  - `RAWORC_API_URL=https://api.remoteagent.com/api/v0`
  - `RAWORC_AUTH_TOKEN=YOUR_TOKEN`
  - `RAWORC_DEFAULT_SPACE=production`
  - `RAWORC_TIMEOUT=30`
  - `LOG_LEVEL=info`
  - `LOG_FORMAT=json`
- Start the service: `nssm start raworc-mcp`

## Security

- Prefer service account tokens with least privilege for `RAWORC_AUTH_TOKEN`.
- Avoid passing secrets on the command line; use environment variables or secret managers.
- Rotate tokens regularly and reload the service.

## Health and Smoke Tests

- Basic smoke test via MCP tools:
  - `health_check`
  - `get_version`
  - `list_spaces` (requires auth)

If needed, keep a lightweight wrapper process that periodically sends a JSON-RPC `ping` to verify the stdin/stdout bridge is responsive.

## Troubleshooting

- 401/403: ensure credentials/token are valid and have required permissions.
- 5xx/transient: built-in retries will reattempt; verify connectivity and Raworc API health.
- Timeouts: increase `RAWORC_TIMEOUT` for long operations.

---

For full REST endpoints and examples, see `docs/REST-API-v0.md`.
