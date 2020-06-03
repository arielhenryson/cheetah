pub fn get_above_scope_path(path: String) -> Option<String> {
    return match path.rfind(".") {
        Some(v) => {
            let new_scope_pos = &path[..v];

            Some(
                String::from(new_scope_pos)
            )
        },
        None => None
    }
}