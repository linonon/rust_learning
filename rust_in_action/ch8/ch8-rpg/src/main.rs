use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}
// 定义一个 trait Enchanter，该 trait 继承 std::fmt::Debug trait
trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64; // 定义一个方法，返回一个 f64 类型的值，表示附魔师的能力

    fn enchant(&self, thing: &mut Thing) {
        // 定义一个方法，接受一个 mutable reference to Thing 类型的参数
        let probability_of_success = self.competency(); // 调用 competency 方法获取附魔师的能力值
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success); // 使用 RNG 实例生成一个随机布尔值，表示附魔是否成功

        print!("{:?} mutters incoherently. ", self); // 打印附魔师的名字和动作
        if spell_is_successful {
            // 如果附魔成功
            println!("The {:?} glows brightly.", thing); // 打印附魔成功的消息
        } else {
            // 如果附魔失败
            println!(
                "The {:?} fizzes, the turns into a worthless trinkey.",
                thing
            );
            *thing = Thing::Trinket; // 将 Thing 类型的参数设置为 Trinket
        }
    }
}

// 为 Dwarf、Elf 和 Human 分别实现 Enchanter trait
impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}

// 主函数
fn main() {
    let mut it = Thing::Sword; // 创建一个 Thing 类型的变量

    let d = Dwarf {}; // 创建一个 Dwarf 类型的变量
    let e = Elf {}; // 创建一个 Elf 类型的变量
    let h = Human {}; // 创建一个 Human 类型的变量

    let party: Vec<&dyn Enchanter> = vec![&d, &e, &h]; // 创建一个 vector，包含三个实现了 Enchanter trait 的对象
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap(); // 使用 RNG 实例从 vector 中随机选择一个元素，
                                                                      // 返回一个 Option<&dyn Enchanter> 类型的对象，调用 unwrap()
                                                                      // 方法将其解包成 &dyn Enchanter 类型

    spellcaster.enchant(&mut it); // 调用选择的对象的 enchant 方法，传入 it 变量的 mutable reference
}
