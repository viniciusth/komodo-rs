use std::{collections::HashMap, error::Error, path::Path};

#[derive(Default)]
pub struct FileExpand {
    // pair of (path, generated module)
    parsed_files: HashMap<String, String>,
}

impl FileExpand {
    pub fn expand_file<P: AsRef<Path>>(entrypoint: P) -> Result<String, Box<dyn Error + 'static>> {
        let mut this = Self::default();
        let entrypoint_path = entrypoint.as_ref().to_str().unwrap().to_string();
        this.parsed_files
            .insert(entrypoint_path.clone(), String::new());
        this.parse_file(entrypoint, true)?;
        let mut final_file = this.parsed_files.remove(&entrypoint_path).unwrap();
        final_file.push('\n');
        for v in this.parsed_files.into_values() {
            final_file.push_str(&v);
            final_file.push('\n');
        }
        Ok(final_file)
    }

    /// Parses files recursively and saves the paths of the files to expand.
    fn parse_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        root: bool,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let path = path.as_ref().to_str().unwrap().to_string();
        let f = std::fs::read_to_string(path.clone()).or_else(|_| {
            // it could be a module file, swap for mod.rs
            std::fs::read_to_string(path.replace(".rs", "/mod.rs"))
        })?;
        let mut file_without_uses = String::new();
        let mut use_stmts = vec![];
        let mut adding = false;
        for line in f.lines() {
            if line.starts_with("use") {
                assert!(adding == false, "use statement in use statement");
                use_stmts.push(String::new());
                adding = true;
            }
            // skip module declarations
            else if (line.starts_with("mod") || line.starts_with("pub mod"))
                && line.ends_with(';')
            {
                continue;
            }
            if adding {
                use_stmts.last_mut().unwrap().push_str(line);
                if line.trim().ends_with(';') {
                    adding = false;
                }
            } else {
                file_without_uses.push_str(line);
                file_without_uses.push('\n');
            }
        }

        let mut all_imports = vec![];

        for mut line in use_stmts {
            if line.starts_with("use crate::") {
                line.retain(|c| !c.is_whitespace());
                let (file_paths, imports) = parse_use(&line, root);
                all_imports.extend(imports);
                for file_path in file_paths {
                    if !self.parsed_files.contains_key(&file_path) {
                        self.parsed_files.insert(file_path.clone(), String::new());
                        self.parse_file(file_path, false)?;
                    }
                }
            }
            // we don't want to skip std imports
            else if line.starts_with("use") {
                all_imports.push(line);
            }
        }

        let generated = format!("{}{}", all_imports.join("\n"), file_without_uses);

        let generated = if root {
            generated
        } else {
            format!(
                "mod {} {{\n{}\n}}",
                path.rsplit_once('/').unwrap().1.replace(".rs", ""),
                generated
                    .lines()
                    .map(|l| format!("    {}", l))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };
        self.parsed_files.insert(path, generated);
        Ok(())
    }
}

/// Parses a statement like
/// `usecrate::library::structures::{sparse_table::SparseTable,segment_tree::SegmentTree};`
/// into a vector of file paths and generated imports.
///
/// # Examples
/// ```
/// use komodo::expand::parse_use;
///
/// let line = "usecrate::library::{structures::{sparse_table::SparseTable,segment_tree::SegmentTree},io::{read,write}};";
/// let (paths, imports) = parse_use(line, false);
/// assert_eq!(paths.len(), 3);
/// assert_eq!(paths[0], "src/library/structures/sparse_table.rs");
/// assert_eq!(paths[1], "src/library/structures/segment_tree.rs");
/// assert_eq!(paths[2], "src/library/io.rs");
///
/// assert_eq!(imports.len(), 4);
/// assert_eq!(imports[0], "use super::sparse_table::SparseTable;");
/// assert_eq!(imports[1], "use super::segment_tree::SegmentTree;");
/// assert_eq!(imports[2], "use super::io::read;");
/// assert_eq!(imports[3], "use super::io::write;");
/// ```
pub fn parse_use(line: &str, root: bool) -> (Vec<String>, Vec<String>) {
    let mut cur_path = String::from("src/");
    let mut len_stk = vec![];
    let mut paths = vec![];

    let cur = line.split_once("::").unwrap().1;
    let mut last_char = '$';
    for c in cur.chars() {
        match c {
            ':' => {
                if !cur_path.ends_with('/') {
                    cur_path.push('/');
                }
            }
            '{' => {
                len_stk.push(cur_path.len());
            }
            ',' | ';' | '}' => {
                if last_char != '}' && last_char != ',' {
                    paths.push(cur_path.clone());
                }
                cur_path.truncate(len_stk.last().copied().unwrap_or_default());
                if c == '}' {
                    len_stk.pop();
                }
            }
            _ => {
                cur_path.push(c);
            }
        }
        last_char = c;
    }

    // Paths are formatted like `src/library/structures/sparse_table/SparseTable`, reformat with .rs
    let mut formatted_paths = vec![];
    let mut imports = vec![];
    for path in paths {
        let (lpath, structure) = path.rsplit_once('/').unwrap();
        let formatted_path = format!("{lpath}.rs");
        if formatted_paths
            .last()
            .map_or(true, |f| f != &formatted_path)
        {
            formatted_paths.push(formatted_path);
        }

        let module_name = lpath.rsplit_once('/').unwrap().1;

        // since all modules will be at the top level, we can use super::.
        // if we are at the root, we don't need to use super::.
        if root {
            imports.push(format!("use {}::{structure};", module_name));
        } else {
            imports.push(format!("use super::{}::{structure};", module_name));
        }
    }

    (formatted_paths, imports)
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_parse_use_simple() {
        let line = "usecrate::library::structures::sparse_table::SparseTable;";
        let (paths, imports) = super::parse_use(line, true);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "src/library/structures/sparse_table.rs");

        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0], "use sparse_table::SparseTable;");
    }

    #[test]
    fn should_parse_use_small() {
        let line = "usecrate::library::test;";
        let (paths, imports) = super::parse_use(line, true);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], "src/library.rs");

        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0], "use library::test;");
    }

    #[test]
    fn trailing_comma() {
        let line = "usecrate::library::{geometry::{convex_hull::convex_hull,point::Point},io::create_io,};";
        let (paths, _) = super::parse_use(line, true);
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0], "src/library/geometry/convex_hull.rs");
        assert_eq!(paths[1], "src/library/geometry/point.rs");
        assert_eq!(paths[2], "src/library/io.rs");
    }
}
