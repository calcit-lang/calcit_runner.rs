
{} (:package |calcit)
  :configs $ {}
    :init-fn |TODO
    :reload-fn |TODO
    :version |TODO
    :modules $ []
  :files $ {}
    |calcit.core $ {}
      :ns $ quote
        ns calcit.core $ :require
      :defs $ {}
        |if-not $ quote
          defmacro if-not (cond true-branch & args)
            &let
              false-branch $ either (first args) nil
              quote-replace $ if ~cond ~false-branch ~true-branch

        |not= $ quote
          defn not= (x y) $ not $ &= x y

        |&<= $ quote
          defn &<= (a b)
            if (&< a b) true (&= a b)

        |&>= $ quote
          defn &>= (a b)
            if (&> a b) true (&= a b)

        |first $ quote
          defn first (xs) (get xs 0)

        |last $ quote
          defn last (xs)
            &get xs
              &- (count xs) 1

        |when $ quote
          defmacro when (condition & body)
            quote-replace $ if ~condition (&let nil ~@body)

        |when-not $ quote
          defmacro when-not (condition & body)
            quote-replace $ if (not ~condition) (&let nil ~@body)

        |+ $ quote
          defn + (x & ys) $ reduce ys x &+

        |- $ quote
          defn - (x & ys)
            if (empty? ys)
              &- 0 x
              reduce ys x &-

        |* $ quote
          defn * (x & ys) $ reduce ys x &*

        |/ $ quote
          defn / (x & ys)
            if (empty? ys)
              &/ 1 x
              reduce ys x &/

        |foldl-compare $ quote
          defn foldl-compare (xs acc f)
            if (empty? xs) true
              if (f acc (first xs))
                recur (rest xs) (first xs) f
                , false

        |foldl' $ quote
          defn foldl' (xs acc f)
            if (empty? xs) acc
              recur (rest xs) (f acc (first xs)) f

        |< $ quote
          defn < (x & ys)
            if
              &= 1 (count ys)
              &< x (first ys)
              foldl-compare ys x &<

        |> $ quote
          defn > (x & ys)
            if
              &= 1 (count ys)
              &> x (first ys)
              foldl-compare ys x &>

        |= $ quote
          defn = (x & ys)
            if
              &= 1 (count ys)
              &= x (first ys)
              foldl-compare ys x &=

        |>= $ quote
          defn >= (x & ys)
            if
              &= 1 (count ys)
              &>= x (first ys)
              foldl-compare ys x &>=

        |<= $ quote
          defn <= (x & ys)
            if
              &= 1 (count ys)
              &<= x (first ys)
              foldl-compare ys x &<=

        |apply $ quote
          defn apply (f args) $ f & args

        |apply-args $ quote
          defn apply-args (args f) $ f & args

        |list? $ quote
          defn list? (x) $ &= (type-of x) :list

        |map? $ quote
          defn map? (x) $ &= (type-of x) :map

        |number? $ quote
          defn number? (x) $ &= (type-of x) :number

        |string? $ quote
          defn string? (x) $ &= (type-of x) :string

        |symbol? $ quote
          defn symbol? (x) $ &= (type-of x) :symbol

        |keyword? $ quote
          defn keyword? (x) $ &= (type-of x) :keyword

        |bool? $ quote
          defn bool? (x) $ &= (type-of x) :bool

        |nil? $ quote
          defn nil? (x) $ &= (type-of x) :nil

        |record? $ quote
          defn record? (x) $ &= (type-of x) :record

        |macro? $ quote
          defn macro? (x) $ &= (type-of x) :macro

        |set? $ quote
          defn set? (x) $ &= (type-of x) :set

        |fn? $ quote
          defn fn? (x)
            if
              &= (type-of x) :fn
              , true
              &= (type-of x) :proc

        |each $ quote
          defn each (xs f)
            if (not (empty? xs))
              &let nil
                f (first xs)
                recur (rest xs) f

        |map $ quote
          defn map (xs f)
            cond
              (list? xs)
                reduce xs ([])
                  fn (acc x) $ append acc (f x)
              (set? xs)
                reduce xs (#{})
                  fn (acc x) $ include acc (f x)
              true
                raise "|expects list or set for map function"

        |take $ quote
          defn take (xs n)
            if (= n (count xs)) xs
              slice xs 0 n

        |drop $ quote
          defn drop (xs n)
            slice xs n (count xs)

        |str $ quote
          defmacro str (x0 & xs)
            if (empty? xs)
              quote-replace $ &str ~x0
              quote-replace $ &str-concat ~x0 $ str ~@xs

        |include $ quote
          defn include (base & xs)
            reduce xs base
              fn (acc item) $ &include acc item

        |exclude $ quote
          defn exclude (base & xs)
            reduce xs base
              fn (acc item) $ &exclude acc item

        |difference $ quote
          defn difference (base & xs)
            reduce xs base
              fn (acc item) $ &difference acc item

        |union $ quote
          defn union (base & xs)
            reduce xs base
              fn (acc item) $ &union acc item

        |intersection $ quote
          defn intersection (base & xs)
            reduce xs base
              fn (acc item) $ &intersection acc item

        |index-of $ quote
          defn index-of (xs0 item)
            apply-args
              [] 0 xs0
              fn (idx xs)
                if (empty? xs) nil
                  if (&= item (first xs)) idx
                    recur (&+ 1 idx) (rest xs)

        |find-index $ quote
          defn find-index (xs0 f)
            apply-args
              [] 0 xs0
              fn (idx xs)
                if (empty? xs) nil
                  if (f (first xs)) idx
                    recur (&+ 1 idx) (rest xs)

        |find $ quote
          defn find (xs f)
            &let
              idx (find-index xs f)
              if (nil? idx) nil (get xs idx)

        |-> $ quote
          defmacro -> (base & xs)
            if (empty? xs)
              quote-replace ~base
              &let
                x0 (first xs)
                if (list? x0)
                  recur
                    concat ([] (first x0) base) (rest x0)
                    , & (rest xs)
                  recur ([] x0 base) & (rest xs)

        |->> $ quote
          defmacro ->> (base & xs)
            if (empty? xs)
              quote-replace ~base
              &let
                x0 (first xs)
                if (list? x0)
                  recur (append x0 base) & (rest xs)
                  recur ([] x0 base) & (rest xs)

        |->% $ quote
          defmacro ->% (base & xs)
            if (empty? xs) base
              let
                  tail $ last xs
                  pairs $ concat
                    [] $ [] '% base
                    map
                      butlast xs
                      fn (x) ([] '% x)
                quote-replace
                  let ~pairs ~tail

        |cond $ quote
          defmacro cond (pair & else)
            assert "|expects a pair"
              if (list? pair) (&= 2 (count pair)) false
            &let
              expr $ nth pair 0
              &let
                branch $ nth pair 1
                quote-replace
                  if ~expr ~branch
                    ~ $ if (empty? else) nil
                      quote-replace
                        cond
                          ~ $ nth else 0
                          ~@ $ rest else

        |&case $ quote
          defmacro &case (item default pattern & others)
            assert "|expects pattern in a pair"
              if (list? pattern) (&= 2 (count pattern)) false
            let
                x $ first pattern
                branch $ last pattern
              quote-replace
                if (&= ~item ~x) ~branch
                  ~ $ if (empty? others) default
                    quote-replace
                      &case ~item ~default ~@others

        |case $ quote
          defmacro case (item & patterns)
            &let
              v (gensym |v)
              quote-replace
                &let
                  ~v ~item
                  &case ~v nil ~@patterns

        |case-default $ quote
          defmacro case (item default & patterns)
            if (empty? patterns)
              raise "|Expected patterns for case-default, got empty"
            &let
              v (gensym |v)
              quote-replace
                &let (~v ~item)
                  &case ~v ~default ~@patterns

        |get $ quote
          defn get (base k)
            cond
              (nil? base) nil
              (string? base) (nth base k)
              (map? base) (&get base k)
              (list? base) (nth base k)
              (record? base) (&get base k)
              true $ raise "|Expected map or list for get"

        |get-in $ quote
          defn get-in (base path)
            assert "|expects path in a list" (list? path)
            cond
              (nil? base) nil
              (empty? path) base
              true
                recur
                  get base (first path)
                  rest path

        |&max $ quote
          defn &max (a b)
            assert "|expects numbers for &max" $ if (number? a) (number? b) false
            if (&> a b) a b

        |&min $ quote
          defn &min (a b)
            assert "|expects numbers for &min" $ if (number? a) (number? b) false
            if (&< a b) a b

        |max $ quote
          defn max (xs)
            if (empty? xs) nil
              reduce (rest xs) (first xs)
                fn (acc x) (&max acc x)

        |min $ quote
          defn min (xs)
            if (empty? xs) nil
              reduce (rest xs) (first xs)
                fn (acc x) (&min acc x)

        |every? $ quote
          defn every? (xs f)
            if (empty? xs) true
              if (f (first xs))
                recur (rest xs) f
                , false

        |any? $ quote
          defn any? (xs f)
            if (empty? xs) false
              if (f (first xs)) true
                recur (rest xs) f

        |mapcat $ quote
          defn mapcat (xs f)
            concat & $ map xs f

        |merge $ quote
          defn merge (x0 & xs)
            reduce xs x0 &merge

        |merge-non-nil $ quote
          defn merge-non-nil (x0 & xs)
            reduce xs x0 &merge-non-nil

        |identity $ quote
          defn identity (x) x

        |map-indexed $ quote
          defn map-indexed (xs f)
            apply-args
              [] ([]) 0 xs
              fn (acc idx ys)
                if (empty? ys) acc
                  recur
                    append acc (f idx (first ys))
                    &+ idx 1
                    rest ys

        |filter $ quote
          defn filter (xs f)
            foldl xs ([])
              fn (acc x)
                if (f x) (append acc x) acc

        |filter-not $ quote
          defn filter-not (xs f)
            reduce xs ([])
              fn (acc x)
                if-not (f x) (append acc x) acc

        |pairs-map $ quote
          defn pairs-map (xs)
            reduce xs ({})
              fn (acc pair)
                assert "|expects pair for pairs-map"
                  if (list? pair)
                    &= 2 (count pair)
                    , false
                assoc acc (first pair) (last pair)

        |some? $ quote
          defn some? (x) $ not $ nil? x

        |some-in? $ quote
          defn some-in? (x path)
            if (nil? x) false
              if (empty? path) true
                &let (k $ first path)
                  if (map? x)
                    if (contains? x k)
                      recur (get x k) (rest path)
                      , false
                    if (list? x)
                      if (number? k)
                        recur (get x k) (rest path)
                        , false
                      raise $ &str-concat "|Unknown structure for some-in? detection" x


        |zipmap $ quote
          defn zipmap (xs0 ys0)
            apply-args
              [] ({})xs0 ys0
              fn (acc xs ys)
                if
                  if (empty? xs) true (empty? ys)
                  , acc
                  recur
                    assoc acc (first xs) (first ys)
                    rest xs
                    rest ys

        |rand-nth $ quote
          defn rand-nth (xs)
            if (empty? xs) nil
              get xs $ rand-int $ &- (count xs) 1

        |contains-symbol? $ quote
          defn contains-symbol? (xs y)
            if (list? xs)
              apply-args
                [] xs
                fn (body)
                  if (empty? body) false
                    if
                      contains-symbol? (first body) y
                      , true
                      recur (rest body)
              &= xs y

        |\ $ quote
          defmacro \ (& xs)
            quote-replace $ fn (? % %2) ~xs

        |\. $ quote
          defmacro \. (args-alias & xs)
            &let
              args $ ->% (turn-string args-alias) (split % |.) (map % turn-symbol)
              &let
                inner-body $ if (&= 1 (count xs)) (first xs)
                  quasiquote
                    &let nil ~@xs
                apply-args
                  [] inner-body args
                  fn (body ys)
                    if (empty? ys)
                      quote-replace ~body
                      &let
                        a0 (last ys)
                        &let
                          code
                            [] (quote-replace defn) (turn-symbol (&str-concat |f_ (turn-string a0))) ([] a0) body
                          recur code (butlast ys)

        |has-index? $ quote
          defn has-index? (xs idx)
            assert "|expects a list" (list? xs)
            assert "|expects list key to be a number" (number? idx)
            assert "|expects list key to be an integer" (&= idx (floor idx))
            if
              &> idx 0
              &< idx (count xs)
              , false

        |update $ quote
          defn update (x k f)
            cond
              (list? x)
                if (has-index? x k)
                  assoc x k $ f (nth x k)
                  , x
              (map? x)
                if (contains? x k)
                  assoc x k $ f (&get x k)
                  , x
              (record? x)
                if (contains? x k)
                  assoc x k $ f (&get x k)
                  , x
              true
                raise $ &str-concat "|Cannot update key on item: " x

        |group-by $ quote
          defn group-by (xs0 f)
            apply-args
              [] ({}) xs0
              fn (acc xs)
                if (empty? xs) acc
                  let
                      x0 $ first xs
                      key $ f x0
                    recur
                      if (contains? acc key)
                        update acc key $ \ append % x0
                        assoc acc key $ [] x0
                      rest xs

        |keys $ quote
          defn keys (x)
            map (to-pairs x) first

        |keys-non-nil $ quote
          defn keys (x)
            apply-args
              [] (#{}) (to-pairs x)
              fn (acc pairs)
                if (empty? pairs) acc
                  &let
                    pair $ first pairs
                    if (nil? (last pair))
                      recur acc (rest pairs)
                      recur (include acc (first pair))
                        rest pairs

        |vals $ quote
          defn vals (x)
            map last (to-pairs x)

        |frequencies $ quote
          defn frequencies (xs0)
            assert "|expects a list for frequencies" (list? xs0)
            apply-args
              [] ({}) xs0
              fn (acc xs)
                &let
                  x0 (first xs)
                  if (empty? xs) acc
                    recur
                      if (contains? acc (first xs))
                        update acc (first xs) (\ &+ % 1)
                        assoc acc (first xs) 1
                      rest xs

        |section-by $ quote
          defn section-by (xs0 n)
            apply-args
              [] ([]) xs0
              fn (acc xs)
                if (&<= (count xs) n)
                  append acc xs
                  recur
                    append acc (take n xs)
                    drop n xs

        |[][] $ quote
          defmacro [][] (& xs)
            &let
              items $ map xs
                fn (ys) $ quote-replace $ [] ~@ys
              quote-replace $ [] ~@items

        |{} $ quote
          defmacro {} (& xs)
            &let
              ys $ concat & xs
              quote-replace $ &{} ~@ys

        |%{} $ quote
          defmacro %{} (R & xs)
            &let
              args $ concat & xs
              quote-replace $ &%{} ~R ~@args

        |fn $ quote
          defmacro fn (args & body)
            quote-replace $ defn f% ~args ~@body

        |assert= $ quote
          defmacro assert= (a b)
            &let
              va $ gensym |va
              &let
                vb $ gensym |vb
                quote-replace
                  &let
                    ~va ~a
                    &let
                      ~vb ~b
                      if (not= ~va ~vb)
                        &let nil
                          echo
                          echo "|Left: " ~va
                          echo "|      " $ format-to-lisp $ quote ~a
                          echo "|Right:" ~vb
                          echo "|      " $ format-to-lisp $ quote ~b
                          raise "|not equal in assertion!"

        |assert-detect $ quote
          defmacro assert-detect (f code)
            &let
              v $ gensym |v
              quote-replace
                &let
                  ~v ~code
                  if (~f ~v) nil
                    &let nil
                      echo
                      echo (quote ~code) "|does not satisfy:" (quote ~f) "| <--------"
                      echo "|  value is:" ~v
                      raise "|Not satisfied in assertion!"

        |swap! $ quote
          defmacro swap! (a f & args)
            quote-replace
              reset! ~a
                ~f (deref ~a) ~@args

        |assoc-in $ quote
          defn assoc-in (data path v)
            if (empty? path) v
              let
                  p0 $ first path
                  d $ either data $ &{}
                assoc d p0
                  assoc-in
                    if (contains? d p0) (get d p0) (&{})
                    rest path
                    , v

        |update-in $ quote
          defn update-in (data path f)
            if (empty? path)
              f data
              &let
                p0 $ first path
                assoc data p0
                  update-in (get data p0) (rest path) f

        |dissoc-in $ quote
          defn dissoc-in (data path)
            cond
              (empty? path) nil
              (&= 1 (count path))
                dissoc data (first path)
              true
                &let
                  p0 $ first path
                  assoc data p0
                    dissoc-in (get data p0) (rest path)

        |inc $ quote
          defn inc (x) $ &+ x 1

        |dec $ quote
          defn dec (x) $ &- x 1

        |starts-with? $ quote
          defn starts-with? (x y)
            &= 0 (str-find x y)

        |ends-with? $ quote
          defn ends-with? (x y)
            &=
              &- (count x) (count y)
              str-find x y

        |loop $ quote
          defmacro loop (pairs & body)
            assert "|expects pairs in loop" (list? pairs)
            assert "|expects pairs in pairs in loop"
              every? pairs
                defn detect-pairs? (x)
                  if (list? x)
                    &= 2 (count x)
                    , false
            let
                args $ map pairs first
                values $ map pairs last
              assert "|loop requires symbols in pairs" (every? args symbol?)
              quote-replace
                apply
                  defn generated-loop ~args ~@body
                  [] ~@values

        |let $ quote
          defmacro let (pairs & body)
            assert "|expects pairs in list for let" (list? pairs)
            if (&= 1 (count pairs))
              quote-replace
                &let
                  ~ $ nth pairs 0
                  ~@ body
              if (empty? pairs)
                quote-replace $ &let nil ~@body
                quote-replace
                  &let
                    ~ $ nth pairs 0
                    let
                      ~ $ rest pairs
                      ~@ body

        |let-sugar $ quote
          defmacro let-sugar (pairs & body)
            assert "|expects pairs in list for let" (list? pairs)
            if (empty? pairs)
              quote-replace $ &let nil ~@body
              &let
                pair $ first pairs
                assert "|expected pair length of 2" (&= 2 (count pair))
                if (&= 1 (count pairs))
                  quote-replace
                    let-destruct ~@pair
                      ~@ body
                  quote-replace
                    let-destruct ~@pair
                      let-sugar
                        ~ $ rest pairs
                        ~@ body

        |let-destruct $ quote
          defmacro let-destruct (pattern v & body)
            if (symbol? pattern)
              quote-replace
                &let (~pattern ~v) ~@body
              if (list? pattern)
                if (&= '[] (first pattern))
                  quote-replace
                    let[] (~ (rest pattern)) ~v ~@body
                  if (&= '{} (first pattern))
                    quote-replace
                      let{} (~ (rest pattern)) ~v ~@body
                    &let nil
                      echo pattern
                      raise "|Unknown pattern to destruct"
                raise "|Unknown structure to destruct"

        |[,] $ quote
          defmacro [,] (& body)
            &let
              xs $ filter body
                fn (x) (/= x ',)
              quote-replace $ [] ~@xs

        |assert $ quote
          defmacro assert (message xs)
            if
              if (string? xs) (not (string? message)) false
              quote-replace $ assert ~xs ~message
              quote-replace
                &let nil
                  if (not (string? ~message))
                    raise "|expects 1st argument to be string"
                  if ~xs nil
                    &let nil
                      echo "|Failed assertion:" (quote ~xs)
                      raise
                        ~ $ &str-concat (&str-concat message "| ") xs

        |println $ quote
          defn println (& xs)
            print & xs
            when
              = (&get-calcit-backend) :nim
              print "|\n"

        |echo $ quote
          def echo println

        |join-str $ quote
          defn join-str (xs0 sep)
            apply-args
              [] | xs0 true
              fn (acc xs beginning?)
                if (empty? xs) acc
                  recur
                    &str-concat
                      if beginning? acc $ &str-concat acc sep
                      first xs
                    rest xs
                    , false

        |join $ quote
          defn join (xs0 sep)
            apply-args
              [] ([]) xs0 true
              fn (acc xs beginning?)
                if (empty? xs) acc
                  recur
                    append
                      if beginning? acc (append acc sep)
                      first xs
                    rest xs
                    , false

        |repeat $ quote
          defn quote (x n0)
            apply-args
              [] ([]) n0
              fn (acc n)
                if (&<= n 0) acc
                  recur (append acc x) (&- n 1)

        |interleave $ quote
          defn interleave (xs0 ys0)
            apply-args
              [] ([]) xs0 ys0
              fn (acc xs ys)
                if
                  if (empty? xs) true (empty? ys)
                  , acc
                  recur
                    -> acc (append (first xs)) (append (first ys))
                    rest xs
                    rest ys

        |map-kv $ quote
          defn map-kv (dict f)
            assert "|expects a map" (map? dict)
            -> dict
              to-pairs
              map $ fn (pair)
                f (first pair) (last pair)

        |either $ quote
          defmacro either (x y)
            quote-replace $ if (nil? ~x) ~y ~x

        |def $ quote
          defmacro def (name x) x

        |and $ quote
          defmacro and (item & xs)
            if (empty? xs)
              quote-replace
                if ~item ~item false
              quote-replace
                if ~item
                  and
                    ~ $ first xs
                    ~@ $ rest xs
                  , false

        |or $ quote
          defmacro or (item & xs)
            if (empty? xs) item
              quote-replace
                if ~item ~item
                  or
                    ~ $ first xs
                    ~@ $ rest xs

        |with-log $ quote
          defmacro with-log (x)
            &let
              v $ gensym |v
              quote-replace
                &let
                  ~v ~x
                  echo (format-to-lisp (quote ~x)) |=> ~v
                  ~ v

        |{,} $ quote
          defmacro {,} (& body)
            &let
              xs $ filter body
                fn (x) (not= x ',)
              quote-replace
                pairs-map $ section-by ([] ~@xs) 2

        |&doseq $ quote
          defmacro &doseq (pair & body)
            assert "|doseq expects a pair"
              if (list? pair) (&= 2 (count pair)) false
            let
                name $ first pair
                xs0 $ last pair
              quote-replace
                apply
                  defn doseq-fn% (xs)
                    if (empty? xs) nil
                      &let
                        ~name $ first xs
                        ~@ body
                        recur $ rest xs
                  [] ~xs0

        |with-cpu-time $ quote
          defmacro with-cpu-time (x)
            let
                started $ gensym |started
                v $ gensym |v
              quote-replace
                let
                    ~started (cpu-time)
                    ~v ~x
                  echo "|[cpu-time]" (quote ~x) |=>
                    format-number
                      &* 1000 (&- (cpu-time) ~started)
                      , 3
                    , |ms
                  ~ v

        |call-with-log $ quote
          defmacro call-with-log (f & xs)
            let
                v $ gensym |v
                args-value $ gensym |args-value
              quote-replace
                let
                    ~args-value $ [] ~@xs
                    ~v $ ~f & ~args-value
                  echo "|call:"
                    format-to-lisp $ quote (call-with-log ~f ~@xs)
                    , |=> ~v
                  echo "|f:   " ~f
                  echo "|args:" ~args-value
                  ~ v

        |defn-with-log $ quote
          defmacro defn-with-log (f-name args & body)
            quote-replace
              defn ~f-name ~args
                &let
                  ~f-name $ defn ~f-name ~args ~@body
                  call-with-log ~f-name ~@args

        |do $ quote
          defmacro do (& body)
            ; echo "|body:" (format-to-lisp body)
            quasiquote
              &let nil
                ~@ body

        |let{} $ quote
          defmacro let{} (items base & body)
            assert (str "|expects symbol names in binding names: " items)
              if (list? items) (every? items symbol?) false
            let
                var-result $ gensym |result
              quote-replace
                &let
                  ~var-result ~base
                  assert (str "|expected map for destructing: " ~var-result) (map? ~var-result)
                  let
                    ~ $ map items
                      defn gen-items% (x)
                        [] x ([] (turn-keyword x) var-result)
                    ~@ body

        |let[] $ quote
          defmacro let[] (vars data & body)
            assert "|expects a list of definitions"
              if (list? vars)
                every? vars symbol?
                , false
            let
                v $ gensym |v
                defs $ apply-args
                  [] ([]) vars 0
                  defn let[]% (acc xs idx)
                    if (empty? xs) acc
                      &let nil
                        when-not
                          symbol? (first xs)
                          raise $ &str-concat "|Expected symbol for vars: " (first xs)
                        if (&= (first xs) '&)
                          &let nil
                            assert "|expected list spreading" (&= 2 (count xs))
                            conj acc $ [] (get xs 1) (quote-replace (slice ~v ~idx))
                          recur
                            conj acc $ [] (first xs) (quote-replace (get ~v ~idx))
                            rest xs
                            inc idx
              quote-replace
                &let
                  ~v ~data
                  let
                    ~ defs
                    ~@ body

        |defrecord $ quote
          defmacro defrecord (name & xs)
            quote-replace
              new-record (quote ~name) ~@xs

        |conj $ quote
          defn conj (xs a)
            append xs a

        |turn-str $ quote
          defn turn-str (x) (turn-string x)

        |reduce $ quote
          defn reduce (xs x0 f)
            foldl xs x0 f

        |dbt $ quote
          def dbt dual-balanced-ternary

        |/= $ quote
          defn /= (a b) (not= a b)
