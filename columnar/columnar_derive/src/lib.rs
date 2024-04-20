use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::Attribute;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Type};

// Is 0?
// pub struct Foo {
//    a: Vec<bool>
// }

/// TODO:
/// 4. "Sequence"
///    a. "Chunked"
///    b. nullability
/// 5. columnar-arrow / examples

#[proc_macro_derive(Columnar, attributes(nested))]
pub fn columnar(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //input
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let named_fields = named_fields(&input.data);
    let columnar_name = format_ident!("{name}Ptrs");

    let fields_comma = fields_comma(named_fields);
    let fields_alloc = fields_alloc(named_fields);
    let fields_read = fields_read(named_fields);
    let fields_chunk = fields_chunk(named_fields);
    let fields_index = fields_index(named_fields);

    let columnar_fields = columnar_fields(named_fields);
    let columnar_at = columnar_at(named_fields);
    let expanded = quote! {
       pub struct #columnar_name {
           #columnar_fields
       }

       impl columnar_trait::ArrayRow for #name {
        type Ptr = #columnar_name;
       }

       impl columnar_trait::ArrayPtr for #columnar_name {
           type Row = #name;

           #[inline]
           fn new(len: usize) -> Self {
            unsafe {
                #fields_alloc
                Self {
                    #fields_comma
                }
            }
           }

           #[inline]
            unsafe fn row(&self, idx: usize) -> Self::Row {
                unsafe {
                    #fields_read
                    Self::Row {
                        #fields_comma
                    }
                }
            }

           #[inline]
            unsafe fn chunk<const N: usize>(&self, idx: usize) -> [Self::Row; N] {
                unsafe {
                    #fields_chunk
                    let mut array = std::mem::MaybeUninit::<[std::mem::MaybeUninit<Self::Row>; N]>::uninit().assume_init();
                    (0..N).for_each(|i| {
                        array[i] = std::mem::MaybeUninit::new(Self::Row {
                            #fields_index
                        });
                    });
                    std::mem::MaybeUninit::array_assume_init(array)
                }
                // let idx = idx as isize;
            }
       }
    };

    // Hand the output tokens back to the compiler
    proc_macro::TokenStream::from(expanded)
}

fn named_fields(data: &Data) -> &FieldsNamed {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn fields_comma(fields: &FieldsNamed) -> TokenStream {
    let columnar_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote!(#name,)
    });
    quote!(#(#columnar_fields)*)
}

fn is_nested_field(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("nested"))
}

// fn columnar_fields(fields: &FieldsNamed) -> TokenStream {
//     let columnar_fields = fields.named.iter().map(|f| parse_field(f));
//     quote!(#(#columnar_fields, )*)
// }
fn columnar_fields(fields: &FieldsNamed) -> TokenStream {
    let columnar_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote!(#name: <#ty as columnar_trait::ArrayRow>::Ptr)
        //let ty = parse_field_type(field);
    });
    quote!(#(#columnar_fields, )*)
}

fn fields_alloc(fields: &FieldsNamed) -> TokenStream {
    let columnar_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote!(let #name = <#ty as columnar_trait::ArrayRow>::Ptr::new(len);)
        //quote!(let #name = alloc(std::alloc::Layout::array::<#ty>(len).unwrap()).cast();)
    });
    quote!(#(#columnar_fields)*)
}

fn fields_chunk(fields: &FieldsNamed) -> TokenStream {
    let columnar_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote!(let #name: [#ty; N] = self.#name.chunk(idx);)
        //quote!(let #name: [#ty; N] = read_array(self.#name.offset(idx));)
    });
    quote!(#(#columnar_fields)*)
}

fn fields_read(fields: &FieldsNamed) -> TokenStream {
    let columnar_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote!(let #name = self.#name.row(idx);)
        //quote!(let #name = self.#name.offset(idx).read();)
    });
    quote!(#(#columnar_fields)*)
}

fn fields_index(fields: &FieldsNamed) -> TokenStream {
    let columnar_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote!(#name: #name[i],)
    });
    quote!(#(#columnar_fields)*)
}

fn columnar_at(fields: &FieldsNamed) -> TokenStream {
    let columnar_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote!(#name: self.#name[idx])
    });
    quote!(#(#columnar_fields, )*)
}
