mod matrix {
    use std::slice::{Iter, IterMut};
    use std::ops::{Index, IndexMut};

    pub struct Matrix<T: Default + Clone> {
        rows: usize,
        columns: usize,
        elements: Vec<T>,
    }

    pub struct RowIter<'a, T>(Iter<'a, T>) where T: Default + Clone;

    impl<'a, T> Iterator for RowIter<'a, T>
    where
    T: Default + Clone,
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    pub struct RowIterMut<'a, T>(IterMut<'a, T>) where T: Default + Clone;

    impl<'a, T> Iterator for RowIterMut<'a, T>
    where
        T: Default + Clone,
    {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    pub struct ColumnIter<'a, T>
    where
        T: Default + Clone,
    {
        row_idx: usize,
        col_idx: usize,
        matrix: &'a Matrix<T>,
    }

    impl<'a, T> Iterator for ColumnIter<'a, T>
    where
        T: Default + Clone,
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.row_idx < self.matrix.rows {
                let index = (self.row_idx, self.col_idx);
                self.row_idx += 1;
                Some(&self.matrix[index])
            } else {
                None
            }
        }
    }

    // pub struct ColumnIterMut<'a, T>
    // where
    //     T: Default + Clone,
    // {
    //     row_idx: usize,
    //     col_idx: usize,
    //     matrix: &'a mut Matrix<T>,
    // }

    // impl<'a, T> Iterator for ColumnIterMut<'a, T>
    // where
    //     T: Default + Clone,
    // {
    //     type Item = &'a mut T;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         if self.row_idx < self.matrix.rows {
    //             let index = (self.row_idx, self.col_idx);
    //             self.row_idx += 1;
    //             Some(&mut self.matrix[index])
    //         } else {
    //             None
    //         }
    //     }
    // }

    impl<T: Default + Clone> Matrix<T> {
        pub fn new(rows: usize, columns: usize, elements: Vec<T>) -> Self {
            Self { rows, columns, elements }
        }

        pub fn empty(rows: usize, columns: usize) -> Self {
            let elements = vec![T::default(); rows * columns];
            Self::new(rows, columns, elements)
        }

        pub fn empty_squared(size: usize) -> Self {
            Self::empty(size, size)
        }

        pub fn shape(&self) -> (usize, usize) {
            (self.rows, self.columns)
        }

        pub fn row(&self, row_idx: usize) -> RowIter<T> {
            let start_idx = row_idx * self.columns;
            let stop_idx = start_idx + self.columns;
            RowIter(self.elements[start_idx..stop_idx].iter())
        }

        pub fn row_mut(&mut self, row_idx: usize) -> RowIterMut<T> {
            let start_idx = row_idx * self.columns;
            let stop_idx = start_idx + self.columns;
            RowIterMut(self.elements[start_idx..stop_idx].iter_mut())
        }

        pub fn column(&self, col_idx: usize) -> ColumnIter<T> {
            ColumnIter {
                row_idx: 0,
                col_idx,
                matrix: &self
            }
        }

        // pub fn column_mut(&self, col_idx: usize) -> ColumnIterMut<T> {
        //     ColumnIterMut {
        //         row_idx: 0,
        //         col_idx,
        //         matrix: &mut self
        //     }
        // }
    }

    impl<T: Default + Clone> Index<(usize, usize)> for Matrix<T> {
        type Output = T;

        fn index(&self, index2d: (usize, usize)) -> &Self::Output {
            let (row_idx, col_idx) = index2d;
            let element_idx = row_idx * self.columns + col_idx;
            &self.elements[element_idx]
        }
    }

    impl<T: Default + Clone> IndexMut<(usize, usize)> for Matrix<T> {
        fn index_mut(&mut self, index2d: (usize, usize)) -> &mut T {
            let (row_idx, col_idx) = index2d;
            let element_idx = row_idx * self.columns + col_idx;
            &mut self.elements[element_idx]
        }
    }
}