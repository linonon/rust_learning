use std::ops::Add;

fn main() {
    println!("Hello, ch19");
}

/// ## Unsafe 超能力
/// - 使用 unsafe 關鍵字來切換到unsafe Rust, 開啟一個塊, 裡面放著 unsafe 代碼
/// - Unsafe Rust 裡面可執行 4 個動作(unsafe 超能力)
///   - 解引用原是指針
///   - 調用 unsafe 函數或方法
///   - 訪問或修改可變的靜態變量
///   - 實現 unsafe trait
/// - 注意:
///   - unsafe 並沒有關閉借用檢查器或者其他安全檢查
///   - 任何內存安全相關的錯誤必須留在 unsafe 塊裡
///   - 盡可能隔離 unsafe 代碼, 最好將其封裝在安全的抽象裡, 提供安全的 API
/// ## 解引用原始指針
/// - 原始指針
///   - 可變的: *mut T
///   - 不可變的: *const T. 意味著指針在解引用以後不能直接對其進行賦值
///   - 注意: 這裡的 `*` 不是解引用符號 , 它是類型名稱的一部分
/// - 與引用不同, 原始指針:
///   - 允許通過同時具有不可變和可變指針或多個指向同一位置的可變指針來忽略借用規則
///   - 無法保證能指向合理的內存
///   - 允許為 null
///   - 不實現任何自動清理
/// - 放棄保證的安全, 換取更好的性能 / 與其他語言或者是硬件接口的能力
#[test]
fn unsafe_super() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("r: {}", *r);
    }
}

#[test]
/// ## 調用 unsafe 函數或方法
/// - unsafe 函數或方法: 在定義前加上了 unsafe 關鍵字
///   - 調用前需手動滿足一些條件 (主要看文檔) , 因為 Rust 無法對這些條件進行驗證
///   - 需要在 unsafe 塊裡調用
/// test
fn unsafe_fn() {
    unsafe fn dangerous() {}

    unsafe { dangerous() }

    // dangerous()
}

#[test]
fn unsafe_example_slice() {
    let mut x = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let (a, b) = x.split_at_mut(3);
    println!("a: {:?}", a); // a: [1, 2, 3]
    println!("b: {:?}", b); // b: [4, 5, 6, 7, 8]
}

#[test]
fn extern_for_c() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("abs -34 is : {}", abs(-34));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just call a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe { COUNTER += inc }
}

#[test]
fn static_mut_counter() {
    add_to_count(3);
    unsafe {
        println!("Counter: {}", COUNTER);
    }
}

#[test]
fn yun_suan_fu_reload() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    println!("a + b = {:?}", Point { x: 1, y: 2 } + Point { x: 2, y: 3 });
}

#[test]
fn millimeter_add_meter() {
    #[derive(Debug, PartialEq)]
    struct Millimeter(u32);
    struct Meter(u32);

    impl Add<Meter> for Millimeter {
        type Output = Millimeter;

        fn add(self, rhs: Meter) -> Self::Output {
            Millimeter(self.0 + (rhs.0 * 1000))
        }
    }

    println!("a + b = {:?}", Millimeter(1) + Meter(1));
}
