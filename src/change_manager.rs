use std::collections::VecDeque;
use egui::*;

#[derive(Clone)]
pub struct CanvasState {
    pub layers: usize,
    pub active_layer: usize,
    pub layer_names: Vec<String>,
    pub layer_name_cnt: usize,
    pub layers_to_show: Vec<bool>,
    pub pixels: Vec<Color32>,
    pub squares: Vec<Shape>,
}

impl CanvasState {
    pub fn new(layers: usize,
        active_layer: usize,
        layer_names: Vec<String>,
        layer_name_cnt: usize,
        layers_to_show: Vec<bool>,
        pixels: Vec<Color32>,
        squares: Vec<Shape>
    ) -> Self {
        Self {
            layers,
            active_layer,
            layer_names,
            layer_name_cnt,
            layers_to_show,
            pixels,
            squares,
        }
    }
}

const STACK_SIZE: usize = 15;
pub struct ChangeManager {
    current_state: CanvasState,
    undo_stk: VecDeque<CanvasState>,
    redo_stk: VecDeque<CanvasState>,
}

impl ChangeManager {
    pub fn new(initial_state: CanvasState) -> Self {
        Self {
            current_state: initial_state,
            undo_stk: VecDeque::with_capacity(STACK_SIZE),
            redo_stk: VecDeque::with_capacity(STACK_SIZE),
        }
    }

    pub fn is_undo_empty(&self) -> bool {
        self.undo_stk.is_empty()
    }

    pub fn is_redo_empty(&self) -> bool {
        self.redo_stk.is_empty()
    }

    pub fn push_undo(&mut self, s: CanvasState) {
        // if overrun
        if self.undo_stk.len() >= STACK_SIZE {
            self.undo_stk.pop_front();
        }

        self.undo_stk.push_back(s);
    }

    pub fn push_redo(&mut self, s: CanvasState) {
        // if overrun
        if self.redo_stk.len() >= STACK_SIZE {
            self.redo_stk.pop_front();
        }

        self.redo_stk.push_back(s);
    }

    pub fn push_new_state(&mut self, c: CanvasState) {
        if !self.is_redo_empty() {
            self.redo_stk.clear();
        }

        let mut temp_state = c;
        std::mem::swap(&mut temp_state, &mut self.current_state);
        // theoretically, prev_state now holds the old current state, and current_state holds the new state

        self.push_undo(temp_state);
    }

    /// Returns the state to undo to
    pub fn undo(&mut self) -> &CanvasState {
        // Undo stack is guaranteed to not be empty when called
        let mut temp_state = self.undo_stk.pop_back().unwrap(); // has what we want to return

        std::mem::swap(&mut temp_state, &mut self.current_state); // after this, temp_state will be holding what we want to push to redo, and current_state has what we want to return a copy of

        self.push_redo(temp_state);
        &self.current_state
    }

    pub fn redo(&mut self) -> &CanvasState {
        // Redo stack is guaranteed to not be empty when called
        let mut temp_state = self.redo_stk.pop_back().unwrap(); // has what we want to return a copy of and move to current_state

        std::mem::swap(&mut temp_state, &mut self.current_state); // after this, temp_state holds what we want to push to undo, and need to return a copy of what is in current_state

        self.push_undo(temp_state);
        &self.current_state
    }
}