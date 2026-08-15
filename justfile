# BuddyASM

# Generate NES tileset
test-nes:
    mkdir -p output/nes
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-nes.toml

# Generate SNES tileset
test-snes:
    mkdir -p output/snes
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-snes.toml

# Generate GameBoy tileset
test-gb:
    mkdir -p output/gb
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-gb.toml

# Generate VirtualBoy tileset
test-vb:
    mkdir -p output/vb
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-vb.toml

# Generate PC-engine tileset
test-pce:
    mkdir -p output/pce
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-pce.toml

# Generate NeoGeo Pocket tileset
test-ngp:
    mkdir -p output/ngp
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-ngp.toml

# Generate WonderSwan tileset
test-ws:
    mkdir -p output/ws
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-ws.toml

# Generate SEGA MasterSystem tileset
test-sms:
    mkdir -p output/sms
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-sms.toml

# Generate SEGA Megadrive tileset
test-md:
    mkdir -p output/md
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-md.toml

# Run the tileset packer sub program
test-all: test-nes test-snes test-gb test-vb test-pce test-ngp test-ws test-sms test-md
