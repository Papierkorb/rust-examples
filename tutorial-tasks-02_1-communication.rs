#[crate_id="tutorial-tasks-02_1-communication"];
//! Rust Tasks and Communication Tutorial - 2.1 Communication
//!
//! Originally, snippets from the Dave Herman's presentation (2013-01-06) about concurrency
//! http://www.infoq.com/presentations/Rust
//!
//! Dave Herman talks about Mozilla Rust and some of the features that make it safe, concurrent, and fast.
//! 
//! Corrected to 0.10-pre
//! http://static.rust-lang.org/doc/master/guide-tasks.html#communication

//! Simple struct to test data exchange.
struct Point {
    x: f64,
    y: f64
}
fn main() {
	let (port, channel): (Port<~Point>, Chan<~Point>) = Chan::new();
	// isolate process using spawn
	spawn(proc() {
		let s = ~Point { x: 1.0, y: 2.0 };
		// the channel moves the pointer
		channel.send(s);
	});
	let s = port.recv();
	assert!(s.x == 1.0);
	assert!(s.y == 2.0);
}
