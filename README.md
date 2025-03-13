# xenosphere-rs
visual graph engine for AI modeling

## xenosphere-core 

should include read / write data model layer (protobuf data), rewrite 


---

## xenosphere-parser

mainly parse the engine script language (`xesl`) to AST, 

// ? then transpile as LLVM's IR 

default script parser in script part (pass) are clang / cpp 

while other script support should build with custom transpiler / processer.
(e.g.xenosphere-transpiler-golang)

---

## xenosphere-generater

based on AST object, to generate target platform project (e.g. pytorch / rust burn / cuda)



---


## xenosphere-marco 



