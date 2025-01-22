
// Provides the colors variables
use tcod::colors::*;
// Provides the console variables
use tcod::console::*;


// Coonstantes para definir o tamnho da tela e fps
const LARGURA_TELA: i32 = 80;
const ALTURA_TELA: i32 = 50;
const LIMITE_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
}

// The struct Object represents a generic object in the game
struct Object {
    x: i32,
    y: i32,
    caractere: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y:i32, caractere: char, color: Color) -> Self {
        Object { x, y, caractere, color }
    }

    // Move the object by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    //Set the color and then draw the object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.caractere, BackgroundFlag::None);
    }
}


// Function to handle the keys pressed by the player
fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {

    // Provides the Key struct and KeyCode enum
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        
        // Comand in the game to go fullscreen and to exit the game
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        },
        Key {code: Escape, ..} => return true,

        // Commands to move the player
        Key {code: Up, ..} => player.move_by(0, -1),
        Key {code: Down, ..} => player.move_by(0, 1),
        Key {code: Left, ..} => player.move_by(-1, 0),
        Key {code: Right, ..} => player.move_by(1, 0),
        _ => {}
    }

    false
}

fn main() {
    tcod::system::set_fps(LIMITE_FPS);

    // Configuration of the game window stored in the root console variable
    let root = Root::initializer()
        .font("arial12x12.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(LARGURA_TELA, ALTURA_TELA)
        .title("Jogo em Rust")
        .init();

    let con = Offscreen::new(LARGURA_TELA, ALTURA_TELA);

    // Inicialize an instace of the struct Tcod using the root console that was just configured
    let mut tcod = Tcod { root, con };

    // Create the player Object
    let player = Object::new(LARGURA_TELA / 2, ALTURA_TELA / 2, '@', WHITE);

    // Create an NPC Object
    let npc = Object::new(LARGURA_TELA / 2 - 5, ALTURA_TELA / 2, '@', YELLOW);

    // Create a list of the existing objects
    let mut objects = [player, npc];


    // Main loop of the game
    while !tcod.root.window_closed() {

        tcod.con.clear();

        // Draw the border of the screan
        for x in 0..LARGURA_TELA {
            tcod.con.put_char(x, 1, '°', BackgroundFlag::None);
            tcod.con.put_char(x, ALTURA_TELA - 2, '°', BackgroundFlag::None);
        }

        // Draw all objects in the list of objects
        for object in &objects {
            object.draw(&mut tcod.con);
        }

        // Blit the contents of the "con" console to the root console
        blit(
            &tcod.con,
            (0, 0),
            (LARGURA_TELA, ALTURA_TELA),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0
        );
        tcod.root.flush(); // Draw everyting on the window at the same time
        tcod.root.wait_for_keypress(true); // Wait for a keypress to continue

        // Handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }
}
