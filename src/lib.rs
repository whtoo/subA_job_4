pub mod traffic_light {
    pub enum TrafficLightKind {
        Red,
        Yellow,
        Green
    }
    
    impl TrafficLightKind {
        pub fn time_duration(&self) -> Option<u32> {
            match *self {
                TrafficLightKind::Red => Some(60),
                TrafficLightKind::Yellow => Some(5),
                TrafficLightKind::Green => Some(25)
            }
        }
        pub fn desc(&self) -> &str {
            match *self {
                TrafficLightKind::Red => "Red",
                TrafficLightKind::Yellow => "Yellow",
                TrafficLightKind::Green => "Green"
            }
        }
    }
}

pub mod safe_add_u32 {
    pub fn add_u32_list(int_list : &[u32] ) -> Option<u32> {
        let mut result : Option<u32> = Some(0);
        for &i in int_list {
            match result {
                Some(x) => result = x.checked_add(i),
                None => return None
            }
        }

        return result
    }
}

use std::num;

pub mod area_calc {
    pub struct Square {
        pub x : f64, // offsetX
        pub y : f64, // offsetY
        pub w : f64 // width and height
    }

    pub struct Triangle {
        pub a : f64,
        pub b : f64,
        pub c : f64
    }

    pub struct Circle {
        pub center_x : f64,
        pub center_y : f64,
        pub radius : f64
    }

    pub trait PPDesc {
        fn desc<'a>(&self) -> &'a str;
    }

    impl PPDesc for Circle {
        fn desc<'a>(&self) ->  &'a str {
            return "Circle"
        }
    }

    impl PPDesc for Square {
        fn desc<'a>(&self) ->  &'a str {
            return "Square"
        }
    }

    impl PPDesc for Triangle {
       fn desc<'a>(&self) -> &'a str {
           return "Triangle"
       }
    }

    pub trait HasArea {
        fn calc_area(&self) -> f64;
    }

    impl HasArea for Square {
        fn calc_area(&self) -> f64 {
            self.w * self.w 
        }
    }

    impl HasArea for Circle {
        fn calc_area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius 
        }
    }

    impl HasArea for Triangle {
        fn calc_area(&self) -> f64 {
            let s : f64 = (self.a + self.b + self.c) / 2.0;
            (s*(s-self.a)*(s-self.b)*(s-self.c)).sqrt()
        }
    }

    pub fn print_area<T:HasArea + PPDesc> (shape:T) {
        println!("{} area is {}",shape.desc(),shape.calc_area())
    }
}