# What ?

Expressive Physics aims to be a fun physics simulation tool allowing users to input forces expressions to apply on multiple points.

# When ?

This project is in pre-pre-pre-alpha state, everything will be eventualy refactored and cleaned up. Until then, the documentation can
help up clarifying what is what and how to use it. Just run :
```
cargo doc --open
```
In the project root.

# How ?

To run the app, clone (or download) the repository and run
```
cargo run --release
```
Compilation may fail due to unsatisfied dependencies of the [raylib-rs](https://docs.rs/crate/raylib/latest) crate. If so, please refer to
its documentation.

# What to do ?

* UI
	1. Stop input fields from everflowing when the user types a long string
	1. Make buttons and input fields actually do something when activated
	1. Show the list of points at the left of the screen
		* make it scrollable.
		* make it prompt for a force in the force list when a point is clicked
	1. Show the list of forces at the right of the screen
		* make it scrollable
		* make it editable
	1. Add options in contextual menu (right click in-app)
* Simulation
	1. Change relation between Point and Force
		* a Point should only have immutables pointers to forces it is submited to
	1. Change World from Vec::< Point > to its very own type and let it handle simulations etm
* Code quality
	1. IMPORTANT: Refactor Widget::check_event_in_tree, Widget::check_activation_in_tree, Widget::draw_tree and
Widget::check_entry_in_tree, these methodes have a shitload of redondancy.
	1. Document and clean up every last bit of code !

When all of this will be completed, the app should be usable !