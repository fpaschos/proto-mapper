use crate::types::NestedType;
use syn::fold::Fold;
use syn::{Path, PathArguments};

#[derive(Debug, Default)]
pub(crate) struct TypeScanner {
    stack: Vec<NestedType>,
}

impl TypeScanner {
    pub(crate) fn scan(&mut self, p: Path) -> NestedType {
        self.stack.clear();
        self.fold_path(p);
        debug_assert_eq!(
            self.stack.len(),
            1,
            "Invalid TypeScanner::scan result stack contains more than one root elements"
        );
        self.stack.pop().unwrap()
    }

    fn stack_un_nest_top(&mut self) -> bool {
        debug_assert!(
            !self.stack.is_empty(),
            "Invalid TypeScanner::stack_nest_top stack is empty"
        );

        let top = self.stack.last_mut().unwrap();
        if let Some(nested_last) = top.args_mut().pop() {
            self.stack.push(nested_last);
            true
        } else {
            false
        }
    }

    fn stack_nest_top(&mut self) {
        debug_assert!(
            self.stack.len() > 1,
            "Invalid TypeScanner::stack_nest_top stack size < 2"
        );
        let top = self.stack.pop().unwrap();
        let pre_top = self.stack.last_mut().unwrap();
        pre_top.nest(top);
    }
}

// TODO try use fold_type_path to fold more generic types
impl Fold for TypeScanner {
    fn fold_path(&mut self, p: Path) -> Path {
        let last = p.segments.last().map(|s| &s.ident);
        let ty = NestedType::new(last);

        if self.stack.is_empty() {
            self.stack.push(ty);
        } else if let Some(last) = self.stack.last_mut() {
            last.nest(ty);
        }

        syn::fold::fold_path(self, p)
    }

    fn fold_path_arguments(&mut self, pa: PathArguments) -> PathArguments {
        let un_nested = self.stack_un_nest_top();
        let inner = syn::fold::fold_path_arguments(self, pa);
        if un_nested {
            self.stack_nest_top();
        }
        inner
    }
}
