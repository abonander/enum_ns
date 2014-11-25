#![feature(plugin_registrar, if_let)]

extern crate rustc;
extern crate syntax;

use rustc::plugin::Registry;


use syntax::ast::{
    DUMMY_NODE_ID,
    EnumDef, Ident, Item, ItemMod, ItemEnum, Mod, 
    MetaItem, MetaWord, MetaList, PathParameters, PathSegment,
    ViewItem, ViewItemUse, ViewPathList, PathListIdent, Visibility
};
use syntax::ast::Path as AstPath;

use syntax::codemap::{Span, dummy_spanned};
use syntax::ext::base::{ExtCtxt, SyntaxExtension};
use syntax::parse::token::intern;
use syntax::ptr::P;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    
    // Promote variants
    let name = intern("promote_variants");
    let ext = SyntaxExtension::Modifier(box promote_variants);

    reg.register_syntax_extension(name, ext);
}

fn promote_variants(ecx: &mut ExtCtxt, sp: Span, meta: &MetaItem, item: P<Item>) -> P<Item> { 
    let export = match meta.node {
        MetaWord(_) => false,
        MetaList(_, ref list) => 
            if let MetaWord(ref word) = list[0].node { word.get() == "export" } else { false },
        _ => { ecx.span_err(sp, "Invalid invocation of `promote_variants`"); return item; },
    };

    item.map(|mod_| promote_or_export(export, ecx, sp, mod_))
}

fn promote_or_export(export: bool, ecx: &mut ExtCtxt, sp: Span, mut item: Item) -> Item {
    if let ItemMod(ref mut mod_) = item.node {
        create_uses(mod_, export);
    } else {
        ecx.span_err(sp, "`promote_variants` must be invoked on a module!`");
    }             

    item
}

fn create_uses(mod_: &mut Mod, export: bool){
    let views = mod_.items.iter().filter_map(|item|
        if let ItemEnum(ref enum_, _) = item.node {
            Some(create_use(item.ident.clone(), enum_, export))
        } else { None }
    );

    mod_.view_items.extend(views);              
} 

fn create_use(ident: Ident, enum_: &EnumDef, export: bool) -> ViewItem {
    let visible = if export { Visibility::Public } else { Visibility::Inherited };
    
    let mut path_list = Vec::new();

    for variant in enum_.variants.iter() {
        path_list.push(dummy_spanned(PathListIdent { 
            name: variant.node.name.clone(), 
            id: DUMMY_NODE_ID,
        }));
    }

    let path = AstPath {
        span: dummy_spanned(()).span,
        global: false,
        segments: vec![
            PathSegment { 
                identifier: ident, 
                parameters: PathParameters::none(),
            }
        ],
    };

    let view_path = ViewPathList(path, path_list, DUMMY_NODE_ID);
    let view_item = ViewItemUse(P(dummy_spanned(view_path)));

    ViewItem {
        node: view_item,
        attrs: Vec::new(),
        vis: visible,
        span: dummy_spanned(()).span,
    }
}

