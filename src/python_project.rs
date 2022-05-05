use lazy_static::lazy_static;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::path::PathBuf;

lazy_static! {
    static ref PYPROJECT_TOML_TEMPLATE: String = r#"[tool.poetry]
name = "__PROJECT_NAME__"
version = "0.1.0"
description = ""
authors = ["__AUTHOR__"]

[tool.poetry.dependencies]
python = ">=3.8,<4"

[tool.poetry.dev-dependencies]
black = "^22.3.0"
flake8 = "^4.0.1"
isort = "^5.10.1"
mypy = "^0.950"
pytest = "^6.2.1"
ipdb = "^0.13.4"

[tool.black]
line-length = 120
# target-version = ["py38"]

[tool.isort]
default_section = "THIRDPARTY"
force_grid_wrap = 0
force_single_line = false
include_trailing_comma = true
known_first_party = ["__PROJECT_NAME__"]
line_length = 120
multi_line_output = 3
use_parentheses = true
"#
    .to_owned();
    static ref SETUP_CFG: String = r#"[flake8]
ignore = E203,E231,E501,W503
max-line-length = 120
select = B,B950,C,E,F,W
exclude = 
    __pycache__/
    .venv/
    .mypy_cache/

[mypy]
check_untyped_defs = True
disallow_any_decorated = False
disallow_any_generics = True
disallow_any_unimported = False
disallow_incomplete_defs = True
disallow_subclassing_any = True
disallow_untyped_calls = True
disallow_untyped_decorators = False
disallow_untyped_defs = True
follow_imports = normal
ignore_errors = False
no_implicit_optional = True
show_error_codes = True
strict_equality = True
strict_optional = True
warn_redundant_casts = True
warn_return_any = True
warn_unreachable = True
warn_unused_configs = True
warn_unused_ignores = True
"#
    .to_owned();
    static ref LINT_BASH: String = r#"#!/bin/bash

poetry run black .
poetry run isort .
poetry run flake8 .
poetry run mypy .
"#
    .to_owned();
}

pub fn setup(target_dir: &Path, author: &str) -> Result<(), io::Error> {
    if !target_dir.exists() {
        fs::create_dir(&target_dir)?
    }

    assert!(target_dir.is_dir());
    let project_name = target_dir.file_name().unwrap().to_str().unwrap();

    setup_module_dir(target_dir, project_name)?;

    setup_pyproject_toml(target_dir, project_name, author)?;
    make_file(&target_dir.join("setup.cfg"), &SETUP_CFG)?;
    make_executable_file(&target_dir.join("lint.bash"), &LINT_BASH)?;

    // make .gitignore
    let resp = reqwest::blocking::get(
        "https://raw.githubusercontent.com/github/gitignore/main/Python.gitignore",
    )
    .unwrap() // FIXME
    .text()
    .unwrap(); // FIXME
    make_file(&target_dir.join(".gitignore"), &resp)?;

    Ok(())
}

fn setup_pyproject_toml(
    target_dir: &Path,
    project_name: &str,
    author: &str,
) -> Result<(), io::Error> {
    let body = PYPROJECT_TOML_TEMPLATE
        .replace("__PROJECT_NAME__", project_name)
        .replace("__AUTHOR__", author);

    let pyproject = target_dir.join("pyproject.toml");
    make_file(&pyproject, &body)
}

fn make_file(path: &PathBuf, body: &String) -> Result<(), io::Error> {
    if path.exists() {
        return Ok(());
    }

    let file = OpenOptions::new().write(true).create(true).open(path)?;

    let mut f = io::BufWriter::new(file);
    write!(f, "{}", body)?;

    Ok(())
}

fn make_executable_file(path: &PathBuf, body: &String) -> Result<(), io::Error> {
    if path.exists() {
        return Ok(());
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .mode(0o770)
        .open(path)?;

    let mut f = io::BufWriter::new(file);
    write!(f, "{}", body)?;

    Ok(())
}

fn setup_module_dir(target_dir: &Path, project_name: &str) -> Result<(), io::Error> {
    let module_dir = target_dir.join(project_name);
    if !module_dir.exists() {
        fs::create_dir(&module_dir)?;
    }

    let empty_files = ["__init__.py", "py.typed"];

    for file_name in empty_files.iter() {
        let fname = module_dir.join(file_name);
        if fname.exists() {
            continue;
        }
        let f = File::create(fname)?;
        let mut f = io::BufWriter::new(f);
        f.write(b"")?;
    }

    Ok(())
}
