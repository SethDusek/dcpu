extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use syn::Attribute;
use syn::MetaItem;
//This is required due to a bit of a complication/bug of quote not properly quoting an Option type
fn quote_cycles(cycles: Option<u64>) -> quote::Tokens {
    cycles.map_or(quote!(None), |val| quote!(Some(#val)))
}
#[proc_macro_derive(InstructionInfo, attributes(opcode, name, cycles, special))]
pub fn derive_instr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    let instr = &ast.ident;
    let attrs = &ast.attrs;
    let (opcode, name, cycles, special) = parse_attrs(attrs);
    if let Some(opcode) = opcode{
        let name = name.unwrap_or(instr.to_string());
        let cycles = quote_cycles(cycles);
        return quote! {
            impl InstructionInfo for #instr {
                const OPCODE: u8 = #opcode;
                const NAME: &'static str = #name;
                const CYCLES: Option<u64> = #cycles;
                const SPECIAL: bool = #special;
            }
        }.parse().unwrap()

    }
    else { panic!("No opcode provided!"); }

}

fn parse_attrs(attrs: &[Attribute]) -> (Option<u8>, Option<String>, Option<u64>, bool) {
    let mut opcode = None;
    let mut name = None;
    let mut cycles = None;
    let mut special = false;
    for val in attrs.iter().map(|x| &x.value) {
        if let &MetaItem::NameValue(ref ident, ref lit) = val {
            if ident == "opcode" {
                match *lit {
                    syn::Lit::Byte(byte) => {opcode = Some(byte)},
                    syn::Lit::Int(int, _) => {opcode = Some(int as u8)}
                    _ => panic!("Invalid opcode")
                }
            }
            else if ident == "name" {
                if let syn::Lit::Str(ref string, _) = *lit {
                    name = Some(string.clone().to_uppercase());
                
                }
            }
            else if ident == "cycles" {
                if let syn::Lit::Int(_cycles, _) = *lit {
                    cycles = Some(_cycles);
                }
            }
            else if ident == "special" {
                if let syn::Lit::Bool(_special) = *lit {
                    special = _special
                }
            }
        }
    }
    (opcode, name, cycles, special)
    
}

