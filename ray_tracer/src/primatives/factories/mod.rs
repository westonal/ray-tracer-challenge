mod triangle;

#[macro_export]
macro_rules! shape {
    (
        surface: $surface:expr;
    ) => {
        $crate::primatives::Shape::new(math::matrix::matrix_4x4::Matrix4x4::identity(), $surface)
    };
    (
        surface: $surface:expr;
        $(matrix: $matrix:expr;)?
        $(material: $material:expr;)?
        $(pattern: $pattern:expr;)?
    ) => {
        {
            let mut shape = $crate::shape!(surface: $surface;);
            $(shape.matrix = $matrix;)?
            $(shape.material = $material.clone();)?
            $(shape.material.pattern = $pattern;)?
            shape
        }
    };
}

#[macro_export]
macro_rules! sphere {
    ($(matrix: $matrix:expr)?) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::UnitSphere;
            $(matrix: $matrix;)?
        )
    };

    (
        $(matrix: $matrix:expr;)?
        $(material: $material:expr;)?
        $(pattern: $pattern:expr$(;)?)?
    ) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::UnitSphere;
            $(matrix: $matrix;)?
            $(material: $material;)?
            $(pattern: $pattern;)?
        )
    };
}

#[macro_export]
macro_rules! plane {
   ($(matrix: $matrix:expr)?) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::PlaneXZ;
            $(matrix: $matrix;)?
        )
    };

    (
        $(matrix: $matrix:expr;)?
        $(material: $material:expr;)?
        $(pattern: $pattern:expr$(;)?)?
    ) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::PlaneXZ;
            $(matrix: $matrix;)?
            $(material: $material;)?
            $(pattern: $pattern;)?
        )
    };
}

#[macro_export]
macro_rules! cube {
    ($(matrix: $matrix:expr)?) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::UnitCube;
            $(matrix: $matrix;)?
        )
    };

    (
        $(matrix: $matrix:expr;)?
        $(material: $material:expr;)?
        $(pattern: $pattern:expr$(;)?)?
    ) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::UnitCube;
            $(matrix: $matrix;)?
            $(material: $material;)?
            $(pattern: $pattern;)?
        )
    };
}

#[macro_export]
macro_rules! cylinder {
    ($(matrix: $matrix:expr)?) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::UnitCylinder($crate::primatives::CylinderCapStyle::Closed);
            $(matrix: $matrix;)?
        )
    };

    (
        $(matrix: $matrix:expr;)?
        $(material: $material:expr;)?
        $(pattern: $pattern:expr$(;)?)?
    ) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::UnitCylinder($crate::primatives::CylinderCapStyle::Closed);
            $(matrix: $matrix;)?
            $(material: $material;)?
            $(pattern: $pattern;)?
        )
    };
}

#[macro_export]
macro_rules! cylinder_open {
    ($(matrix: $matrix:expr)?) => {
        $crate::shape!(surface: $crate::primatives::Surface::UnitCylinder($crate::primatives::CylinderCapStyle::Open); $(matrix: $matrix;)?)
    };
}

#[cfg(test)]
mod cylinder_factory_tests {
    use math::matrix::matrix_4x4::Matrix4x4;

    use crate::primatives::CylinderCapStyle;
    use crate::primatives::Surface::UnitCylinder;
    use crate::{cylinder, cylinder_open};

    #[test]
    fn cylinders_are_closed_by_default() {
        assert_eq!(UnitCylinder(CylinderCapStyle::Closed), cylinder!().surface);
        assert_eq!(
            UnitCylinder(CylinderCapStyle::Closed),
            cylinder!(matrix: Matrix4x4::identity()).surface
        );
    }

    #[test]
    fn open_cylinder() {
        assert_eq!(
            UnitCylinder(CylinderCapStyle::Open),
            cylinder_open!().surface
        );
        assert_eq!(
            UnitCylinder(CylinderCapStyle::Open),
            cylinder_open!(matrix: Matrix4x4::identity()).surface
        );
    }
}

#[macro_export]
macro_rules! triangle {
    (
        $triangle:expr
        $(; matrix: $matrix:expr)?) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::SingleTriangle($triangle.into());
            $(matrix: $matrix;)?
        )
    };

    (
        $triangle:expr;
        $(matrix: $matrix:expr;)?
        $(material: $material:expr;)?
        $(pattern: $pattern:expr$(;)?)?
    ) => {
        $crate::shape!(
            surface: $crate::primatives::Surface::SingleTriangle(triangle.into());
            $(matrix: $matrix;)?
            $(material: $material;)?
            $(pattern: $pattern;)?
        )
    };
}
