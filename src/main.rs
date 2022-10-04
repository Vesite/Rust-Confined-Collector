use crate::egui::Sense;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_egui::{
    egui::{self, Ui},
    EguiContext, EguiPlugin,
};

fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_plugin(EguiPlugin);
    app.insert_resource(GameResources {
        inventory_vec: Vec::new(),
    });
    app.add_startup_system(init_inventory_vec.system());
    app.add_startup_system(draw_a_sprite.system());

    app.add_state(AppState::Play);
    app.add_system_set(
        SystemSet::on_update(AppState::Play)
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            //Debug
            .with_system(step_event.system())
            .with_system(work_add.system()),
    );

    //Windows
    app.add_system(hire_workers_view.system());
    app.add_system(inventory_view.system());
    app.add_system(resources_view.system());
    app.add_system(market_view.system());
    app.add_system(crafting_view.system());
    app.add_system(actions_view.system());

    app.run();
}

const TIME_STEP: f32 = 1.0 / 30.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Play,
}

enum MySprites {
    GOLD = 0,
    WOOD = 1,
    STONE = 2,
    HATCHET = 3,
    O_HATCHET = 4,
    S_HATCHET = 5,
    PICKAXE = 6,
    O_PICKAXE = 7,
    S_PICKAXE = 8,
    WOOD_CUTTER = 9,
    O_WOOD_CUTTER = 10,
    MINER = 11,
    O_MINER = 12,
    SUPER_WORKER = 13,
    O_SUPER_WORKER = 14,
    WHEAT_FIELD = 15,
    WHEAT = 16,
    FINAL_STATUE = 17,
    SYTHE = 18,
    O_SYTHE = 19,
    S_SYTHE = 20,
    O_WHEAT_FIELD = 21,
    O_FINAL_STATUE = 22,
    DEFAULT = 300,
}

struct GameResources {
    /*
    This is out "inventory" / All our items
    Each elemets is a tuple with:
    - Name of the resouce
    - The amount (f64)
    - An enum corresponding to the image "index"
    */
    inventory_vec: Vec<(String, f64, i32)>,
}

#[derive(Clone, Copy)]
enum InvPos {
    //This is what each position in the inventort vec will be (and will always stay at!)
    //The vec is initialized correctly
    GOLD = 0,
    WOOD = 1,
    STONE = 2,
    WHEAT = 3,
    HATCHET = 4,
    PICKAXE = 5,
    SYTHE = 6,
    WOOD_CUTTER = 7,
    MINER = 8,
    SUPER_WORKER = 9,
    WHEAT_FIELD = 10,
    FINAL_STATUE = 11,
}

/*
Add some value to a resource
*/
fn my_add_resource(game_resources: &mut ResMut<GameResources>, inv_pos_enum: InvPos, amount: f64) {
    let pos = inv_pos_enum as usize;
    let mut tuple = game_resources.inventory_vec.remove(pos);
    tuple.1 = tuple.1 + amount;
    game_resources.inventory_vec.insert(pos, tuple);
}

/*
fn my_get_resource_name(game_resources: &mut ResMut<GameResources>, inv_pos_enum: i32) -> String {

    let pos = inv_pos_enum as usize;
    let tuple = &mut game_resources.inventory_vec[pos];
    let string_val = String::from(tuple.0);
    return string_val; //&tuple.0; //

}
*/

fn my_get_resource_count(game_resources: &mut ResMut<GameResources>, inv_pos_enum: i32) -> f64 {
    let pos = inv_pos_enum as usize;
    //i think i need to add the "&" here??
    let tuple = &game_resources.inventory_vec[pos];
    return tuple.1;
}

fn my_get_resource_sprite(game_resources: &mut ResMut<GameResources>, inv_pos_enum: i32) -> i32 {
    let pos = inv_pos_enum as usize;
    //i think i need to add the "&" here??
    let tuple = &game_resources.inventory_vec[pos];
    return tuple.2;
}

fn init_inventory_vec(mut game_resources: ResMut<GameResources>) {
    //Init The vec
    let tuple = (String::from("Gold"), 0.0, MySprites::GOLD as i32);
    game_resources
        .inventory_vec
        .insert(InvPos::GOLD as usize, tuple);

    let tuple = (String::from("Wood"), 0.0, MySprites::WOOD as i32);
    game_resources
        .inventory_vec
        .insert(InvPos::WOOD as usize, tuple);

    let tuple = (String::from("Stone"), 0.0, MySprites::STONE as i32);
    game_resources
        .inventory_vec
        .insert(InvPos::STONE as usize, tuple);

    let tuple = (String::from("Wheat"), 0.0, MySprites::WHEAT as i32);
    game_resources
        .inventory_vec
        .insert(InvPos::WHEAT as usize, tuple);

    let tuple = (String::from("Hatchet"), 0.0, MySprites::HATCHET as i32);
    game_resources
        .inventory_vec
        .insert(InvPos::HATCHET as usize, tuple);

    let tuple = (String::from("Pickaxe"), 0.0, MySprites::PICKAXE as i32);
    game_resources
        .inventory_vec
        .insert(InvPos::PICKAXE as usize, tuple);

    let tuple = (String::from("Sythe"), 0.0, MySprites::SYTHE as i32);
    game_resources
        .inventory_vec
        .insert(InvPos::SYTHE as usize, tuple);

    let tuple = (
        String::from("Wood Cutter"),
        0.0,
        MySprites::WOOD_CUTTER as i32,
    );
    game_resources
        .inventory_vec
        .insert(InvPos::WOOD_CUTTER as usize, tuple);

    let tuple = (String::from("Miner"), 0.0, MySprites::MINER as i32);
    game_resources
        .inventory_vec
        .insert(InvPos::MINER as usize, tuple);

    let tuple = (
        String::from("Super Worker"),
        0.0,
        MySprites::SUPER_WORKER as i32,
    );
    game_resources
        .inventory_vec
        .insert(InvPos::SUPER_WORKER as usize, tuple);

    let tuple = (
        String::from("Wheat Field"),
        0.0,
        MySprites::WHEAT_FIELD as i32,
    );
    game_resources
        .inventory_vec
        .insert(InvPos::WHEAT_FIELD as usize, tuple);

    let tuple = (
        String::from("Final Statue"),
        0.0,
        MySprites::FINAL_STATUE as i32,
    );
    game_resources
        .inventory_vec
        .insert(InvPos::FINAL_STATUE as usize, tuple);
}

fn step_event(keys: Res<Input<KeyCode>>, mut game_resources: ResMut<GameResources>) {

    // if keys.just_pressed(KeyCode::Space) {
    //     my_add_resource(&mut game_resources, InvPos::GOLD, 100.0);
    //     my_add_resource(&mut game_resources, InvPos::WOOD, 100.0);
    //     my_add_resource(&mut game_resources, InvPos::STONE, 100.0);
    //     my_add_resource(&mut game_resources, InvPos::WHEAT, 100.0);
    // }
}

/*
This is ran once every TIME_STEP
*/
fn work_add(mut game_resources: ResMut<GameResources>) {
    //Loop through our inventory, for each "wood_cutter" we add 2 wood each second
    //Each second we update our inventory variable with the resources we got from our "workers"

    let worker_amount_wood_cutter =
        my_get_resource_count(&mut game_resources, InvPos::WOOD_CUTTER as i32);
    let worker_amount_miner = my_get_resource_count(&mut game_resources, InvPos::MINER as i32);
    let worker_amount_super_worker =
        my_get_resource_count(&mut game_resources, InvPos::SUPER_WORKER as i32);

    let wood_cutter_pr_sec = 2. * worker_amount_wood_cutter;
    add_resource_pr_sec(&mut game_resources, wood_cutter_pr_sec, InvPos::WOOD);

    let miner_pr_sec = 2. * worker_amount_miner;
    add_resource_pr_sec(&mut game_resources, miner_pr_sec, InvPos::STONE);

    let sw_wood_pr_sec = 12. * worker_amount_super_worker;
    let sw_stone_pr_sec = 12. * worker_amount_super_worker;
    let sw_wheat_pr_sec = 12. * worker_amount_super_worker;
    add_resource_pr_sec(&mut game_resources, sw_wood_pr_sec, InvPos::WOOD);
    add_resource_pr_sec(&mut game_resources, sw_stone_pr_sec, InvPos::STONE);
    add_resource_pr_sec(&mut game_resources, sw_wheat_pr_sec, InvPos::WHEAT);
}

/*
Run in the "step" system to add
*/
fn add_resource_pr_sec(
    mut game_resources: &mut ResMut<GameResources>,
    amount_of_resource: f64,
    inv_pos_enum: InvPos,
) {
    let resource_pr_step = amount_of_resource * (TIME_STEP as f64);
    my_add_resource(&mut game_resources, inv_pos_enum, resource_pr_step as f64);
}

//Just to debug
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn actions_view(mut egui_context: ResMut<EguiContext>, mut game_resources: ResMut<GameResources>) {
    egui::Window::new("Actions").show(egui_context.ctx(), |ui| {
        //Twigs
        let mut button = egui::Button::new("Collect twigs");
        button = button.enabled(true);

        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::WOOD, 0.3);
        }
        response_for_button.on_hover_text("Collect twigs and sticks using your hands\n+0.3 wood");

        //Pebbles
        let mut button = egui::Button::new("Collect pebbles");
        button = button.enabled(true);

        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::STONE, 0.2);
        }
        response_for_button
            .on_hover_text("Collect small rocks and pebbles with your hands\n+0.2 stone");

        //Chop
        let mut button = egui::Button::new("Chop wood");
        let stone_hatchet_amount =
            my_get_resource_count(&mut game_resources, InvPos::HATCHET as i32);
        if stone_hatchet_amount < 1.0 {
            button = button.enabled(false);
        }

        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::WOOD, 1.5);
        }
        response_for_button
            .on_hover_text("Chop trees and logs using your stone hatchet\n+1.5 wood");

        //Mine
        let mut button = egui::Button::new("Mine Stone");
        let stone_pickaxe_amount =
            my_get_resource_count(&mut game_resources, InvPos::PICKAXE as i32);
        if stone_pickaxe_amount < 1.0 {
            button = button.enabled(false);
        }

        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::STONE, 1.0);
        }
        response_for_button.on_hover_text("Mine rocks with your stone pickaxe\n+1 stone");

        //Pick
        let mut button = egui::Button::new("Pick Wheat");
        let wheat_field_amount =
            my_get_resource_count(&mut game_resources, InvPos::WHEAT_FIELD as i32);
        if wheat_field_amount >= 1.0 {
            button = button.enabled(true);
        } else {
            button = button.enabled(false);
        }

        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::WHEAT, 0.2);
        }
        response_for_button.on_hover_text("Collect wheat and grain with your hands\n+0.2 wheat");

        //Harvest
        let mut button = egui::Button::new("Harvest Wheat");
        let sythe_amount = my_get_resource_count(&mut game_resources, InvPos::SYTHE as i32);
        if sythe_amount >= 1.0 && wheat_field_amount >= 1.0 {
            button = button.enabled(true);
        } else {
            button = button.enabled(false);
        }

        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::WHEAT, 1.0);
        }
        response_for_button
            .on_hover_text("Collect bundles of wheat with your stone sythe\n+1 wheat");
    });
}

fn crafting_view(
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    mut game_resources: ResMut<GameResources>,
) {
    let sprite = asset_server.load("o_hatchet.png");
    let sprite_id_value_1 = MySprites::O_HATCHET as u64;
    egui_context.set_egui_texture(sprite_id_value_1, sprite);
    let sprite = asset_server.load("o_pickaxe.png");
    let sprite_id_value_2 = MySprites::O_PICKAXE as u64;
    egui_context.set_egui_texture(sprite_id_value_2, sprite);
    let sprite = asset_server.load("o_sythe.png");
    let sprite_id_value_3 = MySprites::O_SYTHE as u64;
    egui_context.set_egui_texture(sprite_id_value_3, sprite);

    egui::Window::new("Crafting").show(egui_context.ctx(), |ui| {
        /*
        Craft hatchet
        */
        if true {
            let w_cost = 4.0;
            let s_cost = 3.0;
            let label_text = format!(
                "Stone Hatchet\nCost: {:.0} Wood & {:.0} Stone",
                w_cost, s_cost
            );

            ui.group(|ui| {
                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(
                    egui::TextureId::User(sprite_id_value_1),
                    [32.0 * 2., 32.0 * 2.],
                );
                let wood = my_get_resource_count(&mut game_resources, InvPos::WOOD as i32);
                let stone = my_get_resource_count(&mut game_resources, InvPos::STONE as i32);
                if wood >= w_cost && stone >= s_cost {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }

                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::WOOD, -w_cost);
                    my_add_resource(&mut game_resources, InvPos::STONE, -s_cost);
                    my_add_resource(&mut game_resources, InvPos::HATCHET, 1.0);
                }
                response_for_button.on_hover_text("Craft a sturdy stone hatchet");
            });
        }

        /*
        Craft pickaxe
        */
        if true {
            let w_cost = 18.0;
            let s_cost = 8.0;
            let label_text = format!(
                "Stone Pickaxe\nCost: {:.0} Wood & {:.0} Stone",
                w_cost, s_cost
            );

            ui.group(|ui| {
                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(
                    egui::TextureId::User(sprite_id_value_2),
                    [32.0 * 2., 32.0 * 2.],
                );
                let wood = my_get_resource_count(&mut game_resources, InvPos::WOOD as i32);
                let stone = my_get_resource_count(&mut game_resources, InvPos::STONE as i32);
                if wood >= w_cost && stone >= s_cost {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }

                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::WOOD, -w_cost);
                    my_add_resource(&mut game_resources, InvPos::STONE, -s_cost);
                    my_add_resource(&mut game_resources, InvPos::PICKAXE, 1.0);
                }
                response_for_button.on_hover_text("Craft a tough stone pickaxe");
            });
        }

        /*
        Craft Sythe
        */
        if true {
            let w_cost = 460.0;
            let s_cost = 300.0;
            let wheat_cost = 10.0;
            let label_text = format!(
                "Stone Sythe\nCost: {:.0} Wood\n{:.0} Stone\n{:.0} Wheat",
                w_cost, s_cost, wheat_cost
            );

            ui.group(|ui| {
                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(
                    egui::TextureId::User(sprite_id_value_3),
                    [32.0 * 2., 32.0 * 2.],
                );
                let wood = my_get_resource_count(&mut game_resources, InvPos::WOOD as i32);
                let stone = my_get_resource_count(&mut game_resources, InvPos::STONE as i32);
                let wheat = my_get_resource_count(&mut game_resources, InvPos::WHEAT as i32);
                if wood >= w_cost && stone >= s_cost && wheat >= wheat_cost {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }

                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::WOOD, -w_cost);
                    my_add_resource(&mut game_resources, InvPos::STONE, -s_cost);
                    my_add_resource(&mut game_resources, InvPos::WHEAT, -wheat_cost);
                    my_add_resource(&mut game_resources, InvPos::SYTHE, 1.0);
                }
                response_for_button.on_hover_text("Craft a sharp stone sythe");
            });
        }
    });
}

fn market_view(
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    mut game_resources: ResMut<GameResources>,
) {
    let sprite = asset_server.load("s_hatchet.png");
    let sprite_id_value_1 = MySprites::S_HATCHET as u64;
    egui_context.set_egui_texture(sprite_id_value_1, sprite);

    let sprite = asset_server.load("s_pickaxe.png");
    let sprite_id_value_2 = MySprites::S_PICKAXE as u64;
    egui_context.set_egui_texture(sprite_id_value_2, sprite);

    let sprite = asset_server.load("s_sythe.png");
    let sprite_id_value_3 = MySprites::S_SYTHE as u64;
    egui_context.set_egui_texture(sprite_id_value_3, sprite);

    let sprite = asset_server.load("o_wheat_field.png");
    let sprite_id_value_4 = MySprites::O_WHEAT_FIELD as u64;
    egui_context.set_egui_texture(sprite_id_value_4, sprite);

    let sprite = asset_server.load("o_final_statue.png");
    let sprite_id_value_5 = MySprites::O_FINAL_STATUE as u64;
    egui_context.set_egui_texture(sprite_id_value_5, sprite);

    egui::Window::new("Trade").show(egui_context.ctx(), |ui| {
        /*
        Sell hatchet
        */
        if true {
            let sell_gold = 2.0;
            let label_text = format!("Sell a Hatchet\n{:.0} Gold", sell_gold);

            ui.group(|ui| {
                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(
                    egui::TextureId::User(sprite_id_value_1),
                    [32.0 * 2., 32.0 * 2.],
                );
                let hatchet_amount =
                    my_get_resource_count(&mut game_resources, InvPos::HATCHET as i32);

                if hatchet_amount >= 1. {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }

                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::GOLD, sell_gold);
                    my_add_resource(&mut game_resources, InvPos::HATCHET, -1.0);
                }
                response_for_button.on_hover_text("Sell a hatchet for gold");
            });
        }

        /*
        Sell pickaxe
        */
        if true {
            let sell_gold = 10.0;
            let label_text = format!("Sell a Pickaxe\n{:.0} Gold", sell_gold);

            ui.group(|ui| {
                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(
                    egui::TextureId::User(sprite_id_value_2),
                    [32.0 * 2., 32.0 * 2.],
                );
                let pickaxe_amount =
                    my_get_resource_count(&mut game_resources, InvPos::PICKAXE as i32);

                if pickaxe_amount >= 1. {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }

                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::GOLD, sell_gold);
                    my_add_resource(&mut game_resources, InvPos::PICKAXE, -1.0);
                }
                response_for_button.on_hover_text("Sell a pickaxe for gold");
            });
        }

        /*
        Sell sythe
        */
        if true {
            let sell_gold = 264.0;
            let label_text = format!("Sell a Sythe\n{:.0} Gold", sell_gold);

            ui.group(|ui| {
                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(
                    egui::TextureId::User(sprite_id_value_3),
                    [32.0 * 2., 32.0 * 2.],
                );
                let amount = my_get_resource_count(&mut game_resources, InvPos::SYTHE as i32);

                if amount >= 1. {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }

                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::GOLD, sell_gold);
                    my_add_resource(&mut game_resources, InvPos::SYTHE, -1.0);
                }
                response_for_button.on_hover_text("Sell a sythe for gold");
            });
        }

        /*
        Buy a Wheat field
        */
        if true {
            let g_cost = 200.0;
            let label_text = format!("Buy a wheat field\nCost: {:.0} Gold", g_cost);

            ui.group(|ui| {
                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(
                    egui::TextureId::User(sprite_id_value_4),
                    [32.0 * 2., 32.0 * 2.],
                );
                let gold = my_get_resource_count(&mut game_resources, InvPos::GOLD as i32);
                let wf_amount =
                    my_get_resource_count(&mut game_resources, InvPos::WHEAT_FIELD as i32);
                if gold >= g_cost && wf_amount < 1.0 {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }

                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::GOLD, -g_cost);
                    my_add_resource(&mut game_resources, InvPos::WHEAT_FIELD, 1.0);
                }
                response_for_button
                    .on_hover_text("A wheat field allows harvesting\nof wheat\nMax 1 field");
            });
        }

        /*
        Buy a Final Statue
        */
        if true {
            let g_cost = 2700.0;
            let w_cost = 4200.0;
            let s_cost = 2500.0;
            let wheat_cost = 500.0;
            let label_text = format!(
                "Buy a golden statue\nCost: {:.0} Gold\n{:.0} Wood\n{:.0} Stone\n{:.0} Wheat",
                g_cost, w_cost, s_cost, wheat_cost
            );

            ui.group(|ui| {
                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(
                    egui::TextureId::User(sprite_id_value_5),
                    [32.0 * 2., 32.0 * 2.],
                );
                let gold = my_get_resource_count(&mut game_resources, InvPos::GOLD as i32);
                let wood = my_get_resource_count(&mut game_resources, InvPos::WOOD as i32);
                let stone = my_get_resource_count(&mut game_resources, InvPos::STONE as i32);
                let wheat = my_get_resource_count(&mut game_resources, InvPos::WHEAT as i32);
                if gold >= g_cost && wood >= w_cost && stone >= s_cost && wheat >= wheat_cost {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }

                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::GOLD, -g_cost);
                    my_add_resource(&mut game_resources, InvPos::WOOD, -w_cost);
                    my_add_resource(&mut game_resources, InvPos::STONE, -s_cost);
                    my_add_resource(&mut game_resources, InvPos::WHEAT, -wheat_cost);
                    my_add_resource(&mut game_resources, InvPos::FINAL_STATUE, 1.0);
                }
                response_for_button.on_hover_text(
                    "Buy a shiny golden statue\nIt does nothing, but looks very pretty",
                );
            });
        }
    });
}

/*
fn load_all_my_sprites_enum(mut egui_context: ResMut<EguiContext>, asset_server: Res<AssetServer>) {

    let sprite = asset_server.load("gold.png");
    egui_context.set_egui_texture(MySprites::GOLD as u64, sprite);
    let sprite = asset_server.load("wood.png");
    egui_context.set_egui_texture(MySprites::WOOD as u64, sprite);
    let sprite = asset_server.load("stone.png");
    egui_context.set_egui_texture(MySprites::STONE as u64, sprite);

    let sprite = asset_server.load("hatchet.png");
    egui_context.set_egui_texture(MySprites::HATCHET as u64, sprite);
    let sprite = asset_server.load("pickaxe.png");
    egui_context.set_egui_texture(MySprites::PICKAXE as u64, sprite);
    let sprite = asset_server.load("wood_cutter.png");
    egui_context.set_egui_texture(MySprites::WOOD_CUTTER as u64, sprite);
    let sprite = asset_server.load("miner.png");
    egui_context.set_egui_texture(MySprites::MINER as u64, sprite);

}
*/

//Visualize the vec "game_resources"
fn inventory_view(
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    mut game_resources: ResMut<GameResources>,
) {
    //load_all_my_sprites_enum(mut egui_context, asset_server);
    let sprite = asset_server.load("gold.png");
    egui_context.set_egui_texture(MySprites::GOLD as u64, sprite);
    let sprite = asset_server.load("wood.png");
    egui_context.set_egui_texture(MySprites::WOOD as u64, sprite);
    let sprite = asset_server.load("stone.png");
    egui_context.set_egui_texture(MySprites::STONE as u64, sprite);

    let sprite = asset_server.load("hatchet.png");
    egui_context.set_egui_texture(MySprites::HATCHET as u64, sprite);
    let sprite = asset_server.load("pickaxe.png");
    egui_context.set_egui_texture(MySprites::PICKAXE as u64, sprite);
    let sprite = asset_server.load("sythe.png");
    egui_context.set_egui_texture(MySprites::SYTHE as u64, sprite);

    let sprite = asset_server.load("wood_cutter.png");
    egui_context.set_egui_texture(MySprites::WOOD_CUTTER as u64, sprite);
    let sprite = asset_server.load("miner.png");
    egui_context.set_egui_texture(MySprites::MINER as u64, sprite);
    let sprite = asset_server.load("super_worker.png");
    egui_context.set_egui_texture(MySprites::SUPER_WORKER as u64, sprite);

    let sprite = asset_server.load("wheat_field.png");
    egui_context.set_egui_texture(MySprites::WHEAT_FIELD as u64, sprite);
    let sprite = asset_server.load("wheat.png");
    egui_context.set_egui_texture(MySprites::WHEAT as u64, sprite);
    let sprite = asset_server.load("final_statue.png");
    egui_context.set_egui_texture(MySprites::FINAL_STATUE as u64, sprite);

    egui::Window::new("Inventory")
        .resizable(true)
        .show(egui_context.ctx(), |ui| {
            //Loop through all the things in "game_resources.vec" and for each one we create a image (with relevant hover text)
            let len = game_resources.inventory_vec.len();
            for i in 4..len {
                //my_add_resource(&mut game_resources, InvPos::GOLD, 0.0);
                // let tuple = &mut game_resources.inventory_vec[i];
                // let name = &mut tuple.0;

                let mut name = "Name Here"; //my_get_resource_sprite(&mut game_resources, i as i32);
                let amount = my_get_resource_count(&mut game_resources, i as i32);
                let sprite_id_value = my_get_resource_sprite(&mut game_resources, i as i32);
                if amount > 0.0 {
                    ui.group(|ui| {
                        //ui.vertical(|ui| {
                        match i {
                            0 => name = "Gold",
                            1 => name = "Wood",
                            2 => name = "Stone",
                            3 => name = "Wheat",
                            4 => name = "Hatchet",
                            5 => name = "Pickaxe",
                            6 => name = "Sythe",
                            7 => name = "Wood Cutter",
                            8 => name = "Miner",
                            9 => name = "Super Worker",
                            10 => name = "Wheat field",
                            11 => name = "Golden Statue",
                            _ => name = "Default",
                        }
                        ui.label(format!("{}: {:.0}", name, amount));
                        ui.image(egui::TextureId::User(sprite_id_value as u64), [64.0, 64.0]);
                        //});
                    });
                }
            }
        });
}

//Visualize the vec "game_resources"
fn resources_view(
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    mut game_resources: ResMut<GameResources>,
) {
    //load_all_my_sprites_enum(mut egui_context, asset_server);
    let sprite = asset_server.load("gold.png");
    egui_context.set_egui_texture(MySprites::GOLD as u64, sprite);
    let sprite = asset_server.load("wood.png");
    egui_context.set_egui_texture(MySprites::WOOD as u64, sprite);
    let sprite = asset_server.load("stone.png");
    egui_context.set_egui_texture(MySprites::STONE as u64, sprite);

    let sprite = asset_server.load("hatchet.png");
    egui_context.set_egui_texture(MySprites::HATCHET as u64, sprite);
    let sprite = asset_server.load("pickaxe.png");
    egui_context.set_egui_texture(MySprites::PICKAXE as u64, sprite);
    let sprite = asset_server.load("sythe.png");
    egui_context.set_egui_texture(MySprites::SYTHE as u64, sprite);

    let sprite = asset_server.load("wood_cutter.png");
    egui_context.set_egui_texture(MySprites::WOOD_CUTTER as u64, sprite);
    let sprite = asset_server.load("miner.png");
    egui_context.set_egui_texture(MySprites::MINER as u64, sprite);
    let sprite = asset_server.load("super_worker.png");
    egui_context.set_egui_texture(MySprites::SUPER_WORKER as u64, sprite);

    let sprite = asset_server.load("wheat_field.png");
    egui_context.set_egui_texture(MySprites::WHEAT_FIELD as u64, sprite);
    let sprite = asset_server.load("wheat.png");
    egui_context.set_egui_texture(MySprites::WHEAT as u64, sprite);
    let sprite = asset_server.load("final_statue.png");
    egui_context.set_egui_texture(MySprites::FINAL_STATUE as u64, sprite);

    egui::Window::new("Resources")
        .resizable(true)
        .show(egui_context.ctx(), |ui| {
            //Loop through all the things in "game_resources.vec" and for each one we create a image (with relevant hover text)
            //let len = game_resources.inventory_vec.len();
            for i in 0..4 {
                //my_add_resource(&mut game_resources, InvPos::GOLD, 0.0);
                // let tuple = &mut game_resources.inventory_vec[i];
                // let name = &mut tuple.0;

                let mut name = "Name Here"; //my_get_resource_sprite(&mut game_resources, i as i32);
                let amount = my_get_resource_count(&mut game_resources, i as i32);
                let sprite_id_value = my_get_resource_sprite(&mut game_resources, i as i32);
                if amount > 0.0 {
                    ui.group(|ui| {
                        //ui.vertical(|ui| {
                        match i {
                            0 => name = "Gold",
                            1 => name = "Wood",
                            2 => name = "Stone",
                            3 => name = "Wheat",
                            _ => name = "Default",
                        }
                        if i == 0 {
                            ui.label(format!("{}: {:.0}", name, amount));
                        } else {
                            ui.label(format!("{}: {:.1}", name, amount));
                        }

                        ui.image(egui::TextureId::User(sprite_id_value as u64), [64.0, 64.0]);
                        //});
                    });
                }
            }
        });
}

fn hire_workers_view(
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    mut game_resources: ResMut<GameResources>,
) {
    let sprite = asset_server.load("o_wood_cutter.png");
    let sprite_id_value_1 = MySprites::O_WOOD_CUTTER as u64;
    egui_context.set_egui_texture(sprite_id_value_1, sprite);
    let sprite = asset_server.load("o_miner.png");
    let sprite_id_value_2 = MySprites::O_MINER as u64;
    egui_context.set_egui_texture(sprite_id_value_2, sprite);
    let sprite = asset_server.load("o_super_worker.png");
    let sprite_id_value_3 = MySprites::O_SUPER_WORKER as u64;
    egui_context.set_egui_texture(sprite_id_value_3, sprite);

    egui::Window::new("Hire Workers").show(egui_context.ctx(), |ui| {

        /*
        Hire button for "Wood Cutter"
        */
        if true {

            let wood_cutter_amount = my_get_resource_count(&mut game_resources, InvPos::WOOD_CUTTER as i32);
            let g_cost = 21.0 + 7.0*wood_cutter_amount;
            let label_text = format!("Hire a Wood Cutter\nCost: {:.0} Gold", g_cost);
            
            ui.group(|ui| {

                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(egui::TextureId::User(sprite_id_value_1), [32.0*2., 32.0*2.]);
                let gold = my_get_resource_count(&mut game_resources, InvPos::GOLD as i32);
                if gold >= g_cost {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }
                
                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::GOLD, -g_cost);
                    my_add_resource(&mut game_resources, InvPos::WOOD_CUTTER, 1.0);
                }
                response_for_button.on_hover_text("Hire a trusty wood cutter\nWill gather 2 wood each second");

            });
        }

        /*
        Hire button for "Miner"
        */
        if true {

            let wood_cutter_amount = my_get_resource_count(&mut game_resources, InvPos::MINER as i32);
            let g_cost = 60.0 + 24.0*wood_cutter_amount;
            let label_text = format!("Hire a Miner\nCost: {:.0} Gold", g_cost);
            
            ui.group(|ui| {

                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(egui::TextureId::User(sprite_id_value_2), [32.0*2., 32.0*2.]);
                let gold = my_get_resource_count(&mut game_resources, InvPos::GOLD as i32);
                if gold >= g_cost {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }
                
                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::GOLD, -g_cost);
                    my_add_resource(&mut game_resources, InvPos::MINER, 1.0);
                }
                response_for_button.on_hover_text("Hire a hard working miner\nWill gather 2 stone each second");

            });
        }

        /*
        Hire button for "Super Worker"
        */
        if true {

            let super_worker_amount = my_get_resource_count(&mut game_resources, InvPos::SUPER_WORKER as i32);
            let g_cost = 900. + 380.*super_worker_amount;
            let wheat_cost = 65. + 24.*super_worker_amount;
            let label_text = format!("Hire a Super Worker\nCost: {:.0} Gold & {:.0} Wheat", g_cost, wheat_cost);
            
            ui.group(|ui| {

                //Make labels
                ui.label(label_text);

                let mut button = egui::ImageButton::new(egui::TextureId::User(sprite_id_value_3), [32.0*2., 32.0*2.]);
                let gold = my_get_resource_count(&mut game_resources, InvPos::GOLD as i32);
                let wheat = my_get_resource_count(&mut game_resources, InvPos::WHEAT as i32);
                if gold >= g_cost && wheat >= wheat_cost {
                    //Do nothing (enabled)
                } else {
                    button = button.sense(Sense::hover());
                }
                
                let response_for_button = ui.add(button);
                if response_for_button.clicked() {
                    my_add_resource(&mut game_resources, InvPos::GOLD, -g_cost);
                    my_add_resource(&mut game_resources, InvPos::WHEAT, -wheat_cost);
                    my_add_resource(&mut game_resources, InvPos::SUPER_WORKER, 1.0);
                }
                response_for_button.on_hover_text("Hire a magical super worker\nWill gather 12 wood, stone and wheat each second!");

            });
        }

    });
}

fn draw_a_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("battle_bg_7.png");
    let scale = Vec3::new(2.0, 2.0, 1.0);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        transform: Transform {
            scale,
            ..Default::default()
        },
        ..Default::default()
    });
}

/*
One of my trade buttons contains:
-A sprite of the thing we get
- A word "Craft / Buy / Collect" to
- The amount and enum of the resource we get
- The amount and enum of the cost (arg 1)

We need to init a sprite in that usual way and have the "sprite_id_value" as a argument
*/
/*
fn add_trade_button_cost_arg_1(
    asset_server: Res<AssetServer>,
    egui_context: ResMut<EguiContext>,
    ui: &mut Ui,
    mut game_resources: &mut ResMut<GameResources>,
    top_text: String,
    sprite_id_value: u64,
    rec_cost_1_type: InvPos,
    rec_cost_1_amount: f64,
    rec_get_1_type: InvPos,
    rec_get_1_amount: f64,
    hover_text: String ) {

        ui.group(|ui| {

            let label = egui::Label::new(top_text);

            let mut button = egui::ImageButton::new(egui::TextureId::User(sprite_id_value), [32.*2., 32.*2.]);
            let has_rec_1 = my_get_resource_count(game_resources, rec_cost_1_type as i32);

            if  has_rec_1 >= rec_cost_1_amount {
                //Do nothign (enabled)
            } else {
                button = button.sense(Sense::hover()) //Disabled
            }

            let response_for_button = ui.add(button);
            if response_for_button.clicked() {
                my_add_resource(&mut game_resources, rec_cost_1_type, -rec_cost_1_amount);
                my_add_resource(&mut game_resources, rec_get_1_type, rec_get_1_amount);
            }
            response_for_button.on_hover_text(hover_text);

        });

}


*/
