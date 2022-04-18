use crate::extensions::NodeExt as _;
use gdnative::api::{AnimatedSprite, Area2D, CollisionShape2D, PhysicsBody2D};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[user_data(user_data::MutexData<Player>)]
#[register_with(Self::register_player)]
pub struct Player {
    #[property(default = 400.0)]
    speed: f32,
    screen_size: Vector2,
}

impl Player {
    fn new(_owner: &Area2D) -> Self {
        Player {
            speed: 400.0,
            screen_size: Vector2::new(0.0,0.0),
        }
    }
}

#[methods]
impl Player {
    fn register_player(builder: &ClassBuilder<Self>) {
        builder.signal("hit").done()
    }

    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        let viewport = unsafe { owner.get_viewport().unwrap().assume_safe() };
        self.screen_size = viewport.size();
        // owner.hide();
    }

    #[export]
    fn _process(&mut self, owner: &Area2D, delta: f32) {
        let input = Input::godot_singleton();
        let mut velocity = Vector2::new(0.0, 0.0);
        let animated_sprite = unsafe{ owner.get_typed_node::<AnimatedSprite, _>("AnimatedSprite")};

        if Input::is_action_pressed(&input, "ui_right", false) {
            velocity.x += 1.0
        }
        if Input::is_action_pressed(&input, "ui_left", false) {
            velocity.x -= 1.0
        }
        if Input::is_action_pressed(&input, "ui_down", false) {
            velocity.y += 1.0
        }
        if Input::is_action_pressed(&input, "ui_up", false) {
            velocity.y -= 1.0
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;
            if velocity.x != 0.0 {
                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0);
                animated_sprite.play("right", false);
            } else {
                animated_sprite.set_flip_v(velocity.y > 0.0);
                animated_sprite.play("up", false);
            }
        } else {
            animated_sprite.stop();
        }

        let change = velocity * delta;
        let position = owner.global_position() + change;
        let position = Vector2::new(
            position.x.max(0.0).min(self.screen_size.x),
            position.y.max(0.0).min(self.screen_size.y),
        );
        owner.set_global_position(position);
    }

    #[export]
    pub fn _on_Player_body_entered(&self, owner: &Area2D, _body: Ref<PhysicsBody2D>) {
        owner.hide();
        owner.emit_signal("hit", &[]);

        let collision_shape = unsafe { owner.get_typed_node::<CollisionShape2D, _>("CollisionShape2D") };

        collision_shape.set_deferred("disabled", true);
    }

    #[export]
    pub fn start(&self, owner: &Area2D, pos: Vector2) {
        owner.set_position(pos);
        owner.show();
        let collision_shape = unsafe { owner.get_typed_node::<CollisionShape2D, _>("CollisionShape2D") };

        collision_shape.set_disabled(false);
    }
}

