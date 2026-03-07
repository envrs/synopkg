# Synopkg JavaScript Monorepos.

<a aria-label="synopkg logo" href="https://envrs.github.io/synopkg">
  <img alt="" src="https://img.shields.io/badge/Made%20by%20synopkg-000000.svg?style=flat-square&logo=synopkg&labelColor=000">
</a>
<a aria-label="NPM version" href="https://www.npmjs.com/package/synopkg">
  <img alt="" src="https://img.shields.io/npm/v/synopkg.svg?style=flat-square&labelColor=000000">
</a>
<a aria-label="CI status" href="https://github.com/synopkg/envrs/actions/workflows/test-site.yaml?query=event%3Apush+branch%3Amain">
  <img alt="" src="https://img.shields.io/github/actions/workflow/status/envrs/synopkg/test-site.yaml?event=push&branch=main&style=flat-square&labelColor=000000">
</a>

## Installation

```bash
npm install --save-dev synopkg
```

## Guides

- [Getting Started](https://envrs.github.io/synopkg/)
- [Migrate to 14](https://envrs.github.io/synopkg/guide/migrate-v14/)

## Commands

> All command line options can be combined to target packages and dependencies in multiple ways.

### [lint](https://envrs.github.io/synopkg/command/lint)

Ensure that multiple packages requiring the same dependency define the same version, so that every package requires eg. `react@17.0.2`, instead of a combination of `react@17.0.2`, `react@16.8.3`, and `react@16.14.0`.

#### Examples

```bash
# Find all issues in "dependencies" or "devDependencies"
synopkg lint --dependency-types prod,dev
# Only lint issues in "react" specifically
synopkg lint --dependencies react
# Look for issues in dependencies containing "react" in the name
synopkg lint --dependencies '**react**'
# Find issues in scoped packages only
synopkg lint --dependencies '@types/**'
# Find issues everywhere except "peerDependencies"
synopkg lint --dependency-types '!peer'
# Only look for issues where an exact version is used (eg "1.2.3")
synopkg lint --specifier-types exact
# Sort dependencies by how many times they are used
synopkg lint --sort count
# See more examples
synopkg lint --help
# See a short summary of options
synopkg lint -h
```

### [fix](https://envrs.github.io/synopkg/command/fix)

Fix every autofixable issue found by `synopkg lint`.

#### Examples

```bash
# Only fix issues in dependencies and devDependencies
synopkg fix --dependency-types prod,dev
# Only fix inconsistencies with exact versions (eg "1.2.3")
synopkg fix --specifier-types exact
# Only fix issues in "react" specifically
synopkg fix --dependencies react
# See more examples
synopkg fix --help
# See a short summary of options
synopkg fix -h
```

### [update](https://envrs.github.io/synopkg/command/update)

Update packages to the latest versions from the npm registry, wherever they are in your monorepo.<br/>Semver range preferences are preserved when updating.

#### Examples

```bash
# Accept any update in latest (x.x.x)
synopkg update --target latest
# Only update minor versions (1.x.x)
synopkg update --target minor
# Only update patch versions (1.2.x)
synopkg update --target patch
# Check for outdated dependencies in one package
synopkg update --check --source 'packages/pingu/package.json'
# Update dependencies and devDependencies in the whole monorepo
synopkg update --dependency-types dev,prod
# Only update dependencies with a semver range specifier (^, ~, etc.)
synopkg update --specifier-types range
# Update dependencies where name exactly matches 'react'
synopkg update --dependencies 'react'
# Update dependencies where name contains 'react'
synopkg update --dependencies '**react**'
# Update dependencies with the '@aws-sdk' scope
synopkg update --dependencies '@aws-sdk/**'
# See more examples
synopkg update --help
# See a short summary of options
synopkg update -h
```

### [format](https://envrs.github.io/synopkg/command/format)

Organise package.json files according to a conventional format, where fields appear in a predictable order and nested fields are ordered alphabetically. Shorthand properties are used where available, such as the `"repository"` and `"bugs"` fields.

#### Examples

```bash
# Fix every formatting issue in the monorepo
synopkg format
# List all formatting issues in the monorepo
synopkg format --check
# Check the formatting of one package
synopkg format --check --source 'packages/pingu/package.json'
# See more examples
synopkg format --help
# See a short summary of options
synopkg format -h
```

### [list](https://envrs.github.io/synopkg/command/list)

Query and inspect all dependencies in your project, both valid and invalid.

#### Examples

```bash
# Sort dependencies by how many times they are used
synopkg list --sort count
# Show every instance of each dependency, not just their names
synopkg list --show instances
# Show dependencies ignored in your synopkg config
synopkg list --show ignored
# Show highest level of detail
synopkg list --show all
# Choose only some values
synopkg list --show hints,statuses
# List all "peerDependencies"
synopkg list --dependency-types peer
# List all types packages
synopkg list --dependencies '@types/**'
# List instances of an exact version being used as a peer dependency
synopkg list --specifier-types exact --show instances --dependency-types peer
# See more examples
synopkg list --help
# See a short summary of options
synopkg list -h
```

### [json](https://envrs.github.io/synopkg/command/json)

Output the state of every instance of every dependency as a JSON object, one per line. This command is best used with tools like [`jq`](https://jqlang.org/) for filtering and processing.

#### Examples

```bash
# Output all dependencies as JSON
synopkg json
# Output only AWS SDK dependencies
synopkg json --dependencies '@aws-sdk/**'
# Count dependencies by type
synopkg json | jq -r '.dependencyType' | sort | uniq -c
# See more examples
synopkg json --help
# See a short summary of options
synopkg json -h
```

## Badges

- [![support on ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/C0C4PY4P)
- [![NPM version](http://img.shields.io/npm/v/synopkg.svg?style=flat-square)](https://www.npmjs.com/package/synopkg)
- [![NPM downloads](http://img.shields.io/npm/dm/synopkg.svg?style=flat-square)](https://www.npmjs.com/package/synopkg)
- [![Build Status](https://img.shields.io/github/actions/workflow/status/envrs/synopkg/ci.yaml?branch=main)](https://github.com/envrs/synopkg/actions)
