use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
};

#[derive(Debug)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Hailstone {
    fn new(x: i64, y: i64, z: i64, vx: i64, vy: i64, vz: i64) -> Self {
        Hailstone {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        }
    }
}

pub fn solve(input: &str) -> usize {
    let hailstones: Vec<Hailstone> = input
        .lines()
        .map(|line| {
            let elems: Vec<i64> = line
                .replace('@', ",")
                .split(',')
                .map(|elem| {
                    elem.trim()
                        .parse::<i64>()
                        .expect("Input data should contain valid numbers.")
                })
                .collect();

            Hailstone::new(elems[0], elems[1], elems[2], elems[3], elems[4], elems[5])
        })
        .collect();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in hailstones {
        let xn = Int::from_i64(&ctx, hailstone.x);
        let yn = Int::from_i64(&ctx, hailstone.y);
        let zn = Int::from_i64(&ctx, hailstone.z);
        let vxn = Int::from_i64(&ctx, hailstone.vx);
        let vyn = Int::from_i64(&ctx, hailstone.vy);
        let vzn = Int::from_i64(&ctx, hailstone.vz);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&xn + &vxn * &tn)._eq(&(&x + &vx * &tn)));
        solver.assert(&(&yn + &vyn * &tn)._eq(&(&y + &vy * &tn)));
        solver.assert(&(&zn + &vzn * &tn)._eq(&(&z + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&x).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&y).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&z).unwrap().as_i64().unwrap();

    (x + y + z) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_riddle() {
        let input = include_str!("test_input1.txt");
        assert_eq!(solve(input), 47);
    }
}
