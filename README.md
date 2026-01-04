# 🪲 Borer

**borer** is a self-hosted tunneling prototype.

It implements a minimal tunneling system where a server forwards incoming HTTP requests through a persistent WebSocket connection to a local agent. The local agent executes the request and sends the response back through the tunnel.

The server never executes user requests. Its only responsibility is routing and forwarding traffic to the correct connected client.

---

## What works today

- Persistent WebSocket connection between server and client
- HTTP request forwarding over the WebSocket tunnel
- Request/response correlation using request IDs
- Concurrent HTTP requests supported
- Local agent executes all upstream requests
- Server acts only as a tunnel broker

---

## What this project is

- A **working prototype**
- A **proof of concept** for a self-hosted tunneling system
- A **foundation** for future routing and tunneling features

This project is **not production-ready**.

---

## Planned work

- [ ] Turn the client into a proper CLI tool
- [ ] Add authentication between server and client
- [ ] Add client registration and routing
- [ ] Route requests based on subdomains (`Host` header)
- [ ] Add support for proxying WebSocket connections
- [ ] Improve error handling and timeouts
- [ ] Clean and normalize HTTP headers
- [ ] Support streaming request and response bodies
- [ ] Support multiple clients per server
- [ ] Add TCP tunneling mode (non-HTTP)
- [ ] Replace JSON with a binary protocol
- [ ] Add basic observability (logs and metrics)

---

## Status

🚧 Prototype — active development.

---

