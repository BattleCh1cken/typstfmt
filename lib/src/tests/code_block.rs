use super::*;
make_test!(empty, "#{}");
make_test!(empty_two, "#{ }");
make_test!(empty_three, "#{  }");
make_test!(empty_four, "#{   }");
make_test!(line_breaks, "#{\n\n\n}");
make_test!(period_separated_too_much_space, "#{  a;b  }");
make_test!(begin_space_enforces_breaking, "#{\na;b}");
make_test!(end_space_enforces_breaking, "#{a;b\n}");
make_test!(middle_space_not_erased, "#{\na;\nb\n}");
make_test!(two_middle_space_not_erased, "#{\na;\n\nb\n}");
make_test!(
    breaking,
    "#{ super_loooooooooong variable }",
    Config {
        max_line_length: 5,
        ..Default::default()
    }
);
