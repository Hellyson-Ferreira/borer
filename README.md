🪲 Borer

borer is a self-hosted tunneling system

It allows you to securely expose local services to the internet using persistent tunnels. Instead of running a public proxy that executes requests, borer delegates all request execution to a local agent, keeping your infrastructure minimal and your services private.

borer is designed as a transport-agnostic tunneling core, with HTTP implemented as just one of its operating modes. TLS termination, domains, and certificates are handled externally (e.g. via Caddy), allowing borer to focus strictly on reliable, low-level traffic forwarding.

Key ideas

Self-hosted alternative to ngrok

Local agent executes all requests

Server acts only as a tunnel broker

WebSocket-based transport (for now)

Protocol-first design, transport-agnostic by default

Status

🚧 Work in progress — early prototyping stage.