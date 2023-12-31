use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::fs;

use swc_core::ecma::parser::{lexer::Lexer, Parser, Syntax, StringInput, TsConfig};
use swc_core::ecma::ast::{Module, EsVersion, ClassMember, ClassProp, PropName, Ident, Expr, Lit, Str};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_core::ecma::codegen::{Config, Emitter, text_writer::JsWriter};
use swc_core::common::{SourceMap/*, FileName*/, DUMMY_SP as DUMMY_SPAN};
use swc_core::common::errors::{ColorConfig, Handler};
use swc_core::common::sync::Lrc;

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
  // Implement necessary visit_mut_* methods for actual custom transform.
  // A comprehensive list of possible visitor methods can be found here:
  // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

  fn visit_mut_class_decl(&mut self, node: &mut swc_core::ecma::ast::ClassDecl) {
    node.class.body.push(ClassMember::ClassProp(ClassProp {
      span: DUMMY_SPAN,
      key: PropName::Ident(Ident {
        span: DUMMY_SPAN,
        sym: "__name__".into(),
        optional: false,
      }),
      value: Some(Box::new(Expr::Lit(Lit::Str(Str {
        span: DUMMY_SPAN,
        value: node.ident.sym.clone(),
        raw: None,
      })))),
      type_ann: None,
      is_static: true,
      decorators: Vec::new(),
      accessibility: None,
      is_abstract: false,
      is_optional: false,
      is_override: false,
      readonly: false,
      declare: false,
      definite: false,
    }));
    println!("class: {}", node.ident.sym);
  }
}

use clap::Parser as ArgsParser;

#[derive(ArgsParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Vec<String>,
}

fn traverse(path: PathBuf) -> std::io::Result<Vec<PathBuf>> {
  let mut queue: VecDeque<PathBuf> = VecDeque::new();
  let mut paths: Vec<PathBuf> = Vec::new();

  if path.is_dir() {
    queue.push_back(path);
  } else {
    paths.push(path);
  }

  while let Some(dir) = queue.pop_front() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        queue.push_back(path);
      } else {
        paths.push(path);
      }
    }
  }

  Ok(paths)
}

fn codegen(cm: Lrc<SourceMap>, module: &Module) -> String {
  let mut buf = vec![];
  let wr = JsWriter::new(cm.clone(), "\n", &mut buf, None);
  let mut emitter = Emitter {
    cfg: Config {
      target: EsVersion::Es2022,
      minify: false,
      ascii_only: false,
      omit_last_semi: false,
    },
    cm: cm.clone(),
    comments: None,
    wr,
  };
  emitter.emit_module(&module).unwrap();
  let code = String::from_utf8(buf).unwrap();
  code
}

pub fn main() -> std::io::Result<()> {
  let args = Args::parse();

  let cm: Lrc<SourceMap> = Default::default();
  let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

  for path in args.path {
    for path in traverse(Path::new(&path).to_path_buf())? {
      if let Some(ext) = path.extension() {
        if ext == "ts" && path.with_extension("").extension().is_none() {
          println!("{}", path.display());
          let code = parse(&path, &cm, &handler).map(|mut module| {
            let mut visitor = TransformVisitor {};
            module.visit_mut_with(&mut visitor);
            module
          }).map(|module| {
            codegen(cm.clone(), &module)
          }).unwrap();
          println!("{}", code);
        }
      }
    }
  }

  Ok(())
}

fn parse(path: &Path, cm: &Lrc<SourceMap>, handler: &Handler) -> Option<Module> {
  let fm = cm.load_file(path).expect("failed to load file");
  /*
  let fm = cm.new_source_file(
      FileName::Custom("test.js".into()),
      "function foo(): number {}".into(),
  );
  */
  let lexer = Lexer::new(
      Syntax::Typescript(TsConfig {
        tsx: true,
        decorators: true,
        ..Default::default()
      }),
      EsVersion::Es2022,
      StringInput::from(&*fm),
      None,
  );

  let mut parser = Parser::new_from(lexer);

  for e in parser.take_errors() {
      e.into_diagnostic(&handler).emit();
  }

  match parser.parse_module() {
    Ok(module) => Some(module),
    Err(error) => {
      error.into_diagnostic(&handler).emit();
      None
    },
  }

/*
  let module = parser
      .parse_module()
      .map_err(|e| {
          // Unrecoverable fatal error occurred
          e.into_diagnostic(&handler).emit();
          e
      });

  Ok(module)
*/
}
