//ENUM pattern matching

enum Shape{
    Circle(f64),
    Square(f64),
    Reactagle(f64,f64),
}
s
fn calculate_area(shape: Shape)-> f64{
    match shape{
        Shape::Circle(radius)=> std::f64::consts::PI * radius * radius,
        Shape::Reactagle(width,height )=> width *height,
        Shape::Square(side_length)=> side_length * side_length, 
    }
}

fn main(){
    let circle = Shape::Circle(9.0);
    let rectangle = Shape::Reactagle(9.0,3.0 );
    let square = Shape::Square(9.0);

    print!("Area of circle: {}", calculate_area(circle));
}