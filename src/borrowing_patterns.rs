//! Advanced borrowing patterns and common borrow checker challenges
//!
//! These examples demonstrate patterns that are particularly challenging
//! for LLMs to get right, especially around multiple borrows and complex
//! data structure access patterns.

/// # Problem 1: Double Mutable Borrow from Same Vector
///
/// This is a common mistake - trying to get two mutable references from the same vector.
/// The Rust borrow checker prevents this because it could lead to aliasing issues.

// ❌ This DOESN'T work - commented out to prevent compilation error:
// pub fn wrong_double_mut_borrow(vec: &mut Vec<i32>) {
//     let a = &mut vec[0];  // First mutable borrow
//     let b = &mut vec[1];  // Second mutable borrow - ERROR!
//     *a += 1;
//     *b += 1;
// }

/// ✅ Solution 1: Use split_at_mut to get non-overlapping mutable slices
pub fn double_mut_borrow_split(vec: &mut Vec<i32>) {
    if vec.len() < 2 {
        return;
    }

    let (left, right) = vec.split_at_mut(1);
    let a = &mut left[0];
    let b = &mut right[0]; // right[0] is actually vec[1]
    *a += 1;
    *b += 1;
}

/// ✅ Solution 2: Use slice patterns for specific indices
pub fn double_mut_borrow_slice_pattern(vec: &mut Vec<i32>) {
    if vec.len() < 2 {
        return;
    }

    let slice = &mut vec[..];
    if let [a, b, ..] = slice {
        *a += 1;
        *b += 1;
    }
}

/// ✅ Solution 3: Use iter_mut when you need to modify all elements
pub fn modify_multiple_elements(vec: &mut Vec<i32>, indices: &[usize]) {
    for (i, elem) in vec.iter_mut().enumerate() {
        if indices.contains(&i) {
            *elem += 1;
        }
    }
}

/// ✅ Solution 4: Get mutable references at specific indices safely
pub fn get_two_mut<T>(slice: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    if i == j || i >= slice.len() || j >= slice.len() {
        return None;
    }

    if i < j {
        let (left, right) = slice.split_at_mut(j);
        Some((&mut left[i], &mut right[0]))
    } else {
        let (left, right) = slice.split_at_mut(i);
        Some((&mut right[0], &mut left[j]))
    }
}

/// # Problem 2: 2D Matrix - Borrowing Multiple Unique Points
///
/// Demonstrates how to safely borrow multiple elements from a 2D matrix structure.

/// A simple 2D matrix implemented as a struct containing vectors
pub struct Matrix {
    pub rows: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows: vec![vec![0; cols]; rows],
        }
    }

    /// Get immutable references to two different points
    pub fn get_two_points(&self, a: (usize, usize), b: (usize, usize)) -> Option<(&i32, &i32)> {
        let val_a = self.rows.get(a.0)?.get(a.1)?;
        let val_b = self.rows.get(b.0)?.get(b.1)?;
        Some((val_a, val_b))
    }

    /// ❌ Problem: Can't get two mutable references from same row
    // pub fn wrong_get_two_mut_same_row(&mut self, row: usize, col1: usize, col2: usize) {
    //     let row_vec = &mut self.rows[row];
    //     let a = &mut row_vec[col1];  // First mutable borrow
    //     let b = &mut row_vec[col2];  // Second mutable borrow - ERROR!
    // }

    /// ✅ Solution 1: Get two mutable references from same row using split_at_mut
    pub fn get_two_mut_same_row(
        &mut self,
        row: usize,
        col1: usize,
        col2: usize,
    ) -> Option<(&mut i32, &mut i32)> {
        if col1 == col2 {
            return None;
        }

        let row_vec = self.rows.get_mut(row)?;

        if col1 < col2 {
            let (left, right) = row_vec.split_at_mut(col2);
            Some((&mut left[col1], &mut right[0]))
        } else {
            let (left, right) = row_vec.split_at_mut(col1);
            Some((&mut right[0], &mut left[col2]))
        }
    }

    /// ✅ Solution 2: Get two mutable references from different rows
    /// This is easier because they're in different vectors
    pub fn get_two_mut_different_rows(
        &mut self,
        a: (usize, usize),
        b: (usize, usize),
    ) -> Option<(&mut i32, &mut i32)> {
        if a.0 == b.0 {
            // Same row - use the same row method
            return self.get_two_mut_same_row(a.0, a.1, b.1);
        }

        if a.0 >= self.rows.len() || b.0 >= self.rows.len() {
            return None;
        }

        // Different rows - we can split the rows vector
        let (row_a, row_b) = if a.0 < b.0 {
            let (upper, lower) = self.rows.split_at_mut(b.0);
            (&mut upper[a.0], &mut lower[0])
        } else {
            let (upper, lower) = self.rows.split_at_mut(a.0);
            (&mut lower[0], &mut upper[b.0])
        };

        Some((row_a.get_mut(a.1)?, row_b.get_mut(b.1)?))
    }

    /// ✅ Solution 3: General solution for any two points
    pub fn get_two_mut_general(
        &mut self,
        a: (usize, usize),
        b: (usize, usize),
    ) -> Option<(&mut i32, &mut i32)> {
        // Check for same position
        if a == b {
            return None;
        }

        // Delegate based on whether they're in the same row
        if a.0 == b.0 {
            self.get_two_mut_same_row(a.0, a.1, b.1)
        } else {
            self.get_two_mut_different_rows(a, b)
        }
    }
}

/// # Problem 3: Enum-Indexed 2D Structure
///
/// More complex: accessing elements by enum type then int index

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Input,
    Hidden,
    Output,
}

/// A structure with vectors accessed by enum type and then int index
pub struct LayeredMatrix {
    pub input: Vec<i32>,
    pub hidden: Vec<i32>,
    pub output: Vec<i32>,
}

impl LayeredMatrix {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        LayeredMatrix {
            input: vec![0; input_size],
            hidden: vec![0; hidden_size],
            output: vec![0; output_size],
        }
    }

    /// Get immutable reference by enum and index
    pub fn get(&self, layer: Layer, index: usize) -> Option<&i32> {
        match layer {
            Layer::Input => self.input.get(index),
            Layer::Hidden => self.hidden.get(index),
            Layer::Output => self.output.get(index),
        }
    }

    /// Get mutable reference by enum and index
    pub fn get_mut(&mut self, layer: Layer, index: usize) -> Option<&mut i32> {
        match layer {
            Layer::Input => self.input.get_mut(index),
            Layer::Hidden => self.hidden.get_mut(index),
            Layer::Output => self.output.get_mut(index),
        }
    }

    /// ✅ Solution 1: Get two mutable references from different layers
    /// This works because they're in different vectors
    /// This is not great because we have to match on all combinations
    pub fn get_two_mut_different_layers(
        &mut self,
        a: (Layer, usize),
        b: (Layer, usize),
    ) -> Option<(&mut i32, &mut i32)> {
        // Ensure they're in different layers
        if a.0 == b.0 {
            return None;
        }

        // We need to match on all combinations to satisfy the borrow checker
        match (a.0, b.0) {
            (Layer::Input, Layer::Hidden) => {
                Some((self.input.get_mut(a.1)?, self.hidden.get_mut(b.1)?))
            }
            (Layer::Input, Layer::Output) => {
                Some((self.input.get_mut(a.1)?, self.output.get_mut(b.1)?))
            }
            (Layer::Hidden, Layer::Input) => {
                Some((self.hidden.get_mut(a.1)?, self.input.get_mut(b.1)?))
            }
            (Layer::Hidden, Layer::Output) => {
                Some((self.hidden.get_mut(a.1)?, self.output.get_mut(b.1)?))
            }
            (Layer::Output, Layer::Input) => {
                Some((self.output.get_mut(a.1)?, self.input.get_mut(b.1)?))
            }
            (Layer::Output, Layer::Hidden) => {
                Some((self.output.get_mut(a.1)?, self.hidden.get_mut(b.1)?))
            }
            _ => None, // Same layer
        }
    }

    /// ✅ Solution 2: Get two mutable references from the same layer
    pub fn get_two_mut_same_layer(
        &mut self,
        layer: Layer,
        i: usize,
        j: usize,
    ) -> Option<(&mut i32, &mut i32)> {
        if i == j {
            return None;
        }

        let vec = match layer {
            Layer::Input => &mut self.input,
            Layer::Hidden => &mut self.hidden,
            Layer::Output => &mut self.output,
        };

        get_two_mut(vec, i, j)
    }

    /// ✅ Solution 3: General solution for any two positions
    pub fn get_two_mut_general(
        &mut self,
        a: (Layer, usize),
        b: (Layer, usize),
    ) -> Option<(&mut i32, &mut i32)> {
        if a == b {
            return None;
        }

        if a.0 == b.0 {
            // Same layer - use split_at_mut approach
            self.get_two_mut_same_layer(a.0, a.1, b.1)
        } else {
            // Different layers - can borrow from both
            self.get_two_mut_different_layers(a, b)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BetterLayeredMatrixLayer {
    Input,
    Hidden,
    Output,
}

pub struct BetterMapLayeredMatrix {
    pub layers: std::collections::HashMap<BetterLayeredMatrixLayer, Vec<i32>>,
}

impl BetterMapLayeredMatrix {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        BetterMapLayeredMatrix {
            layers: std::collections::HashMap::from([
                (BetterLayeredMatrixLayer::Input, vec![0; input_size]),
                (BetterLayeredMatrixLayer::Hidden, vec![0; hidden_size]),
                (BetterLayeredMatrixLayer::Output, vec![0; output_size]),
            ]),
        }
    }

    /// ✅ Solution 1: Get two mutable references from different layers
    /// This pattern avoids the exhaustive matching.
    /// Instead, we move the problem into runtime by iterating over the layers.
    /// If iteration is a hotspot, consider BetterVecLayeredMatrix which uses a Vec<(Layer, Vec<i32>)>
    /// and statically defines the order of layers.
    /// This avoids the N^2 combinatorial explosion of match arms.
    pub fn get_two_mut_different_layers_better(
        &mut self,
        a: (BetterLayeredMatrixLayer, usize),
        b: (BetterLayeredMatrixLayer, usize),
    ) -> Option<(&mut i32, &mut i32)> {
        if a.0 == b.0 {
            return None;
        }

        let mut first: Option<&mut i32> = None;
        let mut second: Option<&mut i32> = None;
        for (layer, vec) in &mut self.layers {
            if *layer == a.0 {
                first = Some(vec.get_mut(a.1)?);
            } else if *layer == b.0 {
                second = Some(vec.get_mut(b.1)?);
            }
        }
        
        Some((first?, second?))
    }
}

pub struct BetterVecLayeredMatrix {
    pub layers: Vec<Vec<i32>>,
}

impl BetterVecLayeredMatrix {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        BetterVecLayeredMatrix {
            layers: vec![
                vec![0; input_size],
                vec![0; hidden_size],
                vec![0; output_size],
            ],
        }
    }

    /// ✅ Solution 1: Get two mutable references from different layers
    /// This pattern avoids the exhaustive matching and avoids iteration.
    /// This avoids the N^2 combinatorial explosion of match arms.
    pub fn get_two_mut_different_layers_better(
        &mut self,
        a: (BetterLayeredMatrixLayer, usize),
        b: (BetterLayeredMatrixLayer, usize),
    ) -> Option<(&mut i32, &mut i32)> {
        if a.0 == b.0 {
            return None;
        }

        // a should be the smaller layer
        if a.0 as usize > b.0 as usize {
            return self.get_two_mut_different_layers_better(b, a);
        }

        let split_idx = b.0 as usize;
        let (first_layers_split, second_layers_split) = self.layers.split_at_mut(split_idx);

        // Now we can safely borrow from both splits. indexing into a.0 is safe because a.0 < b.0
        let first_layers = &mut first_layers_split[a.0 as usize];
        let second_layers = &mut second_layers_split[0];
        
        Some((first_layers.get_mut(a.1)?, second_layers.get_mut(b.1)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_mut_borrow_split() {
        let mut vec = vec![1, 2, 3, 4];
        double_mut_borrow_split(&mut vec);
        assert_eq!(vec, vec![2, 3, 3, 4]);
    }

    #[test]
    fn test_double_mut_borrow_slice_pattern() {
        let mut vec = vec![10, 20, 30];
        double_mut_borrow_slice_pattern(&mut vec);
        assert_eq!(vec, vec![11, 21, 30]);
    }

    #[test]
    fn test_get_two_mut() {
        let mut arr = [1, 2, 3, 4, 5];
        if let Some((a, b)) = get_two_mut(&mut arr, 1, 3) {
            *a = 10;
            *b = 20;
        }
        assert_eq!(arr, [1, 10, 3, 20, 5]);
    }

    #[test]
    fn test_get_two_mut_same_index() {
        let mut arr = [1, 2, 3];
        assert!(get_two_mut(&mut arr, 1, 1).is_none());
    }

    #[test]
    fn test_matrix_get_two_points() {
        let mut matrix = Matrix::new(3, 3);
        matrix.rows[0][0] = 5;
        matrix.rows[1][1] = 10;

        if let Some((a, b)) = matrix.get_two_points((0, 0), (1, 1)) {
            assert_eq!(*a, 5);
            assert_eq!(*b, 10);
        } else {
            panic!("Should return Some");
        }
    }

    #[test]
    fn test_matrix_get_two_mut_same_row() {
        let mut matrix = Matrix::new(3, 3);
        if let Some((a, b)) = matrix.get_two_mut_same_row(0, 0, 2) {
            *a = 5;
            *b = 10;
        }
        assert_eq!(matrix.rows[0][0], 5);
        assert_eq!(matrix.rows[0][2], 10);
    }

    #[test]
    fn test_matrix_get_two_mut_different_rows() {
        let mut matrix = Matrix::new(3, 3);
        if let Some((a, b)) = matrix.get_two_mut_different_rows((0, 1), (2, 1)) {
            *a = 7;
            *b = 13;
        }
        assert_eq!(matrix.rows[0][1], 7);
        assert_eq!(matrix.rows[2][1], 13);
    }

    #[test]
    fn test_matrix_get_two_mut_general() {
        let mut matrix = Matrix::new(3, 3);

        // Same row
        if let Some((a, b)) = matrix.get_two_mut_general((0, 0), (0, 2)) {
            *a = 1;
            *b = 2;
        }
        assert_eq!(matrix.rows[0][0], 1);
        assert_eq!(matrix.rows[0][2], 2);

        // Different rows
        if let Some((a, b)) = matrix.get_two_mut_general((1, 1), (2, 2)) {
            *a = 3;
            *b = 4;
        }
        assert_eq!(matrix.rows[1][1], 3);
        assert_eq!(matrix.rows[2][2], 4);
    }

    #[test]
    fn test_layered_matrix_get() {
        let mut lm = LayeredMatrix::new(2, 3, 2);
        lm.input[0] = 5;
        lm.hidden[1] = 10;
        lm.output[0] = 15;

        assert_eq!(lm.get(Layer::Input, 0), Some(&5));
        assert_eq!(lm.get(Layer::Hidden, 1), Some(&10));
        assert_eq!(lm.get(Layer::Output, 0), Some(&15));
    }

    #[test]
    fn test_layered_matrix_get_two_mut_different_layers() {
        let mut lm = LayeredMatrix::new(2, 3, 2);

        if let Some((a, b)) = lm.get_two_mut_different_layers((Layer::Input, 0), (Layer::Output, 1))
        {
            *a = 100;
            *b = 200;
        }

        assert_eq!(lm.input[0], 100);
        assert_eq!(lm.output[1], 200);
    }

    #[test]
    fn test_layered_matrix_get_two_mut_same_layer() {
        let mut lm = LayeredMatrix::new(5, 5, 5);

        if let Some((a, b)) = lm.get_two_mut_same_layer(Layer::Hidden, 1, 3) {
            *a = 42;
            *b = 99;
        }

        assert_eq!(lm.hidden[1], 42);
        assert_eq!(lm.hidden[3], 99);
    }

    #[test]
    fn test_layered_matrix_get_two_mut_general() {
        let mut lm = LayeredMatrix::new(3, 3, 3);

        // Different layers
        if let Some((a, b)) = lm.get_two_mut_general((Layer::Input, 0), (Layer::Hidden, 1)) {
            *a = 11;
            *b = 22;
        }
        assert_eq!(lm.input[0], 11);
        assert_eq!(lm.hidden[1], 22);

        // Same layer
        if let Some((a, b)) = lm.get_two_mut_general((Layer::Output, 0), (Layer::Output, 2)) {
            *a = 33;
            *b = 44;
        }
        assert_eq!(lm.output[0], 33);
        assert_eq!(lm.output[2], 44);
    }
}
