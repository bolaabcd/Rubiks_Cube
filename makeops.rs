fn main() {
	let mut ops = [[[[1;3];3];3];3];
	for i in 0..3 {
		for j in 0..3 {
			for k in 0..3 {
				ops[i][j][k] = [i,j,k];
			}
		}
	}
	println!("{:?}",ops);

	let cop = ops;
	
	// red
	ops[0][0][0] = [0,0,2];
	ops[1][0][0] = [0,0,1];
	ops[2][0][0] = [0,0,0];
	ops[0][0][1] = [1,0,2];
	ops[2][0][1] = [1,0,0];
	ops[0][0][2] = [2,0,2];
	ops[1][0][2] = [2,0,1];
	ops[2][0][2] = [2,0,0];
	println!("{:?}",ops);
	
	ops = cop;
	// green
	ops[2][0][0] = [2,0,2];
	ops[2][1][0] = [2,0,1];
	ops[2][2][0] = [2,0,0];
	ops[2][0][1] = [2,1,2];
	ops[2][2][1] = [2,1,0];
	ops[2][0][2] = [2,2,2];
	ops[2][1][2] = [2,2,1];
	ops[2][2][2] = [2,2,0];
	println!("{:?}",ops);
	
	ops = cop;
	// blue
	ops[0][0][0] = [0,2,0];
	ops[0][1][0] = [0,2,1];
	ops[0][2][0] = [0,2,2];
	ops[0][0][1] = [0,1,0];
	ops[0][2][1] = [0,1,2];
	ops[0][0][2] = [0,0,0];
	ops[0][1][2] = [0,0,1];
	ops[0][2][2] = [0,0,2];
	println!("{:?}",ops);
	
	ops = cop;
	// orange
	ops[0][2][0] = [2,2,0];
	ops[1][2][0] = [2,2,1];
	ops[2][2][0] = [2,2,2];
	ops[0][2][1] = [1,2,0];
	ops[2][2][1] = [1,2,2];
	ops[0][2][2] = [0,2,0];
	ops[1][2][2] = [0,2,1];
	ops[2][2][2] = [0,2,2];
	println!("{:?}",ops);
	
	ops = cop;
	// white
	ops[0][0][0] = [2,0,0];
	ops[1][0][0] = [2,1,0];
	ops[2][0][0] = [2,2,0];
	ops[0][1][0] = [1,0,0];
	ops[2][1][0] = [1,2,0];
	ops[0][2][0] = [0,0,0];
	ops[1][2][0] = [0,1,0];
	ops[2][2][0] = [0,2,0];
	println!("{:?}",ops);
	
	ops = cop;
	// yellow
	ops[0][0][2] = [0,2,2];
	ops[1][0][2] = [0,1,2];
	ops[2][0][2] = [0,0,2];
	ops[0][1][2] = [1,2,2];
	ops[2][1][2] = [1,0,2];
	ops[0][2][2] = [2,2,2];
	ops[1][2][2] = [2,1,2];
	ops[2][2][2] = [2,0,2];
	println!("{:?}",ops);
}
