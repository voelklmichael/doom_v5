# Track 17 (proposed, not started) — Replace raw pointer returns with typed arena indices

## Status

Idea captured 2026-09-06 at the user's request, together with an initial
codebase survey, so it isn't lost across a session boundary. **Nothing has
been implemented.** This is a planning note for whoever (human or Claude)
picks this up next — treat the numbers below as a first pass, not a final
scope.

## The idea

Right now, functions like `P_SpawnMobj` return a raw pointer:

```rust
pub unsafe fn P_SpawnMobj(x: fixed_t, y: fixed_t, z: fixed_t, type_0: mobjtype_t) -> *mut mobj_t
```

The user's proposal: instead of a raw pointer, such functions would return a
small, typed index/handle into an **arena** — a backing store owned by
`GameState` — with **one distinct index type per pointee type** (e.g.
`MobjId`, `SectorId`, `SideId`, ...), rather than one generic index type
reused everywhere. A typed index can't be accidentally used to index the
wrong arena (the newtype pattern), which a bare `u32`/`usize` couldn't
prevent.

Sketch of the shape (illustrative, not decided):

```rust
pub struct MobjId(u32);   // newtype per pointee type
pub struct SectorId(u32);

pub struct Arena<T> {
    items: Vec<T>,        // or a slab/generational-arena crate
}

pub struct GameState {
    pub mobj_arena: Arena<mobj_t>,
    pub sector_arena: Arena<sector_t>,
    // ... one arena field per candidate type, alongside the existing
    // per-module XxxState fields from Track 16
    ...
}
```

`P_SpawnMobj` would then return `MobjId` instead of `*mut mobj_t`, and
callers would look the value up through `GameState`'s arena when they need
to touch fields, instead of dereferencing a raw pointer.

## Why this is worth doing, and why it's tangled up with Track 16

This codebase is a c2rust transpile: every C pointer became a Rust raw
pointer 1:1, including pointers into fixed-size C arrays (`sectors`,
`mobjinfo`, etc., already static globals) and Z_Malloc'd zone-allocated
memory (individual `mobj_t`/thinkers). Track 16 is already relocating many
of those backing arrays into `GameState` fields. Raw pointers into an array
that might someday move, resize, or become a `Vec` are exactly the kind of
thing that becomes fragile under this refactor — an index into an arena
survives relocation in a way a raw address never can, and is a natural
complement to (not a replacement for) the `GameState`-threading work
already underway.

There is also a real precedent for "pointer <-> stable identifier"
translation already living in this codebase: the savegame system
(`p_saveg.rs`) already converts thinker/mobj pointers to save-file-relative
indices when writing a save and reconstructs pointers from indices when
loading one, for exactly the reason that raw addresses aren't stable
identifiers across a save/load boundary. An arena-index scheme would make
that translation the *normal*, *only* way these values are ever handled,
not a special case bolted on just for saving.

## Survey performed

Two different questions were asked; both matter, and they have very
different sizes.

### 1. Functions whose *return type* is a pointer (the literal question asked)

Counted every function definition across `src/*.rs` whose return type is
`*mut T` / `*const T`, for a real named type `T` (i.e. excluding qualified
`::core::ffi::c_void` / `::core::ffi::c_char`, which are a different,
already-tracked modernization — see Track 4's deferred `c_char` note in
`doom_v5_rust_idiomatic_status` memory — not arena candidates). Comments
and string literals were masked before searching; multi-line signatures
were handled.

**27 functions total**, across 15 distinct pointee types:

| Count | Type | Example | Category |
|---|---|---|---|
| 4 | `byte` | `i_scale.rs::GenerateStretchTable` | raw buffer, not an entity — skip |
| 3 | `mobj_t` | `p_mobj.rs::P_SpawnMobj` | **core game entity — candidate** |
| 3 | `sector_t` | `p_setup.rs::GetSectorAtNullAddress` | **core game entity — candidate** |
| 3 | `wad_file_t` | `w_file.rs::W_OpenFile` | file handle, different domain |
| 2 | `default_t` | `m_config.rs::SearchCollection` | config descriptor, maybe |
| 2 | `MEMFILE` | `memio.rs::mem_fopen_read` | in-memory file handle, different domain |
| 2 | `visplane_t` | `r_plane.rs::R_FindPlane` | **core game entity — candidate** |
| 1 | `event_t` | `d_event.rs::D_PopEvent` | already handled specially (Track 16 phase 1) — see below |
| 1 | `XImage` | `doomgeneric_xlib.rs::XCreateImage` | X11 FFI, bin-crate only — out of scope |
| 1 | `Display` | `doomgeneric_xlib.rs::XOpenDisplay` | X11 FFI, bin-crate only — out of scope |
| 1 | `FILE` | `i_system.rs::fopen` | libc file handle — out of scope |
| 1 | `i32` | `m_misc.rs::__errno_location` | libc glue — out of scope |
| 1 | `side_t` | `p_spec.rs::getSide` | **core game entity — candidate** |
| 1 | `subsector_t` | `r_main.rs::R_PointInSubsector` | **core game entity — candidate** |
| 1 | `vissprite_t` | `r_things.rs::R_NewVisSprite` | **core game entity — candidate** |

**Real candidate set: 6 types, 11 return sites** — `mobj_t`, `sector_t`,
`subsector_t`, `side_t`, `visplane_t`, `vissprite_t`. Everything else above
is either a file/stream handle, a libc/X11 FFI type (some of it confined to
the `doomgeneric_xlib` bin crate, which per Track 16's own established rule
can't participate in `GameState` at all), or a raw byte buffer with no
single coherent "entity" identity.

`event_t`'s one hit (`D_PopEvent`) is a pointer into a fixed-size ring
buffer that's popped and used immediately within the same tic, never stored
long-term — it doesn't have the same "dangling reference" risk profile as
the others and was deliberately left as a raw pointer in Track 16 phase 1
(see `doom_v5_rust_idiomatic_status` memory). Revisit it here only if the
arena design ends up wanting uniformity for its own sake.

### 2. How big is the *real* surface, if only counting return sites is misleading

It is. A second, cruder count — every `*mut T` / `*const T` occurrence
anywhere (params, struct fields, locals, returns combined) — shows the
return-type count is a small fraction of how these pointers actually flow
through the code:

| Type | Total pointer occurrences (any position) | Of which are return types |
|---|---|---|
| `mobj_t` | 273 | 3 |
| `line_t` | 95 | 0 |
| `sector_t` | 91 | 3 |
| `thinker_t` | 38 | 0 |
| `vissprite_t` | 23 | 1 |
| `seg_t` | 16 | 0 |
| `subsector_t` | 17 | 1 |
| `visplane_t` | 17 | 2 |
| `thinker_s` | 14 | 0 |
| `side_t` | 15 | 1 |
| `node_t` | 11 | 0 |

**This is the real scoping question for Track 17, not the 27-function
number.** Converting only the 11 return sites to typed indices while
struct fields (`mobj_t.subsector: *mut subsector_t`, thinker
`next`/`prev` links, `sector_t.thinglist`, etc.) and the hundreds of
function *parameters* keep using raw pointers would mean constant
index<->pointer conversion at almost every call boundary — probably not
what delivers the safety benefit the user is after. A coherent version of
this idea likely needs to also retype the relevant **struct fields**, which
is a much larger, more invasive change than Track 16's own per-module
state-threading (that track never had to change any type signature other
than adding a `state` parameter). `line_t`/`thinker_t`/`thinker_s`/`seg_t`/
`node_t` show up here too even though they had zero literal return sites,
because they're reached via fields/params instead — they'd need the same
consideration if the struct-field piece goes ahead.

## Open questions for whoever scopes this next

- **Arena growth vs. Doom's actual lifetime model.** Doom mobjs/thinkers are
  allocated via `Z_Malloc` and freed individually (`P_RemoveMobj` etc, see
  Track 16 phase 16). An arena needs a real removal/reuse story (generational
  indices? free list? tombstones?) that matches this churn, not just a
  `Vec` that only grows. `sector_t`/`side_t`/`subsector_t`/`line_t` are
  different — they're level data, allocated once per level load and never
  individually freed until the whole level unloads, which is a much simpler
  arena shape (more like Track 16's own fixed-size-array-in-GameState
  pattern than a churning allocator).
- **Null-equivalent representation.** Doom code checks pointers against
  `NULL` constantly (`if (*mobj).target.is_null()`). Decide the index
  equivalent up front (`Option<MobjId>` everywhere, or a reserved sentinel
  value) — this touches every comparison site, not just allocation sites.
- **Interaction with the four (five) atomic callback hubs from Track 16's
  own plan** (`StateAction`/`ThinkerFn`, `PIT_*`/`PTR_*`, menu routines,
  `i_scale.rs`'s dispatch table) — several of those callbacks take
  `*mut mobj_t` or similar as their *parameter* type, which is part of the
  hub-locked signature. Retyping the arena candidate types would ripple
  into those signatures too.
- **p_saveg.rs's existing pointer<->index translation** (see "Why this is
  worth doing" above) is the closest existing precedent in this codebase —
  worth reading closely before designing the arena API, since it already
  solves a version of this problem for one specific boundary (save/load).
- **Scope relative to Track 16.** Decide whether this becomes Track 16's
  own eventual endgame (arenas replace the raw-array `GameState` fields
  Track 16 is creating today) or a genuinely separate, later track that
  builds on top of Track 16's finished state-threading. The write-up above
  leans toward "separate, after," since Track 16 is already large and this
  adds a second, orthogonal axis of change (like the `DG_*` platform-trait
  idea that was explicitly deferred in Track 16's own plan doc).

## Survey methodology (for reproducing or extending this)

```bash
# return-type survey (masks comments/strings, handles multi-line signatures)
cd rust-doomgeneric/src
python3 - << 'EOF'
import re, glob
for fn in sorted(glob.glob("*.rs")):
    text = open(fn, encoding='utf-8', errors='replace').read()
    masked = re.sub(r'//[^\n]*', '', text)
    masked = re.sub(r'/\*.*?\*/', '', masked, flags=re.S)
    masked = re.sub(r'"(?:[^"\\]|\\.)*"', '""', masked)
    for m in re.finditer(
        r'\bfn\s+(\w+)\s*\((?:[^()]|\([^()]*\))*\)\s*->\s*\*\s*(mut|const)\s*([A-Za-z_][\w:]*)',
        masked, flags=re.S):
        print(fn, *m.groups())
EOF

# total-occurrence survey (any position: params/fields/locals/returns)
grep -oP '\*mut TYPE_NAME\b|\*const TYPE_NAME\b' *.rs | wc -l
```
