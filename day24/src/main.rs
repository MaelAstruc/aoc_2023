fn main() {
    part_1(
        "19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3",
        7.0,
        17.0
    );

    part_1(&std::fs::read_to_string("input.txt").unwrap(), 200000000000000.0, 400000000000000.0);
}

fn part_1(input: &str, min: f32, max: f32) -> usize {
    let stones = parse_input(input);

    let mut equations: Vec<Equation2D> = Vec::with_capacity(stones.len());
    
    for stone in stones {
        equations.push(Equation2D::from(stone))
    }

    let mut count: usize = 0;

    for (i, equation) in equations.iter().enumerate() {
        for other in &equations[(i+1)..] {
            if let Some((x, y)) = equation.cross(other) {
                if is_inside(x, y, min, max) & equation.is_after(x) & other.is_after(x) {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
    count
}

fn part_1(input: &str, min: f32, max: f32) -> usize {
    let stones = parse_input(input);

    let stone_1 = stones[0];
    let stone_2 = stones[0];
    let stone_3 = stones[0];



    0
}

fn is_inside(x: f32, y: f32, min: f32, max: f32) -> bool {
    (x > min) & (x < max) & (y > min) & (y < max)
}
#[derive(Debug)]
struct Stone {
    x: f32,
    y: f32,
    z: f32,
    dx: f32,
    dy: f32,
    dz: f32,
}

impl Stone {
    fn new(string: &str) -> Self {
        let (positions, velocities) = string.split_once('@').unwrap();
        let pos: Vec<f32> = positions
            .split(",")
            .map(|position| position.trim().parse::<f32>().unwrap())
            .collect();
        let vel: Vec<f32> = velocities
            .split(",")
            .map(|position| position.trim().parse::<f32>().unwrap())
            .collect();

        Self {
            x: pos[0],
            y: pos[1],
            z: pos[2],
            dx: vel[0],
            dy: vel[1],
            dz: vel[2],
        }
    }
}

#[derive(Debug)]
struct Equation2D {
    a: f32,
    b: f32,
    x0: f32,
    dx: f32,
}

impl From<Stone> for Equation2D {
    fn from(stone: Stone) -> Self {
        Self {
            a: stone.y - stone.x * stone.dy / stone.dx,
            b: stone.dy / stone.dx,
            x0: stone.x,
            dx: stone.dx,
        }
    }
}

impl Equation2D {
    fn cross(&self, other: &Equation2D) -> Option<(f32, f32)> {
        if self.b == other.b {
            return None;
        }

        let x = - (self.a - other.a) / (self.b - other.b);
        let y = self.a + self.b * x;

        Some((x, y))
    }

    fn is_after(&self, x: f32) -> bool {
        if self.dx == 0.0 {
            self.x0 == x
        } else if self.dx < 0.0 {
            self.x0 >= x
        } else {
            self.x0 <= x
        }
    }
}

#[derive(Debug)]
struct Equation3D {
    ay: f32,
    by: f32,
    az: f32,
    bz: f32,
    x0: f32,
    dx: f32,
}

impl From<Stone> for Equation3D {
    fn from(stone: Stone) -> Self {
        Self {
            ay: stone.y - stone.x * stone.dy / stone.dx,
            by: stone.dy / stone.dx,
            az: stone.z - stone.x * stone.dz / stone.dx,
            bz: stone.dz / stone.dx,
            x0: stone.x,
            dx: stone.dx,
        }
    }
}

/*
At time t

X(t) = ax + bx * t
Y(t) = ay + by * t
Z(t) = az + bz * t

Two stones 1 and 2 intersect collide at time t1:

X1 = X2
ax1 + bx1 * t1 = ax2 + bx2 * t1
t1 = - (ax1 - ax2) / (bx1 - bx2)

Y1 = Y2
t1 = - (ay1 - ay2) / (by1 - by2)

Z1 = Z2
t1 = - (az1 - az2) / (bz1 - bz2)

We can deduce:

X1(t1) = X2(t1) & Y1(t1) = Y2(t1)
=> (ax1 - ax2) / (bx1 - bx2) = - (ay1 - ay2) / (by1 - by2)
ax1 = ax2 + (ay1 - ay2) * (bx1 - bx2) / (by1 - by2)

X1(t1) = X2(t1) & Z1(t1) = Z2(t1)
=> (ax1 - ax2) / (bx1 - bx2) = - (az1 - az2) / (bz1 - bz2)
ax1 = ax2 + (az1 - az2) * (bx1 - bx2) / (bz1 - bz2)

Hence:

X1(t1) = X2(t1) & Y1(t1) = Y2(t1) & Z1(t1) = Z2(t1)
=> ax2 + (ay1 - ay2) * (bx1 - bx2) / (by1 - by2) = ax2 + (az1 - az2) * (bx1 - bx2) / (bz1 - bz2)
(ay1 - ay2) / (by1 - by2) = (az1 - az2) / (bz1 - bz2)
ay1 = ay2 + (az1 - az2) * (by1 - by2) / (bz1 - bz2)

We define ratios
DxUV = (axU - axV) / (bxU - bxV)
DyUV = (ayU - ayV) / (byU - byV)
DzUV = (azU - azV) / (bzU - bzV)

We know that
t = - DxUV = - DyUV = - DzUV

We use
DUV as a replacement and change it when needed

With a third stone 3 colliding with stone 1 at time t2

From
- X1(t1) = X2(t1) & Z1(t1) = Z2(t1)
- X1(t2) = X3(t2) & Z1(t2) = Z3(t2)

=> ax1 = ax2 + (bx1 - bx2) * D12
=> ax1 = ax3 + (bx1 - bx3) * D13

ax2 + (bx1 - bx2) * D12 = ax3 + (bx1 - bx3) * D13
ax2 - ax3 + bx1 * D12 - bx2 * D12 = bx1 * D13 - bx3 * D13
ax2 - ax3 - bx2 * D12 + bx3 * D13 = bx1 * [D13 - D12]
ax2 - ax3 - bx2 * D12 + bx3 * D12 - bx3 * D12 + bx3 * D13 = bx1 * [D13 - D12]
ax2 - ax3 - (bx2 - bx3) * D12 + bx3 * [D13 - D12] = bx1 * [D13 - D12]
(bx2 - bx3) * [ax2 - ax3 / (bx2 - bx3) - D12] + bx3 * [D13 - D12] = bx1 * [D13 - D12]
(bx2 - bx3) * [D23 - D12] + bx3 * [D13 - D12] = bx1 * [D13 - D12]
bx1 = bx3 + (bx2 - bx3) * [D23 - D12] / [D13 - D12]

From
- X1(t1) = X2(t1) & Y1(t1) = Y2(t1) & Z1(t1) = Z2(t1)
- X1(t2) = X3(t2) & Y1(t2) = Y3(t2) & Z1(t2) = Z3(t2)

=> ay1 = ay2 + (by1 - by2) * D12
=> ay1 = ay3 + (by1 - by3) * D13

by1 = by3 + (by2 - by3) * [D23 - D12] / [D13 - D12]

To sum up:
- ax1 = ax2 + (bx1 - bx2) * D12
    -> depends on bx1, az1, bz1
- ay1 = ay2 + (by1 - by2) * D12
    -> depends on by1, az1, bz1
- bx1 = bx3 + (bx2 - bx3) * [D23 - D12] / [D13 - D12]
    -> depends on az1, bz1
- by1 = by3 + (by2 - by3) * [D23 - D12] / [D13 - D12]
    -> depends on az1, bz1

We still need to define az1 and bz1 independently

With a fourth stone colliding with stone 1 at time t3

az1 = az2 + (bz1 - bz2) * D12
az1 = az4 + (bz1 - bz4) * D14

az2 + (bz1 - bz2) * D12 = az4 + (bz1 - bz4) * D14
az2 - az4 - bz2 * D12 + bz4 * D14 =  bz1 (D14 - D12)
bz1 = [az2 - az4 - bz2 * D12 + bz4 * D14] / (D14 - D12)
bz1 = [az2 - az4 - bz2 * D12 + bz4 * D12 - bz4 * D12 + bz4 * D14] / (D14 - D12)
bz1 = bz4 + [az2 - az4 - (bz2 - bz4) * D12] / (D14 - D12)
bz1 = bz4 + (bz2 - bz4) (D24 - D12) / (D14 - D12)
bz1 (D14 - D12) = bz4 (D14 - D12) + (bz2 - bz4) (D24 - D12)
bz1 * D14 - bz1 * D12 = bz4 * D14 - bz4 * D12 + (bz2 + bz4) * D24 - (bz2 - bz4) * D12
bz1 * D14 - bz1 * D12 + (bz2 - bz4) * D12 - bz4 * D14 + bz4 * D12 = (bz2 + bz4) * D24
bz1 * D14 - bz1 * D12 + bz2 * D12 - bz4 * D14 = (bz2 + bz4) * D24

D12 = (az1 - az2) / (bz1 - bz2)
D14 = (az1 - az4) / (bz1 - bz4)

bz1 * [(az1 - az3) / (bz1 - bz3)]
    - bz1 * [(az1 - az2) / (bz1 - bz2)]
    + bz2 * [(az1 - az2) / (bz1 - bz2)]
    - bz4 * [(az1 - az4) / (bz1 - bz4)]
= (bz2 + bz4) * D24

bz1 * (az1 - az3) * (bz1 - bz2) * (bz1 - bz4)
    - bz1 * (az1 - az2) * (bz1 - bz3) * (bz1 - bz4)
    + bz2 * (az1 - az2) * (bz1 - bz3) * (bz1 - bz4)
    - bz4 * (az1 - az4) * (bz1 - bz3) * (bz1 - bz2)
= (az2 + az4) * (bz1 - bz3) * (bz1 - bz2) * (bz1 - bz4)

bz1 * az1 * bz1 * bz1
    - bz1 * az1 * bz1 * bz4
    - bz1 * az1 * bz2 * bz1
    + bz1 * az1 * bz2 * bz4
    - bz1 * az3 * bz1 * bz1
    + bz1 * az3 * bz1 * bz4
    + bz1 * az3 * bz2 * bz1
    - bz1 * az3 * bz2 * bz4
    - bz1 * az1 * bz1 * bz1
    + bz1 * az1 * bz1 * bz4
    + bz1 * az1 * bz3 * bz1
    - bz1 * az1 * bz3 * bz4
    + bz1 * az2 * bz1 * bz1
    - bz1 * az2 * bz1 * bz4
    - bz1 * az2 * bz3 * bz1
    + bz1 * az2 * bz3 * bz4
    + bz2 * az1 * bz1 * bz1
    - bz2 * az1 * bz1 * bz4
    - bz2 * az1 * bz3 * bz1
    + bz2 * az1 * bz3 * bz4
    - bz2 * az2 * bz1 * bz1
    + bz2 * az2 * bz1 * bz4
    + bz2 * az2 * bz3 * bz1
    - bz2 * az2 * bz3 * bz4
    - bz4 * (az1 - az4) * (bz1 - bz3) * (bz1 - bz2)
= (az2 + az4) * (bz1 - bz3) * (bz1 - bz2) * (bz1 - bz4)




bz1 * [(az1 - az3) * (bz1 - bz2) - (az1 - az2) * (bz1 - bz3)] / [(bz1 - bz3) * (bz1 - bz2)]
    + bz2 * [(az1 - az2) / (bz1 - bz2)]
    - bz4 * [(az1 - az4) / (bz1 - bz4)]
= (bz2 + bz4) * D24

bz1 * [(bz1 - bz2) * (az2 - az3) - (az1 - az2) (bz2 - bz3)] / [(bz1 - bz3) * (bz1 - bz2)]
    + bz2 * [(az1 - az2) / (bz1 - bz2)]
    - bz4 * [(az1 - az4) / (bz1 - bz4)]
= (bz2 + bz4) * D24

bz1 * (bz1 - bz2) * (bz2 - bz3) [ D23 - D12 ] / [(bz1 - bz3) * (bz1 - bz2)]
    + bz2 * [(az1 - az2) / (bz1 - bz2)]
    - bz4 * [(az1 - az4) / (bz1 - bz4)]
= (bz2 + bz4) * D24

bz1 * (bz2 - bz3) [ D23 - D12 ] / (bz1 - bz3)
    + bz2 * [(az1 - az2) / (bz1 - bz2)]
    - bz4 * [(az1 - az4) / (bz1 - bz4)]
= (bz2 + bz4) * D24

bz1 * (bz2 - bz3) [ D23 - D12 ] / (bz1 - bz3)
    + [bz2 * (az1 - az2) * (bz1 - bz4) - bz4 * (az1 - az4) * (bz1 - bz2)] / [(bz1 - bz2) * (bz1 - bz4)]
= (bz2 + bz4) * D24

bz1 * (bz2 - bz3) [ D23 - D12 ] / (bz1 - bz3)
    + (bz2 - bz4) [az1 bz1 - bz1 (az2 bz2 - az4 bz4) / (bz2 - bz4) + bz2 bz4 D24]
    / [(bz1 - bz2) * (bz1 - bz4)]
= (bz2 + bz4) * D24

bz1 * (bz2 - bz3) [ D23 - D12 ] / (bz1 - bz3)
    + (bz2 - bz4) [az1 bz1 - bz1 (az2 (bz2 - bz4) - (az2 - az4) (bz2 - bz4) + (az2 - az4 ) bz2 ) / (bz2 - bz4) + bz2 bz4 D24]
    / [(bz1 - bz2) * (bz1 - bz4)]
= (bz2 + bz4) * D24

bz1 * (bz2 - bz3) [ D23 - D12 ] / (bz1 - bz3)
    + (bz2 - bz4) [az1 bz1 - bz1 az2 + bz1 (az2 - az4) - bz1 bz2 DF24 + bz2 bz4 D24]
    / [(bz1 - bz2) * (bz1 - bz4)]
= (bz2 + bz4) * D24

bz1 * (bz2 - bz3) [ D23 - D12 ] / (bz1 - bz3)
    + (bz2 - bz4) [az1 bz1 - bz1 (az4 - bz2 DF24) + bz2 bz4 D24]
    / [(bz1 - bz2) * (bz1 - bz4)]
= (bz2 + bz4) * D24



bz1 * (bz2 - bz3) [ D23 - D12 ] * (bz1 - bz2) * (bz1 - bz4)
    + (bz2 - bz4) [az1 bz1 - bz1 (az4 - bz2 DF24) + bz2 bz4 D24] * (bz1 - bz3)
= (bz2 + bz4) * D24 * (bz1 - bz3) * (bz1 - bz2) * (bz1 - bz4)

bz1 * (bz1 - bz2) * (bz1 - bz4) * (az2 - az3)
    - bz1 * (az1 - az2) * (bz1 - bz4) * (bz2 - bz3)
    +  az1 bz1 * (bz1 - bz3) * (bz2 - bz4)
    - bz1 * (bz1 - bz3) * (bz2 - bz4) * (az4 - bz2 DF24)
    +  bz2 bz4 * (bz1 - bz3) * (az2 - az4)
= (bz1 - bz3) * (bz1 - bz2) * (bz1 - bz4) * (az2 + az4)

bz1 * bz1 * bz1 * (az2 - az3)
    - bz1 * bz1 * bz4 * (az2 - az3)
    - bz1 * bz2 * bz1 * (az2 - az3)
    + bz1 * bz2 * bz4 * (az2 - az3)
    - bz1 * az1 * bz1 * (bz2 - bz3)
    + bz1 * az1 * bz4 * (bz2 - bz3)
    + bz1 * az2 * bz1 * (bz2 - bz3)
    - bz1 * az2 * bz4 * (bz2 - bz3)
    +  az1 bz1 * bz1 * (bz2 - bz4)
    -  az1 bz1 * bz3 * (bz2 - bz4)
    - bz1 * bz1 * (bz2 - bz4) * (az4 - bz2 DF24)
    + bz1 * bz3 * (bz2 - bz4) * (az4 - bz2 DF24)
    +  bz2 bz4 * bz1 * (az2 - az4)
    -  bz2 bz4 * bz3 * (az2 - az4)
= bz1 * bz1 * bz1 * (az2 + az4)
    - bz1 * bz1 * bz4 * (az2 + az4)
    - bz1 * bz2 * bz1 * (az2 + az4)
    + bz1 * bz2 * bz4 * (az2 + az4)
    - bz3 * bz1 * bz1 * (az2 + az4)
    + bz3 * bz1 * bz4 * (az2 + az4)
    + bz3 * bz2 * bz1 * (az2 + az4)
    - bz3 * bz2 * bz4 * (az2 + az4)

    bz1 * [
        az1 * bz4 * (bz2 - bz3)
        - az1 * bz3 * (bz2 - bz4)
        - az2 * bz4 * (bz2 - bz3)
        + bz2 * bz4 * (az2 - az4)
        + bz3 * (bz2 - bz4) * (az4 - bz2 DF24)
        - bz2 * bz4 * (az3 + az4)
        - bz3 * bz4 * (az2 + az4)
        - bz3 * bz2 * (az2 + az4)
    ]
    + bz1 * bz1 * [
        az2 * (bz2 - bz3)
        + (bz2 + bz4) * (az3 + az4)
        + bz3 * (az2 + az4)
        - az1 * (bz3 + bz4)
        - (bz2 - bz4) * (az4 - bz2 DF24)
    ]
    - bz1 * bz1 * bz1 * (az3 + az4)
= 
    - bz3 * bz2 * bz4 * (az2 + az4)
    - bz2 * bz4 * bz3 * (az2 - az4)





D14 = (ax1 - ax4) / (bx1 - bx4)
D14 = ([ax2 + (bx1 - bx2) * D12] - ax4) / (bx1 - bx4)
D14 = (ax2 - ax4  + ([bx3 + (bx2 - bx3) * [D23 - D12] / [D13 - D12]] - bx2) * D12)
        / ([bx3 + (bx2 - bx3) * [D23 - D12] / [D13 - D12]] - bx4)
D14 = (ax2 - ax4 + ((bx2 - bx3) * [D23 - D12] / [D13 - D12] - (bx2 - bx3)) * D12)
        / ((bx2 - bx3) * [D23 - D12] / [D13 - D12] + bx3 - bx4)
D14 = (
        ax2 - ax4
        + (bx2 - bx3) * [D23 - D13]  / [D13 - D12] * D12
        )
        / (bx2 - bx4 + (bx2 - bx3) * [D23 - D13] / [D13 - D12])
D14 = (ax2 - ax4
        - D12 * (bx2 - bx4)
        + D12 * [bx2 - bx4 + (bx2 - bx3) * [D23 - D13] / [D13 - D12]]
        )
        / (bx2 - bx4 + (bx2 - bx3) * [D23 - D13] / [D13 - D12])
D14 = D12
        + (ax2 - ax4 - D12 * (bx2 - bx4))
        / (bx2 - bx4 + (bx2 - bx3) * [D23 - D13] / [D13 - D12])
D14 = D12
        + (Dx24 - D12)
        / (1 + (bx2 - bx3) / (bx2 - bx4) * [D23 - D13] / [D13 - D12])
D14 = D12 + (Dx24 - D12) / (1 + (bx2 - bx3) / (bx2 - bx4) * [D23 - D13] / [D13 - D12])


az1 = az4 + (bz1 - bz4) * D14

D14 = D14
D14 = [D12 + (Dx24 - D12) / (1 + (bx2 - bx3) / (bx2 - bx4) * [D23 - D13] / [D13 - D12])]
[D14 - D12] * [1 + (bx2 - bx3) / (bx2 - bx4) * [D23 - D13] / [D13 - D12]] = (Dx24 - D12)
[D14 - D12] + (bx2 - bx3) / (bx2 - bx4) * [D14 - D12] * [D23 - D13] / [D13 - D12] = [Dx24 - D12]
(bx2 - bx3) / (bx2 - bx4) * [D14 - D12] * [D23 - D13] / [D13 - D12] = Dx24 - D14
(bx2 - bx3) / (bx2 - bx4) * [D14 - D12] * [D23 - D13] = [Dx24 - D14] * [D13 - D12]

D14 - D12 = (az1 - az4) / (bz1 - bz4) - (az1 - az2) / (bz1 - bz2)
D14 - D12 = [(az1 - az4) * (bz1 - bz2) - (az1 - az2) * (bz1 - bz4)] / [(bz1 - bz4) * (bz1 - bz2)]
D14 - D12 = [(bz1 - bz4) * (az2 - az4) - (az1 - bz4) * (bz2 - bz4)] / [(bz1 - bz4) * (bz1 - bz2)]

(bx2 - bx3)
    / (bx2 - bx4)
    * [(az1 - az4) * (bz1 - bz2) - (az1 - az2) * (bz1 - bz4)] / [(bz1 - bz4) * (bz1 - bz2)]
    * [(az2 - az3) * (bz1 - bz3) - (az1 - az3) * (bz2 - bz3)] / [(bz2 - bz3) * (bz1 - bz3)]
= 
    [(az2 - az4) * (bz1 - bz4) - (az1 - az4) * (bz2 - bz4)] / [(bz2 - bz4) * (bz1 - bz4)]
    * [(az1 - az3) * (bz1 - bz2) - (az1 - az2) * (bz1 - bz3)] / [(bz1 - bz3) * (bz1 - bz2)]

(bx2 - bx3)
    / (bx2 - bx4)
    * [(az1 - az4) * (bz1 - bz2) - (az1 - az2) * (bz1 - bz4)]
    * [(az2 - az3) * (bz1 - bz3) - (az1 - az3) * (bz2 - bz3)] / (bz2 - bz3)
= 
    [(az2 - az4) * (bz1 - bz4) - (az1 - az4) * (bz2 - bz4)] / (bz2 - bz4)
    * [(az1 - az3) * (bz1 - bz2) - (az1 - az2) * (bz1 - bz3)]

(bx2 - bx3)
    / (bx2 - bx4)
    * [(bz1 - bz4) * (az2 - az4) - (az1 - bz4) * (bz2 - bz4)]
    * [(bz1 - bz3) * (az2 - az3) - (az1 - az3) * (bz2 - bz3)]
    * (bz2 - bz4)
= 
    [(bz1 - bz4) * (az2 - az4) - (az1 - bz4) * (bz2 - bz4)]
    * [(bz1 - bz3) * (az2 - az3) - (az1 - az3) * (bz2 - bz3)]
    * (bz2 - bz3)

(bx2 - bx3) / (bx2 - bx4) = (bz2 - bz3)
1 + (bx4 - bx3) / (bx2 - bx4) = (bz2 - bz3)

*/

impl Equation3D {
    fn cross(&self, other: &Equation3D) -> Option<(f32, f32, f32)> {
        if (self.by == other.by) & (self.bz == other.bz) {
            return None;
        }

        let x_y = - (self.ay - other.ay) / (self.by - other.by);
        
        let x_z = - (self.az - other.az) / (self.bz - other.bz);

        if x_y != x_z {
            return None;
        }

        let y = self.ay + self.by * x_y;
        let z = self.az + self.bz * x_z;

        Some((x_y, y, z))
    }

    fn is_after(&self, x: f32) -> bool {
        if self.dx == 0.0 {
            self.x0 == x
        } else if self.dx < 0.0 {
            self.x0 >= x
        } else {
            self.x0 <= x
        }
    }
}

fn parse_input(input: &str) -> Vec<Stone> {
    let mut stones: Vec<Stone> = Vec::new();
    
    for line in input.lines() {
        stones.push(Stone::new(line));
    }

    stones
}
