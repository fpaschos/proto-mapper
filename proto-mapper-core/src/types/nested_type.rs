use proc_macro2::Ident;

#[derive(Debug, Clone)]
pub(crate) enum NestedType {
    Named { name: Ident, args: Vec<NestedType> },
    Unnamed { args: Vec<NestedType> },
}

impl NestedType {
    pub(crate) fn new(ident: Option<&Ident>) -> Self {
        if let Some(ident) = ident {
            Self::Named {
                name: ident.clone(),
                args: Default::default(),
            }
        } else {
            Self::Unnamed {
                args: Default::default(),
            }
        }
    }

    pub(crate) fn name(&self) -> Option<String> {
        match self {
            NestedType::Named { name, .. } => Some(name.to_string()),
            NestedType::Unnamed { .. } => None,
        }
    }

    #[inline]
    pub(crate) fn args(&self) -> &Vec<NestedType> {
        match self {
            NestedType::Named { args, .. } => args,
            NestedType::Unnamed { args, .. } => args,
        }
    }

    #[inline]
    pub(crate) fn args_mut(&mut self) -> &mut Vec<NestedType> {
        match self {
            NestedType::Named { args, .. } => args,
            NestedType::Unnamed { args, .. } => args,
        }
    }

    #[inline]
    pub(crate) fn nest(&mut self, other: NestedType) {
        self.args_mut().push(other);
    }
}

impl ToString for NestedType {
    fn to_string(&self) -> String {
        let mut res = String::new();

        let name = self.name();
        if let Some(name) = &name {
            res.push_str(name);
        }

        if self.args().is_empty() {
            return res;
        }
        if name.is_some() {
            res.push('<');
        } else {
            res.push('(')
        }

        let args: Vec<_> = self.args().iter().map(|arg| arg.to_string()).collect();
        res.push_str(&args.join(","));
        if name.is_some() {
            res.push('>');
        } else {
            res.push(')')
        }

        res
    }
}
