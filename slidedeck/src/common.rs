
use lc3_isa::{self, Addr, Word, util::{AssembledProgram, MemoryDump}};
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

        @RESET_TO_START
            LD R3, @STARTING_PTR;
            LD R4, @NUM_SLIDES;

        @LOOP
            // IN; // Block until we get input!
            GETC;

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

            @_FINISHED
                ADD R4, R4, #-1;
                BRz @RESET_TO_START; // If we're out of slides, start over.

                BRnzp @LOOP; // Else, keep going.

        @NEWLINE .FILL #('\n' as Word);


        @STARTING_PTR .FILL #STARTING;
        @NUM_SLIDES .FILL #num_slides;
        @WIDTH .FILL #(width as Word);
        @HEIGHT .FILL #(height as Word);
    }).into()
}

pub fn make_image(dimensions: (usize, usize), slides: &[&str]) -> MemoryDump {
    let mut image = OS_IMAGE.clone();
    let slide_len = dimensions.0 * dimensions.1;

    image.layer_loadable(base_program(dimensions, slides.len() as Word).into_iter());

    slides.iter().enumerate().for_each(|(idx, s)| {
        image.layer_loadable(slide(dimensions, STARTING + ((idx * slide_len) as Addr), s, true));
    });

    image
}
