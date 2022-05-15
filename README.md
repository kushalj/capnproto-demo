Cap'n Proto demo code for [related article](https://dev.to/kushalj/capn-proto-rpc-at-the-speed-of-rust-part-1-4joo) exploring this protocol.

1. point_demo that creates, saves, and explores messaging and serialisation into the capnp data format
2. client and server code for sending a point to a point-tracker (list of points)
3. pub-sub server and client adapted from capnp-rpc example code

---

## Running point-demo

```bash
cargo run point_demo
```

## Running server and client

Open two terminal panes/windows. In one, run
```bash
cargo run server 127.0.0.1:3000
```

Open two terminal panes/windows. In one, run:
```bash
cargo run client 127.0.0.1:3000
```