To test in dev:
```bash
cargo lambda watch
```

To build for deployment:
```bash
cargo lambda build --arm64 --release
```
To deploy:
```bash
cargo lambda deploy
```