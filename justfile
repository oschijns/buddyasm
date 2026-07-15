# BuddyASM

# Generate NES tileset
test-nes:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-nes.toml  --output output/nes

# Generate SNES tileset
test-snes:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-snes.toml --output output/snes

# Generate GameBoy tileset
test-gb:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-gb.toml   --output output/gb

# Generate VirtualBoy tileset
test-vb:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-vb.toml   --output output/vb

# Generate PC-engine tileset
test-pce:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-pce.toml  --output output/pce

# Generate NeoGeo Pocket tileset
test-ngp:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-ngp.toml  --output output/ngp

# Generate WonderSwan tileset
test-ws:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-ws.toml   --output output/ws

# Generate SEGA MasterSystem tileset
test-sms:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-sms.toml  --output output/sms

# Generate SEGA Megadrive tileset
test-md:
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-md.toml   --output output/md

# Run the tileset packer sub program
test-all: test-nes test-snes test-gb test-vb
