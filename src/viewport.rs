use euclid::{
    TypedPoint2D,
    TypedTransform2D,
    TypedVector2D,
    TypedSize2D,
    TypedRect,
};

// Represents quantities on the screen
pub struct ScreenSpace;
pub type ScreenPoint = TypedPoint2D<f32, ScreenSpace>;
pub type ScreenVector = TypedVector2D<f32, ScreenSpace>;
pub type ScreenRect = TypedRect<f32, ScreenSpace>;

// Represents quantities in our data.
pub struct ModelSpace;
pub type ModelPoint = TypedPoint2D<f32, ModelSpace>;
pub type ModelVector = TypedVector2D<f32, ModelSpace>;
pub type ModelRect = TypedRect<f32, ModelSpace>;

pub type Transform = TypedTransform2D<f32, ModelSpace, ScreenSpace>;

// We need something to manage the transform matrix we send to
// the GPU.

// There are more considerations here than it at first seemed.
// - Should there be a global transform, or per-object transform?
// - Should there be a single transform per viewport,
//   or is a viewport a stack of transforms?
// - Should this really be part of a larger "Canvas" struct,
//   which replicates, as far as possible, a conventional vector graphics model?
//
// Ultimately out goal is to flexibly plot time-series data. For now
// the goal is simply to factor the out the coordinate transformation
// from the drawing code such that regardless of function we plot, the
// result is always visible.
//
// To do that, we need to know the domain and range of the input, and
// the screen dimensions of the output.


// Domain and range are intervals, and I've found it's easier to work
// with these using an abstraction. Note, there's some similarity to
// the Range* traits in std, but those are for iterators, while this
// represents the mathematical concept of a continuous set of numbers
// between two endpoints.
#[derive(Debug)]
pub struct Interval {
    pub lower: f32,
    pub upper: f32,
    pub span: f32,
}

impl Interval {
    pub fn from_endpoints(a: f32, b: f32) -> Interval {
        let (upper, lower) = if a < b {
            (b, a)
        } else {
            (a, b)
        };
        let span = upper - lower;

        Interval {
            lower,
            upper,
            span,
        }
    }
}

pub struct ViewPort {
    // domain: Interval,
    // range: Interval,
    transform: Transform,
}

impl ViewPort {
    pub fn new(domain: Interval, range: Interval) -> ViewPort {
        let model = ModelRect::new(
            ModelPoint::new(domain.lower, range.lower),
            // ... The type mismatch here led me me to change .upper to .span!
            TypedSize2D::new(domain.span, range.span),
        );

        // Use the default opengl viewport for now.
        let screen = ScreenRect::new(
            ScreenPoint::new(-1.0, -1.0),
            TypedSize2D::new(1.0, 1.0)
        );

        // What confused me a great deal here is that the output
        // viewport places 0,0 at the center of the screen rather than
        // a specific corner.
        //
        // Naturally, we could override the default gl viewport, and
        // we must at some point.
        let transform = Transform::identity()
            .pre_translate(-model.center().to_vector())
            .pre_scale(
                2.0 * screen.size.width / domain.span,
                2.0 * screen.size.height / range.span,
            );

        ViewPort {
            // domain,
            // range,
            transform
        }
    }

    pub fn get_transform(&self) -> Transform {
        self.transform
    }
}
