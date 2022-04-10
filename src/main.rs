use  enum_generic_demo::traffic_light::*;
use  enum_generic_demo::safe_add_u32::add_u32_list;
use  enum_generic_demo::area_calc::*;

fn main() {
    
    let traffic_light = TrafficLightKind::Red;

    println!("traffic_light is : {} with {:?}", traffic_light.desc(), traffic_light.time_duration());
    
    let int_list_invalid : [u32;4] = [1,2,3, u32::MAX - 2];
    println!("it should be over flow and return {:?} ",add_u32_list(&int_list_invalid));
    let int_list : [u32;4] = [1,2,3, 100008];
    println!("it should be valid and return {:?} ",add_u32_list(&int_list));

    let triagle  = enum_generic_demo::area_calc::Triangle {
        a : 3.0,
        b : 4.0,
        c : 5.0
    };

    let square  = enum_generic_demo::area_calc::Square {
        x : 3.0,
        y : 4.0,
        w : 5.0
    };

    let cicle = enum_generic_demo::area_calc::Circle {
        center_x : 3.0,
        center_y : 3.0,
        radius : 8.0
    };
    
    print_area(cicle);
    print_area(triagle);
    print_area(square);

    
}
