use macroquad::prelude::*;
use macroquad::rand::gen_range;

const GRID_W: i32 = 20;
const GRID_H: i32 = 12;
const TILE_SIZE: f32 = 40.0;

const UI_TOP: f32 = 80.0;
const SCREEN_W: f32 = GRID_W as f32 * TILE_SIZE;
const SCREEN_H: f32 = UI_TOP + GRID_H as f32 * TILE_SIZE + 90.0;

const MAX_LOG_LINES: usize = 7;

fn window_conf() -> Conf {
    Conf {
        window_title: "Terminal Dungeon DJ".to_string(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        high_dpi: false,
        sample_count: 1,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
    Exit,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PickupKind {
    Treasure,
    Potion,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Vibe {
    Synthwave,
    DungeonJazz,
    CursedTechno,
    IceAmbient,
    GoblinFunk,
}

impl Vibe {
    fn random() -> Self {
        match gen_range(0, 5) {
            0 => Self::Synthwave,
            1 => Self::DungeonJazz,
            2 => Self::CursedTechno,
            3 => Self::IceAmbient,
            _ => Self::GoblinFunk,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Synthwave => "Synthwave Catacombs",
            Self::DungeonJazz => "Dungeon Jazz Lounge",
            Self::CursedTechno => "Cursed Techno Pit",
            Self::IceAmbient => "Ice Ambient Vault",
            Self::GoblinFunk => "Goblin Funk Basement",
        }
    }

    fn bg(&self) -> Color {
        match self {
            Self::Synthwave => Color::from_rgba(24, 10, 46, 255),
            Self::DungeonJazz => Color::from_rgba(40, 28, 15, 255),
            Self::CursedTechno => Color::from_rgba(14, 20, 18, 255),
            Self::IceAmbient => Color::from_rgba(10, 30, 44, 255),
            Self::GoblinFunk => Color::from_rgba(18, 42, 20, 255),
        }
    }

    fn accent(&self) -> Color {
        match self {
            Self::Synthwave => Color::from_rgba(255, 70, 180, 255),
            Self::DungeonJazz => Color::from_rgba(255, 200, 120, 255),
            Self::CursedTechno => Color::from_rgba(80, 255, 160, 255),
            Self::IceAmbient => Color::from_rgba(120, 220, 255, 255),
            Self::GoblinFunk => Color::from_rgba(180, 255, 90, 255),
        }
    }

    fn floor(&self) -> Color {
        match self {
            Self::Synthwave => Color::from_rgba(60, 30, 100, 255),
            Self::DungeonJazz => Color::from_rgba(82, 58, 33, 255),
            Self::CursedTechno => Color::from_rgba(22, 58, 42, 255),
            Self::IceAmbient => Color::from_rgba(34, 72, 92, 255),
            Self::GoblinFunk => Color::from_rgba(40, 80, 35, 255),
        }
    }

    fn wall(&self) -> Color {
        match self {
            Self::Synthwave => Color::from_rgba(140, 70, 200, 255),
            Self::DungeonJazz => Color::from_rgba(150, 100, 50, 255),
            Self::CursedTechno => Color::from_rgba(40, 120, 95, 255),
            Self::IceAmbient => Color::from_rgba(80, 150, 190, 255),
            Self::GoblinFunk => Color::from_rgba(90, 130, 50, 255),
        }
    }
}

#[derive(Clone, Copy)]
struct Player {
    x: i32,
    y: i32,
    hp: i32,
    max_hp: i32,
}

#[derive(Clone, Copy)]
struct Monster {
    x: i32,
    y: i32,
    hp: i32,
}

#[derive(Clone, Copy)]
struct Pickup {
    x: i32,
    y: i32,
    kind: PickupKind,
}

struct FloatingText {
    text: String,
    x: f32,
    y: f32,
    ttl: f32,
    color: Color,
}

struct Game {
    tiles: Vec<Vec<Tile>>,
    player: Player,
    monsters: Vec<Monster>,
    pickups: Vec<Pickup>,
    room_number: u32,
    vibe: Vibe,
    score: i32,
    messages: Vec<String>,
    floaters: Vec<FloatingText>,
    game_over: bool,
    victory_room_flash: f32,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            tiles: vec![vec![Tile::Floor; GRID_W as usize]; GRID_H as usize],
            player: Player {
                x: 1,
                y: 1,
                hp: 12,
                max_hp: 12,
            },
            monsters: Vec::new(),
            pickups: Vec::new(),
            room_number: 1,
            vibe: Vibe::random(),
            score: 0,
            messages: Vec::new(),
            floaters: Vec::new(),
            game_over: false,
            victory_room_flash: 0.0,
        };

        game.generate_room(true);
        game
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn log<S: Into<String>>(&mut self, msg: S) {
        self.messages.push(msg.into());
        if self.messages.len() > MAX_LOG_LINES {
            self.messages.remove(0);
        }
    }

    fn spawn_floater<S: Into<String>>(&mut self, text: S, gx: i32, gy: i32, color: Color) {
        self.floaters.push(FloatingText {
            text: text.into(),
            x: gx as f32 * TILE_SIZE + 8.0,
            y: UI_TOP + gy as f32 * TILE_SIZE + 18.0,
            ttl: 1.0,
            color,
        });
    }

    fn is_inside(x: i32, y: i32) -> bool {
        x >= 0 && x < GRID_W && y >= 0 && y < GRID_H
    }

    fn tile_at(&self, x: i32, y: i32) -> Tile {
        self.tiles[y as usize][x as usize]
    }

    fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        self.tiles[y as usize][x as usize] = tile;
    }

    fn is_blocked(&self, x: i32, y: i32) -> bool {
        if !Self::is_inside(x, y) {
            return true;
        }

        if self.tile_at(x, y) == Tile::Wall {
            return true;
        }

        false
    }

    fn monster_index_at(&self, x: i32, y: i32) -> Option<usize> {
        self.monsters
            .iter()
            .position(|m| m.x == x && m.y == y && m.hp > 0)
    }

    fn pickup_index_at(&self, x: i32, y: i32) -> Option<usize> {
        self.pickups.iter().position(|p| p.x == x && p.y == y)
    }

    fn occupied_by_spawned_entity(&self, x: i32, y: i32) -> bool {
        if self.player.x == x && self.player.y == y {
            return true;
        }

        self.monsters.iter().any(|m| m.x == x && m.y == y && m.hp > 0)
            || self.pickups.iter().any(|p| p.x == x && p.y == y)
            || self.tile_at(x, y) == Tile::Exit
    }

    fn random_empty_position(&self) -> (i32, i32) {
        loop {
            let x = gen_range(1, GRID_W - 1);
            let y = gen_range(1, GRID_H - 1);

            if self.tile_at(x, y) == Tile::Floor && !self.occupied_by_spawned_entity(x, y) {
                return (x, y);
            }
        }
    }

    fn generate_room(&mut self, first_room: bool) {
        self.tiles = vec![vec![Tile::Floor; GRID_W as usize]; GRID_H as usize];
        self.monsters.clear();
        self.pickups.clear();
        self.floaters.clear();
        self.vibe = Vibe::random();
        self.victory_room_flash = 0.8;

        for y in 0..GRID_H {
            for x in 0..GRID_W {
                let border = x == 0 || y == 0 || x == GRID_W - 1 || y == GRID_H - 1;
                if border {
                    self.set_tile(x, y, Tile::Wall);
                }
            }
        }

        let wall_count = gen_range(18, 34);
        for _ in 0..wall_count {
            let x = gen_range(1, GRID_W - 1);
            let y = gen_range(1, GRID_H - 1);

            if (x, y) == (1, 1) {
                continue;
            }

            self.set_tile(x, y, Tile::Wall);
        }

        if first_room {
            self.player.x = 1;
            self.player.y = 1;
        } else {
            let (px, py) = self.random_empty_position();
            self.player.x = px;
            self.player.y = py;
        }

        let (exit_x, exit_y) = self.random_empty_position();
        self.set_tile(exit_x, exit_y, Tile::Exit);

        let treasure_count = gen_range(2, 6);
        for _ in 0..treasure_count {
            let (x, y) = self.random_empty_position();
            self.pickups.push(Pickup {
                x,
                y,
                kind: PickupKind::Treasure,
            });
        }

        let potion_count = 1 + gen_range(0, 2);
        for _ in 0..potion_count {
            let (x, y) = self.random_empty_position();
            self.pickups.push(Pickup {
                x,
                y,
                kind: PickupKind::Potion,
            });
        }

        let monster_count = gen_range(1, 5);
        for _ in 0..monster_count {
            let (x, y) = self.random_empty_position();
            self.monsters.push(Monster {
                x,
                y,
                hp: gen_range(2, 5),
            });
        }

        self.log(format!(
            "Room {} dropped: {}",
            self.room_number,
            self.vibe.name()
        ));
    }

    fn try_move_player(&mut self, dx: i32, dy: i32) {
        if self.game_over {
            return;
        }

        let nx = self.player.x + dx;
        let ny = self.player.y + dy;

        if !Self::is_inside(nx, ny) || self.is_blocked(nx, ny) {
            self.log("You bonk into the architecture.");
            return;
        }

        if let Some(monster_index) = self.monster_index_at(nx, ny) {
            self.fight(monster_index);
            self.monsters.retain(|m| m.hp > 0);
            self.after_player_action();
            return;
        }

        self.player.x = nx;
        self.player.y = ny;

        if let Some(pickup_index) = self.pickup_index_at(nx, ny) {
            let pickup = self.pickups.remove(pickup_index);
            match pickup.kind {
                PickupKind::Treasure => {
                    self.score += 10;
                    self.log("You pocket cursed treasure. +10 score.");
                    self.spawn_floater("+10 GOLD", nx, ny, GOLD);
                }
                PickupKind::Potion => {
                    let heal = gen_range(2, 5);
                    self.player.hp = (self.player.hp + heal).min(self.player.max_hp);
                    self.log(format!("You chug a suspicious potion. +{} HP.", heal));
                    self.spawn_floater(format!("+{} HP", heal), nx, ny, GREEN);
                }
            }
        }

        if self.tile_at(nx, ny) == Tile::Exit {
            self.room_number += 1;
            self.score += 25;
            self.log("You found the exit. Crowd goes wild. +25 score.");
            self.generate_room(false);
            return;
        }

        self.after_player_action();
    }

    fn fight(&mut self, monster_index: usize) {
        let player_hit = gen_range(1, 5);
        let monster_hit = gen_range(0, 4);

        let (mx, my);
        {
            let monster = &mut self.monsters[monster_index];
            monster.hp -= player_hit;
            mx = monster.x;
            my = monster.y;
        }

        self.spawn_floater(format!("-{}", player_hit), mx, my, RED);

        if self.monsters[monster_index].hp <= 0 {
            self.score += 15;
            self.log("Monster deleted from the dance floor. +15 score.");
            self.spawn_floater("KO!", mx, my, ORANGE);
        } else {
            self.player.hp -= monster_hit;
            self.log(format!(
                "You slap the monster for {}. It slaps back for {}.",
                player_hit, monster_hit
            ));
            if monster_hit > 0 {
                self.spawn_floater(format!("-{}", monster_hit), self.player.x, self.player.y, PINK);
            }
        }

        if self.player.hp <= 0 {
            self.player.hp = 0;
            self.game_over = true;
            self.log(self.random_death_message());
        }
    }

    fn after_player_action(&mut self) {
        self.move_monsters();

        if self.player.hp <= 0 {
            self.player.hp = 0;
            self.game_over = true;
            self.log(self.random_death_message());
        }
    }

    fn move_monsters(&mut self) {
        let mut queued_damage = 0;
        let player_pos = (self.player.x, self.player.y);

        let snapshot: Vec<(i32, i32)> = self
            .monsters
            .iter()
            .filter(|m| m.hp > 0)
            .map(|m| (m.x, m.y))
            .collect();

        for i in 0..self.monsters.len() {
            if self.monsters[i].hp <= 0 {
                continue;
            }

            let dx = (player_pos.0 - self.monsters[i].x).signum();
            let dy = (player_pos.1 - self.monsters[i].y).signum();

            let mut options = [(dx, 0), (0, dy), (dx, dy), (0, 0)];

            if gen_range(0, 2) == 0 {
                options.swap(0, 1);
            }

            let mut moved = false;

            for (mx, my) in options {
                let nx = self.monsters[i].x + mx;
                let ny = self.monsters[i].y + my;

                if (nx, ny) == player_pos {
                    let dmg = gen_range(1, 4);
                    queued_damage += dmg;
                    self.spawn_floater(format!("-{}", dmg), self.player.x, self.player.y, RED);
                    self.log(format!("A monster body-checks you for {}.", dmg));
                    moved = true;
                    break;
                }

                if !Self::is_inside(nx, ny) || self.tile_at(nx, ny) == Tile::Wall || self.tile_at(nx, ny) == Tile::Exit {
                    continue;
                }

                let occupied_by_monster = snapshot
                    .iter()
                    .enumerate()
                    .any(|(j, pos)| j != i && *pos == (nx, ny));

                let occupied_by_pickup = self.pickups.iter().any(|p| p.x == nx && p.y == ny);

                if occupied_by_monster || occupied_by_pickup || (nx, ny) == player_pos {
                    continue;
                }

                self.monsters[i].x = nx;
                self.monsters[i].y = ny;
                moved = true;
                break;
            }

            if !moved {
                // no-op
            }
        }

        if queued_damage > 0 {
            self.player.hp -= queued_damage;
        }
    }

    fn random_death_message(&self) -> String {
        match gen_range(0, 5) {
            0 => "You were booed to death by a skeleton trio.".to_string(),
            1 => "The goblin funk consumed you.".to_string(),
            2 => "A cursed beat drop ended your career.".to_string(),
            3 => "You got folded by dungeon nightlife.".to_string(),
            _ => "Your set ended in catastrophic vibes.".to_string(),
        }
    }

    fn update(&mut self, dt: f32) {
        self.victory_room_flash = (self.victory_room_flash - dt).max(0.0);

        for floater in &mut self.floaters {
            floater.y -= 25.0 * dt;
            floater.ttl -= dt;
        }
        self.floaters.retain(|f| f.ttl > 0.0);

        if self.game_over && is_key_pressed(KeyCode::R) {
            self.reset();
        }
    }

    fn draw(&self) {
        clear_background(self.vibe.bg());

        self.draw_header();
        self.draw_grid();
        self.draw_entities();
        self.draw_log();
        self.draw_overlay();
    }

    fn draw_header(&self) {
        let accent = self.vibe.accent();

        draw_rectangle(0.0, 0.0, SCREEN_W, 64.0, Color::from_rgba(0, 0, 0, 120));
        draw_text("TERMINAL DUNGEON DJ", 16.0, 28.0, 32.0, accent);
        draw_text(
            &format!("Room {}", self.room_number),
            16.0,
            56.0,
            24.0,
            WHITE,
        );
        draw_text(
            &format!("HP {}/{}", self.player.hp, self.player.max_hp),
            170.0,
            56.0,
            24.0,
            if self.player.hp <= 4 { RED } else { GREEN },
        );
        draw_text(
            &format!("Score {}", self.score),
            290.0,
            56.0,
            24.0,
            YELLOW,
        );
        draw_text(self.vibe.name(), 430.0, 56.0, 24.0, accent);
    }

    fn draw_grid(&self) {
        let accent = self.vibe.accent();
        let floor_color = self.vibe.floor();
        let wall_color = self.vibe.wall();

        if self.victory_room_flash > 0.0 {
            let alpha = (self.victory_room_flash * 80.0) as u8;
            draw_rectangle(0.0, UI_TOP, SCREEN_W, GRID_H as f32 * TILE_SIZE, Color::from_rgba(255, 255, 255, alpha));
        }

        for y in 0..GRID_H {
            for x in 0..GRID_W {
                let px = x as f32 * TILE_SIZE;
                let py = UI_TOP + y as f32 * TILE_SIZE;

                match self.tile_at(x, y) {
                    Tile::Floor => {
                        draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, floor_color);
                    }
                    Tile::Wall => {
                        draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, wall_color);
                        draw_rectangle_lines(px + 4.0, py + 4.0, TILE_SIZE - 9.0, TILE_SIZE - 9.0, 2.0, BLACK);
                    }
                    Tile::Exit => {
                        draw_rectangle(px, py, TILE_SIZE - 1.0, TILE_SIZE - 1.0, DARKPURPLE);
                        draw_circle(px + TILE_SIZE / 2.0, py + TILE_SIZE / 2.0, 10.0, accent);
                        draw_circle_lines(px + TILE_SIZE / 2.0, py + TILE_SIZE / 2.0, 14.0, 3.0, WHITE);
                    }
                }
            }
        }
    }

    fn draw_entities(&self) {
        for pickup in &self.pickups {
            let cx = pickup.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            let cy = UI_TOP + pickup.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

            match pickup.kind {
                PickupKind::Treasure => {
                    draw_circle(cx, cy, 9.0, GOLD);
                    draw_circle_lines(cx, cy, 12.0, 2.0, YELLOW);
                }
                PickupKind::Potion => {
                    draw_rectangle(cx - 8.0, cy - 10.0, 16.0, 20.0, SKYBLUE);
                    draw_rectangle(cx - 5.0, cy - 14.0, 10.0, 6.0, WHITE);
                }
            }
        }

        for monster in &self.monsters {
            let px = monster.x as f32 * TILE_SIZE + 6.0;
            let py = UI_TOP + monster.y as f32 * TILE_SIZE + 6.0;

            draw_rectangle(px, py, TILE_SIZE - 12.0, TILE_SIZE - 12.0, RED);
            draw_circle(px + 10.0, py + 12.0, 2.0, WHITE);
            draw_circle(px + 22.0, py + 12.0, 2.0, WHITE);
            draw_line(px + 8.0, py + 24.0, px + 24.0, py + 22.0, 2.0, BLACK);
        }

        let px = self.player.x as f32 * TILE_SIZE + 6.0;
        let py = UI_TOP + self.player.y as f32 * TILE_SIZE + 6.0;
        draw_rectangle(px, py, TILE_SIZE - 12.0, TILE_SIZE - 12.0, self.vibe.accent());
        draw_circle(px + 10.0, py + 12.0, 2.0, BLACK);
        draw_circle(px + 22.0, py + 12.0, 2.0, BLACK);
        draw_line(px + 8.0, py + 24.0, px + 24.0, py + 24.0, 2.0, BLACK);

        for floater in &self.floaters {
            let mut c = floater.color;
            c.a = floater.ttl.clamp(0.0, 1.0);
            draw_text(&floater.text, floater.x, floater.y, 24.0, c);
        }
    }

    fn draw_log(&self) {
        let panel_y = UI_TOP + GRID_H as f32 * TILE_SIZE + 8.0;
        draw_rectangle(0.0, panel_y, SCREEN_W, 82.0, Color::from_rgba(0, 0, 0, 140));

        draw_text("LIVE FEED", 12.0, panel_y + 22.0, 24.0, WHITE);

        for (i, msg) in self.messages.iter().rev().take(MAX_LOG_LINES).enumerate() {
            draw_text(msg, 12.0, panel_y + 46.0 + i as f32 * 18.0, 20.0, LIGHTGRAY);
        }
    }

    fn draw_overlay(&self) {
        let hint = "Move: WASD / Arrows   |   R: restart on death";
        draw_text(hint, 16.0, SCREEN_H - 10.0, 22.0, WHITE);

        if self.game_over {
            draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, Color::from_rgba(0, 0, 0, 180));
            draw_text("GAME OVER", SCREEN_W / 2.0 - 120.0, SCREEN_H / 2.0 - 30.0, 48.0, RED);
            draw_text(
                &format!("Final score: {}", self.score),
                SCREEN_W / 2.0 - 110.0,
                SCREEN_H / 2.0 + 10.0,
                32.0,
                WHITE,
            );
            draw_text(
                "Press R to dive back in",
                SCREEN_W / 2.0 - 130.0,
                SCREEN_H / 2.0 + 46.0,
                28.0,
                YELLOW,
            );
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();

        if !game.game_over {
            if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
                game.try_move_player(0, -1);
            } else if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
                game.try_move_player(0, 1);
            } else if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
                game.try_move_player(-1, 0);
            } else if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
                game.try_move_player(1, 0);
            }
        }

        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
