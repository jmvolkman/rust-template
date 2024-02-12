#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64,f64),
}
impl Shape {

}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Triangle(1.0, 1.0), Shape::Square(3.0)];

    let total_area: f64 = shapes
        .iter()
        //.map(|shape| shape_area(shape))
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base,height ) => base * height * 0.5, 
        })
        .sum();

    println!("Total area: {} sq. units", total_area);

   // let new_shapes = vec![Shape::Circle(5.0), Shape::Triangle(1.0, 1.0), Shape::Square(3.0)];

    println!("Biggest Shape {:#?}", big_shape(&hapes));

    println!("Total area: {} sq. units", total_area);
    
}
fn big_shape(vector: &Vec<Shape>) -> Shape{
    let big_shape : Shape;
    let mut big_index = 0;
    let mut current_index = 0 ;
    let mut area :f64 = 0.0;
    for my_shape in vector.iter() {
       
        let new_area = shape_area(my_shape);
        if new_area > area {
            area = new_area;
            big_index = current_index;
           }
           current_index += 1;
       }
   Shape::Circle(5.0)
  //vector.iter_mut()[big_index]
}
    fn biggest_shape(vector: &Vec<Shape>) -> Shape{
         let big_shape : Shape;
         let mut big_index = 0;
         let mut current_index = 0 ;
         let mut area :f64 = 0.0;
         for my_shape in vector.iter() {
            
             let new_area = shape_area(my_shape);
             if new_area > area {
                 area = new_area;
                 big_index = current_index;
                }
                current_index += 1;
            }
        Shape::Circle(5.0)
       //vector.iter_mut()[big_index]
    }
fn shape_area(item: &Shape) -> f64 {
        match item {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length, 
            Shape::Triangle(base,height ) => base * height * 0.5, 
        }
    }