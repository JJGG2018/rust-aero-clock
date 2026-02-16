# Build Stage
FROM rust:1.80-alpine AS builder

WORKDIR /usr/src/rust-clock
COPY . .

# Instalar musl-dev para compilar en Alpine
RUN apk add --no-cache musl-dev

# Compilar release
RUN cargo build --release

# Runtime Stage (Imagen final ultra ligera)
FROM alpine:3.19

WORKDIR /app

# Copiar el binario y el HTML
COPY --from=builder /usr/src/rust-clock/target/release/rust-clock .
# IMPORTANTE: Copiar index.html al contenedor
COPY --from=builder /usr/src/rust-clock/index.html .

# Exponer puerto 3000
EXPOSE 3000

# Ejecutar
CMD ["./rust-clock"]
