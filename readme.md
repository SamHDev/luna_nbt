# Named Binary Tag (NBT)
The The Named Binary Tag, is a structured binary format used by the game Minecraft for a variety
of purposes such as, Player Data and World Saves as well as being used within the Minecraft Protocol.

[NBT Specification](https://wiki.vg/NBT#Specification)

This crate is __yet__ another implementation of the NBT format.

[![crates.io badge](https://img.shields.io/crates/v/luna_nbt.svg)](https://crates.io/crates/luna_nbt)
[![docs.rs badge](https://docs.rs/luna_nbt/badge.svg)](https://docs.rs/luna_nbt)
[![Downloads badge](https://img.shields.io/crates/d/luna_nbt.svg)](https://crates.io/crates/luna_nbt)

## Key features
- Support for Serialisation and Deserialization with the [Serde](https://serde.rs) framework.
- Ability to create partial or complete documents through the `Tag` and `Blob` objects.
- Ability to read/write from a socket or buffer.

## Cargo Features
- `serde`             (default) includes Serde serialisation and deserialization support.
- `serde_boolean`     (default) converts booleans to bytes during serialisation and deserialization.
- `serde_unsigned`    converts unsigned to their signed counterparts during serialisation and deserialization.
- `debug`             (default) debug trait impl for tags and blobs
- `arrays`            utils for writing byte, int and long arrays. (dev branch)
- `compression`       gzip and DEFLATE support. (dev branch)

## Install
Place one of the following in your `Cargo.Toml` file:
```toml
# Stable(ish)
luna_nbt = "0.0.4"

# Master/Dev
git = "https://github.com/samhdev/luna_nbt"
```

## Links
- [Documentation](https://docs.rs/crate/luna_nbt)
- [Repository](https://github.com/samhdev/luna_nbt)
- [Crate](https://crates.io/crates/luna_nbt)
