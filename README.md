# bevy_falseterm
Create a modular visual gui that resembles that of a terminal emulator. 
Use as much bevy defaults as possible. 
    Using NodeBundle to handle background masks and text.
window independent.
builder pattern with dynamic structuring.
user friendly. 
uses the vim theme config.
    The default theme should be lunar or ayu?

### Limitations 
- This is a stripped down version of a proper gui builder. This is purposely left as restrictive to be user friendly. 
- FTNodes can overlap over each other if the window is too small. This isn't designed to exactly emulate the characteristics of the terminal.

### Temp
1. Spawn in the window.
2. Assign a custom FTBuilder component.
3. bts, the entities are spawned or updated accordingly. The user the required to add an identification tag? 
4. The user is responsible for adding text or whatever? 
