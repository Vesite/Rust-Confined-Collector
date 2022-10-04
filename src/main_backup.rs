use crate::egui::Sense;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct HelloPlugin;

/*
Don't need?
*/
enum Upgrades {

    STONE_HATCHET = 0,
    STONE_PICKAXE = 1

}

enum InvPos {

    //This is what each position in the inventort vec will be (and will always stay at!)
    //The vec is initialized correctly
    GOLD = 0,
    WOOD = 1,
    STONE = 2,
    STONE_HATCHET = 3,
    STONE_PICKAXE = 4,
    WOOD_CUTTER = 5,
    MINER = 6,

}

enum MySprites {

    GOLD = 0,
    WOOD = 1,
    STONE = 2,
    STONE_HATCHET = 3,
    STONE_PICKAXE = 4,
    WOOD_CUTTER = 5,
    MINER = 6,
    DEFAULT = 300,
    
}

//#[derive(Copy)]
struct GameResources {
    /*
    This is out "inventory" / All our items
    Each elemets is a tuple with:
    - Name of the resouce
    - The amount (f64)
    - An enum corresponding to the image "index"
    */
    inventory_vec: Vec<(String, f64, i32)>
}

fn main() {

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        //.insert_resource(GameResources { gold: 0.0, stone: 0.0, wood: 0.0, upgrades_vec: Vec::new() })
        .insert_resource(GameResources { inventory_vec: Vec::new() })

        //Need to run the function here on my inventory vector
        //init_inventory_vec(inventory_vec);
        .add_startup_system(init_inventory_vec.system())
        .add_system(step_event.system()) //just for debug currently
        .add_startup_system(draw_a_sprite.system())

        //All the windows
        .add_system(hire_workers_view.system()) 
        .add_system(inventory_view.system())
        .add_system(market_view.system())
        .add_system(crafting_view.system())
        .add_system(actions_view.system())
        //.add_system(resources_view.system())

        .run();
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
    game_resources.inventory_vec.insert(InvPos::GOLD as usize, tuple);
    let tuple = (String::from("Wood"), 0.0, MySprites::WOOD as i32);
    game_resources.inventory_vec.insert(InvPos::WOOD as usize, tuple);
    let tuple = (String::from("Stone"), 0.0, MySprites::STONE as i32);
    game_resources.inventory_vec.insert(InvPos::STONE as usize, tuple);
    let tuple = (String::from("Stone Hatchet"), 0.0, MySprites::STONE_HATCHET as i32);
    game_resources.inventory_vec.insert(InvPos::STONE_HATCHET as usize, tuple);
    let tuple = (String::from("Stone Pickaxe"), 0.0, MySprites::STONE_PICKAXE as i32);
    game_resources.inventory_vec.insert(InvPos::STONE_PICKAXE as usize, tuple);
    let tuple = (String::from("Wood Cutter"), 0.0, MySprites::WOOD_CUTTER as i32);
    game_resources.inventory_vec.insert(InvPos::WOOD_CUTTER as usize, tuple);
    let tuple = (String::from("Miner"), 0.0, MySprites::MINER as i32);
    game_resources.inventory_vec.insert(InvPos::MINER as usize, tuple);

}

fn step_event(keys: Res<Input<KeyCode>>, mut game_resources: ResMut<GameResources>) {

    if keys.just_pressed(KeyCode::Space) {
        my_add_resource(&mut game_resources, InvPos::GOLD, 10.0);
        my_add_resource(&mut game_resources, InvPos::WOOD, 10.0);
        my_add_resource(&mut game_resources, InvPos::STONE, 10.0);

        print_type_of(&mut game_resources);

    }

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
        response_for_button.on_hover_text("Collect small rocks and pebbles with your hands\n+0.2 stone");
        
        //Chop
        let mut button = egui::Button::new("Chop wood");
        let stone_hatchet_amount = my_get_resource_count(&mut game_resources, InvPos::STONE_HATCHET as i32);
        if stone_hatchet_amount < 1.0 {
            button = button.enabled(false);     
        }
        
        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::WOOD, 1.5);
        }
        response_for_button.on_hover_text("Chop trees and logs using your stone hatchet\n+1.5 wood");
        
        //Mine
        let mut button = egui::Button::new("Mine Stone");
        let stone_pickaxe_amount = my_get_resource_count(&mut game_resources, InvPos::STONE_PICKAXE as i32);
        if stone_pickaxe_amount < 1.0 {
            button = button.enabled(false);     
        }

        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::STONE, 1.0);
        }
        response_for_button.on_hover_text("Mine rocks with your stone pickaxe\n+1 stone");

    });
}

fn crafting_view(mut egui_context: ResMut<EguiContext>, mut game_resources: ResMut<GameResources>) {
    egui::Window::new("Crafting").show(egui_context.ctx(), |ui| {
        
        let w_cost = 4.0;
        let s_cost = 3.0;
        let _text = format!("Stone Hatchet\nCost:\n{:.0} Wood\n{:.0} Stone", w_cost, s_cost);
        let mut button = egui::Button::new(_text);
        let wood = my_get_resource_count(&mut game_resources, InvPos::WOOD as i32);
        let stone = my_get_resource_count(&mut game_resources, InvPos::STONE as i32);

        if  wood >= w_cost && stone >= s_cost {
            button = button.enabled(true);
        } else {
            button = button.enabled(false);
        }
        
        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::WOOD, -w_cost);
            my_add_resource(&mut game_resources, InvPos::STONE, -s_cost);
            my_add_resource(&mut game_resources, InvPos::STONE_HATCHET, 1.0);
        }
        response_for_button.on_hover_text(format!("Cost:\n{:.0} Wood\n{:.0} Stone", w_cost, s_cost));

        let w_cost = 18.0;
        let s_cost = 8.0;
        let _text = format!("Stone Pickaxe\nCost:\n{:.0} Wood\n{:.0} Stone", w_cost, s_cost);
        let mut button = egui::Button::new(_text);
        let wood = my_get_resource_count(&mut game_resources, InvPos::WOOD as i32);
        let stone = my_get_resource_count(&mut game_resources, InvPos::STONE as i32);
        if  wood >= w_cost && stone >= s_cost {
            button = button.enabled(true);
        } else {
            button = button.enabled(false);
        }
        
        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::WOOD, -w_cost);
            my_add_resource(&mut game_resources, InvPos::STONE, -s_cost);
            my_add_resource(&mut game_resources, InvPos::STONE_PICKAXE, 1.0);
        }
        response_for_button.on_hover_text(format!("Cost:\n{:.0} Wood\n{:.0} Stone", w_cost, s_cost));

    });
}

fn market_view(mut egui_context: ResMut<EguiContext>, mut game_resources: ResMut<GameResources>) {
    egui::Window::new("Trade").show(egui_context.ctx(), |ui| {

        let hatchet_sell_gold = 2.5;
        let txt = format!("Sell Stone Hatchet\n{:.0} gold", hatchet_sell_gold);
        let mut button = egui::Button::new(txt);
        let stone_hatchet_amount = my_get_resource_count(&mut game_resources, InvPos::STONE_HATCHET as i32);
        if stone_hatchet_amount > 0.0 {
            button = button.enabled(true);
        } else {
            button = button.enabled(false);
        }
        
        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::GOLD, hatchet_sell_gold);
            my_add_resource(&mut game_resources, InvPos::STONE_HATCHET, -1.0);
        }

        let pickaxe_sell_gold = 10.;
        let txt = format!("Sell Stone Pickaxe\n{:.0} gold", pickaxe_sell_gold);
        let mut button = egui::Button::new(txt);
        let stone_pickaxe_amount = my_get_resource_count(&mut game_resources, InvPos::STONE_PICKAXE as i32);
        if stone_pickaxe_amount > 0.0 {
            button = button.enabled(true);
        } else {
            button = button.enabled(false);
        }
        
        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::GOLD, pickaxe_sell_gold);
            my_add_resource(&mut game_resources, InvPos::STONE_PICKAXE, -1.0);
        }

    });
}

fn load_all_my_sprites_enum(mut egui_context: ResMut<EguiContext>, asset_server: Res<AssetServer>) {

    let sprite = asset_server.load("gold.png");                                                                           
    egui_context.set_egui_texture(MySprites::GOLD as u64, sprite);
    let sprite = asset_server.load("wood.png");                                                                           
    egui_context.set_egui_texture(MySprites::WOOD as u64, sprite);
    let sprite = asset_server.load("stone.png");                                                                           
    egui_context.set_egui_texture(MySprites::STONE as u64, sprite);

    let sprite = asset_server.load("stone_hatchet.png");                                                                           
    egui_context.set_egui_texture(MySprites::STONE_HATCHET as u64, sprite);
    let sprite = asset_server.load("stone_pickaxe.png");                                                                           
    egui_context.set_egui_texture(MySprites::STONE_PICKAXE as u64, sprite);
    let sprite = asset_server.load("wood_cutter.png");                                                                           
    egui_context.set_egui_texture(MySprites::WOOD_CUTTER as u64, sprite);
    let sprite = asset_server.load("miner.png");                                                                           
    egui_context.set_egui_texture(MySprites::MINER as u64, sprite);

}

//Visualize the vec "game_resources"
fn inventory_view(mut egui_context: ResMut<EguiContext>, asset_server: Res<AssetServer>, mut game_resources: ResMut<GameResources>) {
    
    //load_all_my_sprites_enum(mut egui_context, asset_server);
    let sprite = asset_server.load("gold.png");                                                                           
    egui_context.set_egui_texture(MySprites::GOLD as u64, sprite);
    let sprite = asset_server.load("wood.png");                                                                           
    egui_context.set_egui_texture(MySprites::WOOD as u64, sprite);
    let sprite = asset_server.load("stone.png");                                                                           
    egui_context.set_egui_texture(MySprites::STONE as u64, sprite);

    let sprite = asset_server.load("stone_hatchet.png");                                                                           
    egui_context.set_egui_texture(MySprites::STONE_HATCHET as u64, sprite);
    let sprite = asset_server.load("stone_pickaxe.png");                                                                           
    egui_context.set_egui_texture(MySprites::STONE_PICKAXE as u64, sprite);
    let sprite = asset_server.load("wood_cutter.png");                                                                           
    egui_context.set_egui_texture(MySprites::WOOD_CUTTER as u64, sprite);
    let sprite = asset_server.load("miner.png");                                                                           
    egui_context.set_egui_texture(MySprites::MINER as u64, sprite);

    egui::Window::new("Inventory").show(egui_context.ctx(), |ui| {
        
        //Loop through all the things in "game_resources.vec" and for each one we create a image (with relevant hover text)
        let len = game_resources.inventory_vec.len();
        for i in 0..len {
            
            //my_add_resource(&mut game_resources, InvPos::GOLD, 0.0);
            // let tuple = &mut game_resources.inventory_vec[i];
            // let name = &mut tuple.0;
            
            let name = "Name Here"; //my_get_resource_sprite(&mut game_resources, i as i32);
            let amount = my_get_resource_count(&mut game_resources, i as i32);
            let sprite_id_value = my_get_resource_sprite(&mut game_resources, i as i32);
            if amount > 0.0 {

                ui.group(|ui| {

                    ui.vertical(|ui| {

                        //ui.label(format!("{:.0}: {:.0}x", name, amount));
                        ui.label(format!("{:.0}: {:.1}", name, amount));
                        ui.image(egui::TextureId::User(sprite_id_value as u64), [64.0, 64.0]);

                    });

                });
            }

            //Add text for the resources?
            
            //Add sprite for the resource
            // let sprite_id_value = game_resources.upgrades_vec[i] as u64;
            // let image = egui::Image::new(egui::TextureId::User(sprite_id_value), [32.0*2., 32.0*2.]);
            // let response_for_button = ui.add(image);
            
        }

    });
}

fn hire_workers_view(mut egui_context: ResMut<EguiContext>, asset_server: Res<AssetServer>, mut game_resources: ResMut<GameResources>) {

    let sprite = asset_server.load("wood_cutter.png");                                                                           
    egui_context.set_egui_texture(MySprites::WOOD_CUTTER as u64, sprite);
    let sprite = asset_server.load("miner.png");                                                                           
    egui_context.set_egui_texture(MySprites::MINER as u64, sprite);

    egui::Window::new("Hire Workers").show(egui_context.ctx(), |ui| {

        let cost = 20.0;
        //let txt = "sdsd";
        let sprite_id_value = MySprites::WOOD_CUTTER as u64;
        let mut button = egui::ImageButton::new(egui::TextureId::User(sprite_id_value), [32.0*2., 32.0*2.]);
        let gold = my_get_resource_count(&mut game_resources, InvPos::GOLD as i32);
        if gold >= cost {
            //Do nothing (enabled)
        } else {
            button = button.sense(Sense::hover())
        }
        
        let response_for_button = ui.add(button);
        if response_for_button.clicked() {
            my_add_resource(&mut game_resources, InvPos::GOLD, -cost);
            my_add_resource(&mut game_resources, InvPos::WOOD_CUTTER, 1.0);
            //TEMP
            my_add_resource(&mut game_resources, InvPos::MINER, 1.0);
        }
        
    });
}
    

fn draw_a_sprite(mut commands: Commands, asset_server: Res<AssetServer>,  mut materials: ResMut<Assets<ColorMaterial>>) {

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

// fn resources_view(keys: Res<Input<KeyCode>>, egui_context: ResMut<EguiContext>, game_resources: ResMut<GameResources>) {
//     egui::Window::new("Resources").show(egui_context.ctx(), |ui| {
//         //I want to input the value of my "Gold2" as a string into "label"

//         //Here i format the "new_thingy" so that is becomes a string "alloc::string::String" apparently
//         ui.label(format!("Gold: {:.1}", game_resources.gold));
//         ui.label(format!("Stone: {:.1}", game_resources.stone));
//         ui.label(format!("Wood: {:.1}", game_resources.wood));

//         //println!(gold_val_as_string);
//         //print_type_of(&gold_val_as_string);
//         //gold_struct2.gold2 += 1;

//         //ui.add(egui::Label::text(gold_val_as_string.0));
//         //ui.add(egui::Label::new("With Options").text_color(egui::Color32::RED));¨


//         /*
//         //Does this make a new "new_thingy" variable every single step? (it does, and instaltly changes the value back to default)
//         let mut new_gold_struct2_defined_here = Gold2 { gold2: 0 };

//         */
        
//     });
// }


/*

fn flip_log_var_resource(mut logical: ResMut<Logical>) {

    logical.0 = !logical.0;
    println!("{}", logical.0);

}

struct GreetTimer(Timer);

struct Person;

struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {

    if timer.0.tick(time.delta()).just_finished() {

        for name in query.iter() {
            println!("hello {}!", name.0);
        }
        
    }
}



impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(5.0, true)))
            .insert_resource(Logical(true))
            //.add_startup_system(add_people.system())
            //.add_system(greet_people.system());
    }
}




struct Health {
    hp: f32,
    extra: f32,
}

struct Health2(f32);

struct PlayerXp(u32);

struct PlayerName(String);

//one way to disable
ui.group(|ui| {

        ui.set_enabled(false);
        let button_var = ui.button("Chop wood");
        if button_var.clicked() {
            game_resources.wood += 1.0;
        }

    });

*/

