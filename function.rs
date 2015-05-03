fn main(){

	print_number(100);
	print_sum(12,14);
	print_number(add_num(100));
	print_number(foo(100));
}


fn print_number(x:i32){
	//函数参数调用
	println!("number is :{}",x);

}

fn print_sum(x:i32,y:i32){
	//多参数调用
	println!("x and y sum is :{}",x+y);

}

fn add_num(x:i32) -> i32{
	//函数仅仅有返回值，可以理解为python中的lambda
	x + 3
}


fn foo(x:i32) -> i32{
	//函数返回值，在指定条件下返回
	println!("x + 100 ={}",x+100);
	if x == 100 {
		return x+100;
	}
	else {
		return x * x;
	}
}