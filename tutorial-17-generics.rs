/**
 * 17 Generics
 * http://static.rust-lang.org/doc/master/tutorial.html#generics
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
fn map<T, U>(vector: &[T], function: |v: &T| -> U) -> ~[U] {
	let mut accumulator = ~[];
	for element in vector.iter() {
		accumulator.push(function(element));
	}
	return accumulator;
}
fn main() {
	let strings = ~["a", "b", "c"];
	let new_strings = map(strings, |&x| { x + x });
	println!("{:?} -> {:?}", strings, new_strings);
}
