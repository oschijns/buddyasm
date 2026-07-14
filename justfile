# BuddyASM

# Run the tileset packer sub program
test-tileset:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" \
        -- --manifest tileset/assets/manifest.toml --output output
