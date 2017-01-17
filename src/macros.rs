macro_rules! texturebase {
    ($base:ident; $($tex:ident $name:expr,)*) => {
        pub struct $base {
            $(pub $tex: Texture,)*
        }

        impl $base {
            fn new(graphics: &Graphics) -> Self {
                $(
                let $tex = Texture::from_png_bytes(graphics, include_bytes!(concat!("../textures/", $name, ".png"))).unwrap();
                )*
                $base {
                    $($tex: $tex,)*
                }
            }
        }
    };
}
