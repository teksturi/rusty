pub fn get_alias_types() -> String {
    include_str!("iec61131-3.types.st").to_string()
}
