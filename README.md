# Raycaster Game

A 3D first-person maze game built with Rust and Raylib.

## Features

- **3D Raycasting Engine**: Smooth 60 FPS first-person perspective rendering
- **Multiple Levels**: 3 progressively challenging maze levels
- **Enemy System**: Avoid enemy sprites that damage the player on contact
- **Lives System**: 2 lives with visual indicators and invulnerability periods
- **Textured Walls**: Support for custom wall textures (PNG format)
- **Sprite Rendering**: Billboard sprites for enemies and objectives
- **Minimap**: Real-time top-down view for navigation
- **Win/Lose Conditions**: Victory and game over screens
- **Background Music**: Optional background music support (MP3 format)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/vicperezch/graphics-project1.git
cd graphics-project1
```

2. Build the project:
```bash
cargo build
```

3. Run the game:
```bash
cargo run
```

## Controls

### Menu Navigation
- **↑/↓ Arrow Keys**: Navigate menu options
- **Enter**: Select option
- **ESC**: Return to previous menu

### In-Game Controls
- **W/A/S/D**: Move forward/left/backward/right
- **Mouse**: Look around (horizontal rotation)
- **ESC**: Return to main menu

## Gameplay

1. **Objective**: Navigate through the maze to reach the goal (golden sprite)
2. **Enemies**: Red sprites that patrol the maze - touching them costs one life
3. **Lives**: You have 2 lives, displayed at the bottom of the screen
4. **Invulnerability**: 2-second invulnerability period after taking damage (red flash effect)
5. **Victory**: Reach the goal marker to complete the level
6. **Game Over**: Lose all lives and return to menu
