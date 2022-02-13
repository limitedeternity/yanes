use std::sync::{Mutex, LockResult};

use lazy_static::lazy_static;

use crate::cpu::CPU;

lazy_static! {
    static ref CPU_SINGLETON: Mutex<CPU> = Mutex::new(CPU::new()); 
}

trait LockResultExt {
    type Guard;

    fn ignore_poison(self) -> Self::Guard;
}

impl<Guard> LockResultExt for LockResult<Guard> {
    type Guard = Guard;

    fn ignore_poison(self) -> Guard {
        self.unwrap_or_else(|e| e.into_inner())
    }
}

#[test]
fn test_0xa9_lda_immidiate_load_data() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![0xa9, 0x05, 0x00]);
    cpu.run();

    assert!(*cpu.a() == 0x05);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![0xa9, 0x00, 0x00]);
    cpu.run();

    assert!(cpu.p().Z()); // zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0xa9_lda_negative_flag() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![0xa9, 0xff, 0x00]);
    cpu.run();

    assert!(!cpu.p().Z()); // non-zero
    assert!(cpu.p().N()); // negative
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![0xa9, 0x05, 0xaa, 0x00]);
    cpu.run();

    assert!(*cpu.a() == *cpu.x()); // ensure transfer
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0xe8_increment_x() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![0xa9, 0x3e, 0xaa, 0xe8, 0x00]);
    cpu.run();

    assert!(*cpu.x() == 0x3f);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0xe8_overflow() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
    cpu.run();

    assert!(*cpu.x() == 0x01);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0x85_0xa5_sta_lda() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x3a, // mov $a, 0x3a
        0x85, 0x30, // mov byte ptr [0x30], $a
        0xa9, 0x00, // mov $a, 0x00
        0xa5, 0x30, // mov $a, word byte [0x30]
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0x8d_0xad_sta_lda() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x3a, // mov $a, 0x3a
        0x8d, 0x10, 0x55, // mov byte ptr [0x5510], $a
        0xa9, 0x00, // mov $a, 0x00
        0xad, 0x10, 0x55, // mov $a, byte ptr [0x5510]
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0x81_0xa1_sta_lda() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x10, // mov $a, 0x10
        0xaa, // mov $x, $a
        0xa9, 0x3a, // mov $a, 0x3a
        0x81, 0x30, // mov byte ptr [0x30 + $x], $a
        0xa9, 0x00, // mov $a, 0x00
        0xa1, 0x30, // mov $a, byte ptr [0x30 + $x]
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0x48_pha_can_push() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x3a, // mov $a, 0x3a
        0x48, // push $a
        0xad, 0xFF, 0x01, // mov $a, byte ptr [0x01FF]
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0x48_0x68_pha_pla_push_pop() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x3a, // mov $a, 0x3a
        0x48, // push $a
        0xa9, 0x10, // mov $a, 0x10
        0x68, // pop $a
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
}

#[test]
fn test_0x48_0x68_pha_pla_multiple() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x3a, // mov $a, 0x3a
        0x48, // push $a
        0xa9, 0x2a, // mov $a, 0x2a
        0x48, // push $a
        0xa9, 0x1a, // mov $a, 0x1a
        0x48, // push $a
        0xa9, 0x00, // mov $a, 0x00
        0x68, // pop $a
        0xa8, // mov $y, $a
        0x68, // pop $a
        0xaa, // mov $x, $a
        0x68, // pop $a
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x3a);
    assert!(*cpu.x() == 0x2a);
    assert!(*cpu.y() == 0x1a);
}

#[test]
fn test_0x20_0x60_jsr_rts() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x3a, // 0x8000: mov $a, 0x3a
        0x20, 0x0D, 0x80, // 0x8002: call 0x800D
        0xad, 0xff, 0x01, // 0x8005: mov $a, byte ptr [0x01FF]
        0xa8, // 0x8008: mov $y, $a
        0xad, 0xFE, 0x01, // 0x8009: mov $a, byte ptr [0x01FE]
        0x00, // 0x800C: brk
        0xaa, // 0x800D: mov $x, $a
        0x60 // 0x800E: ret 
    ]);
    cpu.run();

    assert!(*cpu.x() == 0x3a);
    assert!(*cpu.y() == 0x80);
    assert!(*cpu.a() == 0x05);
}

#[test]
fn test_0x48_pha_single_capacity() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();
    let mut instructions = vec![
        0xa9, 0x3a // mov $a, 0x3a
    ];

    for _ in 0..0x100 {
        instructions.push(0x48);
    }

    instructions.push(0x00);

    cpu.reset();
    cpu.load(instructions);
    cpu.run();

    assert!(*cpu.sp() == 0xFF);
}

#[test]
#[should_panic]
fn test_0x48_pha_overflow() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();
    let mut instructions = vec![
        0xa9, 0x3a // mov $a, 0x3a
    ];

    for _ in 0..=0x100 {
        instructions.push(0x48);
    }

    instructions.push(0x00);

    cpu.reset();
    cpu.load(instructions);
    cpu.run();
}

#[test]
#[should_panic]
fn test_0x68_pla_underflow() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x3a,
        0x48, 0x68,
        0x68, 0x00
    ]);
    cpu.run();
}

#[test]
fn test_0x48_pha_call_capacity() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();
    let mut instructions = vec![];
    let mut jmp_delta = 3;
    for _ in 0..0x80 {
        instructions.push(0x20);
        match (0x8000 as u16 + jmp_delta).to_le_bytes() {
            [lo, hi] => {
                instructions.push(lo);
                instructions.push(hi);
            }
        }

        jmp_delta += 3;
    }

    cpu.reset();
    cpu.load(instructions);
    cpu.run();

    assert!(*cpu.sp() == 0xFF);
}

#[test]
fn test_0x4c_jmp() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0x4c, 0x00, 0x82,
        0x68, 0x00
    ]);
    cpu.run();

    assert!(*cpu.pc() == 0x8201);
}

#[test]
fn test_sbc_basic() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x00, 
        0xe9, 0x01, 
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0xFE);
    assert!(cpu.p().N());
    assert!(!cpu.p().Z());
    assert!(!cpu.p().C());
    assert!(!cpu.p().V());
}

#[test]
fn test_sbc_decimal_mode1() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x00,
        0xe9, 0x00,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x99);
    assert!(cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(!cpu.p().C());
}

#[test]
fn test_sbc_decimal_mode2() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0x38, // SEC
        0xa9, 0x00,
        0xe9, 0x00,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x00);
    assert!(!cpu.p().N());
    assert!(!cpu.p().V());
    assert!(cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_sbc_decimal_mode3() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0x38, // SEC
        0xa9, 0x00,
        0xe9, 0x01,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x99);
    assert!(cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(!cpu.p().C());
}

#[test]
fn test_sbc_decimal_mode4() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0x38, // SEC
        0xa9, 0x0a,
        0xe9, 0x00,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x0a);
    assert!(!cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_sbc_decimal_mode5() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x0b,
        0xe9, 0x00,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x0a);
    assert!(!cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_sbc_decimal_mode6() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0x38, // SEC
        0xa9, 0x9a,
        0xe9, 0x00,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x9a);
    assert!(cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_sbc_decimal_mode7() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x9b,
        0xe9, 0x00,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x9a);
    assert!(cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_adc_basic() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa9, 0x55,
        0x69, 0x55,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0xaa);
    assert!(cpu.p().N());
    assert!(cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(!cpu.p().C());
}

#[test]
fn test_adc_decimal_mode1() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x00,
        0x69, 0x00,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x00);
    assert!(!cpu.p().N());
    assert!(!cpu.p().V());
    assert!(cpu.p().Z());
    assert!(!cpu.p().C());
}

#[test]
fn test_adc_decimal_mode2() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0x38, // SEC
        0xa9, 0x79,
        0x69, 0x00,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x80);
    assert!(cpu.p().N());
    assert!(cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(!cpu.p().C());
}

#[test]
fn test_adc_decimal_mode3() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x24,
        0x69, 0x56,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x80);
    assert!(cpu.p().N());
    assert!(cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(!cpu.p().C());
}

#[test]
fn test_adc_decimal_mode4() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x93,
        0x69, 0x82,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x75);
    assert!(!cpu.p().N());
    assert!(cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_adc_decimal_mode5() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x89,
        0x69, 0x76,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x65);
    assert!(!cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_adc_decimal_mode6() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0x38, // SEC
        0xa9, 0x89,
        0x69, 0x76,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x66);
    assert!(!cpu.p().N());
    assert!(!cpu.p().V());
    assert!(cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_adc_decimal_mode7() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x80,
        0x69, 0xf0,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0xd0);
    assert!(!cpu.p().N());
    assert!(cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_adc_decimal_mode8() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x80,
        0x69, 0xfa,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0xe0);
    assert!(cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(cpu.p().C());
}

#[test]
fn test_adc_decimal_mode9() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0xa9, 0x2f,
        0x69, 0x4f,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x74);
    assert!(!cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(!cpu.p().C());
}

#[test]
fn test_adc_decimal_mode10() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xf8, // SED
        0x38, // SEC
        0xa9, 0x6f,
        0x69, 0x00,
        0x00
    ]);
    cpu.run();

    assert!(*cpu.a() == 0x76);
    assert!(!cpu.p().N());
    assert!(!cpu.p().V());
    assert!(!cpu.p().Z());
    assert!(!cpu.p().C());
}

#[test]
fn test_0x0d_bne() {
    let mut cpu = CPU_SINGLETON.lock().ignore_poison();

    cpu.reset();
    cpu.load(vec![
        0xa2, 0x02, // mov $x, 0x02
        0xca, // sub $x, 1
        0xd0, 0xfd, // cmp $x, 0; jne 0xfd 
        0x00
    ]); 
    cpu.run();

    assert!(*cpu.x() == 0x00);
    assert!(cpu.p().Z());
    assert!(!cpu.p().N());
}
