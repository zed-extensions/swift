; Parse regex syntax within regex literals
((regex_literal) @injection.content
  (#set! injection.language "regex"))

; Documentation comments are Markdown, as Xcode renders them.
((comment) @injection.content
  (#match? @injection.content "^///")
  (#set! injection.language "markdown-inline"))

((multiline_comment) @injection.content
  (#match? @injection.content "^/\\*\\*")
  (#set! injection.language "markdown-inline"))

((comment) @injection.content
  (#not-match? @injection.content "^///")
  (#set! injection.language "comment"))

((multiline_comment) @injection.content
  (#not-match? @injection.content "^/\\*\\*")
  (#set! injection.language "comment"))
