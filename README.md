# art-engine-rs

## README WIP

Art engine for generating assets and the related metadata from a collection of layers (namely for randomly generating NFT collection assets).

**This is very much an experimental project still and development is ongoing, please do not use in production.**

The engine is currently able to take layer order and rarity into account. The code is ***definitely not*** optimized and many performance and syntax improvements can be made. That being said, the engine can successfully randomly generate a collection of arbitrary size, taking rarity into account. Currently the engine has only been tested with `.png` files.

The folder naming convention should be as follows:
```
layers/
    - 01<Base Layer Folder>/
        - 02<Base Trait w/ 50% chance of being chosen>.png
        - 01<Base Trait w/ 25% chance of being chosen>.png
        - 01<Base Trait w/ 25% chance of being chosen>.png
    - 02<Second Layer Folder>/
    ...
    - N<Top Layer Folder>/
```

Note that the layer parent folder does not have to be named `layers/`, it can be anything. Additionally, please ensure that there are no additional files aside from the individual layer folders inside the layer parent folder (e.g. `.DS_Store`). The engine will fail if any other files are present.