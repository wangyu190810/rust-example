fn main(){

	let x = 5;
	let y = 6;

	// 变量定义，rust的变量定义可以智能的判断，同样也可以显示的定义

	//let mut z = 123; 在使用赋值语句时，定义为mut，编译报错，不知道是个人问题还是bug。

	let mut c = 123;
	println!("c is {}",c);
	c = y;
	println!("c is {}",c);
	// 定义变量加上mut这个变量可以被重新赋值，如果不定义为mut，则不能重新赋值.




	let  z = if x == 5 {

			x*2
		}
		else if x != 5{

			x * 3
		}
		else{

			x*x
		};
	let zzz = if x == 5 { x * 2} else if x != 5 { x * 6} else { x * 2};
	println!("z is {}",z);
	println!("zzz is {}",zzz);
	// 判断语句,给z赋值，


	let mut zz;
	if x == 5 {
		println!("x is {}",x);
		zz = x * x;
	}
	else if x != 5{
		println!("x is {}",x);
		zz = x *2;
	}
	else{
		println!("x is {}",x);
		zz = x * 3;
	};
	//同样是判断语句,但是这个这里的赋值语句要这样写
	println!("zz is {}",zz);


	//变量被重新定义时，后者会覆盖前者的所有属性
	let (x,y) = (1,2);
	//() 表示tuple，在python中有定义。
	println!("x is {},y is {}",x,y);
}
