#!/usr/bin/env bash
# Runs a ROM headless in mGBA for a few seconds, saves its log next to the
# ROM and checks that mGBA was still running at the end and that the log
# contains an expected line.
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

# A log left from an earlier run must never satisfy this one.
rm -f "$log"

start=$(date +%s%N)
set +e
SDL_AUDIODRIVER=dummy xvfb-run -a timeout -k 1 "$seconds" stdbuf -oL -eL \
    mgba -C logToStdout=1 -C logLevel.gba.debug=127 "$rom" 2>&1 \
    | grep -v '^ALSA lib' | tee "$log"
statuses=("${PIPESTATUS[@]}")
set -e
elapsed_ms=$((($(date +%s%N) - start) / 1000000))
status=${statuses[0]}

if [[ ${statuses[2]} != 0 ]]; then
    echo "FAIL: could not write the log $log" >&2
    exit 1
fi

# timeout reports 124 when SIGTERM ended the run and 137 when its follow-up
# SIGKILL did. 137 can also mean something else killed mGBA (the OOM killer,
# say), so it only counts once the full time has passed. Anything else means
# mGBA stopped by itself before the time was up.
if [[ $status == 137 && $elapsed_ms -lt $((seconds * 1000)) ]]; then
    echo "FAIL: mGBA was killed after ${elapsed_ms}ms, before the ${seconds}s timeout" >&2
    exit 1
fi
if [[ $status != 124 && $status != 137 ]]; then
    echo "FAIL: mGBA exited early with status $status running $rom" >&2
    exit 1
fi

if grep -qF -- "$expect" "$log"; then
    echo "ok: $rom logged '$expect'"
else
    echo "FAIL: $rom did not log '$expect'" >&2
    exit 1
fi
