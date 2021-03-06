/**
 * Rust Tasks and Communication Tutorial - 2 Basics
 * http://static.rust-lang.org/doc/master/guide-tasks.html#basics
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */

fn generate_task_number() -> int {
	10
}
fn main() {
	// Print something profound in a different task using a named function
	fn print_message() { println!("I am running in a different task!"); }
	spawn(print_message);

	// Print something more profound in a different task using a lambda expression
	spawn( proc() println!("I am also running in a different task!") );


	/*
	 */
	// Generate some state locally
	let child_task_number_10 = generate_task_number();

	spawn(proc() {
		   // Capture it in the remote task
		   println!("I am child number {}", child_task_number_10);
	});

	/*
	 */
	for child_task_number in range(0, 20) {
		spawn( proc()
			print!("I am child number {}\n", child_task_number)
		);
	}

}
