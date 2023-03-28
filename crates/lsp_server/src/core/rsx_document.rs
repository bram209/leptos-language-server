use leptosfmt_formatter::collect_macros_in_file;
use syn::Macro;

pub struct RsxDocument {
    pub macro_callsites: Vec<Macro>,
}

impl RsxDocument {
    pub fn parse(file_source: &str) -> Result<Self, syn::Error> {
        let file = syn::parse_file(file_source)?;
        let macro_callsites = collect_macros_in_file(&file)
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();
        Ok(Self { macro_callsites })
    }
}
