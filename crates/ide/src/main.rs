use itertools::Itertools;

use ide::AdjustmentHints;
use ide::ClosureReturnTypeHints;
use ide::{InlayHintsConfig, LifetimeElisionHints};

use hir::db::DefDatabase;
use ide_db::base_db::fixture::ChangeFixture;
use test_utils::{extract_annotations, RangeOrOffset};

use ide::{Analysis, AnalysisHost, FileId, FilePosition, FileRange};
use syntax::TextRange;

use jwalk::WalkDir;
use std::path::Path;

/// Creates analysis for a single file.
pub fn file(ra_fixture: &str) -> (Analysis, FileId) {
    let mut host = AnalysisHost::default();
    let change_fixture = ChangeFixture::parse(ra_fixture);
    host.raw_database_mut().set_enable_proc_attr_macros(true);
    host.raw_database_mut().apply_change(change_fixture.change);
    (host.analysis(), change_fixture.files[0])
}

/// Creates analysis from a multi-file fixture, returns positions marked with $0.
pub fn position(ra_fixture: &str) -> (Analysis, FilePosition) {
    let mut host = AnalysisHost::default();
    let change_fixture = ChangeFixture::parse(ra_fixture);
    host.raw_database_mut().set_enable_proc_attr_macros(true);
    host.raw_database_mut().apply_change(change_fixture.change);
    let (file_id, range_or_offset) = change_fixture
        .file_position
        .expect("expected a marker ($0)");
    let offset = range_or_offset.expect_offset();
    (host.analysis(), FilePosition { file_id, offset })
}

/// Creates analysis for a single file, returns range marked with a pair of $0.
pub fn range(ra_fixture: &str) -> (Analysis, FileRange) {
    let mut host = AnalysisHost::default();
    let change_fixture = ChangeFixture::parse(ra_fixture);
    host.raw_database_mut().set_enable_proc_attr_macros(true);
    host.raw_database_mut().apply_change(change_fixture.change);
    let (file_id, range_or_offset) = change_fixture
        .file_position
        .expect("expected a marker ($0)");
    let range = range_or_offset.expect_range();
    (host.analysis(), FileRange { file_id, range })
}

/// Creates analysis for a single file, returns range marked with a pair of $0 or a position marked with $0.
pub fn range_or_position(ra_fixture: &str) -> (Analysis, FileId, RangeOrOffset) {
    let mut host = AnalysisHost::default();
    let change_fixture = ChangeFixture::parse(ra_fixture);
    host.raw_database_mut().set_enable_proc_attr_macros(true);
    host.raw_database_mut().apply_change(change_fixture.change);
    let (file_id, range_or_offset) = change_fixture
        .file_position
        .expect("expected a marker ($0)");
    (host.analysis(), file_id, range_or_offset)
}

/// Creates analysis from a multi-file fixture, returns positions marked with $0.
pub fn annotations(ra_fixture: &str) -> (Analysis, FilePosition, Vec<(FileRange, String)>) {
    let mut host = AnalysisHost::default();
    let change_fixture = ChangeFixture::parse(ra_fixture);
    host.raw_database_mut().set_enable_proc_attr_macros(true);
    host.raw_database_mut().apply_change(change_fixture.change);
    let (file_id, range_or_offset) = change_fixture
        .file_position
        .expect("expected a marker ($0)");
    let offset = range_or_offset.expect_offset();

    let annotations = change_fixture
        .files
        .iter()
        .flat_map(|&file_id| {
            let file_text = host.analysis().file_text(file_id).unwrap();
            let annotations = extract_annotations(&file_text);
            annotations
                .into_iter()
                .map(move |(range, data)| (FileRange { file_id, range }, data))
        })
        .collect();
    (
        host.analysis(),
        FilePosition { file_id, offset },
        annotations,
    )
}

/// Creates analysis from a multi-file fixture with annonations without $0
pub fn annotations_without_marker(ra_fixture: &str) -> (Analysis, Vec<(FileRange, String)>) {
    let mut host = AnalysisHost::default();
    let change_fixture = ChangeFixture::parse(ra_fixture);
    host.raw_database_mut().set_enable_proc_attr_macros(true);
    host.raw_database_mut().apply_change(change_fixture.change);

    let annotations = change_fixture
        .files
        .iter()
        .flat_map(|&file_id| {
            let file_text = host.analysis().file_text(file_id).unwrap();
            let annotations = extract_annotations(&file_text);
            annotations
                .into_iter()
                .map(move |(range, data)| (FileRange { file_id, range }, data))
        })
        .collect();
    (host.analysis(), annotations)
}

pub const DISABLED_CONFIG: InlayHintsConfig = InlayHintsConfig {
    render_colons: false,
    type_hints: false,
    parameter_hints: false,
    chaining_hints: false,
    lifetime_elision_hints: LifetimeElisionHints::Never,
    closure_return_type_hints: ClosureReturnTypeHints::Never,
    adjustment_hints: AdjustmentHints::Never,
    binding_mode_hints: false,
    hide_named_constructor_hints: false,
    hide_closure_initialization_hints: false,
    param_names_for_lifetime_elision_hints: false,
    max_length: None,
    closing_brace_hints_min_lines: None,
};

pub const TYPE_HINTS_CONFIG: InlayHintsConfig = InlayHintsConfig {
    type_hints: true,
    hide_named_constructor_hints: true,
    hide_closure_initialization_hints: true,
    closure_return_type_hints: ClosureReturnTypeHints::WithBlock,
    ..DISABLED_CONFIG
};

pub const CHAINING_HINTS_CONFIG: InlayHintsConfig = InlayHintsConfig {
    chaining_hints: true,
    ..DISABLED_CONFIG
};

pub const PARAMETER_HINTS_CONFIG: InlayHintsConfig = InlayHintsConfig {
    parameter_hints: true,
    ..DISABLED_CONFIG
};

pub const BINDING_MODE_HINTS_CONFIG: InlayHintsConfig = InlayHintsConfig {
    binding_mode_hints: true,
    ..DISABLED_CONFIG
};

pub const CLOSING_BRACE_HINTS_CONFIG: InlayHintsConfig = InlayHintsConfig {
    closing_brace_hints_min_lines: Some(2),
    ..DISABLED_CONFIG
};

pub const LIFETIME_HINTS_CONFIG: InlayHintsConfig = InlayHintsConfig {
    lifetime_elision_hints: LifetimeElisionHints::SkipTrivial,
    ..DISABLED_CONFIG
};

pub fn inlay_hints(config: InlayHintsConfig, ra_fixture: &str) -> Vec<(TextRange, String)> {
    let (analysis, file_id) = file(ra_fixture);
    let inlay_hints = analysis.inlay_hints(&config, file_id, None).unwrap();
    inlay_hints
        .into_iter()
        .map(|it| (it.range, it.label.to_string()))
        .sorted_by_key(|(range, _)| range.start())
        .collect::<Vec<_>>()
}

// insert diagnostic code as an markup element around the code causing the diagnostic message
fn markup(source: &str) -> Vec<u8> {
    let type_hints = inlay_hints(TYPE_HINTS_CONFIG, &source);
    let chaining_hints = inlay_hints(CHAINING_HINTS_CONFIG, &source);
    let parameter_hints = inlay_hints(PARAMETER_HINTS_CONFIG, &source);
    let binding_mode_hints = inlay_hints(BINDING_MODE_HINTS_CONFIG, &source);
    let closing_brace_hints = inlay_hints(CLOSING_BRACE_HINTS_CONFIG, &source);
    let lifetime_hints = inlay_hints(LIFETIME_HINTS_CONFIG, &source);
    // println!("type hints: {type_hints:?}");
    // println!("chaining hints: {chaining_hints:?}");
    // println!("parameter hints: {parameter_hints:?}");
    // println!("binding_mode hints: {binding_mode_hints:?}");
    // println!("closing brace hints: {closing_brace_hints:?}");
    // println!("lifetime hints: {lifetime_hints:?}");
    let mut output = Vec::new();
    for (i, c) in source.as_bytes().iter().enumerate() {
        for (range, label) in &type_hints {
            if i == usize::from(range.end()) {
                output.extend(format!(": {}", label).as_bytes());
            }
        }
        for (range, label) in &chaining_hints {
            if i == usize::from(range.end()) {
                output.extend(format!(" // <- {}", label).as_bytes());
            }
        }
        for (range, label) in &parameter_hints {
            if i == usize::from(range.start()) {
                output.extend(format!("{}: ", label).as_bytes());
            }
        }
        for (range, label) in &binding_mode_hints {
            if i == usize::from(range.end()) {
                output.extend(format!(" /* {} */", label).as_bytes());
            }
        }
        for (range, label) in &lifetime_hints {
            if i == usize::from(range.end()) {
                output.extend(format!("{}", label).as_bytes());
            }
        }
        for (range, label) in &closing_brace_hints {
            if i == usize::from(range.end()){
                output.extend(format!(" /* {} */", label).as_bytes());
            }
        }
         output.push(*c);
    }
    output
}

pub fn check(input: &str) {
    let results = markup(input);
    println!("{}", String::from_utf8(results).unwrap());
}

pub fn is_file_with_ext(p: &Path, file_ext: &str) -> bool {
    let ext = match p.extension() {
        Some(e) => e,
        None => return false,
    };
    ext.to_string_lossy() == file_ext
}

fn main() {
    let mut args = std::env::args();
    let mut folder = ".".to_string();
    if args.len() == 2 {
        let arg = args.nth(1).unwrap();
        folder = arg;
    }
    WalkDir::new(folder)
        .sort(true)
        .into_iter()
        .for_each(|entry| {
            if let Ok(e) = entry {
                let p = e.path();
                if !is_file_with_ext(&p, "rs") {
                    return;
                }
                println!("{p:?}");
                if let Ok(s) = std::fs::read_to_string(p) {
                    check(s.as_str());
                }
            }
        });
}
