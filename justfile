# The entry point for building and testing. Recipes only run things; logic
# belongs in tools/.

set shell := ["bash", "-euo", "pipefail", "-c"]
set positional-arguments

target := "thumbv4t-none-eabi"
build_std := "-Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem"
# The SSH alias of the build server. Define it in your own ~/.ssh/config.
host := env_var_or_default("GBA_BUILD_HOST", "gba-build")
remote_dir := "Developer/gba"

default:
    @just --list

# Assemble crates/rt's startup code, which every ROM links.
asm:
    mkdir -p target/rt
    arm-none-eabi-as -mcpu=arm7tdmi -mthumb-interwork -o target/rt/crt0.o crates/rt/src/crt0.s

# Format, then type-check and lint everything for the GBA.
check: asm
    cargo fmt --all --check
    cargo clippy {{build_std}} --target {{target}} --workspace -- -Dwarnings

fmt:
    cargo fmt --all

# Build one ROM with the native backend into target/roms/<name>.gba.
rom name: asm
    cargo build {{build_std}} --target {{target}} --release -p {{name}}
    mkdir -p target/roms
    agb-gbafix target/{{target}}/release/{{name}} -o target/roms/{{name}}.gba
    arm-none-eabi-size target/{{target}}/release/{{name}}

# Build a ROM, run it headless in mGBA and check its log for a line.
test name="hello" expect="hello from gba-sdk": (rom name)
    tools/run-rom.sh target/roms/{{name}}.gba "{{expect}}"

# Sync the working copy to the build server and run a recipe there.
remote +args:
    rsync -az --delete --exclude .git --filter=':- .gitignore' ./ {{host}}:{{remote_dir}}/
    ssh {{host}} "export PATH=\"\$HOME/.cargo/bin:\$HOME/.local/bin:\$PATH\" && cd {{remote_dir}} && mkdir -p target/roms && mise exec -- just $(printf '%q ' "$@")"
    mkdir -p target/roms
    rsync -az {{host}}:{{remote_dir}}/target/roms/ target/roms/

# Build a ROM on the server and open it in a local mgba-qt.
run name="hello": (remote "rom" name)
    mgba-qt target/roms/{{name}}.gba
