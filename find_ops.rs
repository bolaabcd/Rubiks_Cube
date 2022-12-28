// We will represent a cube as having six faces:
/*

    RRR
    RRR
    RRR
YYY BBB WWW
YYY BBB WWW
YYY BBB WWW
    OOO
    OOO
    OOO
    GGG
    GGG
    GGG

cube[n] -> nth face (the order is R,Y,B,W,O,G)
cube[n][i][j] -> ith line jth column of the nth face.

Each operation assumes the person is holding the cube looking at the blue face, with the green
one at the left and the red one on top.

We have Counter-Clockwise (CCW) and Clockwise (CW), for every possible face. Also, for each face
we have the "double" options, to move 180 degrees.

We are allowing redundant movements because we will do many Breadth-First Searchs, so we want
to reach a solution in as few steps as possible.

Also, because a solution may take a lot of steps, we will aim for "checkpoint solutions", just
moving one piece at a time to the right place, without moving the others. We may get stuck at
"local maxima" (situations such that we can't move one piece without moving the others). But
because I don't even know how common these situations are, I will not implement any kind of
backtracking for now, and see what happens.

I expect this to take at most four or five hours. This is my second try at doing this, the
first one was not successfull because I didn't think the orientation of the litte cubes would
be an issue.


I also am not optimizing anything for now, I will see if this works first.

One interesting idea I should try if this succeeds is to check how commonly we repeat states
in our computation (if we can have the same cube position reached twice at different moments).

*/

const operations:[&str;18] = 
[
	"CCW R",
	"CCW Y",
	"CCW B",
	"CCW W",
	"CCW O",
	"CCW G",
	"CW R" ,
	"CW Y" ,
	"CW B" ,
	"CW W" ,
	"CW O" ,
	"CW G" ,
	"DB R" ,
	"DB Y" ,
	"DB B" ,
	"DB W" ,
	"DB O" ,
	"DB G" 
];

const final_cube:[[[char;3];3];6] = 
[
	[
		['R', 'R', 'R'], 
		['R', 'R', 'R'], 
		['R', 'R', 'R']
	], 
	[
		['Y', 'Y', 'Y'], 
		['Y', 'Y', 'Y'], 
		['Y', 'Y', 'Y']
	], 
	[
		['B', 'B', 'B'], 
		['B', 'B', 'B'], 
		['B', 'B', 'B']
	], 
	[
		['W', 'W', 'W'], 
		['W', 'W', 'W'], 
		['W', 'W', 'W']
	], 
	[
		['O', 'O', 'O'], 
		['O', 'O', 'O'], 
		['O', 'O', 'O']
	], 
	[
		['G', 'G', 'G'], 
		['G', 'G', 'G'], 
		['G', 'G', 'G']
	]
];


fn rotate(cube:[[[char;3];3];6], face:usize, direction:&str) -> [[[char;3];3];6]
{
	if direction == "CW"
	{
		return rotate(rotate(rotate(cube,face,"CCW"),face,"CCW"),face,"CCW");
	}
	if direction != "CCW" 
	{
		panic!("Invalid direction.");
	}
	let mut cub2 = cube;
	// First we rotate the face
	cub2[face] =
	[
		[cub2[face][0][2],cub2[face][1][2],cub2[face][2][2]],
		[cub2[face][0][1],cub2[face][1][1],cub2[face][2][1]],
		[cub2[face][0][0],cub2[face][1][0],cub2[face][2][0]]
	];
	// Then we rotate the neighbouring faces (12 cases)
	let cub3 = cub2;
	if face == 0
	{
		cub2[1][0][0] = cub3[5][2][2];
		cub2[1][0][1] = cub3[5][2][1];
		cub2[1][0][2] = cub3[5][2][0];

		cub2[2][0][0] = cub3[1][0][0];
		cub2[2][0][1] = cub3[1][0][1];
		cub2[2][0][2] = cub3[1][0][2];

		cub2[3][0][0] = cub3[2][0][0];
		cub2[3][0][1] = cub3[2][0][1];
		cub2[3][0][2] = cub3[2][0][2];

		cub2[5][2][0] = cub3[3][0][2];
		cub2[5][2][1] = cub3[3][0][1];
		cub2[5][2][2] = cub3[3][0][0];
	} else if face == 1
	{
		cub2[0][0][0] = cub3[2][0][0];
		cub2[0][1][0] = cub3[2][1][0];
		cub2[0][2][0] = cub3[2][2][0];

		cub2[2][0][0] = cub3[4][0][0];
		cub2[2][1][0] = cub3[4][1][0];
		cub2[2][2][0] = cub3[4][2][0];

		cub2[4][0][0] = cub3[5][0][0];
		cub2[4][1][0] = cub3[5][1][0];
		cub2[4][2][0] = cub3[5][2][0];

		cub2[5][0][0] = cub3[0][0][0];
		cub2[5][1][0] = cub3[0][1][0];
		cub2[5][2][0] = cub3[0][2][0];
	} else if face == 2
	{
		cub2[0][2][0] = cub3[3][0][0];
		cub2[0][2][1] = cub3[3][1][0];
		cub2[0][2][2] = cub3[3][2][0];

		cub2[1][0][2] = cub3[0][2][2];
		cub2[1][1][2] = cub3[0][2][1];
		cub2[1][2][2] = cub3[0][2][0];

		cub2[3][0][0] = cub3[4][0][2];
		cub2[3][1][0] = cub3[4][0][1];
		cub2[3][2][0] = cub3[4][0][0];

		cub2[4][0][0] = cub3[1][0][2];
		cub2[4][0][1] = cub3[1][1][2];
		cub2[4][0][2] = cub3[1][2][2];
	} else if face == 3
	{
		cub2[0][0][2] = cub3[5][0][2];
		cub2[0][1][2] = cub3[5][1][2];
		cub2[0][2][2] = cub3[5][2][2];

		cub2[2][0][2] = cub3[0][0][2];
		cub2[2][1][2] = cub3[0][1][2];
		cub2[2][2][2] = cub3[0][2][2];

		cub2[4][0][2] = cub3[2][0][2];
		cub2[4][1][2] = cub3[2][1][2];
		cub2[4][2][2] = cub3[2][2][2];

		cub2[5][0][2] = cub3[4][0][2];
		cub2[5][1][2] = cub3[4][1][2];
		cub2[5][2][2] = cub3[4][2][2];
	} else if face == 4
	{
		cub2[1][2][0] = cub3[2][2][0];
		cub2[1][2][1] = cub3[2][2][1];
		cub2[1][2][2] = cub3[2][2][2];

		cub2[2][2][0] = cub3[3][2][0];
		cub2[2][2][1] = cub3[3][2][1];
		cub2[2][2][2] = cub3[3][2][2];

		cub2[3][2][0] = cub3[5][0][2];
		cub2[3][2][1] = cub3[5][0][1];
		cub2[3][2][2] = cub3[5][0][0];

		cub2[5][0][0] = cub3[1][2][2];
		cub2[5][0][1] = cub3[1][2][1];
		cub2[5][0][2] = cub3[1][2][0];
	} else if face == 5
	{
		cub2[0][0][0] = cub3[1][2][0];
		cub2[0][0][1] = cub3[1][1][0];
		cub2[0][0][2] = cub3[1][0][0];

		cub2[1][0][0] = cub3[4][2][0];
		cub2[1][1][0] = cub3[4][2][1];
		cub2[1][2][0] = cub3[4][2][2];

		cub2[3][0][2] = cub3[0][0][0];
		cub2[3][1][2] = cub3[0][0][1];
		cub2[3][2][2] = cub3[0][0][2];

		cub2[4][2][0] = cub3[3][2][2];
		cub2[4][2][1] = cub3[3][1][2];
		cub2[4][2][2] = cub3[3][0][2];
	} else
	{
		panic!("Invalid face.");
	}
	return cub2;
}

fn apply(cube:[[[char;3];3];6], option:&str) -> [[[char; 3]; 3]; 6]
/* 
If I treid to hard-code the operations, I would need to write manually and correctly 540 
	values, and I don't want to do that. So I will try to simplify the operations.
	match option 
	{
		"CCW R" => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		"CCW Y" => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		"CCW B" => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		"CCW W" => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		"CCW O" => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		"CCW G" => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		"RIYHT" => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		"LEFT"  => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		"UP"    => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		"DWON"  => |cub| [[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]],[[cub[][][]],[cub[][][]],[cub[][][]]]],
		_ => { panic!("Invalid operation."); }
	}
	*/
{
	match option
	{
		"CCW R" => return rotate(cube, 0, "CCW"),
		"CCW Y" => return rotate(cube, 1, "CCW"),
		"CCW B" => return rotate(cube, 2, "CCW"),
		"CCW W" => return rotate(cube, 3, "CCW"),
		"CCW O" => return rotate(cube, 4, "CCW"),
		"CCW G" => return rotate(cube, 5, "CCW"),

		"CW R"  => return rotate(cube, 0, "CW" ),
		"CW Y"  => return rotate(cube, 1, "CW" ),
		"CW B"  => return rotate(cube, 2, "CW" ),
		"CW W"  => return rotate(cube, 3, "CW" ),
		"CW O"  => return rotate(cube, 4, "CW" ),
		"CW G"  => return rotate(cube, 5, "CW" ),

		"DB R"  => return rotate(rotate(cube, 0, "CW"), 0, "CW"),
		"DB Y"  => return rotate(rotate(cube, 1, "CW"), 1, "CW"),
		"DB B"  => return rotate(rotate(cube, 2, "CW"), 2, "CW"),
		"DB W"  => return rotate(rotate(cube, 3, "CW"), 3, "CW"),
		"DB O"  => return rotate(rotate(cube, 4, "CW"), 4, "CW"),
		"DB G"  => return rotate(rotate(cube, 5, "CW"), 5, "CW"),
		_ => { panic!("Invalid operation."); }
	};
}

fn ok(cub1:[[[char;3];3];6],_n:usize,_i:usize,_j:usize) -> bool
{
	for n in 0.._n+1
	{
		for i in 0..3
		{
			if (n == _n && i > _i) 
			{
				break;
			}
			for j in 0..3
			{
				if (n == _n && i == _i && j > _j)
				{
					break;
				}
				if cub1[n][i][j] != final_cube[n][i][j]
				{
					return false;
				}
			}
		}
	}
	return true;
}

use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
fn fix_pos<'a>(cube:[[[char;3];3];6],n:usize,i:usize,j:usize) -> ([[[char; 3]; 3]; 6], Vec<&'a str>)
/*
fixes cube[n][i][j], without changing positions cube[r][s][t] with (r,s,t) < (n,i,j); 
lexicographically smaller.
*/
{
	assert!(n >= 0 && i >= 0 && j >= 0);
	assert!(n <  6 && i <  3 && j <  3);

	let mut last_pos = final_cube;

	let mut q = VecDeque::new(); // BFS
	let mut s = BTreeSet::new(); // Remember already visited states
	let mut m = BTreeMap::new(); // Recover the answer
	q.push_back(cube);
	let mut okk = false;
	while !q.is_empty() 
	{
		let cub = q.pop_front().unwrap();
		if ok(cub,n,i,j)
		{
			last_pos = cub;
			okk = true;
			break;
		}
		for op in operations.iter()
		{
			let cub2 = apply(cub,op);
			if !s.contains(&cub2) {
				s.insert(cub2); // remember we already found cub2
				m.insert(cub2,(cub,op)); // which operation we did just before reaching cub2
				assert!(cub2 != cub);
				//println!("{:?}",s.len());
				q.push_back(cub2); // we may want to explore from cub2 (BFS)
			}
		}
	}

	assert!(okk);
	//assert!(!q.is_empty()); // It shouldn't be possible to search every possible option here.
	// Recovering the answer
	let mut movements : Vec<&str> = Vec::new();
	let mut cub3 = last_pos;
	while cub3 != cube
	{
		movements.push(m[&cub3].1);
		cub3 = m[&cub3].0;
	}
	movements.reverse();
	return (last_pos, movements);
}

fn solve(mut cube:[[[char;3];3];6]) // returns how to solve the cube 
{ 
	let mut ans = vec![];  	
	for n in 0..6 
	{
		for i in 0..3 
		{
			for j in 0..3 
			{
				let (new_cube,mut part_ans) = fix_pos(cube,n,i,j);
				cube = new_cube;
				println!("{:?}/{:?}",9*n+3*i+j+1,6*3*3);
				println!("{:?}",part_ans);
				println!("{:?}",cube);
				ans.append(&mut part_ans);
				//println!("{:?}\n{:?}",ans, cube);
			}
		}
	}
	assert_eq!(cube,final_cube);
	println!("\nAnswer: {:?}", ans);
}

use std::io;
fn main() 
{/*
	let mut s = BTreeSet::new();
	s.insert(['1']);
	s.insert(['2']);
	s.insert(['3']);
	let mut otro = ['1'];
	println!("{}",s.contains(&otro));
	return;*/
	//println!("{:?}\n{:?}",final_cube,apply(final_cube,"CW B"));
	//return;
	println!("Type the cube colors in the following format (and make sure you are looking at it in the right way):");
	print!(
"    RRR
    RRR
    RRR
YYY BBB WWW
YYY BBB WWW
YYY BBB WWW
    OOO
    OOO
    OOO
    GGG
    GGG
    GGG

");
	
	let mut cube = final_cube;
	let stdin = io::stdin();
	for n in 0..6
	{
		for i in 0..3
		{
			for j in 0..3
			{
				let mut line = String::new();
				stdin.read_line(&mut line);
				cube[n][i][j] = line.chars().nth(0).unwrap();
			}
		}
	}
	//println!("{:?}",cube);
	//println!("{:?}",apply(apply(apply(apply(cube,"CCW R"),"CW G"),"CW W"),"CCW B"));
	
	solve(cube);
}

/*
fn getop(s:char) -> usize{
	match s {
		'R' | 'r' => 0,
		'Y' | 'g' => 1,
		'B' | 'b' => 2,
		'W' | 'o' => 3,
		'O' | 'w' => 4,
		'G' | 'y' => 5,
		_ => { panic!("Invalid operation."); }
	}
}
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::BTreeMap;
fn find(cube:[[usize;3];6], ops:[[[[[usize;3];3];3];3];6], good_cub: &dyn Fn([[[[usize;3];3];3];3]) -> bool) {
	let mut s = BTreeSet::new();
	let mut q = VecDeque::new();
	let mut m :BTreeMap<[[[[usize;3];3];3];3],([[[[usize;3];3];3];3],char)> = BTreeMap::new();
	s.insert(cube);
	q.push_back(cube);
	while !q.is_empty() {
		let cub = q.pop_front().unwrap();
		if good_cub(cub)  {
			println!("FWUND IT");
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
				println!("DW NWTHINY!");
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
    println!("Hello Oorld!");

	//let gg = get_good(stay,[0,1,2],[0,0,1]);
	//let test = get_good([[[false;3];3];3],[0,0,0],[0,0,2]);
	//find(ident,ops,&test);
	//let gg = get_good(stay,[1,2,2],[2,2,1]);
	let gg = get_good(stay,[1,2,2],[0,1,2]);
	find(ident,ops,&gg);
}
*/