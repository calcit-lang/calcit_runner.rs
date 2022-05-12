use crate::data::cirru;
use crate::data::edn;
use crate::primes::{Calcit, CalcitItems};
use cirru_edn::Edn;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone, Eq, Ord, PartialOrd, Hash)]
pub struct CalcitStack {
  pub ns: Arc<str>,
  pub def: Arc<str>,
  pub code: Calcit, // built in functions may not contain code
  pub args: Box<CalcitItems>,
  pub kind: StackKind,
}

#[derive(Debug, PartialEq, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum StackKind {
  Fn,
  Proc,
  Macro,
  Syntax,  // rarely used
  Codegen, // track preprocessing
}

impl fmt::Display for CalcitStack {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TODO")
  }
}

pub type CallStackList = rpds::ListSync<CalcitStack>;

// TODO impl fmt

/// create new entry to the tree
pub fn extend_call_stack(
  stack: &CallStackList,
  ns: Arc<str>,
  def: Arc<str>,
  kind: StackKind,
  code: Calcit,
  args: &CalcitItems,
) -> CallStackList {
  stack.push_front(CalcitStack {
    ns,
    def,
    code,
    args: Box::new(args.to_owned()),
    kind,
  })
}

// show simplified version of stack
pub fn show_stack(stack: &CallStackList) {
  println!("\ncall stack:");
  for s in stack {
    let is_macro = s.kind == StackKind::Macro;
    println!("  {}/{}{}", s.ns, s.def, if is_macro { "\t ~macro" } else { "" });
  }
}

pub fn display_stack(failure: &str, stack: &CallStackList) -> Result<(), String> {
  eprintln!("\nFailure: {}", failure);
  eprintln!("\ncall stack:");

  for s in stack {
    let is_macro = s.kind == StackKind::Macro;
    eprintln!("  {}/{}{}", s.ns, s.def, if is_macro { "\t ~macro" } else { "" });
  }

  let mut stack_list: Vec<Edn> = Vec::with_capacity(stack.len());
  for s in stack {
    let mut args: Vec<Edn> = Vec::with_capacity(s.args.len());
    for a in &*s.args {
      args.push(edn::calcit_to_edn(a)?);
    }
    let info = Edn::map_from_iter([
      ("def".into(), format!("{}/{}", s.ns, s.def).into()),
      ("code".into(), cirru::calcit_to_cirru(&s.code)?.into()),
      ("args".into(), args.into()),
      ("kind".into(), name_kind(&s.kind).into()),
    ]);

    stack_list.push(info);
  }

  let content = cirru_edn::format(
    &Edn::map_from_iter([("message".into(), failure.into()), ("stack".into(), stack_list.into())]),
    true,
  )?;
  let _ = fs::write(ERROR_SNAPSHOT, content);
  eprintln!("\nrun `cat {}` to read stack details.", ERROR_SNAPSHOT);
  Ok(())
}

const ERROR_SNAPSHOT: &str = ".calcit-error.cirru";

fn name_kind(k: &StackKind) -> String {
  match k {
    StackKind::Fn => String::from("fn"),
    StackKind::Proc => String::from("proc"),
    StackKind::Macro => String::from("macro"),
    StackKind::Syntax => String::from("syntax"),
    StackKind::Codegen => String::from("codegen"),
  }
}
