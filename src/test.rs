use crate::cpu::CPU;

#[test]
fn test_0xa9_lda_immidiate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
    assert!(*cpu.a() == 0x05);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
    assert!(cpu.p().Z()); // zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0xa9_lda_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
    assert!(!cpu.p().Z()); // non-zero
    assert!(cpu.p().N()); // negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0xaa, 0x00]);
    assert!(*cpu.a() == *cpu.x()); // ensure transfer
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0xe8_increment_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x3e, 0xaa, 0xe8, 0x00]);
    assert!(*cpu.x() == 0x3f);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0xe8_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0x00]);
    assert!(*cpu.x() == 0x01);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0x85_0xa5_sta_lda() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x3a, // mov $a, 0x3a
        0x85, 0x30, // mov byte ptr [0x30], $a
        0xa9, 0x00, // mov $a, 0x00
        0xa5, 0x30, // mov $a, word byte [0x30]
        0x00
    ]);

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0x8d_0xad_sta_lda() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x3a, // mov $a, 0x3a
        0x8d, 0x10, 0x55, // mov word ptr [0x5510], $a
        0xa9, 0x00, // mov $a, 0x00
        0xad, 0x10, 0x55, // mov $a, word ptr [0x5510]
        0x00
    ]);

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0x81_0xa1_sta_lda() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x10, // mov $a, 0x10
        0xaa, // mov $x, $a
        0xa9, 0x3a, // mov $a, 0x3a
        0x81, 0x30, // mov byte ptr [0x30 + $x], $a
        0xa9, 0x00, // mov $a, 0x00
        0xa1, 0x30, // mov $a, byte ptr [0x30 + $x]
        0x00
    ]);

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0x48_pha_can_push() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x3a, // mov $a, 0x3a
        0x48, // push $a
        0xad, 0xFF, 0x01, // mov $a, word ptr [0x01FF]
        0x00
    ]);

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0x48_0x68_pha_pla_push_pop() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x3a, // mov $a, 0x3a
        0x48, // push $a
        0xa9, 0x10, // mov $a, 0x10
        0x68, // pop $a
        0x00
    ]);

    assert!(*cpu.a() == 0x3a);
    assert!(!cpu.p().Z()); // non-zero
    assert!(!cpu.p().N()); // non-negative
    assert!(cpu.p().B()); // break
}

#[test]
fn test_0x48_0x68_pha_pla_multiple() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
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

    assert!(*cpu.a() == 0x3a);
    assert!(*cpu.x() == 0x2a);
    assert!(*cpu.y() == 0x1a);
}

#[test]
fn test_0x20_0x60_jsr_rts() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x3a, // 0xC000: mov $a, 0x3a
        0x20, 0x0D, 0xc0, // 0xC002: call 0xC00D
        0xad, 0xff, 0x01, // 0xC005: mov $a, word ptr [0x01FF]
        0xa8, // 0xC008: mov $y, $a
        0xad, 0xFE, 0x01, // 0xC009: mov $a, word ptr [0x01FE]
        0x00, // 0xC00C: brk
        0xaa, // 0xC00D: mov $x, $a
        0x60 // 0xC00E: ret 
    ]);

    assert!(*cpu.x() == 0x3a);
    assert!(*cpu.y() == 0xc0);
    assert!(*cpu.a() == 0x05);
}

#[test]
fn test_0x48_pha_single_capacity() {
    let mut cpu = CPU::new();
    let mut instructions = vec![
        0xa9, 0x3a // mov $a, 0x3a
    ];

    for _ in 0..0xFF {
        instructions.push(0x48);
    }

    instructions.push(0x00);
    cpu.load_and_run(instructions);
    assert!(*cpu.sp() <= 0x100);
}

#[test]
#[should_panic]
fn test_0x48_pha_overflow() {
    let mut cpu = CPU::new();
    let mut instructions = vec![
        0xa9, 0x3a // mov $a, 0x3a
    ];

    for _ in 0..=0xFF {
        instructions.push(0x48);
    }

    instructions.push(0x00);
    cpu.load_and_run(instructions);
}

#[test]
#[should_panic]
fn test_0x68_pla_underflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xa9, 0x3a,
        0x48, 0x68,
        0x68, 0x00
    ]);
}

#[test]
fn test_0x48_pha_call_capacity() {
    let mut cpu = CPU::new();
    let mut instructions = vec![];

    let mut jmp_delta = 3;
    for _ in 0..0x80 {
        instructions.push(0x20);
        match (0xc000 as u16 + jmp_delta).to_le_bytes() {
            [lo, hi] => {
                instructions.push(lo);
                instructions.push(hi);
            }
        }

        jmp_delta += 3;
    }

    cpu.load_and_run(instructions);
    assert!(*cpu.sp() <= 0x100);
}

