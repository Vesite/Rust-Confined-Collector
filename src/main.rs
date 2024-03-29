use std::path;
use crate::egui::Sense;
use bevy::{time::FixedTimestep, prelude::*};
use bevy_egui::{
    egui::{self, Ui},
    EguiContext, EguiPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(EguiPlugin);
    
    app.insert_resource(SpriteCollection {
        sprites_vec: Vec::new(),
    });
    app.insert_resource(GameResources {
        inventory_vec: Vec::new(),
    });

    app.add_startup_system(init_inventory_vec);
    app.add_startup_system(draw_a_sprite);
    app.add_startup_system(load_sprites);

    app.add_state(AppState::Play);
    app.add_system_set(
        SystemSet::on_update(AppState::Play)
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            //Debug
            .with_system(step_event)
            .with_system(work_add),
    );

    // Windows
    app.add_system(hire_workers_view);
    app.add_system(inventory_view);
    app.add_system(resources_view);
    app.add_system(market_view);
    app.add_system(crafting_view);
    app.add_system(actions_view);

    app.run();
}

const TIME_STEP: f32 = 1.0 / 30.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Play,
}

struct SpriteCollection {
    sprites_vec: Vec<egui::TextureHandle>,
}

enum MySprites {
    O_WOOD_CUTTER = 0,
    O_MINER = 1,
    O_SUPER_WORKER = 2,
    S_HATCHET = 3,
    S_PICKAXE = 4,
    S_SYTHE = 5,
    O_WHEAT_FIELD = 6,
    O_FINAL_STATUE = 7,
    O_HATCHET = 8,
    O_PICKAXE = 9,
    O_SYTHE = 10,
}

// One inventory item.
// Contains name, resource and a handle to the image texture.
struct InventoryItem {
    name: String,
    amount: f64,
    handle: egui::TextureHandle,
}

impl InventoryItem {
    fn new(name: impl Into<String>, amount: f64, handle: egui::TextureHandle) -> Self {
        InventoryItem {
            name: name.into(),
            amount: amount,
            handle: handle,
        }
    }
} 

// Struct for the entire inventory
struct GameResources {
    inventory_vec: Vec<InventoryItem>,
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

// Add some value to a resource
fn my_add_resource(game_resources: &mut ResMut<GameResources>, inv_pos_enum: InvPos, amount: f64) {
    let pos = inv_pos_enum as usize;
    let mut game_resource = game_resources.inventory_vec.remove(pos);
    game_resource.amount += amount;
    game_resources.inventory_vec.insert(pos, game_resource);
}

fn my_get_resource_count(game_resources: &mut ResMut<GameResources>, inv_pos_enum: i32) -> f64 {
    let pos = inv_pos_enum as usize;
    let game_resource = &game_resources.inventory_vec[pos];
    return game_resource.amount;
}

fn my_get_resource_sprite<'a>(game_resources: &'a mut ResMut<GameResources>, inv_pos_enum: i32) -> &'a egui::TextureHandle {
    let pos = inv_pos_enum as usize;
    let game_resource = &game_resources.inventory_vec[pos];
    return &game_resource.handle;
}

fn my_get_sprites_handle<'a>(sprite_collection: &'a mut ResMut<SpriteCollection>, inv_pos_enum: i32) -> &'a egui::TextureHandle {
    let pos = inv_pos_enum as usize;
    let sprite_handle = &sprite_collection.sprites_vec[pos];
    sprite_handle
}

fn load_image(ctx: &egui::Context, image_name: &str) -> egui::TextureHandle {
    let path = format!("assets/{}.png", image_name);
    let image = match load_image_from_path(path::Path::new(&path)) {
        Ok(image) => image,
        Err(err) => {
            panic!("Could not load image {}. Reason: {}", path, err);
        }
    };
    
    let handle = ctx.load_texture(
        image_name,
        image,
        egui::TextureFilter::Linear
    );

    handle
}

fn init_inventory_vec(mut game_resources: ResMut<GameResources>, mut egui_context: ResMut<EguiContext>) {
    let mut ctx = egui_context.ctx_mut();
    
    //Init The vec
    let game_resource = InventoryItem::new("Gold", 0.0, load_image(ctx, "gold"));
    game_resources.inventory_vec.insert(InvPos::GOLD as usize, game_resource);

    let game_resource = InventoryItem::new("Wood", 0.0, load_image(ctx, "wood"));
    game_resources.inventory_vec.insert(InvPos::WOOD as usize, game_resource);

    let game_resource = InventoryItem::new("Stone", 0.0, load_image(ctx, "stone"));
    game_resources.inventory_vec.insert(InvPos::STONE as usize, game_resource);

    let game_resource = InventoryItem::new("Wheat", 0.0, load_image(ctx, "wheat"));
    game_resources.inventory_vec.insert(InvPos::WHEAT as usize, game_resource);

    let game_resource = InventoryItem::new("Hatchet", 0.0, load_image(ctx, "hatchet"));
    game_resources.inventory_vec.insert(InvPos::HATCHET as usize, game_resource);

    let game_resource = InventoryItem::new("Pickaxe", 0.0, load_image(ctx, "pickaxe"));
    game_resources.inventory_vec.insert(InvPos::PICKAXE as usize, game_resource);

    let game_resource = InventoryItem::new("Sythe", 0.0, load_image(ctx, "sythe"));
    game_resources.inventory_vec.insert(InvPos::SYTHE as usize, game_resource);

    let game_resource = InventoryItem::new("Wood Cutter", 0.0, load_image(ctx, "wood_cutter"));
    game_resources.inventory_vec.insert(InvPos::WOOD_CUTTER as usize, game_resource);

    let game_resource = InventoryItem::new("Miner", 0.0, load_image(ctx, "miner"));
    game_resources.inventory_vec.insert(InvPos::MINER as usize, game_resource);

    let game_resource = InventoryItem::new("Super Worker", 0.0, load_image(ctx, "super_worker"));
    game_resources.inventory_vec.insert(InvPos::SUPER_WORKER as usize, game_resource);

    let game_resource = InventoryItem::new("Wheat Field", 0.0, load_image(ctx, "wheat_field"));
    game_resources.inventory_vec.insert(InvPos::WHEAT_FIELD as usize, game_resource);

    let game_resource = InventoryItem::new("Final Statue", 0.0, load_image(ctx, "final_statue"));
    game_resources.inventory_vec.insert(InvPos::FINAL_STATUE as usize, game_resource);
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
    egui::Window::new("Actions").show(egui_context.ctx_mut(), |ui| {
        //Twigs
        let mut button = egui::Button::new("Collect twigs");
        
        if ui.add_enabled(true, button)
            .on_hover_text("Collect twigs and sticks using your hands\n+0.3 wood")
            .clicked() {
                my_add_resource(&mut game_resources, InvPos::WOOD, 0.3);
            }

        //Pebbles
        let mut button = egui::Button::new("Collect pebbles");

        if ui.add_enabled(true, button)
            .on_hover_text("Collect small rocks and pebbles with your hands\n+0.2 stone")
            .clicked() {
                my_add_resource(&mut game_resources, InvPos::STONE, 0.2);
            }

        //Chop
        let button = egui::Button::new("Chop wood");
        let stone_hatchet_amount = my_get_resource_count(&mut game_resources, InvPos::HATCHET as i32);

        let enabled = if stone_hatchet_amount < 1.0 {
            false
        } else {
            true
        };

        if ui.add_enabled(enabled, button)
            .on_hover_text("Chop trees and logs using your stone hatchet\n+1.5 wood")
            .clicked() {
                my_add_resource(&mut game_resources, InvPos::WOOD, 1.5);
            }
            

        //Mine
        let mut button = egui::Button::new("Mine Stone");
        let stone_pickaxe_amount = my_get_resource_count(&mut game_resources, InvPos::PICKAXE as i32);
        
        let enabled = if stone_pickaxe_amount < 1.0 {
            false
        } else {
            true
        };

        if ui.add_enabled(enabled, button)
            .on_hover_text("Mine rocks with your stone pickaxe\n+1 stone")
            .clicked() {
                my_add_resource(&mut game_resources, InvPos::STONE, 1.0);
            }

        //Pick
        let mut button = egui::Button::new("Pick Wheat");
        let wheat_field_amount = my_get_resource_count(&mut game_resources, InvPos::WHEAT_FIELD as i32);
        
        let enabled = if wheat_field_amount < 1.0 {
            false
        } else {
            true
        };

        if ui.add_enabled(enabled, button)
            .on_hover_text("Collect wheat and grain with your hands\n+0.2 wheat")
            .clicked() {
                my_add_resource(&mut game_resources, InvPos::SYTHE, 1.0);
            }
        
        //Harvest
        let button = egui::Button::new("Harvest Wheat");
        let sythe_amount = my_get_resource_count(&mut game_resources, InvPos::SYTHE as i32);
        
        let enabled = if sythe_amount >= 1.0 && wheat_field_amount >= 1.0 {
            true
        } else {
            false
        };

        if ui.add_enabled(enabled, button)
            .on_hover_text("Collect bundles of wheat with your stone sythe\n+1 wheat")
            .clicked() {
                my_add_resource(&mut game_resources, InvPos::WHEAT, 1.0);
            }
    });
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

fn crafting_view(
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    mut game_resources: ResMut<GameResources>,
    mut sprite_collection: ResMut<SpriteCollection>,
) {
    let ctx = egui_context.ctx_mut();

    egui::Window::new("Crafting").show(ctx, |ui| {
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

                let sprite = my_get_resource_sprite(&mut game_resources, MySprites::O_HATCHET as i32);
                let mut button = egui::ImageButton::new(
                    sprite.id(),
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

                let sprite = my_get_resource_sprite(&mut game_resources, MySprites::O_PICKAXE as i32);
                let mut button = egui::ImageButton::new(
                    sprite.id(),
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

                let sprite = my_get_resource_sprite(&mut game_resources, MySprites::O_SYTHE as i32);
                let mut button = egui::ImageButton::new(
                    sprite.id(),
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
    let ctx = egui_context.ctx_mut();

    egui::Window::new("Trade").show(ctx, |ui| {
        /*
        Sell hatchet
        */
        if true {
            let sell_gold = 2.0;
            let label_text = format!("Sell a Hatchet\n{:.0} Gold", sell_gold);

            ui.group(|ui| {
                //Make labels
                ui.label(label_text);

                let sprite = my_get_resource_sprite(&mut game_resources, MySprites::S_HATCHET as i32);
                let mut button = egui::ImageButton::new(
                    sprite.id(),
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

                let sprite = my_get_resource_sprite(&mut game_resources, MySprites::S_PICKAXE as i32);
                let mut button = egui::ImageButton::new(
                    sprite.id(),
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

                let sprite = my_get_resource_sprite(&mut game_resources, MySprites::S_SYTHE as i32);
                let mut button = egui::ImageButton::new(
                    sprite.id(),
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

                let sprite = my_get_resource_sprite(&mut game_resources, MySprites::O_WHEAT_FIELD as i32);
                let mut button = egui::ImageButton::new(
                    sprite.id(),
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

                let sprite = my_get_resource_sprite(&mut game_resources, MySprites::O_FINAL_STATUE as i32);
                let mut button = egui::ImageButton::new(
                    sprite.id(),
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

//Visualize the vec "game_resources"
fn inventory_view(
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    mut game_resources: ResMut<GameResources>,
) {
    let mut ctx = egui_context.ctx_mut();

    egui::Window::new("Inventory")
        .resizable(true)
        .show(ctx, |ui| {
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
                        ui.image(sprite_id_value.id(), [64.0, 64.0]);
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
    let ctx = egui_context.ctx_mut();

    egui::Window::new("Resources")
        .resizable(true)
        .show(ctx, |ui| {
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

                        ui.image(sprite_id_value.id(), [64.0, 64.0]);
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
    mut sprite_collection: ResMut<SpriteCollection>,
) {
    let mut ctx = egui_context.ctx_mut();

    egui::Window::new("Hire Workers").show(ctx, |ui| {
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

                let wood_cutter_handle = my_get_sprites_handle(&mut sprite_collection, MySprites::O_WOOD_CUTTER as i32); 
                let mut button = egui::ImageButton::new(wood_cutter_handle.id(), [32.0*2., 32.0*2.]);
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

                let miner_handle = my_get_sprites_handle(&mut sprite_collection, MySprites::O_MINER as i32); 
                let mut button = egui::ImageButton::new(miner_handle.id(), [32.0*2., 32.0*2.]);
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

                let super_worker_handle = my_get_sprites_handle(&mut sprite_collection, MySprites::O_SUPER_WORKER as i32); 
                let mut button = egui::ImageButton::new(super_worker_handle.id(), [32.0*2., 32.0*2.]);
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
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("battle_bg_7.png"),
        transform: Transform {
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..default()
        },
        ..default()
    });
}

fn load_sprites(mut sprite_collection: ResMut<SpriteCollection>, mut egui_context: ResMut<EguiContext>) {
    let ctx = egui_context.ctx_mut();

    // Workers
    let sprite = load_image(ctx, "o_wood_cutter");
    sprite_collection.sprites_vec.insert(MySprites::O_WOOD_CUTTER as usize, sprite);    

    let sprite = load_image(ctx, "o_miner");
    sprite_collection.sprites_vec.insert(MySprites::O_MINER as usize, sprite);    

    let sprite = load_image(ctx, "o_super_worker");
    sprite_collection.sprites_vec.insert(MySprites::O_SUPER_WORKER as usize, sprite);    

    // Market
    let sprite = load_image(ctx, "s_hatchet");
    sprite_collection.sprites_vec.insert(MySprites::S_HATCHET as usize, sprite);    

    let sprite = load_image(ctx, "s_pickaxe");
    sprite_collection.sprites_vec.insert(MySprites::S_PICKAXE as usize, sprite);    
    
    let sprite = load_image(ctx, "s_sythe");
    sprite_collection.sprites_vec.insert(MySprites::S_SYTHE as usize, sprite);    

    let sprite = load_image(ctx, "o_wheat_field");
    sprite_collection.sprites_vec.insert(MySprites::O_WHEAT_FIELD as usize, sprite);    

    let sprite = load_image(ctx, "o_final_statue");
    sprite_collection.sprites_vec.insert(MySprites::O_FINAL_STATUE as usize, sprite);    

    // Crafting
    let sprite = load_image(ctx, "o_hatchet");
    sprite_collection.sprites_vec.insert(MySprites::O_HATCHET as usize, sprite);    
    
    let sprite = load_image(ctx, "o_pickaxe");
    sprite_collection.sprites_vec.insert(MySprites::O_PICKAXE as usize, sprite);    
    
    let sprite = load_image(ctx, "o_sythe");
    sprite_collection.sprites_vec.insert(MySprites::O_SYTHE as usize, sprite);    
}