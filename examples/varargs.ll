; ModuleID = 'main'
source_filename = "main"

declare i32 @add(i32 %0, i32* %1)

define i32 @main() {
entry:
  %main = alloca i32, align 4
  store i32 0, i32* %main, align 4
  %varargs = alloca [3 x i32], align 4
  store [3 x i32] [i32 1, i32 2, i32 3], [3 x i32]* %varargs, align 4
  %0 = alloca i32, align 4
  store i32 3, i32* %0, align 4
  %xxx = bitcast [3 x i32]* %varargs to i32*
  %call = call i32 @add(i32* %0, i32* %xxx)
  store i32 %call, i32* %main, align 4
  %main_ret = load i32, i32* %main, align 4
  ret i32 %main_ret
}