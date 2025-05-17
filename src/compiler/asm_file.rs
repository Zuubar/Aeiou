use std::fmt::Write;

pub struct AsmFile {
    rodata_section: String,
    data_section: String,
    bss_section: String,
    text_section: String,
}

impl AsmFile {
    pub fn new() -> AsmFile {
        let mut gen = AsmFile {
            rodata_section: String::from("section .rodata\n"),
            data_section: String::from("section .data\n"),
            bss_section: String::from("section .bss\n"),
            text_section: String::from(
                r#"section .text
    global main
    extern printf
    extern exit
main:
"#,
            ),
        };

        gen.write_data("__aeiou__format_i32 db \"%d\", 10, 0");
        gen.write_data("__aeiou__format_f64 db \"%f\", 10, 0");
        gen.write_text("push rbp");
        gen.write_text("mov rbp, rsp");
        gen
    }

    pub fn write_rodata(&mut self, input: &str) {
        writeln!(self.rodata_section, "\t{}", input).unwrap();
    }

    pub fn write_data(&mut self, input: &str) {
        writeln!(self.data_section, "\t{}", input).unwrap();
    }

    pub fn write_bss(&mut self, input: &str) {
        writeln!(self.bss_section, "\t{}", input).unwrap();
    }

    pub fn write_text(&mut self, input: &str) {
        writeln!(self.text_section, "\t{}", input).unwrap();
    }

    pub fn write_instruction2(&mut self, instruction: &str, dst: &str, src: &str) {
        self.write_text(&format!("{} {}, {}", instruction, dst, src,));
    }

    pub fn write_instruction1(&mut self, instruction: &str, src: &str) {
        self.write_text(&format!("{} {}", instruction, src,));
    }

    pub fn finalize(&mut self) -> String {
        self.write_text(
            r#"
	leave
	mov rdi, 0
	call exit
	ret
	"#,
        );
        format!(
            "{}\n{}\n{}\n{}",
            self.rodata_section, self.data_section, self.bss_section, self.text_section
        )
    }
}
