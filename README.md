# JigMorph

This project is a Rust implementation of a jigsaw puzzle solver inspired by [Matt Parkers' video](https://www.youtube.com/watch?v=b5nElEbbnfU). The program generates jigsaw puzzles with customizable settings and attempts to find as many combinations of pieces that forms multiple solutions. It also creates visual representations of the solutions using images.

## Features
- Customizable Grid Size: Adjust the size of the jigsaw puzzle grid.
- Variable Side Types: Define the number of unique side types for the puzzle pieces.
- Minimum Solutions: Set the minimum number of solutions the solver should find.
- Image Generation: Generate images of all found solutions for visual inspection.

## Getting Started

### Prerequisites
- Rust: Make sure you have Rust installed. You can download it from [here](https://www.rust-lang.org/tools/install).
- Image Assets: The program uses specific image assets located in the `./images` directory:
  - `piece_blank.png`
  - `piece_edge.png`
  - `piece_edge_1.png` to `piece_edge_N.png`
- Ensure these images are present in the `./images` folder.

### Installation
1. Clone the repository:
```sh
git clone https://github.com/DylanMcBean/JigMorph.git
cd JigMorph
```

2. Build the project:
```sh
cargo build --release
```

### Usage
Run the program with the following command:
```sh
cargo run --release
```

The program will:
- Initialize the puzzle settings.
- Generate jigsaw puzzles based on the settings.
- Attempt to solve the puzzles until it finds the specified minimum number of solutions.
- Create images for each solution and save them in the ./solutions directory.


#### Customization
You can customize the puzzle by modifying the `Settings` struct in `main.rs`:
```rs
let settings = Settings {
    grid_size: 10,     // Size of the grid (e.g., 10x10)
    min_solutions: 2,  // Minimum number of solutions to find
    side_types: 6,     // Number of unique side types
};
```
> **warning**: larger grid sizes will take longer to solve and generate images.

### Project Structure
- **src/main.rs**: Entry point of the application. Manages settings, initializes the puzzle, and handles image generation.
- **src/settings.rs**: Contains the `Settings` and `ImageCrop` structs for configuration.
- **src/files.rs**:  Handles file system operations, specifically creating the solutions directory.
- **src/images.rs**: Manages image loading and processing for the puzzle pieces and edges.
- **src/jigsaw.rs**: Core logic for generating and solving the jigsaw puzzle.

### Dependencies
- **rand**: For random number generation.
- **image**: For image processing and manipulation.

These are specified in the `Cargo.toml` file and will be downloaded automatically when building the project.

### Troubleshooting
- **Missing Images**: Ensure all required images are present in the `./images` directory.
- **Side Types Exceed Edge Images**: If you receive a panic about side types exceeding edge images, either reduce the `side_types` in settings or add more edge images to match.

### Contributing
Contributions are welcome! Feel free to open issues or submit pull requests for improvements or bug fixes.

### License
This project is licensed under the MIT License. For details, see the [LICENSE](./LICENSE) file.

### Acknowledgements
- [Matt Parker](https://www.youtube.com/user/standupmaths) for inspiration and ideas for the jigsaw puzzle solver.