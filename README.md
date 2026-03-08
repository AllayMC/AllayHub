# NukkitHub

Plugin hub for the [Nukkit](https://github.com/CloudburstMC/Nukkit) and Nukkit-MOT ecosystem.

## Documentation

- [Plugin Indexing Guide](docs/PLUGIN_INDEXING.md) - How plugin repositories are discovered, indexed, and displayed on NukkitHub

## Development

### Requirements

- [Bun](https://bun.sh/) `>= 1.3.6`
- [Rust](https://www.rust-lang.org/tools/install) toolchain

### Run locally

```bash
bun install
bun run dev
```

`bun run dev` will build the search index before starting the Nuxt development server.

### Build the indexer manually

```bash
cargo run --manifest-path indexer/Cargo.toml -- discover
cargo run --manifest-path indexer/Cargo.toml -- build
```

## Credits

- Project structure reference [AllayHub](https://github.com/AllayMC/AllayHub)
- Frontend based on [Modrinth](https://github.com/modrinth/code)

## License

[AGPL-3.0](LICENSE)
