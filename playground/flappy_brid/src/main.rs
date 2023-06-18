use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

/// # State
/// 用來表示遊戲狀態
struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // 清理屏幕
        ctx.cls();

        // 遊戲屏幕座標系從左上角開始，x 往右遞增，y 往下遞增
        // print(x, y, "text") 顯示文字
        ctx.print(1, 1, "Hello Bracket Terminal!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
