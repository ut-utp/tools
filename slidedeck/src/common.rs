//! Helper functions and the actual slide deck LC-3 program.

use lc3_isa::{
    self,
    util::{AssembledProgram, MemoryDump},
    Addr, SignedWord, Word,
};
use lc3_os::OS_IMAGE;

use std::convert::TryInto;

/// Starting address of the data section of the slide deck memory image.
///
/// This can be anything so long as it's in user space and doesn't overlap with
/// the actual program part of the image.
pub const STARTING: Addr = 0x3100;

/// Turns a slide (i.e a string with width * height characters) into the packed
/// format that the slide deck program uses.
///
/// `permissive` controls whether this function will panic if slides don't match
/// the number of characters specified (height * width).
///
/// If permissive is true, extra characters are ignored and unspecified
/// characters are uninitialized in memory (i.e. they're just `'\0'`).
#[must_use]
pub fn slide(
    (width, height): (usize, usize),
    mut addr: Addr,
    slide: &str,
    permissive: bool,
) -> Vec<(Addr, Word)> {
    if !permissive {
        assert_eq!(
            slide.len(),
            width * height,
            "size mismatch (expected {}, got {}) on: `{}`",
            width * height,
            slide.len(),
            slide
        );
    }

    slide
        .chars()
        .take(width * height)
        .map(|c| {
            let a = addr;
            addr += 1;

            (a, c as u16)
        })
        .collect()
}

#[allow(clippy::cognitive_complexity)]
/// The slide deck program, parameterized by the number of slides and height
/// and the width of the slides.
fn base_program((width, height): (usize, usize), num_slides: Word) -> AssembledProgram {
    #[allow(clippy::cast_sign_loss)]
    fn neg_character(c: u8) -> Word {
        (-(TryInto::<SignedWord>::try_into(c).unwrap())) as Word
    }

    let width: Word = width.try_into().unwrap();
    let height: Word = height.try_into().unwrap();

    #[allow(clippy::cast_sign_loss)]
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
        @NEG_LETTER_A .FILL #neg_character(b'a');
        @NEG_LETTER_D .FILL #neg_character(b'd');
        @NEG_LETTER_R .FILL #neg_character(b'r');

        @STARTING_PTR .FILL #STARTING;
        @NUM_SLIDES .FILL #num_slides;
        @WIDTH .FILL #width;
        @HEIGHT .FILL #height;
        @NEG_SLIDE_LEN .FILL #(
            (-(TryInto::<SignedWord>::try_into(height * width).unwrap())) as Word
        );
    })
    .into()
}

/// Given a slide deck and some dimensions, this produces an image containing
/// the slides and the slide deck program.
///
/// `dimensions` is (height, width).
///
/// `permissive` controls whether slides with an "incorrect" number of
/// characters (not equal to height * width) are accepted. See [`slide`] for
/// more details.
#[must_use]
pub fn make_image(dimensions: (usize, usize), slides: &[&str], permissive: bool) -> MemoryDump {
    let mut image = OS_IMAGE.clone();
    let slide_len = dimensions.0 * dimensions.1;

    let _ = image
        .layer_loadable(base_program(dimensions, slides.len().try_into().unwrap()).into_iter());

    slides.iter().enumerate().for_each(|(idx, s)| {
        let _ = image.layer_loadable(slide(
            dimensions,
            STARTING + TryInto::<Addr>::try_into(idx * slide_len).unwrap(),
            s,
            permissive,
        ));
    });

    image
}
