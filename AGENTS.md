# Rusty Console SDK

## Overview

A monorepo of Rust SDKs for game consoles, under the rusty-console-sdk GitHub organization. License: BSD-3-Clause. Each console has its own directory with its own `AGENTS.md` for what is specific to it. This file holds what every SDK shares; read both when working inside an SDK.

| Console | Directory | Crate | Status |
| --- | --- | --- | --- |
| Game Boy Advance | `gba/` | `gba-sdk` | Native backend boots in mGBA. See `gba/AGENTS.md`. |

### Layout

- `<console>/`: one SDK. It is self-contained: its own Cargo workspace (`Cargo.toml`, `Cargo.lock`), its own `rust-toolchain.toml` and `.cargo/config.toml`, its own `justfile`, `tools/` and `AGENTS.md` (with a `CLAUDE.md` that imports it). Its build output goes in `<console>/target/`. Nothing in one SDK reaches into another's directory.
- `justfile`: the entry point. Each SDK's `justfile` is a just module named after its directory (`mod gba`), so its recipes are `just gba::<recipe>` from the root, or plain `just <recipe>` from inside the SDK. Module recipes run in the module's directory. The root also has `remote`.
- `mise.toml`: the tools for every SDK, grouped by the SDK that needs them, so one `mise install` at the root covers the collection. Module recipes inherit the root's tools.
- `.agents/skills/` and `.claude/skills/`: agent skills for the whole repository.
- `LICENSE`, `README.md` (the entry point, with the table of consoles) and `CLAUDE.md` at the root.

Adding a console: make `<console>/` in the same shape, add `mod <console>` to the root `justfile`, add its tools to `mise.toml` under its own heading, and add a row to the console tables here and in the README.

Sharing code between SDKs: not yet. When two SDKs need the same thing (the DS's ARM7 runs GBA code, say), move it to a shared crate in its own directory then, rather than having one SDK depend on another's.

## Building and testing

### Where builds run

Builds and tests run on a remote build server over SSH. The local machine edits code and views results, so don't install Rust or the console toolchains locally just to run a build.

- Connect with `ssh sdk-build`, a host alias each developer defines in their own `~/.ssh/config`. Recipes use the alias; `SDK_BUILD_HOST` overrides it.
- This repository is public. Never commit server addresses, hostnames, usernames, hardware details or anything else about private infrastructure.
- The working copy goes in `~/Developer/sdk` on the server, synced with `rsync` (respecting `.gitignore`). Never edit files there, since the next sync overwrites them.
- Non-interactive SSH sessions don't have the toolchains on `PATH`, so remote commands set it first (`export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"`), trust the root `mise.toml` and run recipes through `mise exec -- just ...`. The `remote` recipe does all three.
- `just remote <recipe...>` syncs the whole repository, runs a recipe there (`just remote gba::test`) and copies every SDK's `target/roms/` back. Arguments are shell-quoted on the way, so `just remote gba::test hello "some line"` works. A failed recipe or fetch stops it.
- Run long builds (mrustc's `core` above all) so that a dropped connection doesn't kill them.
- Anything beyond this project's directories on the server needs the user's say-so. System packages count. Installed so far: `mgba-sdl` and `xvfb`.
- When killing stray processes there, match by exact name (`pkill -x mgba`), never with `pkill -f` and a pattern that appears in your own shell's command line.

### Toolchains

- Rust: each SDK pins its own toolchain in `<console>/rust-toolchain.toml`, which rustup installs on the first `cargo` call there. Recipes run inside the SDK's directory, so they get the right one.
- Everything else goes in the root `mise.toml`. Pin exact versions.

## Searching the Codebase

Use fast, modern tools instead of the slow defaults:

- [`rg`](https://github.com/BurntSushi/ripgrep) (ripgrep) for text search, not `grep`. It respects `.gitignore` for you.
- [`fd`](https://github.com/sharkdp/fd) for finding files by name, not `find`.
- [`ast-grep`](https://ast-grep.github.io) for structural searches and rewrites in Rust. Call it as `ast-grep`. The `sg` alias is deprecated, so don't use it.
- `jq` for reading and querying JSON.

If your harness has dedicated search or file tools that already wrap these, those are fine too.

## Code Quality

Always lint and format:

- `cargo fmt` before committing. CI will reject unformatted code.
- `cargo clippy` with warnings denied, for both the console targets and the host tools. Treat compiler warnings as things to fix, not noise.
- Every library crate has `#![warn(missing_docs)]`, which the `check` recipes turn into an error, so every public item needs a doc comment.

Use modern code practices:

- Every `unsafe` block gets a `// SAFETY:` comment saying why it's sound. Unsafe code lives in each SDK's low-level crates, and the crate games use should offer a safe API wherever the hardware allows.
- MMIO goes through `read_volatile` and `write_volatile` behind typed wrappers. Never use references to MMIO, since the compiler may read through a reference whenever it likes.
- Use newtypes over bare integers for things that shouldn't mix (a palette index and a tile index, say) and `bitflags`-style hand-written types for registers.
- Reach for `core` before adding a dependency.

## Docs (IMPORTANT!!!)

Always update the docs if necessary when changing or adding code. If the docs already contain outdated information, expand your scope to cover that info too. That includes the `AGENTS.md` files: when the layout, build, backends or tooling change, update the overview and the build section in the one that owns it.

The docs are a [Zensical](https://zensical.org) site: `zensical.toml` at the root, pages in `docs/`. `uvx "zensical==0.0.74" build --strict` checks it and writes `site/` (ignored by git), and `uvx zensical serve` previews it. The Docs workflow runs the strict build on pull requests that touch the docs, and the Pages workflow publishes the site on every push to `main` that does (the repository's Pages source is already GitHub Actions). Pages:

- `index.md`: what gba-sdk is, with the hello example, and the status.
- `getting-started.md`: installing, building and running the example, writing a ROM, remote builds.
- `video.md`, `input.md`, `logging.md`: one page per hardware area the SDK covers, each describing every public item of its module. A new module gets a page and a `nav` entry in `zensical.toml`.
- `runtime.md`: what `rt` does before `main`, `entry!`, the memory layout from `gba.ld`, IWRAM placement.
- `backends.md`: the two backends, the pipeline diagram, mrustc's constraints, and the table of what works on each. A feature that only works on one backend gets a row with the reason.
- `development.md`: toolchain, layout, the commands table (keep it in step with the justfile), testing, docs.
- `roadmap.md`: the milestones from this file as checklists.

Code in the docs that forms a whole program is compiled as a ROM before committing; the crate docs (`//!` and `///` comments) stay current too.

Compile the examples you put in the docs before committing them, on every backend the SDK has.

When a roadmap item is finished, check it off (`- [x]`) where it is. DO NOT move it into an earlier section like "Done: the MVP". The roadmap shows how each goal is progressing, so finished items stay with the section they belong to.

The READMEs are only entry points: what the collection or SDK is, one example, building, and links into the docs. Put details in the docs, not a README.

## Diagrams as Mermaid, not ASCII art

Never draw a flow, pipeline or architecture picture out of characters in a code block. Make a Mermaid diagram instead.

## Outdated packages

PLEASE, PLEASE do not use a package that is old or deprecated. EVEN IF IT IS "INDUSTRY STANDARD" THAT DOES NOT MEAN IT IS GOOD!!! USE THE BEST PACKAGE AND LATEST VERSION LIKE A NORMAL PERSON WOULD!

Check the package's repository for its latest release tag (and whether it is still maintained) before adding it or bumping it. That goes for crates, tools in `mise.toml`, GitHub Actions in the workflows and the nightly pins. The deliberate exceptions are language floors an SDK sets for a backend, like the GBA's Rust 1.90 for mrustc.

## Keep things human

Do not use em dashes or other special symbols not normally found in writing. Do not word things in a weird way. Keep it looking human. If you need to use, for example, an arrow icon, use an SVG instead.

Replacing an em dash with a hyphen is NOT a fix. A spaced hyphen doing an em dash's job ("the limit - 4 bytes", "fast - but wrong") is the same tic with a different character. Reword the sentence instead: use a comma, a colon, parentheses, or the actual word ("minus", "to", "through"). Plain hyphens in compound words and numeric ranges like 0-1 are fine.

## Use American spelling

It's fine to be British in the chat if the person is actually in a region where it's spelled like that. But if not, just use American spelling. The thing itself MUST use American spelling.
