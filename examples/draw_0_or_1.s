.origin $8000

_start:
  JMP loop

readKeys:
  LDA $FF
  CMP #$31
  BCS one
  CMP #$30
  BCS zero
  RTS

clearScreen:
  LDA #0
  LDX #0
  JSR iterRow200
  RTS

iterRow200:
  STA $0200, X
  CPX #$FF
  BCS lastRow200
  INX
  JMP iterRow200

lastRow200:
  STA $0200, X
  INX
  RTS

break:
  RTS

zero:
  CPY #$30
  BEQ break
  LDY #$30
  JSR clearScreen
  LDX #1
  STX $0201
  STX $0220
  STX $0240
  STX $0260
  STX $0222
  STX $0242
  STX $0262
  STX $0281
  RTS

one:
  CPY #$31
  BEQ break
  LDY #$31
  JSR clearScreen
  LDX #4
  STX $0220
  STX $0201
  STX $0221
  STX $0241
  STX $0261
  RTS

loop:
  JSR readKeys
  JMP loop
