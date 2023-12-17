use rand::Rng;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    fn generate_points(&self, n: i32) -> Vec<Point> {
        let mut rng = rand::thread_rng();

        let mut vec: Vec<Point> = Vec::new();
        for _ in 0..n {
            let mut random_number: f64 = rng.gen_range(-99.0..99.0);

          random_number=  random_number.round()/0.5;
            let sec_random_number = random_number + rng.gen_range(-99.0..99.0);

            println!("Value {}", random_number);

            let p = Point::new(
                self.x + random_number as f64,
                self.y + sec_random_number as f64,
            );
            vec.push(p);
        }
        vec
    }

    fn find_equation(points: &Vec<Point>) {
        let first_point = points.first();
        let last_point = points.last();

        let x_1 = first_point.unwrap().x;
        let y_1 = first_point.unwrap().y;

        let x_2 = last_point.unwrap().x;
        let y_2 = last_point.unwrap().y;

        println!("X {}", x_1);
        println!("Y {}", y_1);

        println!("{:?}", last_point);

        //y = mx +c;

        // Find Gradient
        //Change in Y/ Change In X
        let m = (y_2 - y_1) / (x_2 - x_1);


        //find C =


        println!("Gradient {}", m);
        // y = mx + c;
        // -c = mx -y => c = -mx +y

        let c =-1.0 *m *x_1 +y_1;

        println!("Y Intercept {}", c);

        //Therefore the equation
        println!("y={}x+ {}", m, c);



    }
}

fn main() {
    let p = Point::generate_points(&Point::new(1.0, 4.0), 8);

    Point::find_equation(&p);

    println!("{:?}", p);
}
