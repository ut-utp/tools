
use lc3_isa::{self, Addr, Word, SignedWord, util::{AssembledProgram, MemoryDump}};
use lc3_os::{OS_IMAGE};

const STARTING: Addr = 0x3100;

fn slide((width, height): (usize, usize), mut addr: Addr, slide: &str, permissive: bool) -> Vec<(Addr, Word)> {
    if !permissive {
        assert_eq!(slide.len(), width * height, "size mismatch (expected {}, got {}) on: `{}`", width * height, slide.len(), slide);
    }

    slide.chars().take(width * height).map(|c| {
        let a = addr;
        addr += 1;

        (a, c as u16)
    }).collect()
}

fn base_program((width, height): (usize, usize), num_slides: Word) -> AssembledProgram {
    (lc3_isa::program! {
        .ORIG #0x3000;

        GETC; // Type _something_ to begin.

        @RESET_TO_START
            LD R3, @STARTING_PTR;
            LD R4, @NUM_SLIDES;
            BRnzp @BEGIN;

        @BACKWARDS
            LD R5, @NUM_SLIDES;
            NOT R5, R5;
            ADD R5, R5, #1;

            ADD R5, R5, R4; // (remaining slides - num slides)
            BRzp @LOOP; // Can't go back further! (shouldn't actually be positive)

            ADD R4, R4, #1;
            LD R5, @NEG_SLIDE_LEN;
            ADD R3, R3, R5;
            ADD R3, R3, R5;
            BRnzp @BEGIN;

        @FORWARDS
            // Check if we've reached the end:
            ADD R4, R4, #0;
            BRz @RESET_TO_START;

            // If not, onwards:
            ADD R4, R4, #-1;
            BRnzp @BEGIN;

        @LOOP
            // IN; // Block until we get input!
            GETC;

            // Restart:
            LD R5, @NEG_LETTER_R;
            ADD R5, R0, R5;
            BRz @RESET_TO_START;

            // Or go back:
            LD R5, @NEG_LETTER_A;
            ADD R5, R0, R5;
            BRz @BACKWARDS;

            // Or go forward:
            LD R5, @NEG_LETTER_D;
            ADD R5, R0, R5;
            BRz @FORWARDS;

            // Or, try again:
            BRnzp @LOOP;

            @BEGIN
                LD R2, @HEIGHT;

            @ROW
                LD R1, @WIDTH;

                LD R0, @NEWLINE;
                OUT;

                @ROW_INNER
                    LDR R0, R3, #0;
                    OUT;

                    ADD R3, R3, #1;
                    ADD R1, R1, #-1;
                    BRnp @ROW_INNER;

            @_END_OF_ROW
                ADD R2, R2, #-1;
                BRnp @ROW;

            BRnzp @LOOP; // And again

        @NEWLINE .FILL #('\n' as Word);
        @NEG_LETTER_A .FILL #((-('a' as SignedWord)) as Word);
        @NEG_LETTER_D .FILL #((-('d' as SignedWord)) as Word);
        @NEG_LETTER_R .FILL #((-('r' as SignedWord)) as Word);

        @STARTING_PTR .FILL #STARTING;
        @NUM_SLIDES .FILL #num_slides;
        @WIDTH .FILL #(width as Word);
        @HEIGHT .FILL #(height as Word);
        @NEG_SLIDE_LEN .FILL #((-((height * width) as SignedWord)) as Word);
    }).into()
}

pub fn make_image(dimensions: (usize, usize), slides: &[&str], permissive: bool) -> MemoryDump {
    let mut image = OS_IMAGE.clone();
    let slide_len = dimensions.0 * dimensions.1;

    image.layer_loadable(base_program(dimensions, slides.len() as Word).into_iter());

    slides.iter().enumerate().for_each(|(idx, s)| {
        image.layer_loadable(slide(dimensions, STARTING + ((idx * slide_len) as Addr), s, permissive));
    });

    image
}
