; ModuleID = 'probe5.bf4acc597fd4882d-cgu.0'
source_filename = "probe5.bf4acc597fd4882d-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@alloc_2e38410fced2c310c68bdf2d45d0c3bd = private unnamed_addr constant [4 x i8] c"\02\00\00\00", align 4
@alloc_7971f3465817cc18ad816e3dbdd7087a = private unnamed_addr constant [7 x i8] c"<anon>\00", align 1
@alloc_1d9e4a30726589abce1472f3c301cfd2 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7971f3465817cc18ad816e3dbdd7087a, [16 x i8] c"\06\00\00\00\00\00\00\00\01\00\00\00+\00\00\00" }>, align 8

; probe5::probe
; Function Attrs: nonlazybind uwtable
define void @_RNvCsgqf77Hl6S9B_6probe55probe() unnamed_addr #0 {
start:
  %x = alloca [4 x i8], align 4
  store i32 1, ptr %x, align 4
; call <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
  call void @_RNvXs5R_NtNtCs4NRVxsYgnAr_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCsgqf77Hl6S9B_6probe5(ptr align 4 %x, ptr align 4 @alloc_2e38410fced2c310c68bdf2d45d0c3bd, ptr align 8 @alloc_1d9e4a30726589abce1472f3c301cfd2)
  ret void
}

; <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @_RNvXs5R_NtNtCs4NRVxsYgnAr_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCsgqf77Hl6S9B_6probe5(ptr align 4 %self, ptr align 4 %other, ptr align 8 %0) unnamed_addr #1 {
start:
  %other1 = load i32, ptr %other, align 4
  %1 = load i32, ptr %self, align 4
  %2 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %1, i32 %other1)
  %_4.0 = extractvalue { i32, i1 } %2, 0
  %_4.1 = extractvalue { i32, i1 } %2, 1
  br i1 %_4.1, label %panic, label %bb1

bb1:                                              ; preds = %start
  store i32 %_4.0, ptr %self, align 4
  ret void

panic:                                            ; preds = %start
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const24panic_const_add_overflow(ptr align 8 %0) #4
  unreachable
}

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; core::panicking::panic_const::panic_const_add_overflow
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const24panic_const_add_overflow(ptr align 8) unnamed_addr #3

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none) }
attributes #3 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #4 = { noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.97.1 (8bab26f4f 2026-07-14)"}
