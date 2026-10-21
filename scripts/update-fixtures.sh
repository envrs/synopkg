#!/usr/bin/env bash

find . -type f | while read -r file; do
  filename=$(basename "$file")
  if [[ "$filename" != "pnpm-workspace.yaml" &&
    "$filename" != "lerna.json" &&
    "$filename" != ".synopkgrc" &&
    "$filename" != ".synopkgrc.cjs" &&
    "$filename" != ".synopkgrc.js" &&
    "$filename" != ".synopkgrc.json" &&
    "$filename" != ".synopkgrc.mjs" &&
    "$filename" != ".synopkgrc.ts" &&
    "$filename" != ".synopkgrc.yaml" &&
    "$filename" != ".synopkgrc.yml" &&
    "$filename" != "synopkg.config.cjs" &&
    "$filename" != "synopkg.config.js" &&
    "$filename" != "synopkg.config.mjs" &&
    "$filename" != "synopkg.config.ts" &&
    "$filename" != "package.json" ]]; then
    rm "$file"
  fi
done

find . -type d -empty -delete
