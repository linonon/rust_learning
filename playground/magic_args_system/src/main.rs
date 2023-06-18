#[derive(Clone)]
pub struct RequestResolver {
    link: String,
    method: &'static str,
    args: Vec<String>,
}

impl RequestResolver {
    pub fn new(link: &str) -> Self {
        Self {
            link: link.to_string(),
            method: "GET",
            args: vec![],
        }
    }
    pub fn with_args(mut self, args: &[&'static str]) -> Self {
        self.args = args.iter().map(|s| s.to_string()).collect();
        self
    }
}

pub struct Method(pub &'static str);
pub struct Args(pub Vec<String>);

// 首先定義這兩個提取器以及標記用的 Trait
pub trait FromContext {
    type Context;
    fn from_context(context: &Self::Context) -> Self;
}

// 實現提取器模式
impl FromContext for Args {
    type Context = RequestResolver;

    fn from_context(context: &Self::Context) -> Self {
        Args(context.args.clone())
    }
}

// 實現提取器模式
impl FromContext for Method {
    type Context = RequestResolver;

    fn from_context(context: &RequestResolver) -> Self {
        Method(context.method.clone())
    }
}

// 實現提取器模式
impl FromContext for String {
    type Context = RequestResolver;

    fn from_context(context: &RequestResolver) -> Self {
        context.link.clone()
    }
}

// 用 trait 根據 context 派發
pub trait Handle<T, C> {
    fn apply(self, context: &C);
}

impl<C, F, T1> Handle<T1, C> for F
where
    F: Fn(T1),
    T1: FromContext<Context = C>,
{
    fn apply(self, context: &C) {
        (self)(T1::from_context(context));
    }
}

impl<C, F, T1, T2> Handle<(T1, T2), C> for F
where
    F: Fn(T1, T2),
    T1: FromContext<Context = C>,
    T2: FromContext<Context = C>,
{
    fn apply(self, context: &C) {
        (self)(T1::from_context(context), T2::from_context(context));
    }
}

fn print_link(link: String, Method(method): Method) {
    println!("HTTP: {method} {link}")
}

fn main() {}
