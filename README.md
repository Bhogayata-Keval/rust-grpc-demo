# rust-grpc-demo
## Rust OTLP Receiver
`cd rust-otlp-receiver && cargo run --bin metrics`

## APM metrics data generate
```cd node-apm-demo && npm login```
(npm login required to install our private npm package)

username: middlewarelabs-devs

password: ObLIXOkdwHyrmyOf6sl7YRQpd4=

OTP will be received

```npm i```

```OTEL_EXPORTER_OTLP_ENDPOINT=[::1]:50057 node server.js```
