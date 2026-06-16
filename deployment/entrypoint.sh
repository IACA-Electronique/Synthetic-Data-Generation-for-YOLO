#!/bin/sh
set -eu

case "${SYSTEM:-linux}" in
  linux) exec /app/app "$@" ;;
  *) echo "Unsupported SYSTEM=$SYSTEM" >&2; exit 1 ;;
esac