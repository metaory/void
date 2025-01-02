<div align="center">
  <h1>
  <img valign="middle" src=".github/icon.png" alt="void-tab" height="72" />
   ㄙ𐊔\𐰷
  </h1>
  <h5>JSON-only fork of void</h5>
</div>


LINEAGE
-------

| This Fork      | Incorporated      | Original       |
|-----------     |---------------    |----------      |
| [metaory/void] | [onbjerg/void]   | [void-rs/void] |
| JSON storage   | Arrow pathfinding | protobuf base  |

<i><sup>find original void readme <a href="#original">here</a>.</sup></i>

---

RATIONALE
---------

This fork transitions void to use `JSON` storage instead of binary `protobuf`

- **Human Readable**: Data can be inspected and edited with any text editor
- **Debuggable**: Easy to examine and fix corrupted files
- **Portable**: Standard format supported by all languages and tools
- **Versionable**: Clean diffs in version control
- **Minimal**: Only stores essential data, reducing complexity

##### Key changes in this fork:
- Uses `JSON` format exclusively
- Removes `protobuf` dependency
- Provides migration tool for old databases
- Maintains data compatibility

---

MIGRATION
---------

If you're upgrading from the original void `<1.2.0`, 
You'll need to migrate your binary database:

```bash
scripts/migrate.sh path/to/old/database path/to/new.json
```

----

USAGE
-----

By default, void looks for `~/.void.json` unless a path is specified:

```bash
# Use default ~/.void.json
void

# Or specify a path
void path/to/notes.json
```

> [!TIP]
> The JSON format excludes auto-generated data
> - Node coordinates (auto-arranged)
> - Colors (randomly generated)
> - Selection state
> - Ephemeral UI state

----

INSTALLATION
------------

Clone and build from source:

```bash
git clone https://github.com/metaory/void
cd void
cargo build --release
```

The binary will be available at `target/release/void`

If you don't have cargo, install it [via rustup](https://rustup.rs).

---

CONFIGURATION
-------------

 Variable         | Description
----------------- | --------------------------------------
 `KEYFILE`        | Path to [key remap file](default.keys)
 `EDITOR`         | Text editor (defaults to vim)
 `LOGFILE`        | Path for debug logging
 `LOCATION_QUERY` | Enable location tracking for nodes

 Feature          | Control         | Feature         | Control
----------------- | --------------- | --------------- | ---------------
 new              | <kbd>C-n</kbd>  | new child       | <kbd>Tab</kbd>
 freeform         |                 | new sibling     | <kbd>Enter</kbd>
 delete           | <kbd>Del</kbd>  | move subtree    | <kbd>LeftDrag</kbd>
 undo             | <kbd>C-z</kbd>  | auto arrange    | <kbd>C-p</kbd>
 mark complete    | <kbd>C-a</kbd>  | drill-down      | <kbd>C-w</kbd>
 pop              | <kbd>C-q</kbd>  | hide children   | <kbd>C-t</kbd>
 open editor      | <kbd>C-k</kbd>  | prefix-jump     | <kbd>a-z</kbd>
 prefix-jump      | <kbd>C-f</kbd>  | hide completed  | <kbd>C-h</kbd>
 select arrow     | <kbd>C-r</kbd>  | erase arrow     | <kbd>C-r</kbd> <kbd>C-r</kbd>
 show debug log   | <kbd>C-l</kbd>  | reparent        | <kbd>LeftDrag</kbd>
 scroll up        | <kbd>PgUp</kbd> | scroll down     | <kbd>PgDn</kbd>
 select up        | <kbd>Up</kbd>   | select down     | <kbd>Down</kbd>
 select left      | <kbd>Left</kbd> | select  right   | <kbd>Right</kbd>
 de-select        | <kbd>Esc</kbd>  | exit            | <kbd>Esc</kbd> <kbd>Esc</kbd> 
 exit             | <kbd>C-c</kbd>  | save            | <kbd>C-x</kbd>
 next weighted    | <kbd>C-v</kbd>  | cut / paste     | <kbd>C-y</kbd>
 move up          | <kbd>C-g</kbd>  | move down       | <kbd>C-d</kbd>
 search below     | <kbd>C-u</kbd>  | select parent   | <kbd>A-P</kbd>
 select next      | <kbd>A-n</kbd>  | select previous | <kbd>A-p</kbd>

---

LICENSE
-------
[GNU General Public License v3.0](LICENSE) 
<i><sup><sub>inherited from upstream</sub></sup></i>

---

CONTRIBUTORS
------------

- [@spacejam] <sup><sub><i>Original</i></sub></sup>
- [@onbjerg]
- [@metaory]

---

<details>
<summary id="original"><b>ORIGINAL VOID DOCUMENTATION</b></summary>

![State](https://img.shields.io/badge/state-alpha-orange.svg?style=flat-square)

## Problems This Tool Addresses

1. Frequently fall out of creative flow
2. Day-to-day work lacks coherence
3. Failure to integrate learnings into a cohesive perspective
4. Execution of tasks lacks focus and motivation
5. Unclear how my efforts are impacting goals

## Core Perspectives

* Things we measure tend to improve
* We should regularly reevaluate priorities
* We should minimize decisions to prevent fatigue
* Individual sensemaking is well served by reflection, journaling, outlining, mind-mapping, etc...
* Don't impose specific workflows, but support many possibilities

## Implementation

* Everything is a tree
* You can collapse subtrees
* You can drill-down the screen focus arbitrarily
* Trees of tasks can be marked with `#task`, all children of marked nodes are implicitly subtasks
* Tasks can be prioritized with `#prio=<n>`, all children implicitly inherit the lowest ancestor's priority
* A task can be chosen automatically, with priorities weighting a random selection
* You can create your own sparklines by using `#plot=done` or `#plot=new`, in combination with `#n=10` for sparkline size, `#since=7d` / `#until=1d` for specifying time window
* Overall completed subtasks are plotted on a sparkline at the top of the screen for the past week
* You can draw arrows between nodes for mind-mapping functionality
* Can shell out and execute the content of a node with C-k. if the node starts with txt: this will be opened in vim or an editor specified in the `EDITOR` env var

[Tutorial](TUTORIAL.md) | [Example Workflow](#what-i-do-dont-do-what-i-do-discover-what-works-for-you)
</details>

[@spacejam]: https://github.com/spacejam
[@onbjerg]: https://github.com/onbjerg
[@metaory]: https://github.com/metaory

[void-rs/void]: https://github.com/void-rs/void
[metaory/void]: https://github.com/metaory/void
[onbjerg/void]: https://github.com/onbjerg/void
