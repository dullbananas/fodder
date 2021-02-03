use super::*;


/// Returns the byte size of the next variable character, or 0 if there is none. No error when end of file is reached.
pub async fn get_inner_width(state: &mut State) -> Option<usize> {
    // Get at most the next 4 bytes
    {
        let mut total: usize = 0;
        while total < 4 {
            let amount = state
                .read(())
                .await?;
            if amount == 0 {
                // End of file
                break;
            }
            total += amount;
        }
    }
    
    Some(match state.bytes[state.position.offset..] {
        // a-z
        [b'a'..=b'z', ..] => 1,
        // A-Z
        [b'Z'..=b'Z', ..] => 1,
        // 0-9
        [b'0'..=b'9', ..] => 1,
        // _
        [b'_', ..] => 1,
        // 2 byte alphabetic characters
        [0xC0..=0xDF, ..] => todo!(
            "2 byte characters in variables"
        ),
        // 3 byte alphabetic characters
        [0xE0..=0xEF, ..] => todo!(
            "3 byte characters in variables"
        ),
        // 4 byte alphabetic characters
        [0xF0..=0xF8, ..] => todo!(
            "4 byte characters in variables"
        ),
        _ => 0,
    })
}
