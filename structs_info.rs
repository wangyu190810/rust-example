

fn main(){
	// 表示坐标我们使用这样的方式
	let origin_x = 0;
	let origin_y = 0;

	println!("origin x is {},y is {}",origin_x,origin_y);
	struct Point{
		x:i32,
		y:i32
	};

	let origin = Point{x:12,y:13};
	println!("origin x is {},y is {}",origin.x,origin.y);

	let mut origin = Point{x:12,y:13};

	origin.x = 10;
	println!("origin x is {},y is {}",origin.x,origin.y);


	// 在struct中定义函数
	struct Circle {
    x: f64,
    y: f64,
    radius: f64,
	};

	impl Circle {
   	 	fn area(&self) -> f64 {
        	std::f64::consts::PI * (self.radius * self.radius)
    	}
	}

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());


}