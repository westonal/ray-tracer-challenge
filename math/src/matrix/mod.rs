mod matrix_2x2;
mod matrix_3x3;
mod matrix_4x4;

trait Determinant {
    fn determinant(&self) -> f32;
}

trait Minor {
    fn minor(&self, row: usize, column: usize) -> f32;
}

trait Cofactor {
    fn cofactor(&self, row: usize, column: usize) -> f32;
}

impl<T: Minor> Cofactor for T {
    fn cofactor(&self, row: usize, column: usize) -> f32 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 1 {
            -minor
        } else {
            minor
        }
    }
}
