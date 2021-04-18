main:
  // stack stuff
  push rbp

  // clear eax / rax
  xor eax, eax

  // stack stuff
  mov rbp, rsp

  // move stack pointer make space for arr1
  sub rsp, 1600000

  // rdx = arr1[0]
  mov rdx, rsp

  // move stack pointer make space for arr2
  sub rsp, 1600000

  // rcx = arr2[0]
  mov rcx, rsp
.L2:
  // add 1 to rax
  add rax, 1

  // clear simd register xmm0
  pxor xmm0, xmm0

  // xmm0[63:0] = rax
  // xmm0[127:64] = xmm0[127:64] which is 0
  cvtsi2sd xmm0, rax

  // fill arr1
  movsd QWORD PTR [rdx-8+rax*8], xmm0

  // fill arr1
  movsd QWORD PTR [rcx-8+rax*8], xmm0

  // loop condition
  cmp rax, 200000
  jne .L2

  // clear eax / rax
  xor eax, eax

  // clear xmm1
  // xmm1 = dot_p
  pxor xmm1, xmm1
.L3:
  // item1 = _mm_loadu_pd(arr1[index])
  movupd xmm0, XMMWORD PTR [rcx+rax]

  // item2 = _mm_loadu_pd(arr2[index])
  movupd xmm2, XMMWORD PTR [rdx+rax]

  // move 2 elements worth of length
  add rax, 16

  // multiplication = _mm_mul_pd(item1, item2)
  mulpd xmm0, xmm2

  // _mm_add_pd(multiplication, dot_p)
  addpd xmm1, xmm0

  // loop condition
  cmp rax, 1600000
  jne .L3

  // After loop

  // xmm0 = buffer[f64;2]
  // xmm1 contains sum = [1,0]
  // Copy the sum(2 x f64) to xmm0 from xmm1
  movapd xmm0, xmm1

  // xmm0 = [1,0]
  // xmm1 = [1,0]

  // Preparing to call libc function `printf`
  mov edi, OFFSET FLAT:.LC0
  mov eax, 1

  // xmm0 = [1,1]
  unpckhpd xmm0, xmm1

  // xmm0 =  [1,0] + [1,1]
  addpd xmm0, xmm1
  call printf
  xor eax, eax
  leave
  ret
