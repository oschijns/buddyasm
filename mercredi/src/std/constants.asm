#once

; 
; NES hardware definition
; 
; ref: https://8bitworkshop.com/blog/platforms/nintendo-nes.md.html
; 

; MARK: PPU
; Picture Processing Unit

    ; Nametables definition
    ppu_nametable_0                 = 0x2000        ; Address of nametable 0
    ppu_nametable_1                 = 0x2400        ; Address of nametable 1
    ppu_nametable_2                 = 0x2800        ; Address of nametable 2
    ppu_nametable_3                 = 0x2C00        ; Address of nametable 3
    PPU_NAMETABLE_SIZE              =  0x400        ; Size of a nametable
    PPU_NAMETABLE_COUNT             =  4            ; Number of nametables
    PPU_NAMETABLE_SIZE_TILE_MAP     =  0x3C0        ; Size of the tilemap part of a nametable
    PPU_NAMETABLE_SIZE_ATTR_MAP     =  0x040        ; Size of the attribute map part of the nametable
    PPU_NAMETABLE_TILE_MAP_WIDTH    = 32            ; Width  of the tilemap in tiles
    PPU_NAMETABLE_TILE_MAP_HEIGHT   = 30            ; Height of the tilemap in tiles
    PPU_NAMETABLE_ATTR_MAP_WIDTH    =  8            ; Width  of the attribute map in metatiles
    PPU_NAMETABLE_ATTR_MAP_HEIGHT   =  8            ; Height of the attribute map in metatiles
    ; NOTE: Only the values in the top half or row 7 are used

    ; Palettes definition
    ppu_palette                     = 0x3F00        ; Starting address for setting palette data
    ppu_palette_bg_0                = 0x3F00        ; Palette 0 for background
    ppu_palette_bg_1                = 0x3F04        ; Palette 1 for background
    ppu_palette_bg_2                = 0x3F08        ; Palette 2 for background
    ppu_palette_bg_3                = 0x3F0C        ; Palette 3 for background
    ppu_palette_obj_0               = 0x3F10        ; Palette 0 for sprites
    ppu_palette_obj_1               = 0x3F14        ; Palette 1 for sprites
    ppu_palette_obj_2               = 0x3F18        ; Palette 2 for sprites
    ppu_palette_obj_3               = 0x3F1C        ; Palette 3 for sprites
    PPU_PALETTE_SIZE                = 4             ; Size of a palette in bytes
    PPU_PALETTE_COUNT               = 8             ; Number of palettes
    PPU_PALETTE_COUNT_BG            = 4             ; Number of background palettes
    PPU_PALETTE_COUNT_OBJ           = 4             ; Number of sprite palettes

    ; Screen definition
    PPU_SCREEN_WIDTH                = 256           ; Width of the screen in pixels
    PPU_SCREEN_HEIGHT               = 240           ; Height of the screen in pixels

    ; Control
    ppu_control                     = 0x2000        ; Address
    PPU_CTRL_NMI                    = 0b1000_0000   ; Enable Non-Maskable Interrupt
    PPU_CTRL_OBJ_8x16               = 0b0010_0000   ; Enable 8x16 sprites size
    PPU_CTRL_PATTERN_BG             = 0b0001_0000   ; Select background pattern 1
    PPU_CTRL_PATTERN_OBJ            = 0b0000_1000   ; Select sprites pattern 1
    PPU_CTRL_VRAM_INC32             = 0b0000_0100   ; Enable VRAM increment by 32
    PPU_CTRL_NAMETABLE              = 0b0000_0011   ; Mask covering nametable bits

    ; Mask
    ppu_mask                        = 0x2001        ; Address
    PPU_MASK_INTENSIFY_B            = 0b1000_0000   ; Intensify Blue channel
    PPU_MASK_INTENSIFY_G            = 0b0100_0000   ; Intensify Green channel
    PPU_MASK_INTENSIFY_R            = 0b0010_0000   ; Intensify Red channel
    PPU_MASK_RENDER_OBJ             = 0b0001_0000   ; Enable rendering of sprites
    PPU_MASK_RENDER_BG              = 0b0000_1000   ; Enable rendering of background
    PPU_MASK_LEFTMOST_OBJ           = 0b0000_0100   ; Enable masking off the left-most sprites
    PPU_MASK_LEFTMOST_BG            = 0b0000_0010   ; Enable masking off the left-most background tiles
    PPU_MASK_GREYSCALE              = 0b0000_0001   ; Enable grayscale mode

    ; PPU Status
    ppu_status                      = 0x2002        ; Address
    PPU_VBLANK                      = 0b1000_0000   ; V-Blank
    PPU_SPRITE_ZERO_HIT             = 0b0100_0000   ; Sprite-zero hit mask
    PPU_SPRITE_OVERFLOW             = 0b0010_0000   ; Sprite overflow

    ; PPU
    ppu_scroll                      = 0x2005        ; Background scrolling
    ppu_address                     = 0x2006        ; Where to set the PPU address
    ppu_data                        = 0x2007        ; Where to write the PPU data

    ; Pattern
    PPU_PATTERN_SIZE                = 0x1000        ; Size of a pattern
    PPU_PATTERN_COUNT               = 2             ; Number of patterns

; end ppu


; MARK: OAM
; Object Attribute Memory

    ; Addresses
    oam_address                 = 0x2003        ; address
    oam_data                    = 0x2004        ; address
    oam_dma                     = 0x4014        ; address

    ; Sprite
    OAM_SPRITE_SIZE             =  4            ; sprites are defined over 4 bytes
    OAM_SPRITE_COUNT            = 64            ; there are atmost 64 sprites handled by the NES
    OAM_SPRITE_SCANLINE_LIMIT   =  8            ; there are atmost 8 sprites per scanline

    ; Attributes
    OAM_ATTR_PALETTE            = 0b0000_0011   ; Mask covering the bits used for palette assignment
    OAM_ATTR_BEHIND             = 0b0010_0000   ; Display the sprite behind the background
    OAM_ATTR_FLIP_H             = 0b0100_0000   ; Horizontal flip
    OAM_ATTR_FLIP_V             = 0b1000_0000   ; Vertical flip

; end oam


; MARK: APU
; Audio Processing Unit

    ; Square pulse channels definition
    apu_square_1_control                = 0x4000        ; 
    apu_square_1_sweep                  = 0x4001        ; 
    apu_square_1_low                    = 0x4002        ; 
    apu_square_1_high                   = 0x4003        ; 
    apu_square_2_control                = 0x4004        ; 
    apu_square_2_sweep                  = 0x4005        ; 
    apu_square_2_low                    = 0x4006        ; 
    apu_square_2_high                   = 0x4007        ; 
    APU_SQUARE_CTRL_DUTY_MASK           = 0b1100_0000   ; 
    APU_SQUARE_CTRL_DUTY_SHIFT          = 6             ; 
    APU_SQUARE_CTRL_LEN_COUNTER_OFF     = 0b0010_0000   ; 
    APU_SQUARE_CTRL_VOL_CONST           = 0b0001_0000   ; 
    APU_SQUARE_CTRL_VOL_ENVELOPE_MASK   = 0b0000_1111   ; 
    APU_SQUARE_SWEEP_ENABLE_MASK        = 0b1000_0000   ; 
    APU_SQUARE_SWEEP_PERIOD_MASK        = 0b0111_0000   ; 
    APU_SQUARE_SWEEP_PERIOD_SHIFT       = 4             ; 
    APU_SQUARE_SWEEP_NEGATE             = 0b0000_1000   ; 
    APU_SQUARE_SWEEP_SHIFT_MASK         = 0b0000_0011   ; 
    APU_SQUARE_HIGH_LEN_MASK            = 0b1111_1000   ; 
    APU_SQUARE_HIGH_LEN_SHIFT           = 3             ; 
    APU_SQUARE_HIGH_FREQ_MASK           = 0b0000_0011   ; 

    ; Triangle channel definition
    apu_triangle_control                = 0x4008        ; 
    apu_triangle_low                    = 0x400A        ; 
    apu_triangle_high                   = 0x400B        ; 
    APU_TRIANGLE_CTRL_LINEAR_COUNTER_ON = 0b1000_0000   ; 
    APU_TRIANGLE_HIGH_LEN_MASK          = 0b1111_1000   ; 
    APU_TRIANGLE_HIGH_LEN_SHIFT         = 3             ; 
    APU_TRIANGLE_HIGH_FREQ_MASK         = 0b0000_0111   ; 

    ; Noise channel definition
    apu_noise_control                   = 0x400C        ; 
    apu_noise_pattern                   = 0x400E        ; 
    apu_noise_length                    = 0x400F        ; 
    APU_NOISE_CTRL_LEN_COUNTER_OFF      = 0b0010_0000   ; 
    APU_NOISE_CTRL_VOL_CONST            = 0b0001_0000   ; 
    APU_NOISE_CTRL_VOL_ENVELOPE_MASK    = 0b0000_1111   ; 
    APU_NOISE_PATTERN_PERIOD_MASK       = 0b0000_1111   ; 
    APU_NOISE_PATTERN_DUTY              = 0b1000_0000   ; 
    APU_NOISE_LEN_MASK                  = 0b1111_1000   ; 
    APU_NOISE_LEN_SHIFT                 = 3             ; 

    ; Delta Modulation Channel
    apu_dmc_control                     = 0x4010        ; 
    apu_dmc_counter                     = 0x4011        ; 
    apu_dmc_address                     = 0x4012        ; 
    apu_dmc_length                      = 0x4013        ; 
    APU_DMC_CTRL_IRQ_ON                 = 0b1000_0000   ; 
    APU_DMC_CTRL_LOOP                   = 0b0100_0000   ; 
    APU_DMC_CTRL_FREQ_MASK              = 0b0000_1111   ; 
    APU_DMC_COUNTER_MASK                = 0b0111_1111   ; 

    ; Enable (or disable) channels
    apu_enable                          = 0x4015        ; Enabled channels
    APU_ENABLE_PULSE1                   = 0b0000_0001   ; Pulse 1  is enabled
    APU_ENABLE_PULSE2                   = 0b0000_0010   ; Pulse 2  is enabled
    APU_ENABLE_TRIANGLE                 = 0b0000_0100   ; Triangle is enabled
    APU_ENABLE_NOISE                    = 0b0000_1000   ; Noise    is enabled
    APU_ENABLE_DMC                      = 0b0001_0000   ; DMC      is enabled

    ; APU status register
    apu_status                          = 0x4015        ; APU status register
    APU_STATUS_ENABLED_PULSE1           = 0b0000_0001   ; Pulse 1  is enabled
    APU_STATUS_ENABLED_PULSE2           = 0b0000_0010   ; Pulse 2  is enabled
    APU_STATUS_ENABLED_TRIANGLE         = 0b0000_0100   ; Triangle is enabled
    APU_STATUS_ENABLED_NOISE            = 0b0000_1000   ; Noise    is enabled
    APU_STATUS_ENABLED_DMC              = 0b0001_0000   ; DMC      is enabled
    APU_STATUS_ENABLEE_MASK             = 0b0001_1111   ; Mask of possible enabled channels
    APU_STATUS_IRQ_FRAME_MASK           = 0b0100_0000   ; Mask      of whether or not frame-counter interrupt is active
    APU_STATUS_IRQ_FRAME_BIT            = 6             ; Bit index of whether or not frame-counter interrupt is active
    APU_STATUS_IRQ_DMC_MASK             = 0b1000_0000   ; Mask      of whether or not DMC interrupt is active
    APU_STATUS_IRQ_DMC_BIT              = 7             ; Bit index of whether or not DMC interrupt is active

    ; Frame counter
    apu_frame_counter                   = 0x4017        ; 
    APU_FRAME_COUNTER_IRQ_OFF           = 0b0100_0000   ; Disable frame-counter interrupts
    APU_FRAME_COUNTER_FOUR_STEP         = 0b0000_0000   ; Four-step mode (faster, allows frame-counter interrupt if not disabled)
    APU_FRAME_COUNTER_FIVE_STEP         = 0b1000_0000   ; Five-step mode (slower)

; end apu


; MARK: JOYPAD

    joy_output          = 0x4016        ; Output address
    joy_input1          = 0x4016        ; Input address for gamepad 1
    joy_input2          = 0x4017        ; Input address for gamepad 2

    ; Bit position of the buttons
    JOY_BIT_BTN_A       = 0             ; A button
    JOY_BIT_BTN_B       = 1             ; B button
    JOY_BIT_SELECT      = 2             ; Select button
    JOY_BIT_START       = 3             ; Start button
    JOY_BIT_UP          = 4             ; D-pad up
    JOY_BIT_DOWN        = 5             ; D-pad down
    JOY_BIT_LEFT        = 6             ; D-pad left
    JOY_BIT_RIGHT       = 7             ; D-pad right

    ; Mask for accessing button states
    JOY_MASK_BTN_A      = 0b0000_0001   ; A button
    JOY_MASK_BTN_B      = 0b0000_0010   ; B button
    JOY_MASK_SELECT     = 0b0000_0100   ; Select button
    JOY_MASK_START      = 0b0000_1000   ; Start button
    JOY_MASK_UP         = 0b0001_0000   ; D-pad up
    JOY_MASK_DOWN       = 0b0010_0000   ; D-pad down
    JOY_MASK_LEFT       = 0b0100_0000   ; D-pad left
    JOY_MASK_RIGHT      = 0b1000_0000   ; D-pad right
    JOY_MASK_ANY_ACT    = 0b0000_0011   ; Any action button (A or B)
    JOY_MASK_ANY_OPT    = 0b0000_1100   ; Any option button (Select or Start)
    JOY_MASK_ANY_DIR    = 0b1111_0000   ; Any D-pad direction
    JOY_MASK_ANY        = 0b1111_1111   ; Any button press

; end joy


; MARK: SYSTEM
; Variable system configuration

    ; Clock speed vary based on video output
    SYS_NTSC = false
    SYS_PAL  = false

    ; Both cannot be true at the same time !
    #assert !(SYS_NTSC && SYS_PAL)

#if SYS_PAL
{
    SYS_FRAME_RATE = 60 ; (f/s)
}
#else ; default to NTSC
{
    SYS_FRAME_RATE = 50 ; (f/s)
}


; end sys