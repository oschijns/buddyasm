# BuddyASM

# Generate NES tileset
test-nes:
    mkdir -p output/nes
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-nes.toml  --output output/nes

# Generate SNES tileset
test-snes:
    mkdir -p output/snes
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-snes.toml --output output/snes

# Generate GameBoy tileset
test-gb:
    mkdir -p output/gb
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-gb.toml   --output output/gb

# Generate VirtualBoy tileset
test-vb:
    mkdir -p output/vb
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-vb.toml   --output output/vb

# Generate PC-engine tileset
test-pce:
    mkdir -p output/pce
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-pce.toml  --output output/pce

# Generate NeoGeo Pocket tileset
test-ngp:
    mkdir -p output/ngp
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-ngp.toml  --output output/ngp

# Generate WonderSwan tileset
test-ws:
    mkdir -p output/ws
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-ws.toml   --output output/ws

# Generate SEGA MasterSystem tileset
test-sms:
    mkdir -p output/sms
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-sms.toml  --output output/sms

# Generate SEGA Megadrive tileset
test-md:
    mkdir -p output/md
    cargo run --package buddyasm_tileset --bin tileset --features "binary" -- --manifest tileset/assets/manifest-md.toml   --output output/md

# Run the tileset packer sub program
test-all: test-nes test-snes test-gb test-vb test-pce test-ngp test-ws test-sms test-md
