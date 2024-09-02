use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref SMART_ENCODING_MAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('\u{00ab}', '"'); // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
        m.insert('\u{00bb}', '"'); // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
        m.insert('\u{201c}', '"'); // LEFT DOUBLE QUOTATION MARK
        m.insert('\u{201d}', '"'); // RIGHT DOUBLE QUOTATION MARK
        m.insert('\u{02ba}', '"'); // MODIFIER LETTER DOUBLE PRIME
        m.insert('\u{02ee}', '"'); // MODIFIER LETTER DOUBLE APOSTROPHE
        m.insert('\u{201f}', '"'); // DOUBLE HIGH-REVERSED-9 QUOTATION MARK
        m.insert('\u{201E}', '"'); // DOUBLE LOW-9 QUOTATION MARK
        m.insert('\u{275d}', '"'); // HEAVY DOUBLE TURNED COMMA QUOTATION MARK ORNAMENT
        m.insert('\u{275e}', '"'); // HEAVY DOUBLE COMMA QUOTATION MARK ORNAMENT
        m.insert('\u{301d}', '"'); // REVERSED DOUBLE PRIME QUOTATION MARK
        m.insert('\u{301e}', '"'); // DOUBLE PRIME QUOTATION MARK
        m.insert('\u{ff02}', '"'); // FULLWIDTH QUOTATION MARK
        m.insert('\u{2018}', '\''); // LEFT SINGLE QUOTATION MARK
        m.insert('\u{2019}', '\''); // RIGHT SINGLE QUOTATION MARK
        m.insert('\u{02BB}', '\''); // MODIFIER LETTER TURNED COMMA
        m.insert('\u{02c8}', '\''); // MODIFIER LETTER VERTICAL LINE
        m.insert('\u{02bc}', '\''); // MODIFIER LETTER APOSTROPHE
        m.insert('\u{02bd}', '\''); // MODIFIER LETTER REVERSED COMMA
        m.insert('\u{02b9}', '\''); // MODIFIER LETTER PRIME
        m.insert('\u{201b}', '\''); // SINGLE HIGH-REVERSED-9 QUOTATION MARK
        m.insert('\u{ff07}', '\''); // FULLWIDTH APOSTROPHE
        m.insert('\u{00b4}', '\''); // ACUTE ACCENT
        m.insert('\u{02ca}', '\''); // MODIFIER LETTER ACUTE ACCENT
        m.insert('\u{0060}', '\''); // GRAVE ACCENT
        m.insert('\u{02cb}', '\''); // MODIFIER LETTER GRAVE ACCENT
        m.insert('\u{275b}', '\''); // HEAVY SINGLE TURNED COMMA QUOTATION MARK ORNAMENT
        m.insert('\u{275c}', '\''); // HEAVY SINGLE COMMA QUOTATION MARK ORNAMENT
        m.insert('\u{0313}', '\''); // COMBINING COMMA ABOVE
        m.insert('\u{0314}', '\''); // COMBINING REVERSED COMMA ABOVE
        m.insert('\u{fe10}', '\''); // PRESENTATION FORM FOR VERTICAL COMMA
        m.insert('\u{fe11}', '\''); // PRESENTATION FORM FOR VERTICAL IDEOGRAPHIC COMMA
        m.insert('\u{00F7}', '/'); // DIVISION SIGN
        m.insert('\u{00bc}', '¼'); // VULGAR FRACTION ONE QUARTER
        m.insert('\u{00bd}', '½'); // VULGAR FRACTION ONE HALF
        m.insert('\u{00be}', '¾'); // VULGAR FRACTION THREE QUARTERS
        m.insert('\u{29f8}', '/'); // BIG SOLIDUS
        m.insert('\u{0337}', '/'); // COMBINING SHORT SOLIDUS OVERLAY
        m.insert('\u{0338}', '/'); // COMBINING LONG SOLIDUS OVERLAY
        m.insert('\u{2044}', '/'); // FRACTION SLASH
        m.insert('\u{2215}', '/'); // DIVISION SLASH
        m.insert('\u{ff0f}', '/'); // FULLWIDTH SOLIDUS
        m.insert('\u{29f9}', '\\'); // BIG REVERSE SOLIDUS
        m.insert('\u{29f5}', '\\'); // REVERSE SOLIDUS OPERATOR
        m.insert('\u{20e5}', '\\'); // COMBINING REVERSE SOLIDUS OVERLAY
        m.insert('\u{fe68}', '\\'); // SMALL REVERSE SOLIDUS
        m.insert('\u{ff3c}', '\\'); // FULLWIDTH REVERSE SOLIDUS
        m.insert('\u{0332}', '_'); // COMBINING LOW LINE
        m.insert('\u{ff3f}', '_'); // FULLWIDTH LOW LINE
        m.insert('\u{20d2}', '|'); // COMBINING LONG VERTICAL LINE OVERLAY
        m.insert('\u{20d3}', '|'); // COMBINING SHORT VERTICAL LINE OVERLAY
        m.insert('\u{2223}', '|'); // DIVIDES
        m.insert('\u{ff5c}', '|'); // FULLWIDTH VERTICAL LINE
        m.insert('\u{23b8}', '|'); // LEFT VERTICAL BOX LINE
        m.insert('\u{23b9}', '|'); // RIGHT VERTICAL BOX LINE
        m.insert('\u{23d0}', '|'); // VERTICAL LINE EXTENSION
        m.insert('\u{239c}', '|'); // LEFT PARENTHESIS EXTENSION
        m.insert('\u{239f}', '|'); // RIGHT PARENTHESIS EXTENSION
        m.insert('\u{23bc}', '-'); // HORIZONTAL SCAN LINE-7
        m.insert('\u{23bd}', '-'); // HORIZONTAL SCAN LINE-9
        m.insert('\u{2015}', '-'); // HORIZONTAL BAR
        m.insert('\u{fe63}', '-'); // SMALL HYPHEN-MINUS
        m.insert('\u{ff0d}', '-'); // FULLWIDTH HYPHEN-MINUS
        m.insert('\u{2010}', '-'); // HYPHEN
        m.insert('\u{2043}', '-'); // HYPHEN BULLET
        m.insert('\u{fe6b}', '@'); // SMALL COMMERCIAL AT
        m.insert('\u{ff20}', '@'); // FULLWIDTH COMMERCIAL AT
        m.insert('\u{fe69}', '$'); // SMALL DOLLAR SIGN
        m.insert('\u{ff04}', '$'); // FULLWIDTH DOLLAR SIGN
        m.insert('\u{01c3}', '!'); // LATIN LETTER RETROFLEX CLICK
        m.insert('\u{fe15}', '!'); // PRESENTATION FORM FOR VERTICAL EXLAMATION MARK
        m.insert('\u{fe57}', '!'); // SMALL EXCLAMATION MARK
        m.insert('\u{ff01}', '!'); // FULLWIDTH EXCLAMATION MARK
        m.insert('\u{fe5f}', '#'); // SMALL NUMBER SIGN
        m.insert('\u{ff03}', '#'); // FULLWIDTH NUMBER SIGN
        m.insert('\u{fe6a}', '%'); // SMALL PERCENT SIGN
        m.insert('\u{ff05}', '%'); // FULLWIDTH PERCENT SIGN
        m.insert('\u{fe60}', '&'); // SMALL AMPERSAND
        m.insert('\u{ff06}', '&'); // FULLWIDTH AMPERSAND
        m.insert('\u{201a}', ','); // SINGLE LOW-9 QUOTATION MARK
        m.insert('\u{0326}', ','); // COMBINING COMMA BELOW
        m.insert('\u{fe50}', ','); // SMALL COMMA
        m.insert('\u{fe51}', ','); // SMALL IDEOGRAPHIC COMMA
        m.insert('\u{ff0c}', ','); // FULLWIDTH COMMA
        m.insert('\u{ff64}', ','); // HALFWIDTH IDEOGRAPHIC COMMA
        m.insert('\u{2768}', '('); // MEDIUM LEFT PARENTHESIS ORNAMENT
        m.insert('\u{276a}', '('); // MEDIUM FLATTENED LEFT PARENTHESIS ORNAMENT
        m.insert('\u{fe59}', '('); // SMALL LEFT PARENTHESIS
        m.insert('\u{ff08}', '('); // FULLWIDTH LEFT PARENTHESIS
        m.insert('\u{27ee}', '('); // MATHEMATICAL LEFT FLATTENED PARENTHESIS
        m.insert('\u{2985}', '('); // LEFT WHITE PARENTHESIS
        m.insert('\u{2769}', ')'); // MEDIUM RIGHT PARENTHESIS ORNAMENT
        m.insert('\u{276b}', ')'); // MEDIUM FLATTENED RIGHT PARENTHESIS ORNAMENT
        m.insert('\u{fe5a}', ')'); // SMALL RIGHT PARENTHESIS
        m.insert('\u{ff09}', ')'); // FULLWIDTH RIGHT PARENTHESIS
        m.insert('\u{27ef}', ')'); // MATHEMATICAL RIGHT FLATTENED PARENTHESIS
        m.insert('\u{2986}', ')'); // RIGHT WHITE PARENTHESIS
        m.insert('\u{204e}', '*'); // LOW ASTERISK
        m.insert('\u{2217}', '*'); // ASTERISK OPERATOR
        m.insert('\u{229B}', '*'); // CIRCLED ASTERISK OPERATOR
        m.insert('\u{2722}', '*'); // FOUR TEARDROP-SPOKED ASTERISK
        m.insert('\u{2723}', '*'); // FOUR BALLOON-SPOKED ASTERISK
        m.insert('\u{2724}', '*'); // HEAVY FOUR BALLOON-SPOKED ASTERISK
        m.insert('\u{2725}', '*'); // FOUR CLUB-SPOKED ASTERISK
        m.insert('\u{2731}', '*'); // HEAVY ASTERISK
        m.insert('\u{2732}', '*'); // OPEN CENTRE ASTERISK
        m.insert('\u{2733}', '*'); // EIGHT SPOKED ASTERISK
        m.insert('\u{273a}', '*'); // SIXTEEN POINTED ASTERISK
        m.insert('\u{273b}', '*'); // TEARDROP-SPOKED ASTERISK
        m.insert('\u{273c}', '*'); // OPEN CENTRE TEARDROP-SPOKED ASTERISK
        m.insert('\u{273d}', '*'); // HEAVY TEARDROP-SPOKED ASTERISK
        m.insert('\u{2743}', '*'); // HEAVY TEARDROP-SPOKED PINWHEEL ASTERISK
        m.insert('\u{2749}', '*'); // BALLOON-SPOKED ASTERISK
        m.insert('\u{274a}', '*'); // EIGHT TEARDROP-SPOKED PROPELLER ASTERISK
        m.insert('\u{274b}', '*'); // HEAVY EIGHT TEARDROP-SPOKED PROPELLER ASTERISK
        m.insert('\u{29c6}', '*'); // SQUARED ASTERISK
        m.insert('\u{fe61}', '*'); // SMALL ASTERISK
        m.insert('\u{ff0a}', '*'); // FULLWIDTH ASTERISK
        m.insert('\u{02d6}', '+'); // MODIFIER LETTER PLUS SIGN
        m.insert('\u{fe62}', '+'); // SMALL PLUS SIGN
        m.insert('\u{ff0b}', '+'); // FULLWIDTH PLUS SIGN
        m.insert('\u{3002}', '.'); // IDEOGRAPHIC FULL STOP
        m.insert('\u{fe52}', '.'); // SMALL FULL STOP
        m.insert('\u{ff0e}', '.'); // FULLWIDTH FULL STOP
        m.insert('\u{ff61}', '.'); // HALFWIDTH IDEOGRAPHIC FULL STOP
        m.insert('\u{ff10}', '0'); // FULLWIDTH DIGIT ZERO
        m.insert('\u{ff11}', '1'); // FULLWIDTH DIGIT ONE
        m.insert('\u{ff12}', '2'); // FULLWIDTH DIGIT TWO
        m.insert('\u{ff13}', '3'); // FULLWIDTH DIGIT THREE
        m.insert('\u{ff14}', '4'); // FULLWIDTH DIGIT FOUR
        m.insert('\u{ff15}', '5'); // FULLWIDTH DIGIT FIVE
        m.insert('\u{ff16}', '6'); // FULLWIDTH DIGIT SIX
        m.insert('\u{ff17}', '7'); // FULLWIDTH DIGIT SEVEN
        m.insert('\u{ff18}', '8'); // FULLWIDTH DIGIT EIGHT
        m.insert('\u{ff19}', '9'); // FULLWIDTH DIGIT NINE
        m.insert('\u{02d0}', ':'); // MODIFIER LETTER TRIANGULAR COLON
        m.insert('\u{02f8}', ':'); // MODIFIER LETTER RAISED COLON
        m.insert('\u{2982}', ':'); // Z NOTATION TYPE COLON
        m.insert('\u{a789}', ':'); // MODIFIER LETTER COLON
        m.insert('\u{fe13}', ':'); // PRESENTATION FORM FOR VERTICAL COLON
        m.insert('\u{ff1a}', ':'); // FULLWIDTH COLON
        m.insert('\u{204f}', ';'); // REVERSED SEMICOLON
        m.insert('\u{fe14}', ';'); // PRESENTATION FORM FOR VERTICAL SEMICOLON
        m.insert('\u{fe54}', ';'); // SMALL SEMICOLON
        m.insert('\u{ff1b}', ';'); // FULLWIDTH SEMICOLON
        m.insert('\u{fe64}', '<'); // SMALL LESS-THAN SIGN
        m.insert('\u{ff1c}', '<'); // FULLWIDTH LESS-THAN SIGN
        m.insert('\u{0347}', '='); // COMBINING EQUALS SIGN BELOW
        m.insert('\u{a78a}', '='); // MODIFIER LETTER SHORT EQUALS SIGN
        m.insert('\u{fe66}', '='); // SMALL EQUALS SIGN
        m.insert('\u{ff1d}', '='); // FULLWIDTH EQUALS SIGN
        m.insert('\u{fe65}', '>'); // SMALL GREATER-THAN SIGN
        m.insert('\u{ff1e}', '>'); // FULLWIDTH GREATER-THAN SIGN
        m.insert('\u{fe16}', '?'); // PRESENTATION FORM FOR VERTICAL QUESTION MARK
        m.insert('\u{fe56}', '?'); // SMALL QUESTION MARK
        m.insert('\u{ff1f}', '?'); // FULLWIDTH QUESTION MARK
        m.insert('\u{ff21}', 'A'); // FULLWIDTH LATIN CAPITAL LETTER A
        m.insert('\u{1d00}', 'A'); // LATIN LETTER SMALL CAPITAL A
        m.insert('\u{ff22}', 'B'); // FULLWIDTH LATIN CAPITAL LETTER B
        m.insert('\u{0299}', 'B'); // LATIN LETTER SMALL CAPITAL B
        m.insert('\u{ff23}', 'C'); // FULLWIDTH LATIN CAPITAL LETTER C
        m.insert('\u{1d04}', 'C'); // LATIN LETTER SMALL CAPITAL C
        m.insert('\u{ff24}', 'D'); // FULLWIDTH LATIN CAPITAL LETTER D
        m.insert('\u{1d05}', 'D'); // LATIN LETTER SMALL CAPITAL D
        m.insert('\u{ff25}', 'E'); // FULLWIDTH LATIN CAPITAL LETTER E
        m.insert('\u{1d07}', 'E'); // LATIN LETTER SMALL CAPITAL E
        m.insert('\u{ff26}', 'F'); // FULLWIDTH LATIN CAPITAL LETTER F
        m.insert('\u{a730}', 'F'); // LATIN LETTER SMALL CAPITAL F
        m.insert('\u{ff27}', 'G'); // FULLWIDTH LATIN CAPITAL LETTER G
        m.insert('\u{0262}', 'G'); // LATIN LETTER SMALL CAPITAL G
        m.insert('\u{ff28}', 'H'); // FULLWIDTH LATIN CAPITAL LETTER H
        m.insert('\u{029c}', 'H'); // LATIN LETTER SMALL CAPITAL H
        m.insert('\u{ff29}', 'I'); // FULLWIDTH LATIN CAPITAL LETTER I
        m.insert('\u{026a}', 'I'); // LATIN LETTER SMALL CAPITAL I
        m.insert('\u{ff2a}', 'J'); // FULLWIDTH LATIN CAPITAL LETTER J
        m.insert('\u{1d0a}', 'J'); // LATIN LETTER SMALL CAPITAL J
        m.insert('\u{ff2b}', 'K'); // FULLWIDTH LATIN CAPITAL LETTER K
        m.insert('\u{1d0b}', 'K'); // LATIN LETTER SMALL CAPITAL K
        m.insert('\u{ff2c}', 'L'); // FULLWIDTH LATIN CAPITAL LETTER L
        m.insert('\u{029f}', 'L'); // LATIN LETTER SMALL CAPITAL L
        m.insert('\u{ff2d}', 'M'); // FULLWIDTH LATIN CAPITAL LETTER M
        m.insert('\u{1d0d}', 'M'); // LATIN LETTER SMALL CAPITAL M
        m.insert('\u{ff2e}', 'N'); // FULLWIDTH LATIN CAPITAL LETTER N
        m.insert('\u{0274}', 'N'); // LATIN LETTER SMALL CAPITAL N
        m.insert('\u{ff2f}', 'O'); // FULLWIDTH LATIN CAPITAL LETTER O
        m.insert('\u{1d0f}', 'O'); // LATIN LETTER SMALL CAPITAL O
        m.insert('\u{ff30}', 'P'); // FULLWIDTH LATIN CAPITAL LETTER P
        m.insert('\u{1d18}', 'P'); // LATIN LETTER SMALL CAPITAL P
        m.insert('\u{ff31}', 'Q'); // FULLWIDTH LATIN CAPITAL LETTER Q
        m.insert('\u{ff32}', 'R'); // FULLWIDTH LATIN CAPITAL LETTER R
        m.insert('\u{0280}', 'R'); // LATIN LETTER SMALL CAPITAL R
        m.insert('\u{ff33}', 'S'); // FULLWIDTH LATIN CAPITAL LETTER S
        m.insert('\u{a731}', 'S'); // LATIN LETTER SMALL CAPITAL S
        m.insert('\u{ff34}', 'T'); // FULLWIDTH LATIN CAPITAL LETTER T
        m.insert('\u{1d1b}', 'T'); // LATIN LETTER SMALL CAPITAL T
        m.insert('\u{ff35}', 'U'); // FULLWIDTH LATIN CAPITAL LETTER U
        m.insert('\u{1d1c}', 'U'); // LATIN LETTER SMALL CAPITAL U
        m.insert('\u{ff36}', 'V'); // FULLWIDTH LATIN CAPITAL LETTER V
        m.insert('\u{1d20}', 'V'); // LATIN LETTER SMALL CAPITAL V
        m.insert('\u{ff37}', 'W'); // FULLWIDTH LATIN CAPITAL LETTER W
        m.insert('\u{1d21}', 'W'); // LATIN LETTER SMALL CAPITAL W
        m.insert('\u{ff38}', 'X'); // FULLWIDTH LATIN CAPITAL LETTER X
        m.insert('\u{ff39}', 'Y'); // FULLWIDTH LATIN CAPITAL LETTER Y
        m.insert('\u{028f}', 'Y'); // LATIN LETTER SMALL CAPITAL Y
        m.insert('\u{ff3a}', 'Z'); // FULLWIDTH LATIN CAPITAL LETTER Z
        m.insert('\u{1d22}', 'Z'); // LATIN LETTER SMALL CAPITAL Z
        m.insert('\u{02c6}', '^'); // MODIFIER LETTER CIRCUMFLEX ACCENT
        m.insert('\u{0302}', '^'); // COMBINING CIRCUMFLEX ACCENT
        m.insert('\u{ff3e}', '^'); // FULLWIDTH CIRCUMFLEX ACCENT
        m.insert('\u{1dcd}', '^'); // COMBINING DOUBLE CIRCUMFLEX ABOVE
        m.insert('\u{2774}', '{'); // MEDIUM LEFT CURLY BRACKET ORNAMENT
        m.insert('\u{fe5b}', '{'); // SMALL LEFT CURLY BRACKET
        m.insert('\u{ff5b}', '{'); // FULLWIDTH LEFT CURLY BRACKET
        m.insert('\u{2775}', '}'); // MEDIUM RIGHT CURLY BRACKET ORNAMENT
        m.insert('\u{fe5c}', '}'); // SMALL RIGHT CURLY BRACKET
        m.insert('\u{ff5d}', '}'); // FULLWIDTH RIGHT CURLY BRACKET
        m.insert('\u{ff3b}', '['); // FULLWIDTH LEFT SQUARE BRACKET
        m.insert('\u{ff3d}', ']'); // FULLWIDTH RIGHT SQUARE BRACKET
        m.insert('\u{02dc}', '~'); // SMALL TILDE
        m.insert('\u{02f7}', '~'); // MODIFIER LETTER LOW TILDE
        m.insert('\u{0303}', '~'); // COMBINING TILDE
        m.insert('\u{0330}', '~'); // COMBINING TILDE BELOW
        m.insert('\u{0334}', '~'); // COMBINING TILDE OVERLAY
        m.insert('\u{223c}', '~'); // TILDE OPERATOR
        m.insert('\u{ff5e}', '~'); // FULLWIDTH TILDE
        m.insert('\u{00a0}', ' '); // NO-BREAK SPACE
        m.insert('\u{2000}', ' '); // EN QUAD
        m.insert('\u{2002}', ' '); // EN SPACE
        m.insert('\u{2003}', ' '); // EM SPACE
        m.insert('\u{2004}', ' '); // THREE-PER-EM SPACE
        m.insert('\u{2005}', ' '); // FOUR-PER-EM SPACE
        m.insert('\u{2006}', ' '); // SIX-PER-EM SPACE
        m.insert('\u{2007}', ' '); // FIGURE SPACE
        m.insert('\u{2008}', ' '); // PUNCTUATION SPACE
        m.insert('\u{2009}', ' '); // THIN SPACE
        m.insert('\u{200a}', ' '); // HAIR SPACE
        m.insert('\u{202f}', ' '); // NARROW NO-BREAK SPACE
        m.insert('\u{205f}', ' '); // MEDIUM MATHEMATICAL SPACE
        m.insert('\u{3000}', '　'); // IDEOGRAPHIC SPACE
        m.insert('\u{008d}', ''); // REVERSE LINE FEED (standard LF looks like \n, this looks like a space)
        m.insert('\u{009f}', ''); // <control>
        m.insert('\u{0080}', ''); // C1 CONTROL CODES
        m.insert('\u{0090}', ''); // DEVICE CONTROL STRING
        m.insert('\u{009b}', ''); // CONTROL SEQUENCE INTRODUCER
        m.insert('\u{0010}', ''); // ESCAPE, DATA LINK (not visible)
        m.insert('\u{0009}', '\t'); // TAB (7 spaces based on print statement in Python interpreter)
        m.insert('\u{0000}', '�'); // NULL
        m.insert('\u{0003}', ''); // END OF TEXT
        m.insert('\u{0004}', ''); // END OF TRANSMISSION
        m.insert('\u{0017}', ''); // END OF TRANSMISSION BLOCK
        m.insert('\u{0019}', ''); // END OF MEDIUM
        m.insert('\u{0011}', ''); // DEVICE CONTROL ONE
        m.insert('\u{0012}', ''); // DEVICE CONTROL TWO
        m.insert('\u{0013}', ''); // DEVICE CONTROL THREE
        m.insert('\u{0014}', ''); // DEVICE CONTROL FOUR
        m.insert('\u{2060}', '⁠'); // WORD JOINER
        m.insert('\u{2017}', '_'); // DOUBLE LOW LINE
        m.insert('\u{2014}', '-'); // EM DASH
        m.insert('\u{2013}', '-'); // EN DASH
        m.insert('\u{2039}', '>'); // Single left-pointing angle quotation mark
        m.insert('\u{203a}', '<'); // Single right-pointing angle quotation mark
        m.insert('\u{203c}', '‼'); // Double exclamation mark
        m.insert('\u{2028}', ' '); // Whitespace: Line Separator
        m.insert('\u{2029}', ' '); // Whitespace: Paragraph Separator
        m.insert('\u{2026}', '…'); // HORIZONTAL ELLIPSIS
        m.insert('\u{2001}', ' '); // EM QUAD
        m.insert('\u{200b}', '​'); // ZERO WIDTH SPACE
        m.insert('\u{3001}', ','); // IDEOGRAPHIC COMMA
        m.insert('\u{feff}', '﻿'); // ZERO WIDTH NO-BREAK SPACE
        m.insert('\u{2022}', '-'); // BULLET
        m
    };
}
