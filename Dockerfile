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
COPY --from=builder /usr/src/rust-clock/target/release/rust-aero-clock .
# IMPORTANTE: Copiar index.html al contenedor
COPY --from=builder /usr/src/rust-clock/index.html .

# Exponer puerto 3030
EXPOSE 3030

# Ejecutar
CMD ["./rust-aero-clock"]
