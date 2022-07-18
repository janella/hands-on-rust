use crate::prelude::*;

pub fn display(
    title: &str,
    map: &Map,
    player_start: &Point,
    amulet_start: &Point,
    monster_spawns: &Vec<Point>,
    item_spawns: &Vec<(Point, &Name)>,
) {
    use colored::*;
    let mut output = vec!['.'; NUM_TILES];

    map.tiles.iter().enumerate().for_each(|(idx, t)| match *t {
        TileType::Floor => output[idx] = '.',
        TileType::Wall => output[idx] = '#',
    });

    output[map.point2d_to_index(*player_start)] = '@';
    output[map.point2d_to_index(*amulet_start)] = 'A';
    for p in monster_spawns.iter() {
        output[map.point2d_to_index(*p)] = 'M';
    }
    for p in item_spawns.iter() {
        println!("found {}", p.1 .0);
        match p.1 .0.as_ref() {
            // todo fix this - this isn't picking up the names
            "Health Potion" => output[map.point2d_to_index(p.0)] = '!',
            "Dungeon Map" => output[map.point2d_to_index(p.0)] = '{',
            _ => {}
        }
    }

    print!("\x1B[2J"); // CLS!
    println!(
        "----------------------\n{}\n----------------------",
        title.bright_yellow()
    );
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            match output[get_map_idx(x, y)] {
                '#' => print!("{}", "#".bright_green()),
                '@' => print!("{}", "@".bright_yellow()),
                'M' => print!("{}", "M".bright_red()),
                'A' => print!("{}", "A".bright_magenta()),
                '!' => print!("{}", "!".bright_purple()),
                '{' => print!("{}", "{".bright_purple()),
                _ => print!("{}", ".".truecolor(64, 64, 64)),
            }
        }
        println!("");
    }
}
