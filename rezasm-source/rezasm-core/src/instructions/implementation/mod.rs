mod arithmetic_instructions;
mod branch_instructions;
mod comparison_instructions;
mod float_arithmetic_instructions;
mod function_instructions;
mod memory_instructions;
mod terminal_input_instructions;
mod terminal_output_instructions;

pub fn register_instructions() {
    arithmetic_instructions::register_instructions();
    branch_instructions::register_instructions();
    comparison_instructions::register_instructions();
    float_arithmetic_instructions::register_instructions();
    terminal_input_instructions::register_instructions();
    function_instructions::register_instructions();
    memory_instructions::register_instructions();
    terminal_output_instructions::register_instructions();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::ops::Deref;

    use arithmetic_instructions::ADD;

    use crate::instructions::argument_type::ArgumentType;
    use crate::instructions::targets::input_output_target::InputOutputTarget;
    use crate::instructions::targets::input_target::InputTarget;
    use crate::parser::lexer::parse_lines;
    use crate::simulation::simulator::Simulator;
    use crate::test_utils::{workspace_root, TestWriter};
    use crate::util::io::RezasmFileReader;
    use crate::util::raw_data::RawData;

    use super::*;

    // Moved from Trevor's implementation in tests/core.rs
    #[test]
    pub fn test_macro() {
        register_instructions();
        let mut simulator: Simulator = Simulator::new();
        let word_size = simulator.get_word_size();

        let args = vec![
            ArgumentType::InputOutput(InputOutputTarget::RegisterInputOutput(22)),
            ArgumentType::Input(InputTarget::RegisterInput(22)),
            ArgumentType::Input(InputTarget::ImmediateInput(RawData::from_int(
                121, word_size,
            ))),
        ];

        let seq = ADD
            .call_function(&mut simulator, &args)
            .expect("Failed to call function");
        simulator
            .apply_transformation(seq)
            .expect("Failed to apply transformation sequence");
        assert_eq!(
            simulator
                .get_registers()
                .get_register("T0")
                .expect("Register access error")
                .get_data()
                .int_value(),
            121
        );
    }

    // Moved from Trevor's test in tests/core.rs
    #[test]
    pub fn test_print_instructions() {
        register_instructions();
        let writer = Box::new(TestWriter::new());
        let mut simulator: Simulator = Simulator::new_writer(writer);
        let program = "
        move $s2 \"Print Instructions Work!\\n\"
        add $s1 '\\n' 0
        add $t0 1 2
        printi $t0
        printc $s1
        addf $t1 1.5 0
        printf $t1
        printc $s1
        prints $s2";
        let lines = parse_lines(program, simulator.get_word_size()).unwrap();
        simulator.add_lines(lines, "".to_string()).unwrap();
        while !simulator.is_done() {
            simulator.run_line_from_pc().unwrap();
        }
        let writer = simulator.get_writer();
        let output = writer
            .deref()
            .as_any()
            .downcast_ref::<TestWriter>()
            .unwrap()
            .get_data();
        assert_eq!(output.as_str(), "3\n1.5\nPrint Instructions Work!\n");
    }

    // FIXME: need to move this test elsewhere; it covers the parser I/O, not simulator I/O
    // Moved from Trevor's test in tests/core.rs
    #[test]
    pub fn test_io() {
        register_instructions();

        // Read into rezasm file
        let file_path = workspace_root().join("example").join("arithmetic_fib.ez");
        let mut rezasmfile = RezasmFileReader::new(file_path.clone()).expect("failed to read file");

        // Reads with fs
        let reg_bytes = fs::read(file_path).unwrap();
        let reg_read: Vec<String> = String::from_utf8(reg_bytes.clone())
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect();

        // Compare
        assert_eq!(rezasmfile.lines().expect("failed to get lines"), reg_read);

        // FIXME: need to change seek integers due to added comments in the file
        // assert_eq!(32, rezasmfile.seek_absolute_byte(1).unwrap());
        // assert_eq!(101, rezasmfile.seek_absolute_byte(3).unwrap());
        // assert_eq!(32, rezasmfile.seek_relative_byte(-2).unwrap());
        // assert_eq!(35, rezasmfile.peek_absolute_byte(0).unwrap());
        //
        // rezasmfile.seek_start();

        let mut bytes = vec![];
        while let Some(byte) = rezasmfile.next() {
            bytes.push(byte);
        }

        // Compare the bytes.
        assert_eq!(reg_bytes, bytes);

        // Cursor should now be at the end. Attempt to peek 0.
        assert_eq!(35, rezasmfile.peek_absolute_byte(0).unwrap());

        // Check validity (should be invalid).
        assert!(!rezasmfile.is_cursor_valid());

        // Convert to a writer.
        let file_path = workspace_root().join("example").join("arith_modified.ez");
        let path: String = file_path.to_string_lossy().into();
        let mut writer = rezasmfile.into_writer(Some(path)).unwrap();
        let write_str = "\n# This comment was generated by io tests!";
        writer.extend_from_slice(write_str.as_bytes());
        writer.flush().unwrap();
    }
}
