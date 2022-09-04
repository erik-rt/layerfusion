# artengine-rs

## README WIP

Art engine for generating assets and the related metadata from a collection of layers (namely for randomly generating NFT collection assets).

**This is very much an experimental project still and development is ongoing, please do not use in production.**

The engine is currently able to take layer order and rarity into account. The code is **_definitely not_** optimized and many performance and syntax improvements can be made. That being said, the engine can successfully randomly generate a collection of arbitrary size, taking rarity into account. Currently the engine has only been tested with `.png` files.

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

Note that the layer parent folder does not have to be named `layers/`, it can be anything. Additionally, please ensure that there are no additional files aside from the individual layer folders inside the layer parent folder (e.g. `.DS_Store`). The engine will fail if any other files are present. You can use the following in the terminal to quickly find and delete any `.DS_Store` files in the current directory and any subdirectories:

```bash
find . -name ".DS_Store" -print -delete
```

## How to Use

Clone the repository and make a directory (e.g., `layers/`) that houses all layer subdirectories as shown in the directory structure above. From the project root, run the following command from the terminal:

```rust
cargo run --release layers
```

If structured correctly, the program will start outputting the assets into a directory called `outputs/` and the metadata into a directory called `metadata/`. If there are already assets in the `outputs/` and `metadata/` directories, the engine will start outputting IDs from the last available number (e.g., if IDs 0 - 200 already exist, once the engine is run again, the first ID outputted will be 201).

**It should be noted that the engine only tracks unique assets within the current run. Duplicates are highly likely if the engine is run multiple times to produce the collection.**

The number of assets generated currently is controlled via the `collection_size` variable in `cli.rs`. This will be updated in the future to be a runtime argument.
