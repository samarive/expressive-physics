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

## What can you currently do in-app

* Add multiple points (Right click > Add point)
* Set the x and y force of the last point (Type in input fields > press "Apply Force !")
	* Force expression examples :
		1. Gravity :
			* x = 0
			* y = 1
		1. Smooth left-right :
			* x = (400 - px)/100
			* y = 0
		1. Fly simulation :
			* x = (400 - px)/100
			* y = (400 - px)/200
		1. Boiiiing :
			* x = (400 - px)/50 - 0.01 * vx
			* y = 0
		1. Rocket launch :
			* x = vy + 1
			* y = vx - 1.0000001

# What to do ?

* UI
	1. Stop input fields from everflowing when the user types a long string
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
* Expressivity
	1. Make expressions accept negative values rather than having to substract to zero.
	1. Make expressions accept trigonometrics functions, exponentials, sqrt, powers etc...
* Code quality
	1. IMPORTANT: Refactor ```Widget::check_event_in_tree```, ```Widget::check_activation_in_tree```, ```Widget::draw_tree```,
```Widget::check_entry_in_tree``` and ```Widget::get_entry_in_tree```, these methodes have a shitload of redundancy.
	1. Document and clean up every last bit of code !

When all of this will be completed, the app should be usable !