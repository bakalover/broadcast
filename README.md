# Broadcast

## Blazingly fast MPMC broadcast lock-free rust channel.

Inspiration - [link](https://docs.rs/tokio/latest/src/tokio/sync/broadcast.rs.html#312)

Primary goal - create replacement analogue of kotlin channel with `tokio::sync::broadcast` semantic and integrate it to tokio runtime as well

---

## Performance

---

## Roadmap

- [ ] Implement "infinite array"
- [ ] Optimal array MO
- [ ] Array loom tests
- [ ] Design shared state
- [ ] Design send/recv
- [ ] Implement send/recv
- [ ] Optimal MO on send/recv
- [ ] send/recv loom tests
