extern crate gdnative;
use gdnative::*;

use euclid::rect;
use euclid::vec2;

#[derive(NativeClass)]
#[inherit(gdnative::Node2D)]
#[user_data(user_data::ArcData<GUIPaintNode>)]
pub struct GUIPaintNode;
#[methods]
impl GUIPaintNode {
    fn _init(_owner: Node2D) -> Self { GUIPaintNode }

    #[export]
    fn _ready(&self, _owner: Node2D) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("NODE PAINT READY");
    }

    #[export]
    fn _draw(&self, mut s: Node2D) {
        unsafe {
            godot_print!("DRAW: {} ", s.get_name().to_string());
            s.draw_rect(rect(10.0, 10.0, 200.0, 200.0), Color::rgba(255.0, 1.0, 0.0, 255.0), true);
            s.draw_circle(vec2(50.0, 50.0), 20.0, Color::rgb(1.0, 0.0, 1.0));
        }
    }
}


/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(gdnative::Node)]
#[user_data(user_data::ArcData<HelloWorld>)]
pub struct HelloWorld;

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {

    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        HelloWorld
    }

//    #[export]

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are"attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    fn _ready(&self, _owner: Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("hello, world. YE!");
    }

    #[export]
    fn _process(&self, owner: Node, delta: f64) {
        unsafe {
            if let Some(n) = owner.get_node(NodePath::from_str("Ship")) {
                let mut s : Spatial = n.cast().unwrap();
//                godot_print!("DELTA: {} : {}", s.get_name().to_string(), delta);
                s.rotate_y(delta);
            }
            if let Some(n) = owner.get_node(NodePath::from_str("CanvasLayer/Node2D")) {
//                if let Some(x) = n
                let mut s : Node2D = n.cast().unwrap();
//                godot_print!("DELTA: {} : {}", s.get_name().to_string(), delta);
                s.draw_rect(rect(10.0, 10.0, 200.0, 200.0), Color::rgb(1.0, 1.0, 0.0), true);
                s.update();
//                s.rotate_y(delta);
            }
        }
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<HelloWorld>();
    handle.add_class::<GUIPaintNode>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
