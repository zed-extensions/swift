; Parse regex syntax within regex literals

((regex_literal) @injection.content
 (#set! injection.language "regex"))

((comment) @injection.content
 (#set! injection.language "comment"))

((multiline_comment) @injection.content
 (#set! injection.language "comment"))
