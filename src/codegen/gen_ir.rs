use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use cirru_edn::{format, Edn, EdnListView};

use crate::primes::{Calcit, CalcitItems, ImportRule, SymbolResolved::*};
use crate::program;

#[derive(Debug)]
struct IrDataFile {
  defs: HashMap<Arc<str>, Edn>,
}

impl From<IrDataFile> for Edn {
  fn from(data: IrDataFile) -> Self {
    Edn::map_from_iter([(Edn::tag("defs"), data.defs.into())])
  }
}

#[derive(Debug, Clone)]
struct IrDataConfig {
  init_fn: String,
  reload_fn: String,
}

impl From<IrDataConfig> for Edn {
  fn from(x: IrDataConfig) -> Edn {
    Edn::map_from_iter([(Edn::tag("init-fn"), x.init_fn.into()), (Edn::tag("reload-fn"), x.reload_fn.into())])
  }
}

#[derive(Debug)]
pub struct IrData {
  configs: IrDataConfig,
  files: HashMap<Arc<str>, IrDataFile>,
}

impl From<IrData> for Edn {
  fn from(x: IrData) -> Edn {
    Edn::map_from_iter([(Edn::tag("configs"), x.configs.into()), (Edn::tag("files"), x.files.into())])
  }
}

pub fn emit_ir(init_fn: &str, reload_fn: &str, emit_path: &str) -> Result<(), String> {
  let program_data = program::clone_evaled_program();

  let mut files: HashMap<Arc<str>, IrDataFile> = HashMap::new();

  for (ns, file_info) in program_data {
    let mut defs: HashMap<Arc<str>, Edn> = HashMap::new();
    for (def, code) in file_info {
      defs.insert(def, dump_code(&code));
    }

    let file = IrDataFile { defs };
    files.insert(ns, file);
  }

  let data = IrData {
    configs: IrDataConfig {
      init_fn: init_fn.to_owned(),
      reload_fn: reload_fn.to_owned(),
    },
    files,
  };

  let content = match format(&data.into(), true) {
    Ok(v) => v,
    Err(e) => return Err(format!("failed {e}")),
  };

  let code_emit_path = Path::new(emit_path);
  if !code_emit_path.exists() {
    let _ = fs::create_dir(code_emit_path);
  }

  let js_file_path = code_emit_path.join("program-ir.cirru");
  let _ = fs::write(&js_file_path, content);
  println!("wrote to: {}", js_file_path.to_str().expect("extract path"));

  Ok(())
}

pub(crate) fn dump_code(code: &Calcit) -> Edn {
  match code {
    Calcit::Number(n) => Edn::Number(*n),
    Calcit::Nil => Edn::Nil,
    Calcit::Str(s) => Edn::Str((**s).into()),
    Calcit::Bool(b) => Edn::Bool(b.to_owned()),
    Calcit::Tag(s) => Edn::Tag(s.to_owned()),
    Calcit::Symbol {
      sym,
      ns,
      at_def,
      resolved,
      location,
    } => {
      let resolved = match resolved {
        Some(resolved) => match &**resolved {
          ResolvedDef {
            ns: r_ns,
            def: r_def,
            rule: import_rule,
          } => Edn::map_from_iter([
            (Edn::tag("kind"), Edn::tag("def")),
            (Edn::tag("ns"), Edn::Str((**r_ns).into())),
            (Edn::tag("at-def"), Edn::Str((**at_def).into())),
            (Edn::tag("def"), Edn::Str((**r_def).into())),
            (
              Edn::tag("rule"),
              match import_rule.to_owned().map(|x| (*x).to_owned()) {
                Some(ImportRule::NsAs(_n)) => Edn::tag("ns"),
                Some(ImportRule::NsDefault(_n)) => Edn::tag("default"),
                Some(ImportRule::NsReferDef(_ns, _def)) => Edn::tag("def"),
                None => Edn::Nil,
              },
            ),
            (
              Edn::tag("location"),
              match location {
                Some(xs) => (**xs).to_owned().into(),
                None => Edn::Nil,
              },
            ),
          ]),
          ResolvedLocal => Edn::map_from_iter([("kind".into(), Edn::tag("local"))]),
          ResolvedRaw => Edn::map_from_iter([("kind".into(), Edn::tag("raw"))]),
        },
        None => Edn::map_from_iter([("kind".into(), Edn::Nil)]),
      };

      Edn::map_from_iter([
        (Edn::tag("kind"), Edn::tag("symbol")),
        (Edn::tag("val"), Edn::Str((**sym).into())),
        (Edn::tag("at-def"), Edn::Str((**at_def).into())),
        (Edn::tag("ns"), Edn::Str((**ns).into())),
        (Edn::tag("resolved"), resolved),
      ])
    }

    Calcit::Fn {
      name, def_ns, args, body, ..
    } => {
      Edn::map_from_iter([
        (Edn::tag("kind"), Edn::tag("fn")),
        (Edn::tag("name"), Edn::Str((**name).into())),
        (Edn::tag("ns"), Edn::Str((**def_ns).into())),
        (Edn::tag("args"), dump_args_code(args)), // TODO
        (Edn::tag("code"), dump_items_code(body)),
      ])
    }
    Calcit::Macro {
      name, def_ns, args, body, ..
    } => {
      Edn::map_from_iter([
        (Edn::tag("kind"), Edn::tag("macro")),
        (Edn::tag("name"), Edn::Str((**name).into())),
        (Edn::tag("ns"), Edn::Str((**def_ns).into())),
        (Edn::tag("args"), dump_args_code(args)), // TODO
        (Edn::tag("code"), dump_items_code(body)),
      ])
    }
    Calcit::Proc(name) => Edn::map_from_iter([
      (Edn::tag("kind"), Edn::tag("proc")),
      (Edn::tag("name"), Edn::Str(name.to_string().into())),
      (Edn::tag("builtin"), Edn::Bool(true)),
    ]),
    Calcit::Syntax(name, _ns) => Edn::map_from_iter([
      (Edn::tag("kind"), Edn::tag("syntax")),
      (Edn::tag("name"), Edn::Str((name.to_string()).into())),
    ]),
    Calcit::Thunk(code, _) => dump_code(code),
    Calcit::List(xs) => {
      let mut ys: Vec<Edn> = Vec::with_capacity(xs.len());
      for x in xs {
        ys.push(dump_code(x));
      }
      Edn::List(ys)
    }
    Calcit::Method(method, kind) => Edn::map_from_iter([
      (Edn::tag("kind"), Edn::tag("method")),
      (Edn::tag("behavior"), Edn::Str((kind.to_string()).into())),
      (Edn::tag("method"), Edn::Str((method.to_string()).into())),
    ]),
    Calcit::RawCode(_, code) => Edn::map_from_iter([
      (Edn::tag("kind"), Edn::tag("raw-code")),
      (Edn::tag("code"), Edn::Str((code.to_string()).into())),
    ]),
    Calcit::CirruQuote(code) => Edn::map_from_iter([(Edn::tag("kind"), Edn::tag("cirru-quote")), (Edn::tag("code"), code.into())]),
    a => unreachable!("invalid data for generating code: {:?}", a),
  }
}

fn dump_items_code(xs: &CalcitItems) -> Edn {
  let mut ys = EdnListView::default();
  for x in xs {
    ys.push(dump_code(x));
  }
  ys.into()
}

fn dump_args_code(xs: &[Arc<str>]) -> Edn {
  let mut ys = EdnListView::default();
  for x in xs {
    ys.push(Edn::sym(&*x.to_owned()));
  }
  ys.into()
}
