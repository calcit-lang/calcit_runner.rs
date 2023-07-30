use std::{
  fmt::{Display, Formatter},
  str::FromStr,
};

/// represent builtin functions for performance reasons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CalcitProc {
  // meta
  TypeOf,
  Recur,
  FormatToLisp,
  FormatToCirru,
  NativeResetGenSymIndex,
  NativeGetCalcitRunningMode,
  GenerateId,
  TurnSymbol,
  TurnTag,
  NativeCompare,
  NativeGetOs,
  NativeFormatTernaryTree,
  NativeBuffer,
  NativeHash,
  NativeExtractCodeIntoEdn,
  // "::", unstable
  NativeTuple,
  // "%::"
  NativeClassTuple,
  NativeTupleNth,
  NativeTupleAssoc,
  NativeTupleCount,
  NativeTupleClass,
  NativeTupleParams,
  NativeTupleWithClass,
  NativeDisplayStack,
  Raise,
  Quit,
  GetEnv,
  NativeGetCalcitBackend,
  ReadFile,
  WriteFile,
  // external format
  ParseCirru,
  ParseCirruList,
  FormatCirru,
  ParseCirruEdn,
  FormatCirruEdn,
  NativeCirruQuoteToList,
  // time
  CpuTime,
  // logics
  NativeEquals,
  NativeLessThan,
  NativeGreaterThan,
  Not,
  Identical,
  // math
  NativeAdd,
  NativeMinus,
  NativeMultiply,
  NativeDivide,
  Round,
  Floor,
  Sin,
  Cos,
  Pow,
  Ceil,
  Sqrt,
  IsRound,
  NativeNumberFract,
  NativeNumberRem,
  NativeNumberFormat,
  NativeNumberDisplayBy,
  BitShl,
  BitShr,
  BitAnd,
  BitOr,
  BitXor,
  BitNot,
  // strings
  NativeStrConcat,
  Trim,
  NativeStr,
  TurnString,
  Split,
  SplitLines,
  StartsWith,
  EndsWith,
  GetCharCode,
  CharFromCode,
  PrStr,
  ParseFloat,
  IsBlank,
  NativeStrCompare,
  NativeStrReplace,
  NativeStrSlice,
  NativeStrFindIndex,
  NativeStrEscape,
  NativeStrCount,
  NativeStrEmpty,
  NativeStrContains,
  NativeStrIncludes,
  NativeStrNth,
  NativeStrFirst,
  NativeStrRest,
  NativeStrPadLeft,
  NativeStrPadRight,
  // lists
  List,
  Append,
  Prepend,
  Butlast,
  Range,
  Sort,
  Foldl,
  FoldlShortcut,
  FoldrShortcut,
  NativeListReverse,
  NativeListConcat,
  NativeListCount,
  NativeListEmpty,
  NativeListSlice,
  NativeListAssocBefore,
  NativeListAssocAfter,
  NativeListContains,
  NativeListIncludes,
  NativeListNth,
  NativeListFirst,
  NativeListRest,
  NativeListAssoc,
  NativeListDissoc,
  NativeListToSet,
  NativeListDistinct,
  // maps
  NativeMap,
  NativeMerge,
  ToPairs,
  NativeMergeNonNil,
  NativeMapGet,
  NativeMapDissoc,
  NativeMapToList,
  NativeMapCount,
  NativeMapEmpty,
  NativeMapContains,
  NativeMapIncludes,
  NativeMapFirst,
  NativeMapRest,
  NativeMapAssoc,
  NativeMapDiffNew,
  NativeMapDiffKeys,
  NativeMapCommonKeys,
  // sets
  Set,
  NativeInclude,
  NativeExclude,
  NativeDifference,
  NativeUnion,
  NativeSetIntersection,
  NativeSetToList,
  NativeSetCount,
  NativeSetEmpty,
  NativeSetIncludes,
  NativeSetFirst,
  NativeSetRest,
  NativeSetAssoc,
  // refs
  Atom,
  Deref,
  AddWatch,
  RemoveWatch,
  // records
  NewRecord,
  NativeRecord,
  NativeRecordMatches,
  NativeRecordFromMap,
  NativeRecordGetName,
  NativeRecordToMap,
  NativeRecordCount,
  NativeRecordContains,
  NativeRecordGet,
  NativeRecordAssoc,
  NativeRecordExtendAs,
}

impl FromStr for CalcitProc {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "type-of" => Ok(Self::TypeOf),
      "recur" => Ok(Self::Recur),
      "format-to-lisp" => Ok(Self::FormatToLisp),
      "format-to-cirru" => Ok(Self::FormatToCirru),
      "&reset-gensym-index!" => Ok(Self::NativeResetGenSymIndex),
      "&get-calcit-running-mode" => Ok(Self::NativeGetCalcitRunningMode),
      "generate-id!" => Ok(Self::GenerateId),
      "turn-symbol" => Ok(Self::TurnSymbol),
      "turn-tag" => Ok(Self::TurnTag),
      "&compare" => Ok(Self::NativeCompare),
      "&get-os" => Ok(Self::NativeGetOs),
      "&format-ternary-tree" => Ok(Self::NativeFormatTernaryTree),
      "&buffer" => Ok(Self::NativeBuffer),
      "&hash" => Ok(Self::NativeHash),
      "&extract-code-into-edn" => Ok(Self::NativeExtractCodeIntoEdn),
      // tuples // unstable
      "::" => Ok(Self::NativeTuple),
      "%::" => Ok(Self::NativeClassTuple),
      "&tuple:nth" => Ok(Self::NativeTupleNth),
      "&tuple:assoc" => Ok(Self::NativeTupleAssoc),
      "&tuple:count" => Ok(Self::NativeTupleCount),
      "&tuple:class" => Ok(Self::NativeTupleClass),
      "&tuple:params" => Ok(Self::NativeTupleParams),
      "&tuple:with-class" => Ok(Self::NativeTupleWithClass),
      // effects
      "&display-stack" => Ok(Self::NativeDisplayStack),
      "raise" => Ok(Self::Raise),
      "quit!" => Ok(Self::Quit),
      "get-env" => Ok(Self::GetEnv),
      "&get-calcit-backend" => Ok(Self::NativeGetCalcitBackend),
      "read-file" => Ok(Self::ReadFile),
      "write-file" => Ok(Self::WriteFile),
      // external format
      "parse-cirru" => Ok(Self::ParseCirru),
      "parse-cirru-list" => Ok(Self::ParseCirruList),
      "format-cirru" => Ok(Self::FormatCirru),
      "parse-cirru-edn" => Ok(Self::ParseCirruEdn),
      "format-cirru-edn" => Ok(Self::FormatCirruEdn),
      "&cirru-quote:to-list" => Ok(Self::NativeCirruQuoteToList),
      // time
      "cpu-time" => Ok(Self::CpuTime),
      // logics
      "&=" => Ok(Self::NativeEquals),
      "&<" => Ok(Self::NativeLessThan),
      "&>" => Ok(Self::NativeGreaterThan),
      "not" => Ok(Self::Not),
      "identical?" => Ok(Self::Identical),
      // math
      "&+" => Ok(Self::NativeAdd),
      "&-" => Ok(Self::NativeMinus),
      "&*" => Ok(Self::NativeMultiply),
      "&/" => Ok(Self::NativeDivide),
      "round" => Ok(Self::Round),
      "floor" => Ok(Self::Floor),
      "sin" => Ok(Self::Sin),
      "cos" => Ok(Self::Cos),
      "pow" => Ok(Self::Pow),
      "ceil" => Ok(Self::Ceil),
      "sqrt" => Ok(Self::Sqrt),
      "round?" => Ok(Self::IsRound),
      "&number:fract" => Ok(Self::NativeNumberFract),
      "&number:rem" => Ok(Self::NativeNumberRem),
      "&number:format" => Ok(Self::NativeNumberFormat),
      "&number:display-by" => Ok(Self::NativeNumberDisplayBy),
      "bit-shl" => Ok(Self::BitShl),
      "bit-shr" => Ok(Self::BitShr),
      "bit-and" => Ok(Self::BitAnd),
      "bit-or" => Ok(Self::BitOr),
      "bit-xor" => Ok(Self::BitXor),
      "bit-not" => Ok(Self::BitNot),
      // strings
      "&str:concat" => Ok(Self::NativeStrConcat),
      "trim" => Ok(Self::Trim),
      "&str" => Ok(Self::NativeStr),
      "turn-string" => Ok(Self::TurnString),
      "split" => Ok(Self::Split),
      "split-lines" => Ok(Self::SplitLines),
      "starts-with?" => Ok(Self::StartsWith),
      "ends-with?" => Ok(Self::EndsWith),
      "get-char-code" => Ok(Self::GetCharCode),
      "char-from-code" => Ok(Self::CharFromCode),
      "pr-str" => Ok(Self::PrStr),
      "parse-float" => Ok(Self::ParseFloat),
      "blank?" => Ok(Self::IsBlank),
      "&str:compare" => Ok(Self::NativeStrCompare),
      "&str:replace" => Ok(Self::NativeStrReplace),
      "&str:slice" => Ok(Self::NativeStrSlice),
      "&str:find-index" => Ok(Self::NativeStrFindIndex),
      "&str:escape" => Ok(Self::NativeStrEscape),
      "&str:count" => Ok(Self::NativeStrCount),
      "&str:empty?" => Ok(Self::NativeStrEmpty),
      "&str:contains?" => Ok(Self::NativeStrContains),
      "&str:includes?" => Ok(Self::NativeStrIncludes),
      "&str:nth" => Ok(Self::NativeStrNth),
      "&str:first" => Ok(Self::NativeStrFirst),
      "&str:rest" => Ok(Self::NativeStrRest),
      "&str:pad-left" => Ok(Self::NativeStrPadLeft),
      "&str:pad-right" => Ok(Self::NativeStrPadRight),
      // lists
      "[]" => Ok(Self::List),
      // used as an alias for `[]`, experimental => Ok(Self::),
      "'" => Ok(Self::List),
      "append" => Ok(Self::Append),
      "prepend" => Ok(Self::Prepend),
      "butlast" => Ok(Self::Butlast),
      "range" => Ok(Self::Range),
      "sort" => Ok(Self::Sort),
      "foldl" => Ok(Self::Foldl),
      "foldl-shortcut" => Ok(Self::FoldlShortcut),
      "foldr-shortcut" => Ok(Self::FoldrShortcut),
      "&list:reverse" => Ok(Self::NativeListReverse),
      "&list:concat" => Ok(Self::NativeListConcat),
      "&list:count" => Ok(Self::NativeListCount),
      "&list:empty?" => Ok(Self::NativeListEmpty),
      "&list:slice" => Ok(Self::NativeListSlice),
      "&list:assoc-before" => Ok(Self::NativeListAssocBefore),
      "&list:assoc-after" => Ok(Self::NativeListAssocAfter),
      "&list:contains?" => Ok(Self::NativeListContains),
      "&list:includes?" => Ok(Self::NativeListIncludes),
      "&list:nth" => Ok(Self::NativeListNth),
      "&list:first" => Ok(Self::NativeListFirst),
      "&list:rest" => Ok(Self::NativeListRest),
      "&list:assoc" => Ok(Self::NativeListAssoc),
      "&list:dissoc" => Ok(Self::NativeListDissoc),
      "&list:to-set" => Ok(Self::NativeListToSet),
      "&list:distinct" => Ok(Self::NativeListDistinct),
      // maps
      "&{}" => Ok(Self::NativeMap),
      "&merge" => Ok(Self::NativeMerge),
      "to-pairs" => Ok(Self::ToPairs),
      "&merge-non-nil" => Ok(Self::NativeMergeNonNil),
      "&map:get" => Ok(Self::NativeMapGet),
      "&map:dissoc" => Ok(Self::NativeMapDissoc),
      "&map:to-list" => Ok(Self::NativeMapToList),
      "&map:count" => Ok(Self::NativeMapCount),
      "&map:empty?" => Ok(Self::NativeMapEmpty),
      "&map:contains?" => Ok(Self::NativeMapContains),
      "&map:includes?" => Ok(Self::NativeMapIncludes),
      "&map:first" => Ok(Self::NativeMapFirst),
      "&map:rest" => Ok(Self::NativeMapRest),
      "&map:assoc" => Ok(Self::NativeMapAssoc),
      "&map:diff-new" => Ok(Self::NativeMapDiffNew),
      "&map:diff-keys" => Ok(Self::NativeMapDiffKeys),
      "&map:common-keys" => Ok(Self::NativeMapCommonKeys),
      // sets
      "#{}" => Ok(Self::Set),
      "&include" => Ok(Self::NativeInclude),
      "&exclude" => Ok(Self::NativeExclude),
      "&difference" => Ok(Self::NativeDifference),
      "&union" => Ok(Self::NativeUnion),
      "&set:intersection" => Ok(Self::NativeSetIntersection),
      "&set:to-list" => Ok(Self::NativeSetToList),
      "&set:count" => Ok(Self::NativeSetCount),
      "&set:empty?" => Ok(Self::NativeSetEmpty),
      "&set:includes?" => Ok(Self::NativeSetIncludes),
      "&set:first" => Ok(Self::NativeSetFirst),
      "&set:rest" => Ok(Self::NativeSetRest),
      "&set:assoc" => Ok(Self::NativeSetAssoc),
      // refs
      "atom" => Ok(Self::Atom),
      "deref" => Ok(Self::Deref),
      "add-watch" => Ok(Self::AddWatch),
      "remove-watch" => Ok(Self::RemoveWatch),
      // records
      "new-record" => Ok(Self::NewRecord),
      "&%{}" => Ok(Self::NativeRecord),
      "&record:matches?" => Ok(Self::NativeRecordMatches),
      "&record:from-map" => Ok(Self::NativeRecordFromMap),
      "&record:get-name" => Ok(Self::NativeRecordGetName),
      "&record:to-map" => Ok(Self::NativeRecordToMap),
      "&record:count" => Ok(Self::NativeRecordCount),
      "&record:contains?" => Ok(Self::NativeRecordContains),
      "&record:get" => Ok(Self::NativeRecordGet),
      "&record:assoc" => Ok(Self::NativeRecordAssoc),
      "&record:extend-as" => Ok(Self::NativeRecordExtendAs),
      _ => Err(format!("unknown proc: {}", s)),
    }
  }
}

impl Display for CalcitProc {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::TypeOf => write!(f, "type-of"),
      Self::Recur => write!(f, "recur"),
      Self::FormatToLisp => write!(f, "format-to-lisp"),
      Self::FormatToCirru => write!(f, "format-to-cirru"),

      Self::NativeResetGenSymIndex => write!(f, "&reset-gensym-index!"),
      Self::NativeGetCalcitRunningMode => write!(f, "&get-calcit-running-mode"),
      Self::GenerateId => write!(f, "generate-id!"),
      Self::TurnSymbol => write!(f, "turn-symbol"),
      Self::TurnTag => write!(f, "turn-tag"),
      Self::NativeCompare => write!(f, "&compare"),
      Self::NativeGetOs => write!(f, "&get-os"),
      Self::NativeFormatTernaryTree => write!(f, "&format-ternary-tree"),
      Self::NativeBuffer => write!(f, "&buffer"),
      Self::NativeHash => write!(f, "&hash"),
      Self::NativeExtractCodeIntoEdn => write!(f, "&extract-code-into-edn"),
      Self::NativeTuple => write!(f, "::"),
      Self::NativeClassTuple => write!(f, "%::"),
      Self::NativeTupleNth => write!(f, "&tuple:nth"),
      Self::NativeTupleAssoc => write!(f, "&tuple:assoc"),
      Self::NativeTupleCount => write!(f, "&tuple:count"),
      Self::NativeTupleClass => write!(f, "&tuple:class"),
      Self::NativeTupleParams => write!(f, "&tuple:params"),
      Self::NativeTupleWithClass => write!(f, "&tuple:with-class"),
      Self::NativeDisplayStack => write!(f, "&display-stack"),
      Self::Raise => write!(f, "raise"),
      Self::Quit => write!(f, "quit!"),
      Self::GetEnv => write!(f, "get-env"),
      Self::NativeGetCalcitBackend => write!(f, "&get-calcit-backend"),
      Self::ReadFile => write!(f, "read-file"),
      Self::WriteFile => write!(f, "write-file"),
      Self::ParseCirru => write!(f, "parse-cirru"),
      Self::ParseCirruList => write!(f, "parse-cirru-list"),
      Self::FormatCirru => write!(f, "format-cirru"),
      Self::ParseCirruEdn => write!(f, "parse-cirru-edn"),
      Self::FormatCirruEdn => write!(f, "format-cirru-edn"),
      Self::NativeCirruQuoteToList => write!(f, "&cirru-quote:to-list"),
      Self::CpuTime => write!(f, "cpu-time"),
      Self::NativeEquals => write!(f, "&="),
      Self::NativeLessThan => write!(f, "&<"),
      Self::NativeGreaterThan => write!(f, "&>"),
      Self::Not => write!(f, "not"),
      Self::Identical => write!(f, "identical?"),
      Self::NativeAdd => write!(f, "&+"),
      Self::NativeMinus => write!(f, "&-"),
      Self::NativeMultiply => write!(f, "&*"),
      Self::NativeDivide => write!(f, "&/"),
      Self::Round => write!(f, "round"),
      Self::Floor => write!(f, "floor"),
      Self::Sin => write!(f, "sin"),
      Self::Cos => write!(f, "cos"),
      Self::Pow => write!(f, "pow"),
      Self::Ceil => write!(f, "ceil"),
      Self::Sqrt => write!(f, "sqrt"),
      Self::IsRound => write!(f, "round?"),
      Self::NativeNumberFract => write!(f, "&number:fract"),
      Self::NativeNumberRem => write!(f, "&number:rem"),
      Self::NativeNumberFormat => write!(f, "&number:format"),
      Self::NativeNumberDisplayBy => write!(f, "&number:display-by"),
      Self::BitShl => write!(f, "bit-shl"),
      Self::BitShr => write!(f, "bit-shr"),
      Self::BitAnd => write!(f, "bit-and"),
      Self::BitOr => write!(f, "bit-or"),
      Self::BitXor => write!(f, "bit-xor"),
      Self::BitNot => write!(f, "bit-not"),
      Self::NativeStrConcat => write!(f, "&str:concat"),
      Self::Trim => write!(f, "trim"),
      Self::NativeStr => write!(f, "&str"),
      Self::TurnString => write!(f, "turn-string"),
      Self::Split => write!(f, "split"),
      Self::SplitLines => write!(f, "split-lines"),
      Self::StartsWith => write!(f, "starts-with?"),
      Self::EndsWith => write!(f, "ends-with?"),
      Self::GetCharCode => write!(f, "get-char-code"),
      Self::CharFromCode => write!(f, "char-from-code"),
      Self::PrStr => write!(f, "pr-str"),
      Self::ParseFloat => write!(f, "parse-float"),
      Self::IsBlank => write!(f, "blank?"),
      Self::NativeStrCompare => write!(f, "&str:compare"),
      Self::NativeStrReplace => write!(f, "&str:replace"),
      Self::NativeStrSlice => write!(f, "&str:slice"),
      Self::NativeStrFindIndex => write!(f, "&str:find-index"),
      Self::NativeStrEscape => write!(f, "&str:escape"),
      Self::NativeStrCount => write!(f, "&str:count"),
      Self::NativeStrEmpty => write!(f, "&str:empty?"),
      Self::NativeStrContains => write!(f, "&str:contains?"),
      Self::NativeStrIncludes => write!(f, "&str:includes?"),
      Self::NativeStrNth => write!(f, "&str:nth"),
      Self::NativeStrFirst => write!(f, "&str:first"),
      Self::NativeStrRest => write!(f, "&str:rest"),
      Self::NativeStrPadLeft => write!(f, "&str:pad-left"),
      Self::NativeStrPadRight => write!(f, "&str:pad-right"),
      Self::List => write!(f, "[]"),
      Self::Append => write!(f, "append"),
      Self::Prepend => write!(f, "prepend"),
      Self::Butlast => write!(f, "butlast"),
      Self::Range => write!(f, "range"),
      Self::Sort => write!(f, "sort"),
      Self::Foldl => write!(f, "foldl"),
      Self::FoldlShortcut => write!(f, "foldl-shortcut"),
      Self::FoldrShortcut => write!(f, "foldr-shortcut"),
      Self::NativeListReverse => write!(f, "&list:reverse"),
      Self::NativeListConcat => write!(f, "&list:concat"),
      Self::NativeListCount => write!(f, "&list:count"),
      Self::NativeListEmpty => write!(f, "&list:empty?"),
      Self::NativeListSlice => write!(f, "&list:slice"),
      Self::NativeListAssocBefore => write!(f, "&list:assoc-before"),
      Self::NativeListAssocAfter => write!(f, "&list:assoc-after"),
      Self::NativeListContains => write!(f, "&list:contains?"),
      Self::NativeListIncludes => write!(f, "&list:includes?"),
      Self::NativeListNth => write!(f, "&list:nth"),
      Self::NativeListFirst => write!(f, "&list:first"),
      Self::NativeListRest => write!(f, "&list:rest"),
      Self::NativeListAssoc => write!(f, "&list:assoc"),
      Self::NativeListDissoc => write!(f, "&list:dissoc"),
      Self::NativeListToSet => write!(f, "&list:to-set"),
      Self::NativeListDistinct => write!(f, "&list:distinct"),
      Self::NativeMap => write!(f, "&{{}}"),
      Self::NativeMerge => write!(f, "&merge"),
      Self::ToPairs => write!(f, "to-pairs"),
      Self::NativeMergeNonNil => write!(f, "&merge-non-nil"),
      Self::NativeMapGet => write!(f, "&map:get"),
      Self::NativeMapDissoc => write!(f, "&map:dissoc"),
      Self::NativeMapToList => write!(f, "&map:to-list"),
      Self::NativeMapCount => write!(f, "&map:count"),
      Self::NativeMapEmpty => write!(f, "&map:empty?"),
      Self::NativeMapContains => write!(f, "&map:contains?"),
      Self::NativeMapIncludes => write!(f, "&map:includes?"),
      Self::NativeMapFirst => write!(f, "&map:first"),
      Self::NativeMapRest => write!(f, "&map:rest"),
      Self::NativeMapAssoc => write!(f, "&map:assoc"),
      Self::NativeMapDiffNew => write!(f, "&map:diff-new"),
      Self::NativeMapDiffKeys => write!(f, "&map:diff-keys"),
      Self::NativeMapCommonKeys => write!(f, "&map:common-keys"),
      Self::Set => write!(f, "#{{}}"),
      Self::NativeInclude => write!(f, "&include"),
      Self::NativeExclude => write!(f, "&exclude"),
      Self::NativeDifference => write!(f, "&difference"),
      Self::NativeUnion => write!(f, "&union"),
      Self::NativeSetIntersection => write!(f, "&set:intersection"),
      Self::NativeSetToList => write!(f, "&set:to-list"),
      Self::NativeSetCount => write!(f, "&set:count"),
      Self::NativeSetEmpty => write!(f, "&set:empty?"),
      Self::NativeSetIncludes => write!(f, "&set:includes?"),
      Self::NativeSetFirst => write!(f, "&set:first"),
      Self::NativeSetRest => write!(f, "&set:rest"),
      Self::NativeSetAssoc => write!(f, "&set:assoc"),
      Self::Atom => write!(f, "atom"),
      Self::Deref => write!(f, "deref"),
      Self::AddWatch => write!(f, "add-watch"),
      Self::RemoveWatch => write!(f, "remove-watch"),
      Self::NewRecord => write!(f, "new-record"),
      Self::NativeRecord => write!(f, "&%{{}}"),
      Self::NativeRecordMatches => write!(f, "&record:matches?"),
      Self::NativeRecordFromMap => write!(f, "&record:from-map"),
      Self::NativeRecordGetName => write!(f, "&record:get-name"),
      Self::NativeRecordToMap => write!(f, "&record:to-map"),
      Self::NativeRecordCount => write!(f, "&record:count"),
      Self::NativeRecordContains => write!(f, "&record:contains?"),
      Self::NativeRecordGet => write!(f, "&record:get"),
      Self::NativeRecordAssoc => write!(f, "&record:assoc"),
      Self::NativeRecordExtendAs => write!(f, "&record:extend-as"),
    }
  }
}
