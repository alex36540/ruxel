# Project Title

Team members:

- Alex Lee (al3774)
- Isaac Mixon (igm3923)
- Nicholas Deary (njd5368)

## Summary Description

Ruxel is a pixel art editor with an egui frontend that enables artists to unleash their creativity through an intuitive interface and toolbox.

## Checkpoint Progress Summary

So far, we have decided to pivot from Iced for our frontend due to a lack of documentation. We are now using Egui and Eframe instead, with Catpuccin for our color themeing. With the new frontend framework, we have created a GUI with a color picker, toolbar, canvas, and layers panel. 

The color picker changes and displays the active color at the top of the color panel. The future plan is to add an "edit color" mode for the color picker menu, which has not been implemented yet. The set color is whatever the color was when you stopped drawing the last line. This is a temporary bug because we don't plan to keep the same line drawing logic as it currently stands. The line drawing logic in our program at this checkpoint is copied from the [Egui demos](https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/painting.rs). 

The toolbar includes the following tools: Draw, Erase, Fill, Shape, and File. When selecting any of the tools exluding file, the current tool is switched to the selected tool. Functionality for the other tools is not yet implemented, but the default tool is set to draw. The file menu has options to save, load, create new, and close your current file.

The canvas, centered in the middle of the window, is where the user can draw and, in the future, use the other tools in the toolbar. 

The layer panel is currently unimplemented, but reserves a spot within the UI on the right side of the screen. Future plans include an addition to the layer system that allows for animation editing, following the use case of sprite animation. 

## Additional Details

- List any external Rust crates required for the project (i.e., what
  `[dependencies]` have been added to `Cargo.toml` files).  
  - catppuccin = "2.2.0"
  - eframe = "0.27.1"
  - egui = "0.27.1"
  - image = "0.25.0"

- Briefly describe the structure of the code (what are the main components, the
  module dependency structure).  
  - Main.rs - Calls Ruxel from the UI module. 
  - Model.rs - Contains logic for the Canvas
  - Ui.rs - Sets up the panels for the UI and displays a window with egui. 
    - Palette.rs - Contains the color palette.
- Pose any questions that you may have about your project and/or request
  feedback on specific aspects of the project.
   - "How the f*k do we resize anything that isn't a widget???!!!!!!???!?!?!?" - Nicholas
   - Is it possible to split up our panels into different modules? (ui/color_panel.rs, ui/tool_panel.rs, etc.) This seems to create circular dependencies with the Ruxel struct, unless we copy many of the values from the Ruxel struct for each function. 
