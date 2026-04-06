# TizenClaw DTV Usage Guide

## Purpose

This guide explains how to deploy TizenClaw to a Tizen TV / DTV target that is
reachable over `ssh` instead of `sdb`. It is intended for developers and
operators who build packages on a separate server and then copy the result to
the TV manually.

The standard emulator and `sdb` workflow is still documented in
[USAGE.md](USAGE.md). Use this guide when your TV behaves more like a generic
Linux target with `ssh`, `scp`, `rpm`, and `systemctl`.

## When to Use This Guide

Use this guide when:

- the target is a Tizen TV / DTV device reachable over `ssh`
- the build happens on a separate build server
- you want to deploy the result with `scp` and `ssh`

Use [USAGE.md](USAGE.md) instead when you are working with the repository's
default `sdb`-driven emulator or device flow.

## Prerequisites

You should have the following available before you begin:

- a reachable TV or DTV target that accepts `ssh`
- permission to install packages or copy files on the TV
- `scp` and `ssh` available on the host machine you will deploy from
- your existing build-server command, such as `<your-vbs-build-command>`
- `rpm` and `systemctl` available on the TV for the primary deployment path

It also helps to confirm the TV architecture before copying any artifacts:

```bash
ssh root@<tv-ip> 'uname -m; command -v rpm; command -v systemctl'
```

## Operating Model

The SSH-based DTV flow is a split workflow:

1. run your existing build command on the build server
2. collect the resulting package or install tree
3. copy the output to the TV over `scp`
4. finish installation and service restart over `ssh`

This guide is a manual deployment alternative. It does not replace the current
`deploy.sh` script, which still assumes an `sdb` transport.

## Primary Workflow: RPM Deployment

This is the recommended path because it matches the repository's packaging
layout and lets the TV install the files into the expected runtime locations.

### Step 1: Run the Existing Build Command

On your build server, run the command you already use to produce the TizenClaw
RPM:

```bash
<your-vbs-build-command>
```

After the build finishes, locate the generated RPM. The exact path depends on
your build environment, but the output should be a package such as:

```text
tizenclaw-<version>.<arch>.rpm
```

### Step 2: Copy the RPM to the TV

```bash
scp /path/to/tizenclaw-*.rpm root@<tv-ip>:/tmp/
```

If you also produce related RPMs that should be installed together, copy them
to `/tmp/` in the same way before continuing.

### Step 3: Install the RPM and Restart Services

```bash
ssh root@<tv-ip> '
  mount -o remount,rw / || true
  rpm -Uvh --force /tmp/tizenclaw-*.rpm
  systemctl daemon-reload
  systemctl enable tizenclaw-tool-executor.socket || true
  systemctl restart tizenclaw-tool-executor.socket || true
  systemctl restart tizenclaw
'
```

### Step 4: Verify the Service

```bash
ssh root@<tv-ip> '
  systemctl status tizenclaw -l --no-pager || true
  systemctl status tizenclaw-tool-executor.socket --no-pager || true
  journalctl -u tizenclaw -n 100 --no-pager || true
'
```

## What the RPM Installs

When the RPM installs successfully, the package layout matches the repository's
packaging metadata:

- binaries under `/usr/bin`
- service and socket units under `/lib/systemd/system`
- runtime assets under `/opt/usr/share/tizenclaw`
- shared libraries under the system library directory used by the target
- parser plugin libraries under `/etc/package-manager/parserlib/metadata`
- parser plugin metadata under `/usr/share/parser-plugins`

That means a successful RPM install should already place the main daemon, CLI,
tool executor, dashboard binary, service units, config files, docs, embedded
tool descriptors, and packaged web assets where the runtime expects them.

## Fallback Workflow: Raw File Deployment

Use this path only when RPM installation is not available on the TV. This is a
manual bring-up path, so it is easier to drift away from the packaged layout if
files are omitted.

### Required File Groups

Copy the following groups of files from your build output to the TV:

- binaries to `/usr/bin`
  - `tizenclaw`
  - `tizenclaw-cli`
  - `tizenclaw-tool-executor`
  - `tizenclaw-web-dashboard`
  - `start_mcp_tunnel.sh` if you use that helper
- service and socket units to `/lib/systemd/system`
  - `tizenclaw.service`
  - `tizenclaw-tool-executor.service`
  - `tizenclaw-tool-executor.socket`
- runtime assets to `/opt/usr/share/tizenclaw`
  - `config/`
  - `web/`
  - `docs/`
  - `embedded/`
- shared libraries to the system library directory for the target
  - `libtizenclaw.so`
  - `libtizenclaw-core.so`
- parser plugin files if your TV environment uses them
  - libraries under `/etc/package-manager/parserlib/metadata`
  - `.info` files under `/usr/share/parser-plugins`

The exact library directory may be `/usr/lib` or `/usr/lib64` depending on the
target image and packaging environment.

### Example Copy Flow

The exact commands depend on how your build server stages the output, but the
shape of the manual deployment is:

```bash
scp /path/to/build/bin/tizenclaw root@<tv-ip>:/usr/bin/
scp /path/to/build/bin/tizenclaw-cli root@<tv-ip>:/usr/bin/
scp /path/to/build/bin/tizenclaw-tool-executor root@<tv-ip>:/usr/bin/
scp /path/to/build/bin/tizenclaw-web-dashboard root@<tv-ip>:/usr/bin/
scp /path/to/build/systemd/tizenclaw*.service root@<tv-ip>:/lib/systemd/system/
scp /path/to/build/systemd/tizenclaw-tool-executor.socket \
  root@<tv-ip>:/lib/systemd/system/
scp -r /path/to/build/share/tizenclaw root@<tv-ip>:/opt/usr/share/
```

After copying the files:

```bash
ssh root@<tv-ip> '
  systemctl daemon-reload
  systemctl enable tizenclaw-tool-executor.socket || true
  systemctl restart tizenclaw-tool-executor.socket || true
  systemctl restart tizenclaw
'
```

## Verification Checklist

Use these checks after either deployment path:

```bash
ssh root@<tv-ip> '
  uname -m
  command -v rpm || true
  command -v systemctl
  systemctl status tizenclaw -l --no-pager || true
  journalctl -u tizenclaw -n 100 --no-pager || true
'
```

Useful extra checks:

- confirm the installed binaries exist under `/usr/bin`
- confirm web assets exist under `/opt/usr/share/tizenclaw/web`
- confirm the socket unit is active if you expect tool execution support

## Troubleshooting

### TV reachable over `ssh`, but `rpm` is missing

Use the raw file deployment path instead of the RPM path. Make sure you copy
the binaries, service units, assets, shared libraries, and plugin files needed
by your target image.

### Service fails after restart

Check:

- `journalctl -u tizenclaw -n 100 --no-pager`
- `systemctl status tizenclaw -l --no-pager`
- whether the shared libraries were copied into the correct system library
  directory
- whether the config and web asset trees exist under `/opt/usr/share/tizenclaw`

### Root filesystem is read-only

Try:

```bash
ssh root@<tv-ip> 'mount -o remount,rw /'
```

If the target image still blocks writes, install into whatever writable
location your TV policy allows and adjust the service/unit strategy
accordingly.

### Web dashboard or config files are missing

Verify that the runtime asset tree exists:

```bash
ssh root@<tv-ip> 'find /opt/usr/share/tizenclaw -maxdepth 2 -type d | sort'
```

The dashboard and config content should live under:

- `/opt/usr/share/tizenclaw/web`
- `/opt/usr/share/tizenclaw/config`
- `/opt/usr/share/tizenclaw/docs`
- `/opt/usr/share/tizenclaw/embedded`

### Architecture mismatch between build output and TV

Confirm the target architecture first:

```bash
ssh root@<tv-ip> 'uname -m'
```

Then make sure the build server produced artifacts for that same architecture
before copying them to the TV.

## Relationship to Current Repo Tooling

The repository's current `deploy.sh` script assumes an `sdb`-based workflow for
emulators and device-oriented bring-up. This guide is a manual `ssh`/`scp`
alternative for Tizen TV / DTV environments that do not use that transport.

If you later automate this flow, a dedicated SSH-based deploy script should be
introduced separately instead of overloading the current `sdb`-oriented path.
