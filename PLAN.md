# Plan: Add llm.txt to docs.argclass.com

## Overview

Add an `llm.txt` file to the argclass Sphinx documentation so it gets served at `https://docs.argclass.com/llm.txt`.

## Step 1: Create the `llm.txt` file

Create `docs/llm.txt` following the [llmstxt.org](https://llmstxt.org) specification:

```markdown
# argclass

> A Python library for building type-safe command-line interfaces using declarative classes with type hints, configuration files, and environment variables. Zero external dependencies. Requires Python 3.10+.

argclass wraps Python's standard `argparse` module, letting you describe argument parsers as classes with type annotations instead of imperative `add_argument()` calls. It supports INI/JSON/TOML config files, environment variable binding, secret masking, argument groups, and hierarchical subcommands — all with full IDE autocompletion.

Key API classes: `Parser` (main entry point), `Argument`, `Group`, `Secret`, `INIConfig`, `JSONConfig`, `TOMLConfig`, `LogLevel`.

## Docs

- [Quick Start](https://docs.argclass.com/quickstart.html): 5-minute overview of essential concepts
- [Tutorial](https://docs.argclass.com/tutorial.html): 30-minute guide building a complete CLI application
- [Arguments](https://docs.argclass.com/arguments.html): Typed arguments, defaults, nargs, enums, boolean flags
- [Groups](https://docs.argclass.com/groups.html): Reusable argument groups with prefix support
- [Subparsers](https://docs.argclass.com/subparsers.html): Hierarchical subcommands and nested parsers
- [Config Files](https://docs.argclass.com/config-files.html): INI, JSON, and TOML configuration file support
- [Environment Variables](https://docs.argclass.com/environment.html): Automatic env var binding with prefixes
- [Secrets](https://docs.argclass.com/secrets.html): Secret masking to prevent accidental exposure in logs
- [API Reference](https://docs.argclass.com/api.html): Full API documentation with all classes and functions

## Optional

- [Examples Gallery](https://docs.argclass.com/examples.html): Ready-to-use patterns from simple CLIs to multi-command tools
- [Error Handling](https://docs.argclass.com/errors.html): Error types and handling strategies
- [Common Pitfalls](https://docs.argclass.com/pitfalls.html): Frequent mistakes and how to avoid them
- [Integrations](https://docs.argclass.com/integrations.html): Using argclass with other tools
- [Security](https://docs.argclass.com/security.html): Security policy and best practices
```

## Step 2: Make Sphinx serve the file as-is

Sphinx won't copy arbitrary files to the build output by default. There are two approaches:

### Option A: Use `html_extra_path` in `conf.py` (Recommended)

Add to `docs/conf.py`:
```python
html_extra_path = ["llm.txt"]
```

This tells Sphinx to copy `llm.txt` directly into the HTML build output root. The file will be served at `/llm.txt` without any Sphinx processing — exactly what we want for a plain-text/markdown file.

### Option B: Use `extra_files` in `_static/`

Place the file as `docs/_static/llm.txt` — but this would serve it at `/_static/llm.txt` rather than `/llm.txt`, which doesn't match the spec.

**Recommendation: Option A** — it's the standard Sphinx mechanism for serving files at the site root.

## Step 3: Verify

After building docs locally with `make html` (or `uv run --group docs sphinx-build`), check that `docs/_build/html/llm.txt` exists and contains the expected content.

## Summary of Changes

| File | Change |
|------|--------|
| `docs/llm.txt` | **New file** — the llm.txt content |
| `docs/conf.py` | Add `html_extra_path = ["llm.txt"]` |

Two files changed, minimal and non-breaking.
