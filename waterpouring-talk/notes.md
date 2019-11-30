# Waterpouring in Rust: a performance study

alternate name: How fast is Rust ?

## Intro

## Water Pouring
 
 - Demo
 - rules
 - code
 
## Algorithm

- principle
- code (available, move)
- TailRec
- TailRec2
- Imp

## Performance

- before bench: Test

- benches
 [Unstable](https://doc.rust-lang.org/unstable-book/library-features/test.html#test)
 [cargo benchcmp](https://github.com/BurntSushi/cargo-benchcmp)
 
- [criterion](https://github.com/bheisler/criterion.rs) (need gnuplot)
  [critcmp](https://github.com/BurntSushi/critcmp)
  Note extension: [criterion-cycles-per-byte ](https://gitlab.com/sio4/code/criterion-cycles-per-byte)

- demo / livecode

- [flamegraph](https://github.com/ferrous-systems/flamegraph) 
- [hyperfine](https://github.com/sharkdp/hyperfine) 

## How fast is Rust

- vs JVM JIT (JMH)
- in CLI, vs: JVM, NodeJs, Go?,  OCaml? with [hyperfine](https://github.com/sharkdp/hyperfine)
  vs KotlinNative, ScalaNative, GraalVM
  
JVM:
  - Kotlin tailrec 
  - Scala tailrec 
  - Java Imp 
  - Clojure ?
JVM native: 
  - kotlin-native
  - kotlin-graal
  - scala-native
  - scala-graal
  - Java-graal
Dynamic:
  - js-node
  - python ?
  - ruby ?
Other:
  - C ?
  - C++ ?
  - OCaml ?
  - Haskell ?
  - Go ?
  - Swift
 
## Conclusion

As ton besoin de perf
Perf => mesure
TEST before
Criterion is cool
Attention à ne pas faire dire au chiffre ce que l'on veut

Langages:

- learn != language
- does we need perf ?
- Rust seems pretty good with perf, at least for developper like me
