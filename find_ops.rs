fn apply(cube:[[[[usize;3];3];3];3], op:[[[[usize;3];3];3];3]) -> [[[[usize;3];3];3];3] {
	return
	[
		[
			[
				cube[op[0][0][0][0]][op[0][0][0][1]][op[0][0][0][2]],
				cube[op[0][0][1][0]][op[0][0][1][1]][op[0][0][1][2]],
				cube[op[0][0][2][0]][op[0][0][2][1]][op[0][0][2][2]]
			],
			[
				cube[op[0][1][0][0]][op[0][1][0][1]][op[0][1][0][2]],
				cube[op[0][1][1][0]][op[0][1][1][1]][op[0][1][1][2]],
				cube[op[0][1][2][0]][op[0][1][2][1]][op[0][1][2][2]]
			],
			[
				cube[op[0][2][0][0]][op[0][2][0][1]][op[0][2][0][2]],
				cube[op[0][2][1][0]][op[0][2][1][1]][op[0][2][1][2]],
				cube[op[0][2][2][0]][op[0][2][2][1]][op[0][2][2][2]]
			]
		],
		[
			[
				cube[op[1][0][0][0]][op[1][0][0][1]][op[1][0][0][2]],
				cube[op[1][0][1][0]][op[1][0][1][1]][op[1][0][1][2]],
				cube[op[1][0][2][0]][op[1][0][2][1]][op[1][0][2][2]]
			],
			[
				cube[op[1][1][0][0]][op[1][1][0][1]][op[1][1][0][2]],
				cube[op[1][1][1][0]][op[1][1][1][1]][op[1][1][1][2]],
				cube[op[1][1][2][0]][op[1][1][2][1]][op[1][1][2][2]]
			],
			[
				cube[op[1][2][0][0]][op[1][2][0][1]][op[1][2][0][2]],
				cube[op[1][2][1][0]][op[1][2][1][1]][op[1][2][1][2]],
				cube[op[1][2][2][0]][op[1][2][2][1]][op[1][2][2][2]]
			]
		],
		[
			[
				cube[op[2][0][0][0]][op[2][0][0][1]][op[2][0][0][2]],
				cube[op[2][0][1][0]][op[2][0][1][1]][op[2][0][1][2]],
				cube[op[2][0][2][0]][op[2][0][2][1]][op[2][0][2][2]]
			],
			[
				cube[op[2][1][0][0]][op[2][1][0][1]][op[2][1][0][2]],
				cube[op[2][1][1][0]][op[2][1][1][1]][op[2][1][1][2]],
				cube[op[2][1][2][0]][op[2][1][2][1]][op[2][1][2][2]]
			],
			[
				cube[op[2][2][0][0]][op[2][2][0][1]][op[2][2][0][2]],
				cube[op[2][2][1][0]][op[2][2][1][1]][op[2][2][1][2]],
				cube[op[2][2][2][0]][op[2][2][2][1]][op[2][2][2][2]]
			]
		]
	]
}

fn getop(s:char) -> usize{
	match s {
		'R' | 'r' => 0,
		'G' | 'g' => 1,
		'B' | 'b' => 2,
		'O' | 'o' => 3,
		'W' | 'w' => 4,
		'Y' | 'y' => 5,
		_ => { panic!("Invalid operation."); }
	}
}
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::BTreeMap;
fn find(cube:[[[[usize;3];3];3];3], ops:[[[[[usize;3];3];3];3];6], good_cub: &dyn Fn([[[[usize;3];3];3];3]) -> bool) {
	let mut s = BTreeSet::new();
	let mut q = VecDeque::new();
	let mut m :BTreeMap<[[[[usize;3];3];3];3],([[[[usize;3];3];3];3],char)> = BTreeMap::new();
	s.insert(cube);
	q.push_back(cube);
	while !q.is_empty() {
		let cub = q.pop_front().unwrap();
		if good_cub(cub)  {
			println!("FOUND IT");
			if cub != cube {
				let mut ans = VecDeque::new();
				let mut tup = m.get(&cub).unwrap();
				while tup.0 != cube {
					ans.push_back(tup.1);
					tup = m.get(&tup.0).unwrap();
				};
				ans.push_back(tup.1);
				println!("{}",ans.len());
				while !ans.is_empty() {
					print!("{}",ans.pop_back().unwrap());
				}
				println!("");
			} else {
				println!("DO NOTHING!");
			}
			
			return;
		}
		//for op in ['r','g','b','o','w','y'].iter() {
		for op in ['b','o','y'].iter() {
			let cub2 = apply(cub,ops[getop(*op)]);
			if !s.contains(&cub2) {
				//prev[cub2] = (cub,op);
				m.insert(cub2,(cub,*op));
				s.insert(cub2);
				println!("{:?}",s.len());
				q.push_back(cub2);
			}
		}
	}
}

fn get_good(stay:[[[bool;3];3];3], from:[usize;3], to :[usize;3]) -> impl Fn([[[[usize;3];3];3];3]) -> bool {
	let ans = move |cube:[[[[usize;3];3];3];3]| {
		if cube[to[0]][to[1]][to[2]] != from {
			return false;
		}
		for i in 0..3 {
			for j in 0..3 {
				for k in 0..3 {
					if stay[i][j][k] && cube[i][j][k] != [i,j,k] {
						return false;
					}
				}
			}
		}
		return true;
	};
	return ans;
}

fn main() {
	let ident = [[[[0, 0, 0], [0, 0, 1], [0, 0, 2]], [[0, 1, 0], [0, 1, 1], [0, 1, 2]], [[0, 2, 0], [0, 2, 1], [0, 2, 2]]], [[[1, 0, 0], [1, 0, 1], [1, 0, 2]], [[1, 1, 0], [1, 1, 1], [1, 1, 2]], [[1, 2, 0], [1, 2, 1], [1, 2, 2]]], [[[2, 0, 0], [2, 0, 1], [2, 0, 2]], [[2, 1, 0], [2, 1, 1], [2, 1, 2]], [[2, 2, 0], [2, 2, 1], [2, 2, 2]]]];
	//let stay = [[[true, false,false], [true, true, false], [true, true, false]], [[true, true, false], [true,true, true], [true, true, false]], [[true, true, false], [true, true, false], [true, false, false]]];
	//let stay = [[[true, true,false], [true, true, false], [true, true, false]], [[true, true, false], [true,true, true], [true, true, false]], [[true, true, false], [true, true, false], [true, false, false]]];
	let stay = [[[true, true,false], [true, true, false], [true, true, false]], [[true, true, true], [true,true, true], [true, true, false]], [[true, true, false], [true, true, false], [true, true, false]]];
	let ops = [
		[[[[0, 0, 2], [1, 0, 2], [2, 0, 2]], [[0, 1, 0], [0, 1, 1], [0, 1, 2]], [[0, 2, 0], [0, 2, 1], [0, 2, 2]]], [[[0, 0, 1], [1, 0, 1], [2, 0, 1]], [[1, 1, 0], [1, 1, 1], [1, 1, 2]], [[1, 2, 0], [1, 2, 1], [1, 2, 2]]], [[[0, 0, 0], [1, 0, 0], [2, 0, 0]], [[2, 1, 0], [2, 1, 1], [2, 1, 2]], [[2, 2, 0], [2, 2, 1], [2, 2, 2]]]],// red
		[[[[0, 0, 0], [0, 0, 1], [0, 0, 2]], [[0, 1, 0], [0, 1, 1], [0, 1, 2]], [[0, 2, 0], [0, 2, 1], [0, 2, 2]]], [[[1, 0, 0], [1, 0, 1], [1, 0, 2]], [[1, 1, 0], [1, 1, 1], [1, 1, 2]], [[1, 2, 0], [1, 2, 1], [1, 2, 2]]], [[[2, 0, 2], [2, 1, 2], [2, 2, 2]], [[2, 0, 1], [2, 1, 1], [2, 2, 1]], [[2, 0, 0], [2, 1, 0], [2, 2, 0]]]],// green
		[[[[0, 2, 0], [0, 1, 0], [0, 0, 0]], [[0, 2, 1], [0, 1, 1], [0, 0, 1]], [[0, 2, 2], [0, 1, 2], [0, 0, 2]]], [[[1, 0, 0], [1, 0, 1], [1, 0, 2]], [[1, 1, 0], [1, 1, 1], [1, 1, 2]], [[1, 2, 0], [1, 2, 1], [1, 2, 2]]], [[[2, 0, 0], [2, 0, 1], [2, 0, 2]], [[2, 1, 0], [2, 1, 1], [2, 1, 2]], [[2, 2, 0], [2, 2, 1], [2, 2, 2]]]],// blue
		[[[[0, 0, 0], [0, 0, 1], [0, 0, 2]], [[0, 1, 0], [0, 1, 1], [0, 1, 2]], [[2, 2, 0], [1, 2, 0], [0, 2, 0]]], [[[1, 0, 0], [1, 0, 1], [1, 0, 2]], [[1, 1, 0], [1, 1, 1], [1, 1, 2]], [[2, 2, 1], [1, 2, 1], [0, 2, 1]]], [[[2, 0, 0], [2, 0, 1], [2, 0, 2]], [[2, 1, 0], [2, 1, 1], [2, 1, 2]], [[2, 2, 2], [1, 2, 2], [0, 2, 2]]]],// orange
		[[[[2, 0, 0], [0, 0, 1], [0, 0, 2]], [[1, 0, 0], [0, 1, 1], [0, 1, 2]], [[0, 0, 0], [0, 2, 1], [0, 2, 2]]], [[[2, 1, 0], [1, 0, 1], [1, 0, 2]], [[1, 1, 0], [1, 1, 1], [1, 1, 2]], [[0, 1, 0], [1, 2, 1], [1, 2, 2]]], [[[2, 2, 0], [2, 0, 1], [2, 0, 2]], [[1, 2, 0], [2, 1, 1], [2, 1, 2]], [[0, 2, 0], [2, 2, 1], [2, 2, 2]]]],// white
		[[[[0, 0, 0], [0, 0, 1], [0, 2, 2]], [[0, 1, 0], [0, 1, 1], [1, 2, 2]], [[0, 2, 0], [0, 2, 1], [2, 2, 2]]], [[[1, 0, 0], [1, 0, 1], [0, 1, 2]], [[1, 1, 0], [1, 1, 1], [1, 1, 2]], [[1, 2, 0], [1, 2, 1], [2, 1, 2]]], [[[2, 0, 0], [2, 0, 1], [0, 0, 2]], [[2, 1, 0], [2, 1, 1], [1, 0, 2]], [[2, 2, 0], [2, 2, 1], [2, 0, 2]]]] // yellow
	];
	
	println!("{:?}",ident);
	println!("{:?}",apply(ident,ops[getop('b')]));
    println!("Hello World!");

	//let gg = get_good(stay,[0,1,2],[0,0,1]);
	//let test = get_good([[[false;3];3];3],[0,0,0],[0,0,2]);
	//find(ident,ops,&test);
	//let gg = get_good(stay,[1,2,2],[2,2,1]);
	let gg = get_good(stay,[1,2,2],[0,1,2]);
	find(ident,ops,&gg);
}
