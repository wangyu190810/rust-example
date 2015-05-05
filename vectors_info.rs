fn main(){
	// 这个东西不知道怎么用
	let v = vec![1,2,3];
	//let m = v.pop();
	println!("{}",v.len());
	println!("{}",v[2]);

	//如果不使用mut定义，v是一个不可变的，所以
	//v.push(3);
	// 会报错，需要注释掉
	for x in v.iter(){
		println!("{}",x);
	}

	//可变定义
	let mut vv = vec![];
	vv.push(123);
	vv.push(1);
	vv.push(11);

	for x in vv.iter(){
		println!("{}",x);
	}

}