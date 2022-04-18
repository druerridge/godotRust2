use crate::extensions::NodeExt as _;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
#[user_data(user_data::ArcData<Hud>)]
#[register_with(Self::register_hud)]
pub struct Hud;

impl Hud {
    fn new(_owner: &CanvasLayer) -> Self {
        Hud
    }
}

#[methods]
impl Hud {
    fn register_hud(builder: &ClassBuilder<Self>) {
        builder.signal( "start_game").done()
    }

    #[export]
    pub fn show_message(&self, owner: &CanvasLayer, text: String) {
        let message = unsafe{ owner.get_typed_node::<Label, _>("Message")};
        message.set_text(text);
        message.show();

        let message_timer = unsafe{ owner.get_typed_node::<Timer, _>("MessageTimer")};
        message_timer.start(0.0);
    }

    #[export]
    pub fn show_game_over(&self, owner: &CanvasLayer) {
        godot_print!("begin show_game_over");
        self.show_message(owner, "Game Over".into());
        
        // let message_timer = unsafe{ owner.get_typed_node::<Timer, _>("MessageTimer")};
        // owner.yield(message_timer, "timeout");
    
        let message = unsafe{ owner.get_typed_node::<Label, _>("Message")};
        message.set_text("Dodge the\nCreeps!");
        message.show();
        // owner.yield(owner.get_tree().create_timer(1), "timeout");

        let start_button = unsafe{ owner.get_typed_node::<Button, _>("StartButton")};
        start_button.show();
    }

    #[export]
    pub fn update_score(&self, owner: &CanvasLayer, score: i64) {
        let score_label = unsafe{ owner.get_typed_node::<Label, _>("ScoreLabel")};
        
        score_label.set_text(score.to_string());
    }
    
    #[export]
    fn _on_StartButton_pressed(&self, owner: &CanvasLayer) {
        let start_button = unsafe{ owner.get_typed_node::<Button, _>("StartButton")};
        start_button.hide();
        owner.emit_signal("start_game", &[]);
    }
    
    #[export]
    fn _on_MessageTimer_timeout(&self, owner: &CanvasLayer) {
        let message = unsafe{ owner.get_typed_node::<Label, _>("Message")};
        message.hide()
    }
}