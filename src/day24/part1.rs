#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    _z: f64,
    vx: f64,
    vy: f64,
    _vz: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Hailstone {
    fn new(x: f64, y: f64, _z: f64, vx: f64, vy: f64, _vz: f64) -> Self {
        Hailstone {
            x,
            y,
            _z,
            vx,
            vy,
            _vz,
            a: vy,
            b: -1f64 * vx,
            c: vy * x - vx * y,
        }
    }

    fn intersects(&self, other: &Hailstone) -> Option<(f64, f64)> {
        if self.a * other.b == self.b * other.a {
            return None;
        }

        let x = (other.b * self.c - self.b * other.c) / (self.a * other.b - self.b * other.a);
        let y = (self.a * other.c - other.a * self.c) / (self.a * other.b - other.a * self.b);

        match (x - self.x < 0.0) == (self.vx < 0.0)
            && (y - self.y < 0.0) == (self.vy < 0.0)
            && (x - other.x < 0.0) == (other.vx < 0.0)
            && (y - other.y < 0.0) == (other.vy < 0.0)
        {
            true => Some((x, y)),
            false => None,
        }
    }
}

pub fn solve(input: &str) -> usize {
    solve_in_bounds(input, 200000000000000, 400000000000000)
}

pub fn solve_in_bounds(input: &str, min: usize, max: usize) -> usize {
    let hailstones: Vec<Hailstone> = input
        .lines()
        .map(|line| {
            let elems: Vec<f64> = line
                .replace('@', ",")
                .split(',')
                .map(|elem| {
                    elem.trim()
                        .parse::<f64>()
                        .expect("Input data should contain valid numbers.")
                })
                .collect();

            Hailstone::new(elems[0], elems[1], elems[2], elems[3], elems[4], elems[5])
        })
        .collect();

    let mut count = 0;

    for (i, hailstone1) in hailstones.iter().enumerate() {
        for hailstone2 in hailstones.iter().skip(i + 1) {
            if let Some(intersection) = hailstone1.intersects(hailstone2) {
                if intersection.0 >= min as f64
                    && intersection.0 <= max as f64
                    && intersection.1 >= min as f64
                    && intersection.1 <= max as f64
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve_in_bounds(input, 7, 27), 2);
    }
}
