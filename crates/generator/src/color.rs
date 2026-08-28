pub trait ColorExt {
    fn red(&self) -> String;
    fn bold_red(&self) -> String;
    fn green(&self) -> String;
    fn bold_green(&self) -> String;
    fn cyan(&self) -> String;
}

macro_rules! impl_color_ext {
    ($ty:ty) => {
        impl ColorExt for $ty {
            fn red(&self) -> String {
                format!("\x1b[0;31m{}\x1b[m", self)
            }
            fn bold_red(&self) -> String {
                format!("\x1b[1;31m{}\x1b[m", self)
            }
            fn green(&self) -> String {
                format!("\x1b[0;32m{}\x1b[m", self)
            }
            fn bold_green(&self) -> String {
                format!("\x1b[1;32m{}\x1b[m", self)
            }
            fn cyan(&self) -> String {
                format!("\x1b[0;96m{}\x1b[m", self)
            }
        }
    };
}

impl_color_ext!(str);
impl_color_ext!(String);
