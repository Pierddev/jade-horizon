# Jade for Horizon

[Français](README.fr.md)

Unofficial port of [Jade](https://www.curseforge.com/minecraft/mc-mods/jade) (by Snownee) to [Horizon](https://playhorizon.dev) (game 26.3, mod API 0.2). The original Jade is a Java NeoForge mod and cannot run in Horizon: this mod rewrites its main features in Rust (WebAssembly), using the official SDK.

## What it shows

A tooltip at the top of the screen for whatever the crosshair is on:

- **Block**: icon, name (in the game's language), originating mod, and depending on the block:
  - crop growth (wheat, carrots, potatoes, beetroots, melon and pumpkin stems, cocoa, nether wart, sweet berries, torchflower, pitcher plant, and any modded crop with an `age` property): "Growth: 57%" or "Mature";
  - honey level of a beehive (0 to 5), compost level of a composter (0 to 8), charges of a respawn anchor;
  - cooking progress of a furnace, smoker or blast furnace, brewing progress of a brewing stand;
  - whether the item in your hand can harvest the block (for blocks that drop nothing without the right tool);
  - a progress bar while mining.
- **Entity**: name, health (❤ 18 / 20), originating mod, and for animals: time until adulthood, time until they can breed again, love mode, time until the next egg (chickens).

The **numpad 0** key shows or hides the tooltip (rebindable in Options > Controls).

Health, animal age and furnace/brewing progress are only known to the server. The mod requests them a few times per second, only for a target within 24 blocks. On a server without this mod, the tooltip still works, just without those lines.

Not ported from Jade yet: chest and furnace contents, horse stats, owner of tamed animals, themes and the config screen.

## Build and install

```powershell
.\build.ps1
```

Builds in WSL (Ubuntu), copies the result into `%APPDATA%\horizon\horizon-mods\` and enables it in `horizon-mods.txt`. Restart Horizon afterwards. Use `.\build.ps1 -NoInstall` to build only.

## Licence and credits

CC-BY-NC-SA-4.0 (see [LICENSE](LICENSE) and [NOTICE](NOTICE)), like Jade. Non-commercial use only.

Jade is the work of [Snownee and contributors](https://github.com/Snownee/Jade), licensed under CC BY-NC-SA 4.0. This unofficial port is an adaptation (a rewrite in Rust for Horizon, covering only part of Jade's features). It contains no Minecraft assets or data: icons and names are read from the player's own game. It is not affiliated with Snownee or the Horizon project.
