pub mod effects;
mod lists;
mod logics;
mod maps;
mod math;
pub mod meta;
mod records;
mod refs;
mod regexes;
mod sets;
mod strings;
mod syntax;

use crate::data::json;
use crate::primes::{Calcit, CalcitItems, CalcitScope};
use crate::program::ProgramCodeData;

pub fn is_proc_name(s: &str) -> bool {
  matches!(
    s,
    // meta
    "type-of"
      | "recur"
      | "format-to-lisp"
      | "gensym"
      | "&reset-gensym-index!"
      | "&get-calcit-running-mode"
      | "generate-id!"
      | "turn-symbol"
      | "turn-keyword"
      | "&compare"
      // tuples
      | "::" // unstable
      | "&tuple:nth"
      | "&tuple:assoc"
      // effects
      | "&display-stack"
      | "echo"
      | "println" // alias for echo
      | "echo-values"
      | "raise"
      | "quit!"
      | "get-env"
      | "&get-calcit-backend"
      | "read-file"
      | "write-file"
      | "&ffi-message"
      // external format
      | "parse-cirru"
      | "format-cirru"
      | "parse-cirru-edn"
      | "format-cirru-edn"
      | "parse-json"
      | "stringify-json"
      // regex
      | "re-matches"
      | "re-find"
      | "re-find-index"
      | "re-find-all"
      // time
      | "cpu-time"
      | "format-time"
      | "parse-time"
      | "get-time!"
      // logics
      | "&="
      | "&<"
      | "&>"
      | "not"
      | "identical?"
      // math
      | "&+"
      | "&-"
      | "&*"
      | "&/"
      | "round"
      | "rand"
      | "rand-int"
      | "floor"
      | "sin"
      | "cos"
      | "pow"
      | "ceil"
      | "sqrt"
      | "round?"
      | "&number:fract"
      | "&number:rem"
      | "&number:format"
      | "bit-shl"
      | "bit-shr"
      // strings
      | "&str:concat"
      | "trim"
      | "&str"
      | "turn-string"
      | "split"
      | "split-lines"
      | "starts-with?"
      | "ends-with?"
      | "get-char-code"
      | "pr-str"
      | "parse-float"
      | "blank?"
      | "&str:compare"
      | "&str:replace"
      | "&str:slice"
      | "&str:find-index"
      | "&str:escape"
      | "&str:count"
      | "&str:empty?"
      | "&str:contains?"
      | "&str:includes?"
      | "&str:nth"
      | "&str:first"
      | "&str:rest"
      // lists
      | "[]"
      | "'" // used as an alias for `[]`, experimental
      | "append"
      | "prepend"
      | "butlast"
      | "range"
      | "&list:reverse"
      | "&list:concat"
      | "&list:count"
      | "&list:empty?"
      | "&list:slice"
      | "&list:assoc-before"
      | "&list:assoc-after"
      | "&list:contains?"
      | "&list:includes?"
      | "&list:nth"
      | "&list:first"
      | "&list:rest"
      | "&list:assoc"
      | "&list:dissoc"
      | "&list:to-set"
      | "&list:distinct"
      // maps
      | "&{}"
      | "&merge"
      | "to-pairs"
      | "&merge-non-nil"
      | "&map:get"
      | "&map:dissoc"
      | "&map:to-list"
      | "&map:count"
      | "&map:empty?"
      | "&map:contains?"
      | "&map:includes?"
      | "&map:first"
      | "&map:rest"
      | "&map:assoc"
      | "&map:diff-new"
      | "&map:diff-keys"
      | "&map:common-keys"
      // sets
      | "#{}"
      | "&include"
      | "&exclude"
      | "&difference"
      | "&union"
      | "&set:intersection"
      | "&set:to-list"
      | "&set:count"
      | "&set:empty?"
      | "&set:includes?"
      | "&set:first"
      | "&set:rest"
      | "&set:assoc"
      // refs
      | "deref"
      | "add-watch"
      | "remove-watch"
      // records
      | "new-record"
      | "&%{}"
      | "&record:matches?"
      | "&record:from-map"
      | "&record:get-name"
      | "&record:to-map"
      | "&record:count"
      | "&record:contains?"
      | "&record:nth"
      | "&record:get"
      | "&record:assoc"
  )
}

pub fn handle_proc(name: &str, args: &CalcitItems) -> Result<Calcit, String> {
  match name {
    // meta
    "type-of" => meta::type_of(args),
    "recur" => meta::recur(args),
    "format-to-lisp" => meta::format_to_lisp(args),
    "gensym" => meta::gensym(args),
    "&reset-gensym-index!" => meta::reset_gensym_index(args),
    "&get-calcit-running-mode" => effects::calcit_running_mode(args),
    "generate-id!" => meta::generate_id(args),
    "turn-symbol" => meta::turn_symbol(args),
    "turn-keyword" => meta::turn_keyword(args),
    "&compare" => meta::native_compare(args),
    // tuple
    "::" => meta::new_tuple(args), // unstable solution for the name
    "&tuple:nth" => meta::tuple_nth(args),
    "&tuple:assoc" => meta::assoc(args),
    // effects
    "&display-stack" => meta::display_stack(args),
    "echo" => effects::echo(args),
    "println" => effects::echo(args), // alias
    "echo-values" => effects::echo_values(args),
    "raise" => effects::raise(args),
    "quit!" => effects::quit(args),
    "get-env" => effects::get_env(args),
    "&get-calcit-backend" => effects::call_get_calcit_backend(args),
    "read-file" => effects::read_file(args),
    "write-file" => effects::write_file(args),
    "&ffi-message" => effects::ffi_message(args),
    // external data format
    "parse-cirru" => meta::parse_cirru(args),
    "format-cirru" => meta::write_cirru(args),
    "parse-cirru-edn" => meta::parse_cirru_edn(args),
    "format-cirru-edn" => meta::write_cirru_edn(args),
    "parse-json" => json::parse_json(args),
    "stringify-json" => json::stringify_json(args),
    // time
    "cpu-time" => effects::cpu_time(args),
    "parse-time" => effects::parse_time(args),
    "format-time" => effects::format_time(args),
    "get-time!" => effects::now_bang(args),
    // regex
    "re-matches" => regexes::re_matches(args),
    "re-find" => regexes::re_find(args),
    "re-find-index" => regexes::re_find_index(args),
    "re-find-all" => regexes::re_find_all(args),
    // logics
    "&=" => logics::binary_equal(args),
    "&<" => logics::binary_less(args),
    "&>" => logics::binary_greater(args),
    "not" => logics::not(args),
    // in Rust, no real pointer `identical?`, fallback to value equal
    "identical?" => logics::binary_equal(args),
    // math
    "&+" => math::binary_add(args),
    "&-" => math::binary_minus(args),
    "&*" => math::binary_multiply(args),
    "&/" => math::binary_divide(args),
    "rand" => math::rand(args),
    "rand-int" => math::rand_int(args),
    "floor" => math::floor(args),
    "sin" => math::sin(args),
    "cos" => math::cos(args),
    "pow" => math::pow(args),
    "ceil" => math::ceil(args),
    "sqrt" => math::sqrt(args),
    "round" => math::round(args),
    "round?" => math::round_ques(args),
    "&number:rem" => math::rem(args),
    "&number:fract" => math::fractional(args),
    "&number:format" => strings::format_number(args),
    "bit-shr" => math::bit_shr(args),
    "bit-shl" => math::bit_shl(args),
    // strings
    "trim" => strings::trim(args),
    "&str" => strings::call_str(args),
    "turn-string" => strings::turn_string(args),
    "split" => strings::split(args),
    "split-lines" => strings::split_lines(args),
    "starts-with?" => strings::starts_with_ques(args),
    "ends-with?" => strings::ends_with_ques(args),
    "get-char-code" => strings::get_char_code(args),
    "parse-float" => strings::parse_float(args),
    "pr-str" => strings::pr_str(args),
    "blank?" => strings::blank_ques(args),
    "&str:concat" => strings::binary_str_concat(args),
    "&str:slice" => strings::str_slice(args),
    "&str:compare" => strings::compare_string(args),
    "&str:find-index" => strings::find_index(args),
    "&str:replace" => strings::replace(args),
    "&str:escape" => strings::escape(args),
    "&str:count" => strings::count(args),
    "&str:empty?" => strings::empty_ques(args),
    "&str:contains?" => strings::contains_ques(args),
    "&str:includes?" => strings::includes_ques(args),
    "&str:nth" => strings::nth(args),
    "&str:first" => strings::first(args),
    "&str:rest" => strings::rest(args),
    // lists
    "[]" => lists::new_list(args),
    "'" => lists::new_list(args), // alias
    "append" => lists::append(args),
    "prepend" => lists::prepend(args),
    "butlast" => lists::butlast(args),
    "&list:concat" => lists::concat(args),
    "range" => lists::range(args),
    "&list:reverse" => lists::reverse(args),
    "&list:slice" => lists::slice(args),
    "&list:assoc-before" => lists::assoc_before(args),
    "&list:assoc-after" => lists::assoc_after(args),
    "&list:count" => lists::count(args),
    "&list:empty?" => lists::empty_ques(args),
    "&list:contains?" => lists::contains_ques(args),
    "&list:includes?" => lists::includes_ques(args),
    "&list:nth" => lists::nth(args),
    "&list:first" => lists::first(args),
    "&list:rest" => lists::rest(args),
    "&list:assoc" => lists::assoc(args),
    "&list:dissoc" => lists::dissoc(args),
    "&list:to-set" => lists::list_to_set(args),
    "&list:distinct" => lists::distinct(args),
    // maps
    "&{}" => maps::call_new_map(args),
    "&merge" => maps::call_merge(args),
    "to-pairs" => maps::to_pairs(args),
    "&merge-non-nil" => maps::call_merge_non_nil(args),
    "&map:to-list" => maps::to_list(args),
    "&map:count" => maps::count(args),
    "&map:empty?" => maps::empty_ques(args),
    "&map:contains?" => maps::contains_ques(args),
    "&map:includes?" => maps::includes_ques(args),
    "&map:first" => maps::first(args),
    "&map:rest" => maps::rest(args),
    "&map:get" => maps::get(args),
    "&map:assoc" => maps::assoc(args),
    "&map:dissoc" => maps::dissoc(args),
    "&map:diff-new" => maps::diff_new(args),
    "&map:diff-keys" => maps::diff_keys(args),
    "&map:common-keys" => maps::common_keys(args),
    // sets
    "#{}" => sets::new_set(args),
    "&include" => sets::call_include(args),
    "&exclude" => sets::call_exclude(args),
    "&difference" => sets::call_difference(args),
    "&union" => sets::call_union(args),
    "&set:intersection" => sets::call_intersection(args),
    "&set:to-list" => sets::set_to_list(args),
    "&set:count" => sets::count(args),
    "&set:empty?" => sets::empty_ques(args),
    "&set:includes?" => sets::includes_ques(args),
    "&set:first" => sets::first(args),
    "&set:rest" => sets::rest(args),
    // refs
    "deref" => refs::deref(args),
    "add-watch" => refs::add_watch(args),
    "remove-watch" => refs::remove_watch(args),
    // records
    "new-record" => records::new_record(args),
    "&%{}" => records::call_record(args),
    "&record:from-map" => records::record_from_map(args),
    "&record:get-name" => records::get_record_name(args),
    "&record:to-map" => records::turn_map(args),
    "&record:matches?" => records::matches(args),
    "&record:count" => records::count(args),
    "&record:contains?" => records::contains_ques(args),
    "&record:nth" => records::nth(args),
    "&record:get" => records::get(args),
    "&record:assoc" => records::assoc(args),
    a => Err(format!("No such proc: {}", a)),
  }
}

pub fn is_syntax_name(s: &str) -> bool {
  matches!(
    s,
    "defn"
      | "defmacro"
      | "if"
      | "&let"
      | "quote"
      | "quasiquote"
      | "eval"
      | "macroexpand"
      | "macroexpand-1"
      | "macroexpand-all"
      | "foldl" // for performance
      | "foldl-shortcut" // for performance
      | "try"
      | "sort" // TODO need better solution
      | "defatom"
      | "reset!"
  )
}

pub fn handle_syntax(
  name: &str,
  nodes: &CalcitItems,
  scope: &CalcitScope,
  file_ns: &str,
  program: &ProgramCodeData,
) -> Result<Calcit, String> {
  match name {
    "defn" => syntax::defn(nodes, scope, file_ns, program),
    "eval" => syntax::eval(nodes, scope, file_ns, program),
    "defmacro" => syntax::defmacro(nodes, scope, file_ns, program),
    "quote" => syntax::quote(nodes, scope, file_ns, program),
    "quasiquote" => syntax::quasiquote(nodes, scope, file_ns, program),
    "if" => syntax::syntax_if(nodes, scope, file_ns, program),
    "&let" => syntax::syntax_let(nodes, scope, file_ns, program),
    "foldl" => lists::foldl(nodes, scope, file_ns, program),
    "foldl-shortcut" => lists::foldl_shortcut(nodes, scope, file_ns, program),
    "macroexpand" => syntax::macroexpand(nodes, scope, file_ns, program),
    "macroexpand-1" => syntax::macroexpand_1(nodes, scope, file_ns, program),
    "macroexpand-all" => syntax::macroexpand_all(nodes, scope, file_ns, program),
    "try" => syntax::call_try(nodes, scope, file_ns, program),
    "sort" => lists::sort(nodes, scope, file_ns, program),
    // "define reference" although it uses a confusing name "atom"
    "defatom" => refs::defatom(nodes, scope, file_ns, program),
    "reset!" => refs::reset_bang(nodes, scope, file_ns, program),
    a => Err(format!("TODO syntax: {}", a)),
  }
}

// detects extra javascript things in js mode,
// mostly internal procs, also some syntaxs
pub fn is_js_syntax_procs(s: &str) -> bool {
  matches!(
    s,
    "aget"
      | "aset"
      | "exists?"
      | "extract-cirru-edn"
      | "foldl"
      | "instance?"
      | "&js-object"
      | "js-array"
      | "load-console-formatter!"
      | "printable"
      | "new"
      | "set!"
      | "timeout-call"
      | "to-calcit-data"
      | "to-cirru-edn"
      | "to-js-data"
      | "invoke-method" // dynamically
  )
}
