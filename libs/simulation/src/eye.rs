use crate::*;
use std::f32::consts::*;

/// How far our eye can see:
///
/// -----------------
/// |               |
/// |               |
/// |               |
/// |@      %      %|
/// |               |
/// |               |
/// |               |
/// -----------------
///
/// If @ marks our birdie and % marks food, then a FOV_RANGE of:
///
/// - 0.1 = 10% of the map = bird sees no foods (at least in this case)
/// - 0.5 = 50% of the map = bird sees one of the foods
/// - 1.0 = 100% of the map = bird sees both foods
const FOV_RANGE: f32 = 0.25;

/// How wide our eye can see.
///
/// If @> marks our birdie (rotated to the right) and . marks the area
/// our birdie sees, then a FOV_ANGLE of:
///
/// - PI/2 = 90° =
///   -----------------
///   |             /.|
///   |           /...|
///   |         /.....|
///   |       @>......|
///   |         \.....|
///   |           \...|
///   |             \.|
///   -----------------
///
/// - PI = 180° =
///   -----------------
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   |       @>......|
///   |       |.......|
///   |       |.......|
///   |       |.......|
///   -----------------
///
/// - 2 * PI = 360° =
///   -----------------
///   |...............|
///   |...............|
///   |...............|
///   |.......@>......|
///   |...............|
///   |...............|
///   |...............|
///   -----------------
///
/// Field of view depends on both FOV_RANGE and FOV_ANGLE:
///
/// - FOV_RANGE=0.4, FOV_ANGLE=PI/2:
///   -----------------
///   |       @       |
///   |     /.v.\     |
///   |   /.......\   |
///   |   ---------   |
///   |               |
///   |               |
///   |               |
///   -----------------
///
/// - FOV_RANGE=0.5, FOV_ANGLE=2*PI:
///   -----------------
///   |               |
///   |      ---      |
///   |     /...\     |
///   |    |..@..|    |
///   |     \.../     |
///   |      ---      |
///   |               |
///   -----------------
const FOV_ANGLE: f32 = FRAC_PI_4;

/// How much photoreceptors there are in a single eye.
///
/// More cells means our birds will have more "crisp" vision, allowing
/// them to locate the food more precisely - but the trade-off is that
/// the evolution process will then take longer, or even fail, unable
/// to find any solution.
///
/// I've found values between 3~11 sufficient, with eyes having more
/// than ~20 photoreceptors yielding progressively worse results.
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    // FOV_RANGE, FOV_ANGLE & CELLS are the values we'll use during
    // simulation - but being able to create an arbitrary eye will
    // come handy during the testing:
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(&self, position: Point, rotation: Rotation, foods: &[Food]) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;

            // ^ Represents a *vector* from food to us
            //
            // In case this is the first time you hear the word `vector`, a
            // quick definition would be:
            //
            // > A vector is an object that has *magnitude* (aka length)
            // > and *direction*.
            //
            // You could say a vector is an arrow:
            //
            //   ---> this is a vector of magnitude=3 (if we count each
            //        dash as a single "unit of space") and direction=0°
            //        (at least relative to the X axis)
            //
            //    |   this is a vector of magnitude=1 and direction=90°
            //    v   (at least when we treat direction clockwise)
            //
            // Our food-to-birdie vectors are no different:
            //
            // ---------
            // |       |  gets us this vector:
            // |@     %|          <-----
            // |       |  (magnitude=5, direction=180°)
            // ---------
            //
            // ---------  gets us this vector:
            // |   %   |           |
            // |       |           |
            // |   @   |           v
            // ---------  (magnitude=2, direction=90°)
            //
            // This is not to be confused with Rust's `Vec` or C++'s
            // `std::vector`, which technically *are* vectors, but in a more
            // abstract sense -- better not to overthink it.
            //
            // (https://stackoverflow.com/questions/581426/why-is-a-c-vector-called-a-vector).
            //
            // ---
            // | Fancy way of saying "length of the vector".
            // ----------- v----v
            let dist = vec.norm();

            if dist >= self.fov_range {
                continue;
            }

            // Returns vector's direction relative to the Y axis, that is:
            //
            //    ^
            //    |  = 0° = 0
            //
            //   --> = 90° = -PI / 2
            //
            //    |  = 180° = -PI
            //    v
            //
            // (if you've been measuring rotations before - this is atan2
            // in disguise.)
            let angle = na::Rotation2::rotation_between(&na::Vector2::y(), &vec).angle();

            // Because our bird is *also* rotated, we have to include its
            // rotation too:
            let angle = angle - rotation.angle();

            // Rotation is wrapping (from -PI to PI), that is:
            //
            //   = angle of 2*PI
            //   = angle of PI    (because 2*PI >= PI)
            //   = angle of 0     (          PI >= PI)
            //                    (           0 < PI)
            //
            //  angle of 2*PI + PI/2
            //  = angle of 1*PI + PI/2  (because 2*PI + PI/2 >= PI)
            //  = angle of PI/2         (          PI + PI/2 >= PI)
            //                          (               PI/2 < PI)
            //
            //  angle of -2.5*PI
            //  = angle of -1.5*PI  (because -2.5*PI <= -PI)
            //  = angle of -0.5*PI  (        -1.5*PI <= -PI)
            //                      (        -0.5*PI > -PI)
            //
            // Intuitively:
            //
            // - when you rotate yourself twice around the axis, it's the
            //   same as if you rotated once, as if you've never rotated
            //   at all.
            //
            //   (your bony labyrinth might have a different opinion tho.)
            //
            // - when you rotate by 90° and then by 360°, it's the same
            //   as if you rotated only by 90° (*or* by 270°, just in the
            //   opposite direction).
            let angle = na::wrap(angle, -PI, PI);

            // If current angle is outside our birdie's field of view, jump
            // to the next food
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            // Makes angle *relative* to our birdie's field of view - that is:
            // transforms it from <-FOV_ANGLE/2,+FOV_ANGLE/2> to <0,FOV_ANGLE>.
            //
            // After this operation:
            // - an angle of 0° means "the beginning of the FOV",
            // - an angle of self.fov_angle means "the ending of the FOV".
            let angle = angle + self.fov_angle / 2.0;

            // Since this angle is now in range <0,FOV_ANGLE>, by dividing it by
            // FOV_ANGLE, we transform it to range <0,1>.
            //
            // The value we get can be treated as a percentage, that is:
            //
            // - 0.2 = the food is seen by the "20%-th" eye cell
            //         (practically: it's a bit to the left)
            //
            // - 0.5 = the food is seen by the "50%-th" eye cell
            //         (practically: it's in front of our birdie)
            //
            // - 0.8 = the food is seen by the "80%-th" eye cell
            //         (practically: it's a bit to the right)
            let cell = angle / self.fov_angle;

            // With cell in range <0,1>, by multiplying it by the number of
            // cells we get range <0,CELLS> - this corresponds to the actual
            // cell index inside our `cells` array.
            //
            // Say, we've got 8 eye cells:
            // - 0.2 * 8 = 20% * 8 = 1.6 ~= 1 = second cell (indexing from 0!)
            // - 0.5 * 8 = 50% * 8 = 4.0 ~= 4 = fifth cell
            // - 0.8 * 8 = 80% * 8 = 6.4 ~= 6 = seventh cell
            let cell = cell * (self.cells as f32);

            // Our `cell` is of type `f32` - before we're able to use it to
            // index an array, we have to convert it to `usize`.
            //
            // We're also doing `.min()` to cover an extreme edge case: for
            // cell=1.0 (which corresponds to a food being maximally to the
            // right side of our birdie), we'd get `cell` of `cells.len()`,
            // which is one element *beyond* what the `cells` array contains
            // (its range is <0, cells.len()-1>).
            //
            // Being honest, I've only caught this thanks to unit tests we'll
            // write in a moment, so if you consider my explanation
            // insufficient (pretty fair!), please feel free to drop the
            // `.min()` part later and see which tests fail - and why!
            let cell = (cell as usize).min(cells.len() - 1);

            // Energy is inversely proportional to the distance between our
            // birdie and the currently checked food; that is - an energy of:
            //
            // - 0.0001 = food is barely in the field of view (i.e. far away),
            // - 1.0000 = food is right in front of the bird.
            //
            // We could also model energy in reverse manner - "the higher the
            // energy, the further away the food" - but from what I've seen, it
            // makes the learning process a bit harder.
            //
            // As always, feel free to experiment! -- overall this isn't the
            // only way of implementing eyes.
            let energy = (self.fov_range - dist) / self.fov_range;

            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    struct TestCase {
        foods: Vec<Food>,
        range: f32,
        angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected: &'static str,
    }
    /// All our tests will use eyes hard-coded to thirteen eye cells.
    ///
    /// As for the "why":
    ///
    /// While we certainly *could* implement tests for different number of
    /// eye cells, after a while I've decided it's just not worth the
    /// hassle - as you'll see in a moment, we'll already get a good coverage
    /// via the other parameters, so creating a separate set of tests for
    /// different values of eye cells seemed like a waste of time.
    ///
    /// As for the "why this number in particular":
    ///
    /// I've checked a few numbers by hand and generally found 13 to yield
    /// pretty good results. As always, nothing special about 13 in
    /// particular, your (eye) mileage may vary.
    const TEST_EYE_CELLS: usize = 13;
    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.range, self.angle, TEST_EYE_CELLS);

            let f32_vision = eye.process_vision(
                Point::new(self.x, self.y),
                Rotation::new(self.rot),
                &self.foods,
            );
            let actual_vision: Vec<_> = f32_vision
                .into_iter()
                .map(|cell| {
                    // As a reminder, the higher cell's value, the closer
                    // the food is:

                    if cell >= 0.7 {
                        // <0.7, 1.0>
                        // food is right in front of us
                        "#"
                    } else if cell >= 0.3 {
                        // <0.3, 0.7)
                        // food is somewhat further
                        "+"
                    } else if cell > 0.0 {
                        // <0.0, 0.3)
                        // food is pretty far away
                        "."
                    } else {
                        // 0.0
                        // no food in sight, this cell sees empty space
                        " "
                    }
                })
                .collect();

            // As before, there's nothing special about the cell values
            // (`0.7`, `0.3`, `0.0`) or the characters (`#`, `+`, `.`).
            //
            // I've chosen hash because to my eye it seems to occupy the
            // most "visual space" out of all the ASCII characters (thus
            // it represents a food being close), and then plus and dot
            // are just smaller (representing food being further away).

            // `.join()` converts `Vec<String>` into `String` using a
            // separator - e.g. `vec!["a", "b", "c"].join("|")` would
            // return `a|b|c`.
            let actual_vision = actual_vision.join("");

            assert_eq!(actual_vision, self.expected);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    #[test_case(1.0, "      +      ")] // Food is inside the FOV
    #[test_case(0.9, "      +      ")] // ditto
    #[test_case(0.8, "      +      ")] // ditto
    #[test_case(0.7, "      .      ")] // Food slowly disappears
    #[test_case(0.6, "      .      ")] // ditto
    #[test_case(0.5, "             ")] // Food disappeared!
    #[test_case(0.4, "             ")]
    #[test_case(0.3, "             ")]
    #[test_case(0.2, "             ")]
    #[test_case(0.1, "             ")]
    fn ranges(range: f32, expected: &'static str) {
        TestCase {
            foods: vec![food(0.5, 1.0)],
            range,
            angle: FRAC_PI_2,
            x: 0.5,
            y: 0.5,
            rot: 0.0,
            expected,
        }
        .run()
    }

    #[test_case(0.00 * PI, "         +   ")] // Food is to our right
    #[test_case(0.25 * PI, "        +    ")]
    #[test_case(0.50 * PI, "      +      ")] // Food is in front of us
    #[test_case(0.75 * PI, "    +        ")]
    #[test_case(1.00 * PI, "   +         ")] // Food is to our left
    #[test_case(1.25 * PI, " +           ")]
    #[test_case(1.50 * PI, "            +")] // Food is behind us
    #[test_case(1.75 * PI, "           + ")] // (we continue to see it
    #[test_case(2.00 * PI, "         +   ")] // due to 360° fov_angle.)
    #[test_case(2.25 * PI, "        +    ")]
    #[test_case(2.50 * PI, "      +      ")]
    fn rotations(rot: f32, expected: &'static str) {
        TestCase {
            foods: vec![food(0.0, 0.5)],
            range: 1.0,
            angle: 2.0 * PI,
            x: 0.5,
            y: 0.5,
            rot,
            expected,
        }
        .run()
    }

    // Checking the X axis:
    // (you can see the bird is "flying away" from the foods)
    #[test_case(0.9, 0.5, "#           #")]
    #[test_case(0.8, 0.5, "  #       #  ")]
    #[test_case(0.7, 0.5, "   +     +   ")]
    #[test_case(0.6, 0.5, "    +   +    ")]
    #[test_case(0.5, 0.5, "    +   +    ")]
    #[test_case(0.4, 0.5, "     + +     ")]
    #[test_case(0.3, 0.5, "     . .     ")]
    #[test_case(0.2, 0.5, "     . .     ")]
    #[test_case(0.1, 0.5, "     . .     ")]
    #[test_case(0.0, 0.5, "             ")]
    //
    // Checking the Y axis:
    // (you can see the bird is "flying alongside" the foods)
    #[test_case(0.5, 0.0, "            +")]
    #[test_case(0.5, 0.1, "          + .")]
    #[test_case(0.5, 0.2, "         +  +")]
    #[test_case(0.5, 0.3, "        + +  ")]
    #[test_case(0.5, 0.4, "      +  +   ")]
    #[test_case(0.5, 0.6, "   +  +      ")]
    #[test_case(0.5, 0.7, "  + +        ")]
    #[test_case(0.5, 0.8, "+  +         ")]
    #[test_case(0.5, 0.9, ". +          ")]
    #[test_case(0.5, 1.0, "+            ")]
    fn positions(x: f32, y: f32, expected: &'static str) {
        TestCase {
            foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
            range: 1.0,
            angle: FRAC_PI_2,
            rot: 3.0 * FRAC_PI_2,
            x,
            y,
            expected,
        }
        .run()
    }

    #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
    #[test_case(0.50 * PI, ".  +     +  .")]
    #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
    #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
    #[test_case(1.25 * PI, "   . + + .   ")]
    #[test_case(1.50 * PI, ".   .+ +.   .")]
    #[test_case(1.75 * PI, ".   .+ +.   .")]
    #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is the widest = 8 foods
    fn fov_angles(angle: f32, expected: &'static str) {
        TestCase {
            foods: vec![
                food(0.0, 0.0),
                food(0.0, 0.33),
                food(0.0, 0.66),
                food(0.0, 1.0),
                food(1.0, 0.0),
                food(1.0, 0.33),
                food(1.0, 0.66),
                food(1.0, 1.0),
            ],
            range: 1.0,
            angle,
            x: 0.5,
            y: 0.5,
            rot: 3.0 * FRAC_PI_2,
            expected,
        }
        .run()
    }
}
