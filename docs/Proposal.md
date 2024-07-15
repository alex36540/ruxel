# Ruxel

Team members:

- Alex Lee (al3774)
- Isaac Mixon (igm3923)
- Nicholas Deary (njd5368)
- Yaqim Auguste (yaa6681)

## Summary Description

Ruxel is a pixel editor that enables artists to unleash their creativity
through an intuitive interface and toolbox.

## Additional Details

- One or more typical “use cases”. These might include “storyboards” explaining
  how a user would interact with the program or some interesting “input/output”
  examples.

  - An artist wants to create an image of a castle, so they open Ruxel, create
    the desired image using tool, and then save the image as a Ruxel file or
    png.
  - An artist wants to create a quick and easy square/circle (outlined or
    filled), so the artist selects their color and the square/circle tool,
    clicks and drags on the canvas, and sees the new square/circle.
  - An artist wants to create some variation in their image, so they select
    noise brush, and some of the pixels they paint are changed into a noise
    pattern.
  - An artist wants to quickly add text to their beautiful pixel art map. They
    select the text tool, select a font, type in the name of their new kingdom,
    and then move the text where they want it.
  - An artist wants to create an animation, so they use the animation timeline
    to create several images of the same sprite, decide on the step time
    between each image, and export as a gif (pronounced like the peanut butter).
  - An artist wants to create several of the same blocks of pixels, so they use
    select tools to copy and paste blocks of art.

- A sketch of intended components (key functions, key data structures, separate
  modules).

  - Functions:
    - Get/Handle mouse input,
    - Add pixel/remove pixel,
    - Adjust pixel size,
    - Add pre-generated shapes (squares, circles, lines)
    - Save image, create new image, redraw/reset image, load image
    - Fill shape, 
    - Create noise texture,
    - initialize window,
    - copy/paste pixels,
    - add text

  - Data Structures: 
    - Pixel struct representation with width and height,
    - Struct to keep track of Mouse input (mouse_up, mouse_down, mouse_position),
    - color and shape enums for pre-generated options

  - Modules to handle: 
    - Canvas
    - Tools (Draw, Erase, Select, Save/Load, etc.)
    - Colors
    - Shapes
    - Mouse events
    - Pixels

- Thoughts on testing. These might include critical functions or data structures
  that will be given `#[test]` functions. Also consider using the
  [`test_case`](https://crates.io/crates/test-case) crate,
  [`quickcheck`](https://crates.io/crates/quickcheck) crate,
  [`proptest`](https://crates.io/crates/proptest) crate, or [`cargo
  fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) tool.

  - Frontend
    - Iced (the GUI library we are leaning towards) does not provide easy test
      infrastructure. The word "test" appears less than 30 times in the offical
      repo. Because of this, we probably will not test our GUI beyond manual
      tests.

  - Backend
    - Test Canvas
      - Assign values out of range. (Only test in palette selection?)
      - Test individual tools.
        - Test that a mouse event colors the correct pixel.
        - Test that bucket fill fills multiple types of shapes.
        - Test that draw shape creates shapes of the correct size and fills
          conditionally.
        - Test that copy/paste work as expected.
    - Test color palette.
      - Test selecting (inputting) colors out of range are capped at 255.
      - Test that selecting the same color twice preserves the color.
      - Test that coloring a pixel after changing the color twice uses the
        correct color.
    - Test layers (if we have layers).
    - Test animation.
      - Test exporting gifs.
    - Test saving and loading files.
      - Test saving as a Ruxel file.
      - Test saving as a png file.
  
  The test_case crate looks helpful, but the others don't seem as valuable yet.
  We might change our minds as we start testing the project.


- Thoughts on a “minimal viable product” and “stretch goals”. Be sure to review
  the final project grading rubric and consider organizing the project around a
  core deliverable that will almost certainly be achieved and then a number of
  extensions and features that could be added to ensure that project is of
  suitable size/scope/effort.

  Our minimal viable product will be a GUI that allows users to "draw" colored
  pixels on a screen using a frontend framework and implementing the logic in
  Rust. First, the only function will be drawing lines and erasing, but
  additional features such as drawing shapes, adding text, selecting regions and
  copy/pasting, animation, etc. will be added to make sure that the project is a
  suitable size. 

- Expected functionality to be completed at the Checkpoint.

  By April 3rd, we have the MVP done (a GUI that allows us to add pixels and save
  to a file). In addition, we will complete one of the following stretch goals:

    - Bucket Tool
    - Line Tool
    - Shape Tool
    - Noise Tool
    - Animation Creator

