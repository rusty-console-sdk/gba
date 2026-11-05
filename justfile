# The entry point for the whole collection. Each SDK is a module named after
# its directory, so the GBA's recipes are `just gba::check`, `just gba::rom
# hello` and so on. Recipes only run things; logic belongs in each SDK's tools/.

set shell := ["bash", "-euo", "pipefail", "-c"]
set positional-arguments

mod gba

# The SSH alias of the build server. Define it in your own ~/.ssh/config.
host := env_var_or_default("SDK_BUILD_HOST", "sdk-build")
remote_dir := "Developer/sdk"

default:
    @just --list --list-submodules

# Built ROMs come back to <sdk>/target/roms/.
# Sync to the build server and run a recipe there, like `just remote gba::test`.
remote +args:
    rsync -az --delete --exclude .git --filter=':- .gitignore' ./ {{host}}:{{remote_dir}}/
    ssh {{host}} "export PATH=\"\$HOME/.cargo/bin:\$HOME/.local/bin:\$PATH\" && cd {{remote_dir}} && mise trust -q && mise exec -- just $(printf '%q ' "$@")"
    rsync -az -m --include='/*/' --include='/*/target/' --include='/*/target/roms/***' --exclude='*' {{host}}:{{remote_dir}}/ ./
