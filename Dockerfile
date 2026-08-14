# ==========================================
# Stage 1: Build Rust -> WebAssembly
# ==========================================
FROM rust:latest AS wasm-build

WORKDIR /app

# Install wasm-pack
RUN cargo install wasm-pack

# Copy Rust project
COPY crates ./crates

# Build the WASM package
# This generates the JS/WASM bindings into /app/pkg
RUN wasm-pack build ./crates \
    --target web \
    --release \
    --out-dir /app/pkg


# ==========================================
# Stage 2: Build frontend
# ==========================================
FROM node:latest AS frontend-build

WORKDIR /app

# Copy package files first for better Docker layer caching
COPY package.json package-lock.json ./

RUN npm install

# Copy the rest of the frontend
COPY . ./

# Replace/copy the WASM package generated in stage 1
COPY --from=wasm-build /app/pkg ./pkg

# Build frontend
RUN NODE_OPTIONS=--openssl-legacy-provider npm run build


# ==========================================
# Stage 3: Production
# ==========================================
FROM nginx:stable-alpine

# Your webpack build appears to produce dist/
COPY --from=frontend-build /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
