// Convert hex to base64
// Rule: Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.

fn hex_to_byte(hex: str) -> Vec<u8> {

}

fn byte_to_b64(bytes: Vec<u8>) -> str {

}

fn convert(hex: str) -> str {
    let byte_array: Vec<u8> = hex_to_byte(hex);
    return byte_to_b64(byte_array);
}

fn main() {
    let hex_input: str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let desired_b64_output: str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let hex_output: str = convert(hex_input);

    assert!(hex_output, desired_b64_output);
}
