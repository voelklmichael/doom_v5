# Build and Run the C Version

This guide builds the Linux X11 version of doomgeneric from the C sources.

## Prerequisites

On Debian or Ubuntu, install the compiler, Make, and X11 development headers:

```bash
sudo apt update
sudo apt install build-essential clang libx11-dev
```

The project can use either Clang or GCC. The Makefile defaults to Clang.

You also need a legally obtained Doom IWAD file, such as the shareware `doom1.wad`.
The IWAD is not included in this repository.

## Build

From the repository root:

```bash
cd /home/michael/doom_v5/doomgeneric/doomgeneric
make
```

The build creates:

- `build/`: intermediate object files
- `doomgeneric`: the Linux X11 executable
- `doomgeneric.map`: the linker map file

For verbose compiler and linker commands, use:

```bash
make V=1
```

To remove generated build files:

```bash
make clean
```

## Run

Start the game by passing the IWAD path with `-iwad`:

```bash
./doomgeneric -iwad /path/to/doom1.wad
```

For example, if the WAD is in the Downloads directory:

```bash
./doomgeneric -iwad /home/michael/Downloads/Doom1.WAD
```

The X11 version requires an active graphical display. Check that the `DISPLAY`
environment variable is set before launching:

```bash
echo "$DISPLAY"
```

Run the command from an X11 desktop session. Remote or headless sessions may
need X11 forwarding or a virtual display.

## Runtime Files

The game may create configuration and save-game files in the directory where it
runs, including `.default.cfg` and `.savegame/`.

## Troubleshooting

### `X11/Xlib.h: No such file or directory`

Install the X11 development package:

```bash
sudo apt install libx11-dev
```

Then rebuild with `make clean && make`.

### `cannot open display`

The process cannot connect to an X11 display. Run it from a graphical X11
session and verify that `DISPLAY` is set.

### WAD not found or invalid

Use the full path to the WAD and confirm that the file exists:

```bash
ls -lh /path/to/doom1.wad
```

Use an IWAD rather than a PWAD. The shareware `doom1.wad` is sufficient for a
basic test.
