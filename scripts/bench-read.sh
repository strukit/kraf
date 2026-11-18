#!/bin/sh
# Measures reading the same file with kraf-read (sha2, sha3, no hash), cat, and
# sha256sum, in two cache states:
#
# - warm: the same file is read N times in a row, so the page cache is hot.
# - cold: four copies with different names are read, and each read is preceded
#   by evicting that file from the page cache, so the cache is cold.
#
# The file is AGENTS.md repeated 90 times, generated in a temporary directory.
# The peak memory (RSS) is measured in a separate run with GNU time, when it is
# available. Linux only: the cold state needs dd with iflag=nocache.
#
# Usage: sh scripts/bench-read.sh
# Environment: KRAF_READ (binary, default kraf-read), N (runs, default 50),
# PAUSE (seconds before each command, default 0).

set -u

KRAF_READ="${KRAF_READ:-kraf-read}"
N="${N:-50}"
PAUSE="${PAUSE:-0}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

FILE="$WORK/agents-x90.md"
i=0
while [ "$i" -lt 90 ]; do
    cat "$ROOT/AGENTS.md"
    i=$((i + 1))
done > "$FILE"

for n in 1 2 3 4; do
    cp "$FILE" "$WORK/agents-x90-$n.md"
done

# Runs one tool on one file. The tool name selects the command.
read_with() {
    tool="$1"
    file="$2"

    case "$tool" in
        sha2) "$KRAF_READ" --digest sha2 "file://$file" ;;
        sha3) "$KRAF_READ" --digest sha3 "file://$file" ;;
        none) "$KRAF_READ" --digest none "file://$file" ;;
        cat) cat "$file" ;;
        sha256) $SHA256 "$file" ;;
    esac
}

# Drops one file from the page cache, so the next read of it is cold.
evict() {
    dd if="$1" iflag=nocache count=0 status=none 2> /dev/null
}

# Prints the peak RSS of one read, with GNU time when it is available.
peak_rss() {
    tool="$1"
    file="$2"

    if ! /usr/bin/time --version > /dev/null 2>&1; then
        echo "n/a"
        return
    fi

    case "$tool" in
        sha2) set -- "$KRAF_READ" --digest sha2 "file://$file" ;;
        sha3) set -- "$KRAF_READ" --digest sha3 "file://$file" ;;
        none) set -- "$KRAF_READ" --digest none "file://$file" ;;
        cat) set -- cat "$file" ;;
        sha256) set -- $SHA256 "$file" ;;
    esac

    echo "$(/usr/bin/time -f '%M' "$@" 2>&1 > /dev/null | tail -1) KB"
}

# Warm: N runs of the same file, timed as one block.
warm() {
    label="$1"
    tool="$2"

    if [ "$PAUSE" -gt 0 ]; then
        echo "Waiting ${PAUSE}s before: $label (warm)"
        sleep "$PAUSE"
    fi

    start=$(date +%s%N)
    i=0
    while [ "$i" -lt "$N" ]; do
        read_with "$tool" "$FILE" > /dev/null 2>&1
        i=$((i + 1))
    done
    end=$(date +%s%N)
    per_run_us=$(( (end - start) / N / 1000 ))

    printf '%-24s warm %10s us per run   peak RSS %s\n' "$label" "$per_run_us" "$(peak_rss "$tool" "$FILE")"
}

# Cold: N cycles over the four named copies. Each run is evicted first, and only
# the read itself is timed.
cold() {
    label="$1"
    tool="$2"

    if [ "$PAUSE" -gt 0 ]; then
        echo "Waiting ${PAUSE}s before: $label (cold)"
        sleep "$PAUSE"
    fi

    total_us=0
    runs=0
    i=0
    while [ "$i" -lt "$N" ]; do
        for n in 1 2 3 4; do
            file="$WORK/agents-x90-$n.md"
            evict "$file"
            start=$(date +%s%N)
            read_with "$tool" "$file" > /dev/null 2>&1
            end=$(date +%s%N)
            total_us=$(( total_us + (end - start) / 1000 ))
            runs=$((runs + 1))
        done
        i=$((i + 1))
    done

    printf '%-24s cold %10s us per run   peak RSS %s\n' "$label" "$((total_us / runs))" "n/a"
}

if command -v sha256sum > /dev/null 2>&1; then
    SHA256="sha256sum"
else
    SHA256="shasum -a 256"
fi

echo "File: $(wc -l < "$FILE") lines, $(wc -c < "$FILE") bytes"
echo "Runs per command: $N (warm) and $N x 4 (cold)"
echo

warm "kraf-read sha2" sha2
warm "kraf-read sha3" sha3
warm "kraf-read no hash" none
warm "cat" cat
warm "sha256sum" sha256

echo

cold "kraf-read sha2" sha2
cold "kraf-read sha3" sha3
cold "kraf-read no hash" none
cold "cat" cat
cold "sha256sum" sha256
