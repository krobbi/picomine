# PicoMine
_2D sandbox game_  
__Copyright &copy; 2024 Chris Roberts__ (Krobbizoid).  
_All rights reserved._

# Contents
1. [About](#about)
2. [Dependencies](#dependencies)
3. [Credits](#credits)
4. [License](#license)

# About
PicoMine is an unfinished 2D sandbox game inspired by
[Minicraft](https://github.com/Miserlou/Minicraft).

Currently, the player can move around a pseudo-infinite, procedurally-generated
world with the `W`, `A`, `S`, and `D` keys. Grass and stone tiles can be placed
with the left and right mouse buttons respectively. The player can slow down by
holding the left shift key.

The game loads textures relative to the working directory it was started from
in the `res/` folder. Starting the game from outside of the repository root
will result in missing textures.

Chunks of tiles loaded by the player are never unloaded or saved.

# Dependencies
PicoMine uses the following libraries:
* [image](https://crates.io/crates/image) - Texture image loading.
* [minifb](https://crates.io/crates/minifb) - Window management, input, and
framebuffer rendering.
* [noise](https://crates.io/crates/noise) - Simplex noise for terrain
generation.
* [strum](https://crates.io/crates/strum) - Enum helpers.

# Credits
PicoMine uses the color palette
[Faraway48](https://lospec.com/palette-list/faraway48) by Igor Ferreira.

# License
PicoMine is released under the Krobbizoid Game License (KGL):  
https://krobbi.github.io/license/2024/kgl.txt

See [LICENSE.txt](/LICENSE.txt) for a full copy of the license text.
