# Story System Documentation

## Overview
The story content has been extracted to JSON files for easy modification and expansion. Each story is contained in its own directory with modular JSON files.

## Directory Structure
```
assets/stories/
  shardbearer/                    # Story ID directory
    metadata.json                 # Story metadata and configuration
    ascii_art.json               # ASCII art assets
    nodes/                       # Story nodes directory
      start.json                 # Individual story node
      jack_in.json
      end_liberation.json
      ... (other nodes)
```

## File Formats

### metadata.json
Contains story information:
```json
{
  "id": "shardbearer",
  "title": "Shardbearer: A New Carthage Story",
  "author": "Original Story",
  "version": "1.0.0",
  "description": "...",
  "start_node": "start",
  "tags": ["cyberpunk", "choices-matter"]
}
```

### Story Node Format (nodes/*.json)
Each node represents a scene in the story:
```json
{
  "id": "node_id",
  "title": "Scene Title",
  "ascii_art": "art_key",         // Optional, references ascii_art.json
  "body": "Story text...",
  "choices": [
    {
      "text": "Choice description",
      "next": "next_node_id"
    }
  ],
  "ending": {                      // Optional, for ending nodes
    "code": "ENDING_CODE",
    "name": "Ending Name",
    "summary": "What happened..."
  }
}
```

### ascii_art.json
Maps art keys to ASCII art strings:
```json
{
  "alley": "ASCII art string...",
  "terminal": "ASCII art string...",
  "spire": "ASCII art string..."
}
```

## Creating New Stories

1. Create a new directory under `assets/stories/` with your story ID
2. Create `metadata.json` with story information
3. Create `ascii_art.json` with any ASCII art
4. Create nodes in the `nodes/` subdirectory
5. Ensure all node IDs referenced in choices exist

## Modifying Existing Stories

Simply edit the JSON files. Changes include:
- Text changes: Edit the `body` field in node files
- New choices: Add to the `choices` array
- New paths: Create new node files and link them
- ASCII art: Add new keys to `ascii_art.json`

## Benefits

- **No Recompilation**: Modify story content without rebuilding the game
- **Version Control**: Track story changes separately from code
- **Modding Support**: Players can create custom stories
- **Localization**: Easy to translate by copying story directories
- **Collaboration**: Writers can work on JSON while developers code

## Future Enhancements

- Hot-reloading during development
- Story validation tool
- Visual story editor
- Multiple story pack support
- Conditional choices based on game state