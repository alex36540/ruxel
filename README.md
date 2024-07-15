![Ruxel logo](ruxelogo.png)
# Ruxel

Team members:

- Alex Lee (al3774@rit.edu)
- Isaac Mixon (igm3923@rit.edu)
- Nicholas Deary (njd5368@rit.edu)

## Summary Description

Ruxel is a pixel art editor with an egui frontend that enables artists to unleash their creativity through an intuitive interface and toolbox.

## Project Execution Summary
The first step of creating our project was doing research to find what framework would be the best for creating a pixel editor. We decided on egui because of the documentation and flexibility, which ended up being a good decision. Next, we planned out the basic aspects of our project, like how to represent the pixels, where the UI elements would go, and what we would need to implement first. During all of this, we were looking at examples of how others used egui to create their projects, learning how to use the framework along the way. 

The biggest success and lesson of the project was how we structured our development. Everything we did was built on already functioning and robust code, reducing merge conflicts and making it less painful to figure them out. This also made it so that while creating new features, less original code had to be modified since it already was prepared for updates. An example of this was for implementing layers: the methods in the canvas struct already supported indexing by layer and almost none of it needed to be changed. The takeaway is that if we write code with the future in mind, development will always go smoother.

Our project was mainly done feature by feature. First we implemented the most basic functionality for drawing squares on the canvas with the correct color. Next we added in each of the different tools, layer functionality, saving an image to a file, undo/redo, and brush size. 

Each new feature brought a different challenge because of the way the program had to be implemented with egui. It felt like we were cheating the framework, as evident by how much we were fighting against it a lot of the time. There were no magic functions that did exactly what we wanted to do, such as with there not being any way to represent a grid in the UI nicely. 

One thing we were constantly worried about was the performance. Given that the painter object that egui provides takes ownership of a vector of shapes to draw and that egui is an immediate mode gui, meaning that each element is drawn every frame, we had to pass in a copy of a large vector to the painter for every frame. Given this, we were always doing our best to limit the amount of copies that were done and we were always trying to do operations in bulk instead of iterating. 

## Code Structure
Our project is split into two main parts, the models and the ui file. The ui file acts as both a view and a controller, holding a reference to and interacting with both the canvas and the state manager. We did it this way because we are not interacting with the model so much that we need separate controller files. It is mostly getting the correct data from the model using methods we already have. 

As for the model files, we have `change_manger.rs`, and `model.rs`. Importantly, `file_interactions.rs` does serve as a controller separate from the ui file because it made sense to partition that. `model` holds the main structs for the functionality, such as the canvas and the camara. `change_manager` holds the struct that manages holding states from the canvas so that it can be undone/redone using VecDequeues. This file uses a special struct called `CanvasState` which is used to only hold the useful information from the canvas so that a new canvas doesn't need to be created every time we would like to undo and redo. The canvas can then load in this state to update itself.

The ui folder holds miscellaneous things, such as the color palette we use for the program (we think it looks pretty nice!) as well as a file that is used to keep track of the state of the new file modal menu.

## Code Excerpt

The two best examples of Ruxel utilizing Rust specific features come from `src/ui.rs:canvas_ui` and `src/model.rs:update_squares`.
First, `canvas_ui` is a method used for displaying the central panel of the window. This panel is the one that displays the image.
In it, it checks that there is a canvas object with a patern match, then jumps into a large match statement.

```rust
fn canvas_ui(&mut self, ui: &mut eframe::egui::Ui) {
    CentralPanel::default().show_inside(ui, |ui| {
        if let Some(c) = &mut self.canvas {
            Frame::canvas(ui.style()).show(ui, |ui| {
                ...
                if response.dragged_by(PointerButton::Middle) {
                    c.scroll(&response.drag_delta());
                } else {
                    ui.ctx().input(|i| {
                        for event in &i.events {
                            match event {
                                Event::Zoom(amount) => c.zoom(amount),
                                Event::Scroll(amount) => c.scroll(amount),
                                _ => {}
                            }
                        }
                        match self.active_tool {
                            Tool::Draw => {
                                ...
                            }
                            Tool::Erase => {
                                ...
                            }
                            Tool::Fill => {
                                ...
                            }
                            Tool::Rectangle(start) => {  // WOW! An enum with a value!
                                ...
                            }
                            Tool::Eyedrop => {
                                ...
                            }
                        }
                    });
                }

                // Draw shapes
                painter.extend(squares);
            });
        } else {
            ...
        }
    });
}
```

The `Rectangle` tool in this example has a value of `Option<Pos2>`. This lets us know if we are currently
dragging the rectangle shape. If the value is `None`, then the user has not started drawing a rectangle.
If the value is `Some(Pos2)`, the user is shown a shadow of the rectangle they are drawing.

![rect shadow](./ruxel/images/rect_shadow.png)

The second Rust specific feature that was used to increase performance was `.par_iter_mut()`. This iterator
comes from the Rayon library, and it is a drop in replacement for `.iter_mut()`. With a little bit of 
finessing, what was three nested for loops were converted into an iterator over the squares. This then
let us use Rayon to speed up the computation.

```rust
pub fn update_squares(&mut self) {
    self.squares
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, sq)| {
            let l = (idx / (self.width * self.height)) as isize - 1; // -1 for the alpha layer.
            let h = (idx % (self.width * self.height)) / self.width;
            let w = (idx % (self.width * self.height)) % self.width;

            if l < 0 {
                // Alpha layer
                let screen_cords = self
                    .camera
                    .pixel_cords_to_screen_cords(w as isize, h as isize);
                let color = if (h / self.alpha_ratio) % 2 == (w / self.alpha_ratio) % 2 {
                    Color32::DARK_GRAY
                } else {
                    Color32::LIGHT_GRAY
                };
                *sq = self.camera.square_from_screen_cords(screen_cords, color);
            } else {
                // Rest of the layers
                let l = l as usize;

                let screen_cords = self
                    .camera
                    .pixel_cords_to_screen_cords(w as isize, h as isize);
                let color = self.pixels[w + (h * self.width) + (self.width * self.height * l)];
                *sq = self.camera.square_from_screen_cords(screen_cords, color);
            }
        });
}
```

## Difficulties with Rust

The main difficulty with the project was the egui library. This library, while great for many types of applications,
did not seem to be optimized for graphics intensive applications. In particular, the painter object from `src/ui.rs:canvas_ui`
that we saw in the first code excerpt has an `extend` method that takes an object implementing the `Into_Iter` trait. This
makes it easy to paint pixels, but it also means that the pixels have to be copied every frame since `.into_iter()` consumes the vector.
After looking for work arounds online, there didn't seem to be an easy way to optimize this part of the code.

## Approaches attempted
One approach we abandoned early was the use of our first choice GUI library, Iced.rs. Iced has a lot of benefits, such as being an immediate-mode gui with good examples, that lead us to it in the beginning. Ultimately, we ended up pivoting away from it because of the lack of documentation.

A hotly debated topic within our group was the representation of the data structure used to model the canvas. Initially, it seemed obvious to use a 2-dimensional vector to represent the pixels, but as we thought more about trying to optimize the speed of the program we decided that just using a one dimensional vector and indexing it using math based on the size of the canvas would be better for efficiency.

The actual display structure of the canvas was also something we needed to figure out. Initially, we had the idea of making a big grid of buttons that would serve as pixels, but that was quickly improved upon as soon as we saw that we had the ability to place squares down at the mouse cursor position. 

Probably the biggest example of an abandoned approach was how to actually do the drawing. The egui demo has a painter, which we thought would be a great source of inspiration. However, the way it was done in the demo was that it stored what was drawn as a vector of lines, which wouldn't work at all for our goal of creating a pixel editor, since we needed a "grid" representation. 

## Additional Details

- List any external Rust crates required for the project (i.e., what
  `[dependencies]` have been added to `Cargo.toml` files).  
  - catppuccin = "2.2.0"
    - Catpuccin is the color theme we use for the Ruxel UI
  - eframe = "0.27.1"
    - Eframe is a framework for writing apps using egui
  - egui = "0.27.1"
    - Egui is an immediate mode GUI library for Rust
  - image = "0.25.1"
    - Image is a crate used for image encoding and decoding
  - rayon = "1.10.0"
    - Rayon is a data-parallelism library. 
  - rfd = "0.14.1"
    - rfd is a library that allows for the usage of native file dialogs

***

