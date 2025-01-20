
build
```bash
cargo lambda build --release --include "assets/**/*" --arm64 -j 8
```
subir
```bash
cargo lambda deploy \
    --memory 1024 \
    --include assets \
    --env-var RUST_LOG=info \
    --runtime provided \
    --retry-attempts 1 \
    --tag Environment=prod --enable-function-url
```
