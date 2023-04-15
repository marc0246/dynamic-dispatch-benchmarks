#![feature(test)]

#[cfg(test)]
mod tests {
    use rand::Rng;

    extern crate test;

    #[bench]
    fn vtable_pointer_inside_object(bencher: &mut test::Bencher) {
        struct Shape {
            vtable: &'static VTable,
        }

        struct VTable {
            area: unsafe fn(*const Shape) -> f32,
        }

        struct Square {
            vtable: &'static VTable,
            side: f32,
        }

        let square_vtable = Box::leak(Box::new(VTable {
            area: |square_ptr| {
                let square = unsafe { &(*square_ptr.cast::<Square>()) };
                square.side * square.side
            },
        }));

        struct Rectangle {
            vtable: &'static VTable,
            width: f32,
            height: f32,
        }

        let rectangle_vtable = Box::leak(Box::new(VTable {
            area: |square_ptr| {
                let rectangle = unsafe { &(*square_ptr.cast::<Rectangle>()) };
                rectangle.width * rectangle.height
            },
        }));

        struct Triangle {
            vtable: &'static VTable,
            base: f32,
            height: f32,
        }

        let triangle_vtable = Box::leak(Box::new(VTable {
            area: |triangle_ptr| {
                let triangle = unsafe { &(*triangle_ptr.cast::<Triangle>()) };
                triangle.base * triangle.height / 2.0
            },
        }));

        struct Circle {
            vtable: &'static VTable,
            radius: f32,
        }

        let circle_vtable = Box::leak(Box::new(VTable {
            area: |circle_ptr| {
                let circle = unsafe { &(*circle_ptr.cast::<Circle>()) };
                circle.radius * circle.radius
            },
        }));

        // Construct a bunch of random shapes
        let mut rng = rand::thread_rng();
        let shapes = (0..100_000)
            .map(|_| match rng.gen_range(0..4) {
                0 => Box::into_raw(Box::new(Square {
                    vtable: square_vtable,
                    side: rng.gen(),
                })) as _,
                1 => Box::into_raw(Box::new(Rectangle {
                    vtable: rectangle_vtable,
                    width: rng.gen(),
                    height: rng.gen(),
                })) as _,
                2 => Box::into_raw(Box::new(Triangle {
                    vtable: triangle_vtable,
                    base: rng.gen(),
                    height: rng.gen(),
                })) as _,
                3 => Box::into_raw(Box::new(Circle {
                    vtable: circle_vtable,
                    radius: rng.gen(),
                })) as _,
                _ => unreachable!(),
            })
            .collect::<Vec<*const Shape>>();

        // Benchmark the sum of their areas
        bencher.iter(|| {
            let mut sum = 0.0;
            for shape_ptr in shapes.iter().copied() {
                sum += unsafe { ((*shape_ptr).vtable.area)(shape_ptr) };
            }
            sum
        });
    }

    #[bench]
    fn vtable_pointer_alongside_object_pointer(bencher: &mut test::Bencher) {
        struct VTable {
            area: unsafe fn(*const ()) -> f32,
        }

        #[derive(Clone, Copy)]
        struct ShapePtr {
            shape: *const (),
            vtable: &'static VTable,
        }

        struct Square {
            side: f32,
        }

        let square_vtable = Box::leak(Box::new(VTable {
            area: |square_ptr| {
                let square = unsafe { &(*square_ptr.cast::<Square>()) };
                square.side * square.side
            },
        }));

        struct Rectangle {
            width: f32,
            height: f32,
        }

        let rectangle_vtable = Box::leak(Box::new(VTable {
            area: |square_ptr| {
                let rectangle = unsafe { &(*square_ptr.cast::<Rectangle>()) };
                rectangle.width * rectangle.height
            },
        }));

        struct Triangle {
            base: f32,
            height: f32,
        }

        let triangle_vtable = Box::leak(Box::new(VTable {
            area: |triangle_ptr| {
                let triangle = unsafe { &(*triangle_ptr.cast::<Triangle>()) };
                triangle.base * triangle.height / 2.0
            },
        }));

        struct Circle {
            radius: f32,
        }

        let circle_vtable = Box::leak(Box::new(VTable {
            area: |circle_ptr| {
                let circle = unsafe { &(*circle_ptr.cast::<Circle>()) };
                circle.radius * circle.radius
            },
        }));

        // Construct a bunch of random shapes
        let mut rng = rand::thread_rng();
        let shapes = (0..100_000)
            .map(|_| match rng.gen_range(0..4) {
                0 => ShapePtr {
                    shape: Box::into_raw(Box::new(Square { side: rng.gen() })) as _,
                    vtable: square_vtable,
                },
                1 => ShapePtr {
                    shape: Box::into_raw(Box::new(Rectangle {
                        width: rng.gen(),
                        height: rng.gen(),
                    })) as _,
                    vtable: rectangle_vtable,
                },
                2 => ShapePtr {
                    shape: Box::into_raw(Box::new(Triangle {
                        base: rng.gen(),
                        height: rng.gen(),
                    })) as _,
                    vtable: triangle_vtable,
                },
                3 => ShapePtr {
                    shape: Box::into_raw(Box::new(Circle { radius: rng.gen() })) as _,
                    vtable: circle_vtable,
                },
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        // Benchmark the sum of their areas
        bencher.iter(|| {
            let mut sum = 0.0;
            for shape_ptr in shapes.iter().copied() {
                sum += unsafe { (shape_ptr.vtable.area)(shape_ptr.shape) };
            }
            sum
        });
    }

    #[bench]
    fn vtable_alongside_object_pointer(bencher: &mut test::Bencher) {
        #[derive(Clone, Copy)]
        struct VTable {
            area: unsafe fn(*const ()) -> f32,
        }

        #[derive(Clone, Copy)]
        struct ShapePtr {
            shape: *const (),
            vtable: VTable,
        }

        struct Square {
            side: f32,
        }

        let square_vtable = VTable {
            area: |square_ptr| {
                let square = unsafe { &(*square_ptr.cast::<Square>()) };
                square.side * square.side
            },
        };

        struct Rectangle {
            width: f32,
            height: f32,
        }

        let rectangle_vtable = VTable {
            area: |square_ptr| {
                let rectangle = unsafe { &(*square_ptr.cast::<Rectangle>()) };
                rectangle.width * rectangle.height
            },
        };

        struct Triangle {
            base: f32,
            height: f32,
        }

        let triangle_vtable = VTable {
            area: |triangle_ptr| {
                let triangle = unsafe { &(*triangle_ptr.cast::<Triangle>()) };
                triangle.base * triangle.height / 2.0
            },
        };

        struct Circle {
            radius: f32,
        }

        let circle_vtable = VTable {
            area: |circle_ptr| {
                let circle = unsafe { &(*circle_ptr.cast::<Circle>()) };
                circle.radius * circle.radius
            },
        };

        // Construct a bunch of random shapes
        let mut rng = rand::thread_rng();
        let shapes = (0..100_000)
            .map(|_| match rng.gen_range(0..4) {
                0 => ShapePtr {
                    shape: Box::into_raw(Box::new(Square { side: rng.gen() })) as _,
                    vtable: square_vtable,
                },
                1 => ShapePtr {
                    shape: Box::into_raw(Box::new(Rectangle {
                        width: rng.gen(),
                        height: rng.gen(),
                    })) as _,
                    vtable: rectangle_vtable,
                },
                2 => ShapePtr {
                    shape: Box::into_raw(Box::new(Triangle {
                        base: rng.gen(),
                        height: rng.gen(),
                    })) as _,
                    vtable: triangle_vtable,
                },
                3 => ShapePtr {
                    shape: Box::into_raw(Box::new(Circle { radius: rng.gen() })) as _,
                    vtable: circle_vtable,
                },
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        // Benchmark the sum of their areas
        bencher.iter(|| {
            let mut sum = 0.0;
            for shape_ptr in shapes.iter().copied() {
                sum += unsafe { (shape_ptr.vtable.area)(shape_ptr.shape) };
            }
            sum
        });
    }
}

fn main() {}
