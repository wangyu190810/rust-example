fn main(){


	let  x = 5;
	//这里简单的运用，相当于switch，
	match x {
		1 => println!("one"),
		2 => println!("tow"),
		3 => println!("three"),
		4 => println!("four"),
		5 => println!("five"),
		_ => println!("something else"),
	}

	let y = 5;
	//同样可以用来辅助,if else 中也有
	let numer = match y {
    	1 => "one",
    	2 => "two",
    	3 => "three",
    	4 => "four",
   	 	5 => "five",
    	_ => "something else",
	};
	println!("y is {}",numer);

	let z = 1;
	// or 使用
	// 当是一个表达式时要加上“;”,但是是match 不是赋值表达式时，不加“；”

	match z {
    	1 | 2 => println!("one or two"),
    	3 => println!("three"),
    	_ => println!("anything"),
	}




}