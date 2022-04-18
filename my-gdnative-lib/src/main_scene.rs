use crate::extensions::NodeExt as _;
use crate::hud::Hud;
use crate::mob::Mob;
use crate::player::Player;
use gdnative::api::{Area2D, PathFollow2D, Position2D, RigidBody2D};
use gdnative::prelude::*;
use rand::*;
use std::f64::consts::PI;


#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::LocalCellData<Main>)]
pub struct Main {
    #[property]
    mob: Ref<PackedScene>,
    score: i64,
}

#[methods]
impl Main {
    fn new(_owner: &Node) -> Self {
        Main {
            mob: PackedScene::new().into_shared(),
            score: 0,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        // owner.randomize(owner);
        // self.new_game(owner);
    }

    #[export]
    fn game_over(&self, owner: &Node) {
        let score_timer = unsafe { owner.get_typed_node::<Timer, _>("ScoreTimer") };
        let mob_timer = unsafe { owner.get_typed_node::<Timer, _>("MobTimer") };
        
        score_timer.stop();
        mob_timer.stop();

        let hud_node = unsafe { owner.get_typed_node::<CanvasLayer, _>("HUD") };
        hud_node
            .cast_instance::<Hud>()
            .and_then(|hud_refinst| hud_refinst.map(|hud, o| hud.show_game_over(&*o)).ok())
            .unwrap_or_else(|| godot_print!("Unable to get hud"));

        let scene_tree = unsafe { owner.get_tree().unwrap().assume_safe() };
        scene_tree.call_group("mobs", "queue_free", &[]);
    }

    #[export]
    fn new_game(&mut self, owner: &Node) {
        self.score = 0;
        
        let start_position = unsafe { owner.get_typed_node::<Position2D, _>("StartPosition") };
        let player_node = unsafe { owner.get_typed_node::<Area2D, _>("Player") };
        player_node
        .cast_instance::<Player>()
        .and_then(|player_refinst| {
            player_refinst
                .map(|player, owner| 
                    player.start(&*owner, start_position.position()))
                .ok()
        })
        .unwrap_or_else(|| godot_print!("Unable to get player"));

        let start_timer = unsafe { owner.get_typed_node::<Timer, _>("StartTimer") };
        start_timer.start(0.0);

        let hud_node = unsafe { owner.get_typed_node::<CanvasLayer, _>("HUD") };
        
        hud_node.cast_instance::<Hud>()
            .and_then(|hud_refinst| {
                hud_refinst.map(|hud, owner_tref| {
                    hud.update_score(&*owner_tref, self.score);
                    hud.show_message(&*owner_tref, "Get Ready".into());
                })
                .ok()
            })
            .unwrap_or_else(|| godot_print!("Unable to get hud"));
    }

    #[export]
    fn _on_StartTimer_timeout(&self, owner: &Node) {
        let mob_timer = unsafe { owner.get_typed_node::<Timer, _>("MobTimer") };
        let score_timer = unsafe { owner.get_typed_node::<Timer, _>("ScoreTimer") };
        mob_timer.start(0.0);
        score_timer.start(0.0);
    }

    #[export]
    fn _on_ScoreTimer_timeout(&mut self, owner: &Node) {
        self.score += 1;

        let hud_node = unsafe { owner.get_typed_node::<CanvasLayer, _>("HUD") };
        hud_node
            .cast_instance::<Hud>()
            .and_then(|hud_refinst| 
                hud_refinst.map(|hud, owner| hud.update_score(&*owner, self.score)).ok())
            .unwrap_or_else(|| godot_print!("Unable to get hud"));
    }

    #[export]
    fn _on_MobTimer_timeout(&mut self, owner: &Node) {
        let mob_spawn_location = unsafe { owner.get_typed_node::<PathFollow2D, _>("MobPath/MobSpawnLocation") };
        
        let mob_scene: Ref<RigidBody2D, _> = instance_scene(&self.mob);

        let mut rng = rand::thread_rng();

        let offset = rng.gen_range(std::u32::MIN..std::u32::MAX);

        mob_spawn_location.set_offset(offset.into());

        let mut direction = mob_spawn_location.rotation() + PI / 2.0;

        mob_scene.set_position(mob_spawn_location.position());

        direction += rng.gen_range(-PI / 4.0..PI / 4.0);
        mob_scene.set_rotation(direction);
        let d = direction as f32;

        let mob_scene = unsafe {mob_scene.into_shared().assume_safe()};
        owner.add_child(mob_scene, false);

        let mob = mob_scene.cast_instance::<Mob>().unwrap();

        mob.map(|x, mob_owner| {
            mob_owner.set_linear_velocity(Vector2::new(rng.gen_range(x.min_speed..x.max_speed), 0.0));
            mob_owner.set_linear_velocity(mob_owner.linear_velocity().rotated(d));
        })
        .unwrap();
    }
}

fn instance_scene<Root>(scene: &Ref<PackedScene, Shared>) -> Ref<Root, Unique>
    where Root: gdnative::object::GodotObject<Memory = ManuallyManaged> + SubClass<Node>
{
    let scene = unsafe {scene.assume_safe()};

    let instance = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .expect("should be able to instance scene");

    let instance = unsafe { instance.assume_unique() };

    instance.try_cast::<Root>()
        .expect("root node type should be correct")
}