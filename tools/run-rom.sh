#!/usr/bin/env bash
# Runs a ROM headless in mGBA for a few seconds, saves its log next to the
# ROM and checks that the log contains an expected line.
#
#   tools/run-rom.sh <rom.gba> <expected line> [seconds]
#
# timeout is mGBA's direct parent, since mGBA ignores SIGTERM and -k follows
# up with SIGKILL; stdbuf keeps the log line-buffered through the pipe.
set -euo pipefail

rom=$1
expect=$2
seconds=${3:-4}
log=${rom%.gba}.log

SDL_AUDIODRIVER=dummy xvfb-run -a timeout -k 1 "$seconds" stdbuf -oL -eL \
    mgba -C logToStdout=1 -C logLevel.gba.debug=127 "$rom" 2>&1 \
    | grep -v '^ALSA lib' | tee "$log" || true

if grep -qF -- "$expect" "$log"; then
    echo "ok: $rom logged '$expect'"
else
    echo "FAIL: $rom did not log '$expect'" >&2
    exit 1
fi
