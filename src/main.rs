use macroquad::prelude::*;

#[macroquad::main("Testing gametools")]

async fn main(){
    loop {
        clear_background(DARKPURPLE);
        next_frame().await
    }
}
