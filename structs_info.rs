fn main(){
	// 表示坐标我们使用这样的方式
	let origin_x = 0;
	let origin_y = 0;

	struct Point{
		x:i32,
		y:i32
	};

	let origin = Point{x:12,y:13};
	println!("origin x is {},y is {}",origin.x,origin.y);


}