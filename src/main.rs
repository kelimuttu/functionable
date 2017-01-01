fn main() {

    let angka = hitung(10, "+");
    let angka2 = hitung(15, "*");

    println!("{}, {}", angka, angka2);

}

fn hitung(mut accumulator: i32, program: &str) -> i32 {
	for token in program.chars() {
    	match token {
    		'+' => accumulator += 1,
    		'-' => accumulator -= 1,
    		'*' => accumulator *= 2,
    		'/' => accumulator /= 2,
    		_ => {}
    	}
    }
    accumulator
}
