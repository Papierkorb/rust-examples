/**
 * 15.3 Do syntax
 * http://static.rust-lang.org/doc/master/tutorial.html#do-syntax
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */

// A function that takes a procedure as argument:
fn call_it(op: proc(v: int)) {
	op(10)
}

fn each(v: &[int], op: |v: &int|) {
	let mut n = 0;
	while n < v.len() {
		op(&v[n]);
		n += 1;
	}
}

fn main() {
	// As a caller, if we use a closure to provide the final operator argument, we can write it in a way that has a pleasant, block-like structure.
	call_it(proc(n) {
		println!("{:?}", n);
	});
	each([1, 2, 3], |n: &int| {
		println!("{:?}", n);
	});

	// |v: &int| is stack closure, not procedure: procedure is allocated at heap, and you can call it only once.
}
