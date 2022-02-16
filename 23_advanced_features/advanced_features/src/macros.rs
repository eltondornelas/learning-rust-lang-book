pub fn macros_main() {
    // declarative_macros_with_macro_rules();
    // procedural_macros_generating_code_from_attributes();
    write_custom_derive_macro();
}

fn write_custom_derive_macro() {
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    // impl HelloMacro for Pancakes {
    //     fn hello_macro() {
    //         println!("Hello, Macro! My name is Pancakes!");
    //     }
    // }
    // will get this impl from the macro

    // fn main() {}
    Pancakes::hello_macro();
}

fn procedural_macros_generating_code_from_attributes() {
    // use proc_macro;
    //
    // #[some_attribute]
    // fn some_name(input: TokenStream) -> TokenStream {
    // }
}

fn declarative_macros_with_macro_rules() {
    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
}
