mod parser;
mod state;

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    /// Tabs
    fn test_example_001() {
        let parser = Parser::from_reader("\tfoo\tbaz\t\tbim\n".as_bytes());
        assert_eq!("<pre><code>foo\tbaz\t\tbim\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_002() {
        let parser = Parser::from_reader("  \tfoo\tbaz\t\tbim\n".as_bytes());
        assert_eq!("<pre><code>foo\tbaz\t\tbim\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_003() {
        let parser = Parser::from_reader("    a\ta\n    ὐ\ta\n".as_bytes());
        assert_eq!("<pre><code>a\ta\nὐ\ta\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_004() {
        let parser = Parser::from_reader("  - foo\n\n\tbar\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_005() {
        let parser = Parser::from_reader("- foo\n\n\t\tbar\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>foo</p>\n<pre><code>  bar\n</code></pre>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_006() {
        let parser = Parser::from_reader(">\t\tfoo\n".as_bytes());
        assert_eq!("<blockquote>\n<pre><code>  foo\n</code></pre>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_007() {
        let parser = Parser::from_reader("-\t\tfoo\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<pre><code>  foo\n</code></pre>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_008() {
        let parser = Parser::from_reader("    foo\n\tbar\n".as_bytes());
        assert_eq!("<pre><code>foo\nbar\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_009() {
        let parser = Parser::from_reader(" - foo\n   - bar\n\t - baz\n".as_bytes());
        assert_eq!("<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_010() {
        let parser = Parser::from_reader("#\tFoo\n".as_bytes());
        assert_eq!("<h1>Foo</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Tabs
    fn test_example_011() {
        let parser = Parser::from_reader("*\t*\t*\t\n".as_bytes());
        assert_eq!("<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_012() {
        let parser = Parser::from_reader("\\!\\\"\\#\\$\\%\\&\\'\\(\\)\\*\\+\\,\\-\\.\\/\\:\\;\\<\\=\\>\\?\\@\\[\\\\\\]\\^\\_\\`\\{\\|\\}\\~\n".as_bytes());
        assert_eq!("<p>!&quot;#$%&amp;'()*+,-./:;&lt;=&gt;?@[\\]^_`{|}~</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_013() {
        let parser = Parser::from_reader("\\\t\\A\\a\\ \\3\\φ\\«\n".as_bytes());
        assert_eq!("<p>\\\t\\A\\a\\ \\3\\φ\\«</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_014() {
        let parser = Parser::from_reader("\\*not emphasized*\n\\<br/> not a tag\n\\[not a link](/foo)\n\\`not code`\n1\\. not a list\n\\* not a list\n\\# not a heading\n\\[foo]: /url \"not a reference\"\n\\&ouml; not a character entity\n".as_bytes());
        assert_eq!("<p>*not emphasized*\n&lt;br/&gt; not a tag\n[not a link](/foo)\n`not code`\n1. not a list\n* not a list\n# not a heading\n[foo]: /url &quot;not a reference&quot;\n&amp;ouml; not a character entity</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_015() {
        let parser = Parser::from_reader("\\\\*emphasis*\n".as_bytes());
        assert_eq!("<p>\\<em>emphasis</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_016() {
        let parser = Parser::from_reader("foo\\\nbar\n".as_bytes());
        assert_eq!("<p>foo<br />\nbar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_017() {
        let parser = Parser::from_reader("`` \\[\\` ``\n".as_bytes());
        assert_eq!("<p><code>\\[\\`</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_018() {
        let parser = Parser::from_reader("    \\[\\]\n".as_bytes());
        assert_eq!("<pre><code>\\[\\]\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_019() {
        let parser = Parser::from_reader("~~~\n\\[\\]\n~~~\n".as_bytes());
        assert_eq!("<pre><code>\\[\\]\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_020() {
        let parser = Parser::from_reader("<https://example.com?find=\\*>\n".as_bytes());
        assert_eq!("<p><a href=\"https://example.com?find=%5C*\">https://example.com?find=\\*</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_021() {
        let parser = Parser::from_reader("<a href=\"/bar\\/)\">\n".as_bytes());
        assert_eq!("<a href=\"/bar\\/)\">\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_022() {
        let parser = Parser::from_reader("[foo](/bar\\* \"ti\\*tle\")\n".as_bytes());
        assert_eq!("<p><a href=\"/bar*\" title=\"ti*tle\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_023() {
        let parser = Parser::from_reader("[foo]\n\n[foo]: /bar\\* \"ti\\*tle\"\n".as_bytes());
        assert_eq!("<p><a href=\"/bar*\" title=\"ti*tle\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Backslash escapes
    fn test_example_024() {
        let parser = Parser::from_reader("``` foo\\+bar\nfoo\n```\n".as_bytes());
        assert_eq!("<pre><code class=\"language-foo+bar\">foo\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_025() {
        let parser = Parser::from_reader("&nbsp; &amp; &copy; &AElig; &Dcaron;\n&frac34; &HilbertSpace; &DifferentialD;\n&ClockwiseContourIntegral; &ngE;\n".as_bytes());
        assert_eq!("<p>\u{a0} &amp; © Æ Ď\n¾ ℋ ⅆ\n∲ ≧\u{338}</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_026() {
        let parser = Parser::from_reader("&#35; &#1234; &#992; &#0;\n".as_bytes());
        assert_eq!("<p># Ӓ Ϡ �</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_027() {
        let parser = Parser::from_reader("&#X22; &#XD06; &#xcab;\n".as_bytes());
        assert_eq!("<p>&quot; ആ ಫ</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_028() {
        let parser = Parser::from_reader("&nbsp &x; &#; &#x;\n&#87654321;\n&#abcdef0;\n&ThisIsNotDefined; &hi?;\n".as_bytes());
        assert_eq!("<p>&amp;nbsp &amp;x; &amp;#; &amp;#x;\n&amp;#87654321;\n&amp;#abcdef0;\n&amp;ThisIsNotDefined; &amp;hi?;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_029() {
        let parser = Parser::from_reader("&copy\n".as_bytes());
        assert_eq!("<p>&amp;copy</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_030() {
        let parser = Parser::from_reader("&MadeUpEntity;\n".as_bytes());
        assert_eq!("<p>&amp;MadeUpEntity;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_031() {
        let parser = Parser::from_reader("<a href=\"&ouml;&ouml;.html\">\n".as_bytes());
        assert_eq!("<a href=\"&ouml;&ouml;.html\">\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_032() {
        let parser = Parser::from_reader("[foo](/f&ouml;&ouml; \"f&ouml;&ouml;\")\n".as_bytes());
        assert_eq!("<p><a href=\"/f%C3%B6%C3%B6\" title=\"föö\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_033() {
        let parser = Parser::from_reader("[foo]\n\n[foo]: /f&ouml;&ouml; \"f&ouml;&ouml;\"\n".as_bytes());
        assert_eq!("<p><a href=\"/f%C3%B6%C3%B6\" title=\"föö\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_034() {
        let parser = Parser::from_reader("``` f&ouml;&ouml;\nfoo\n```\n".as_bytes());
        assert_eq!("<pre><code class=\"language-föö\">foo\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_035() {
        let parser = Parser::from_reader("`f&ouml;&ouml;`\n".as_bytes());
        assert_eq!("<p><code>f&amp;ouml;&amp;ouml;</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_036() {
        let parser = Parser::from_reader("    f&ouml;f&ouml;\n".as_bytes());
        assert_eq!("<pre><code>f&amp;ouml;f&amp;ouml;\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_037() {
        let parser = Parser::from_reader("&#42;foo&#42;\n*foo*\n".as_bytes());
        assert_eq!("<p>*foo*\n<em>foo</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_038() {
        let parser = Parser::from_reader("&#42; foo\n\n* foo\n".as_bytes());
        assert_eq!("<p>* foo</p>\n<ul>\n<li>foo</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_039() {
        let parser = Parser::from_reader("foo&#10;&#10;bar\n".as_bytes());
        assert_eq!("<p>foo\n\nbar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_040() {
        let parser = Parser::from_reader("&#9;foo\n".as_bytes());
        assert_eq!("<p>\tfoo</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Entity and numeric character references
    fn test_example_041() {
        let parser = Parser::from_reader("[a](url &quot;tit&quot;)\n".as_bytes());
        assert_eq!("<p>[a](url &quot;tit&quot;)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Precedence
    fn test_example_042() {
        let parser = Parser::from_reader("- `one\n- two`\n".as_bytes());
        assert_eq!("<ul>\n<li>`one</li>\n<li>two`</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_043() {
        let parser = Parser::from_reader("***\n---\n___\n".as_bytes());
        assert_eq!("<hr />\n<hr />\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_044() {
        let parser = Parser::from_reader("+++\n".as_bytes());
        assert_eq!("<p>+++</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_045() {
        let parser = Parser::from_reader("===\n".as_bytes());
        assert_eq!("<p>===</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_046() {
        let parser = Parser::from_reader("--\n**\n__\n".as_bytes());
        assert_eq!("<p>--\n**\n__</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_047() {
        let parser = Parser::from_reader(" ***\n  ***\n   ***\n".as_bytes());
        assert_eq!("<hr />\n<hr />\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_048() {
        let parser = Parser::from_reader("    ***\n".as_bytes());
        assert_eq!("<pre><code>***\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_049() {
        let parser = Parser::from_reader("Foo\n    ***\n".as_bytes());
        assert_eq!("<p>Foo\n***</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_050() {
        let parser = Parser::from_reader("_____________________________________\n".as_bytes());
        assert_eq!("<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_051() {
        let parser = Parser::from_reader(" - - -\n".as_bytes());
        assert_eq!("<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_052() {
        let parser = Parser::from_reader(" **  * ** * ** * **\n".as_bytes());
        assert_eq!("<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_053() {
        let parser = Parser::from_reader("-     -      -      -\n".as_bytes());
        assert_eq!("<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_054() {
        let parser = Parser::from_reader("- - - -    \n".as_bytes());
        assert_eq!("<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_055() {
        let parser = Parser::from_reader("_ _ _ _ a\n\na------\n\n---a---\n".as_bytes());
        assert_eq!("<p>_ _ _ _ a</p>\n<p>a------</p>\n<p>---a---</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_056() {
        let parser = Parser::from_reader(" *-*\n".as_bytes());
        assert_eq!("<p><em>-</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_057() {
        let parser = Parser::from_reader("- foo\n***\n- bar\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n</ul>\n<hr />\n<ul>\n<li>bar</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_058() {
        let parser = Parser::from_reader("Foo\n***\nbar\n".as_bytes());
        assert_eq!("<p>Foo</p>\n<hr />\n<p>bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_059() {
        let parser = Parser::from_reader("Foo\n---\nbar\n".as_bytes());
        assert_eq!("<h2>Foo</h2>\n<p>bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_060() {
        let parser = Parser::from_reader("* Foo\n* * *\n* Bar\n".as_bytes());
        assert_eq!("<ul>\n<li>Foo</li>\n</ul>\n<hr />\n<ul>\n<li>Bar</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Thematic breaks
    fn test_example_061() {
        let parser = Parser::from_reader("- Foo\n- * * *\n".as_bytes());
        assert_eq!("<ul>\n<li>Foo</li>\n<li>\n<hr />\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_062() {
        let parser = Parser::from_reader("# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo\n".as_bytes());
        assert_eq!("<h1>foo</h1>\n<h2>foo</h2>\n<h3>foo</h3>\n<h4>foo</h4>\n<h5>foo</h5>\n<h6>foo</h6>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_063() {
        let parser = Parser::from_reader("####### foo\n".as_bytes());
        assert_eq!("<p>####### foo</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_064() {
        let parser = Parser::from_reader("#5 bolt\n\n#hashtag\n".as_bytes());
        assert_eq!("<p>#5 bolt</p>\n<p>#hashtag</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_065() {
        let parser = Parser::from_reader("\\## foo\n".as_bytes());
        assert_eq!("<p>## foo</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_066() {
        let parser = Parser::from_reader("# foo *bar* \\*baz\\*\n".as_bytes());
        assert_eq!("<h1>foo <em>bar</em> *baz*</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_067() {
        let parser = Parser::from_reader("#                  foo                     \n".as_bytes());
        assert_eq!("<h1>foo</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_068() {
        let parser = Parser::from_reader(" ### foo\n  ## foo\n   # foo\n".as_bytes());
        assert_eq!("<h3>foo</h3>\n<h2>foo</h2>\n<h1>foo</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_069() {
        let parser = Parser::from_reader("    # foo\n".as_bytes());
        assert_eq!("<pre><code># foo\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_070() {
        let parser = Parser::from_reader("foo\n    # bar\n".as_bytes());
        assert_eq!("<p>foo\n# bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_071() {
        let parser = Parser::from_reader("## foo ##\n  ###   bar    ###\n".as_bytes());
        assert_eq!("<h2>foo</h2>\n<h3>bar</h3>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_072() {
        let parser = Parser::from_reader("# foo ##################################\n##### foo ##\n".as_bytes());
        assert_eq!("<h1>foo</h1>\n<h5>foo</h5>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_073() {
        let parser = Parser::from_reader("### foo ###     \n".as_bytes());
        assert_eq!("<h3>foo</h3>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_074() {
        let parser = Parser::from_reader("### foo ### b\n".as_bytes());
        assert_eq!("<h3>foo ### b</h3>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_075() {
        let parser = Parser::from_reader("# foo#\n".as_bytes());
        assert_eq!("<h1>foo#</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_076() {
        let parser = Parser::from_reader("### foo \\###\n## foo #\\##\n# foo \\#\n".as_bytes());
        assert_eq!("<h3>foo ###</h3>\n<h2>foo ###</h2>\n<h1>foo #</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_077() {
        let parser = Parser::from_reader("****\n## foo\n****\n".as_bytes());
        assert_eq!("<hr />\n<h2>foo</h2>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_078() {
        let parser = Parser::from_reader("Foo bar\n# baz\nBar foo\n".as_bytes());
        assert_eq!("<p>Foo bar</p>\n<h1>baz</h1>\n<p>Bar foo</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// ATX headings
    fn test_example_079() {
        let parser = Parser::from_reader("## \n#\n### ###\n".as_bytes());
        assert_eq!("<h2></h2>\n<h1></h1>\n<h3></h3>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_080() {
        let parser = Parser::from_reader("Foo *bar*\n=========\n\nFoo *bar*\n---------\n".as_bytes());
        assert_eq!("<h1>Foo <em>bar</em></h1>\n<h2>Foo <em>bar</em></h2>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_081() {
        let parser = Parser::from_reader("Foo *bar\nbaz*\n====\n".as_bytes());
        assert_eq!("<h1>Foo <em>bar\nbaz</em></h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_082() {
        let parser = Parser::from_reader("  Foo *bar\nbaz*\t\n====\n".as_bytes());
        assert_eq!("<h1>Foo <em>bar\nbaz</em></h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_083() {
        let parser = Parser::from_reader("Foo\n-------------------------\n\nFoo\n=\n".as_bytes());
        assert_eq!("<h2>Foo</h2>\n<h1>Foo</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_084() {
        let parser = Parser::from_reader("   Foo\n---\n\n  Foo\n-----\n\n  Foo\n  ===\n".as_bytes());
        assert_eq!("<h2>Foo</h2>\n<h2>Foo</h2>\n<h1>Foo</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_085() {
        let parser = Parser::from_reader("    Foo\n    ---\n\n    Foo\n---\n".as_bytes());
        assert_eq!("<pre><code>Foo\n---\n\nFoo\n</code></pre>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_086() {
        let parser = Parser::from_reader("Foo\n   ----      \n".as_bytes());
        assert_eq!("<h2>Foo</h2>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_087() {
        let parser = Parser::from_reader("Foo\n    ---\n".as_bytes());
        assert_eq!("<p>Foo\n---</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_088() {
        let parser = Parser::from_reader("Foo\n= =\n\nFoo\n--- -\n".as_bytes());
        assert_eq!("<p>Foo\n= =</p>\n<p>Foo</p>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_089() {
        let parser = Parser::from_reader("Foo  \n-----\n".as_bytes());
        assert_eq!("<h2>Foo</h2>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_090() {
        let parser = Parser::from_reader("Foo\\\n----\n".as_bytes());
        assert_eq!("<h2>Foo\\</h2>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_091() {
        let parser = Parser::from_reader("`Foo\n----\n`\n\n<a title=\"a lot\n---\nof dashes\"/>\n".as_bytes());
        assert_eq!("<h2>`Foo</h2>\n<p>`</p>\n<h2>&lt;a title=&quot;a lot</h2>\n<p>of dashes&quot;/&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_092() {
        let parser = Parser::from_reader("> Foo\n---\n".as_bytes());
        assert_eq!("<blockquote>\n<p>Foo</p>\n</blockquote>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_093() {
        let parser = Parser::from_reader("> foo\nbar\n===\n".as_bytes());
        assert_eq!("<blockquote>\n<p>foo\nbar\n===</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_094() {
        let parser = Parser::from_reader("- Foo\n---\n".as_bytes());
        assert_eq!("<ul>\n<li>Foo</li>\n</ul>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_095() {
        let parser = Parser::from_reader("Foo\nBar\n---\n".as_bytes());
        assert_eq!("<h2>Foo\nBar</h2>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_096() {
        let parser = Parser::from_reader("---\nFoo\n---\nBar\n---\nBaz\n".as_bytes());
        assert_eq!("<hr />\n<h2>Foo</h2>\n<h2>Bar</h2>\n<p>Baz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_097() {
        let parser = Parser::from_reader("\n====\n".as_bytes());
        assert_eq!("<p>====</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_098() {
        let parser = Parser::from_reader("---\n---\n".as_bytes());
        assert_eq!("<hr />\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_099() {
        let parser = Parser::from_reader("- foo\n-----\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n</ul>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_100() {
        let parser = Parser::from_reader("    foo\n---\n".as_bytes());
        assert_eq!("<pre><code>foo\n</code></pre>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_101() {
        let parser = Parser::from_reader("> foo\n-----\n".as_bytes());
        assert_eq!("<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_102() {
        let parser = Parser::from_reader("\\> foo\n------\n".as_bytes());
        assert_eq!("<h2>&gt; foo</h2>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_103() {
        let parser = Parser::from_reader("Foo\n\nbar\n---\nbaz\n".as_bytes());
        assert_eq!("<p>Foo</p>\n<h2>bar</h2>\n<p>baz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_104() {
        let parser = Parser::from_reader("Foo\nbar\n\n---\n\nbaz\n".as_bytes());
        assert_eq!("<p>Foo\nbar</p>\n<hr />\n<p>baz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_105() {
        let parser = Parser::from_reader("Foo\nbar\n* * *\nbaz\n".as_bytes());
        assert_eq!("<p>Foo\nbar</p>\n<hr />\n<p>baz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Setext headings
    fn test_example_106() {
        let parser = Parser::from_reader("Foo\nbar\n\\---\nbaz\n".as_bytes());
        assert_eq!("<p>Foo\nbar\n---\nbaz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_107() {
        let parser = Parser::from_reader("    a simple\n      indented code block\n".as_bytes());
        assert_eq!("<pre><code>a simple\n  indented code block\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_108() {
        let parser = Parser::from_reader("  - foo\n\n    bar\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_109() {
        let parser = Parser::from_reader("1.  foo\n\n    - bar\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_110() {
        let parser = Parser::from_reader("    <a/>\n    *hi*\n\n    - one\n".as_bytes());
        assert_eq!("<pre><code>&lt;a/&gt;\n*hi*\n\n- one\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_111() {
        let parser = Parser::from_reader("    chunk1\n\n    chunk2\n  \n \n \n    chunk3\n".as_bytes());
        assert_eq!("<pre><code>chunk1\n\nchunk2\n\n\n\nchunk3\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_112() {
        let parser = Parser::from_reader("    chunk1\n      \n      chunk2\n".as_bytes());
        assert_eq!("<pre><code>chunk1\n  \n  chunk2\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_113() {
        let parser = Parser::from_reader("Foo\n    bar\n\n".as_bytes());
        assert_eq!("<p>Foo\nbar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_114() {
        let parser = Parser::from_reader("    foo\nbar\n".as_bytes());
        assert_eq!("<pre><code>foo\n</code></pre>\n<p>bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_115() {
        let parser = Parser::from_reader("# Heading\n    foo\nHeading\n------\n    foo\n----\n".as_bytes());
        assert_eq!("<h1>Heading</h1>\n<pre><code>foo\n</code></pre>\n<h2>Heading</h2>\n<pre><code>foo\n</code></pre>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_116() {
        let parser = Parser::from_reader("        foo\n    bar\n".as_bytes());
        assert_eq!("<pre><code>    foo\nbar\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_117() {
        let parser = Parser::from_reader("\n    \n    foo\n    \n\n".as_bytes());
        assert_eq!("<pre><code>foo\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Indented code blocks
    fn test_example_118() {
        let parser = Parser::from_reader("    foo  \n".as_bytes());
        assert_eq!("<pre><code>foo  \n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_119() {
        let parser = Parser::from_reader("```\n<\n >\n```\n".as_bytes());
        assert_eq!("<pre><code>&lt;\n &gt;\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_120() {
        let parser = Parser::from_reader("~~~\n<\n >\n~~~\n".as_bytes());
        assert_eq!("<pre><code>&lt;\n &gt;\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_121() {
        let parser = Parser::from_reader("``\nfoo\n``\n".as_bytes());
        assert_eq!("<p><code>foo</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_122() {
        let parser = Parser::from_reader("```\naaa\n~~~\n```\n".as_bytes());
        assert_eq!("<pre><code>aaa\n~~~\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_123() {
        let parser = Parser::from_reader("~~~\naaa\n```\n~~~\n".as_bytes());
        assert_eq!("<pre><code>aaa\n```\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_124() {
        let parser = Parser::from_reader("````\naaa\n```\n``````\n".as_bytes());
        assert_eq!("<pre><code>aaa\n```\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_125() {
        let parser = Parser::from_reader("~~~~\naaa\n~~~\n~~~~\n".as_bytes());
        assert_eq!("<pre><code>aaa\n~~~\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_126() {
        let parser = Parser::from_reader("```\n".as_bytes());
        assert_eq!("<pre><code></code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_127() {
        let parser = Parser::from_reader("`````\n\n```\naaa\n".as_bytes());
        assert_eq!("<pre><code>\n```\naaa\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_128() {
        let parser = Parser::from_reader("> ```\n> aaa\n\nbbb\n".as_bytes());
        assert_eq!("<blockquote>\n<pre><code>aaa\n</code></pre>\n</blockquote>\n<p>bbb</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_129() {
        let parser = Parser::from_reader("```\n\n  \n```\n".as_bytes());
        assert_eq!("<pre><code>\n  \n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_130() {
        let parser = Parser::from_reader("```\n```\n".as_bytes());
        assert_eq!("<pre><code></code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_131() {
        let parser = Parser::from_reader(" ```\n aaa\naaa\n```\n".as_bytes());
        assert_eq!("<pre><code>aaa\naaa\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_132() {
        let parser = Parser::from_reader("  ```\naaa\n  aaa\naaa\n  ```\n".as_bytes());
        assert_eq!("<pre><code>aaa\naaa\naaa\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_133() {
        let parser = Parser::from_reader("   ```\n   aaa\n    aaa\n  aaa\n   ```\n".as_bytes());
        assert_eq!("<pre><code>aaa\n aaa\naaa\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_134() {
        let parser = Parser::from_reader("    ```\n    aaa\n    ```\n".as_bytes());
        assert_eq!("<pre><code>```\naaa\n```\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_135() {
        let parser = Parser::from_reader("```\naaa\n  ```\n".as_bytes());
        assert_eq!("<pre><code>aaa\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_136() {
        let parser = Parser::from_reader("   ```\naaa\n  ```\n".as_bytes());
        assert_eq!("<pre><code>aaa\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_137() {
        let parser = Parser::from_reader("```\naaa\n    ```\n".as_bytes());
        assert_eq!("<pre><code>aaa\n    ```\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_138() {
        let parser = Parser::from_reader("``` ```\naaa\n".as_bytes());
        assert_eq!("<p><code> </code>\naaa</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_139() {
        let parser = Parser::from_reader("~~~~~~\naaa\n~~~ ~~\n".as_bytes());
        assert_eq!("<pre><code>aaa\n~~~ ~~\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_140() {
        let parser = Parser::from_reader("foo\n```\nbar\n```\nbaz\n".as_bytes());
        assert_eq!("<p>foo</p>\n<pre><code>bar\n</code></pre>\n<p>baz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_141() {
        let parser = Parser::from_reader("foo\n---\n~~~\nbar\n~~~\n# baz\n".as_bytes());
        assert_eq!("<h2>foo</h2>\n<pre><code>bar\n</code></pre>\n<h1>baz</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_142() {
        let parser = Parser::from_reader("```ruby\ndef foo(x)\n  return 3\nend\n```\n".as_bytes());
        assert_eq!("<pre><code class=\"language-ruby\">def foo(x)\n  return 3\nend\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_143() {
        let parser = Parser::from_reader("~~~~    ruby startline=3 $%@#$\ndef foo(x)\n  return 3\nend\n~~~~~~~\n".as_bytes());
        assert_eq!("<pre><code class=\"language-ruby\">def foo(x)\n  return 3\nend\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_144() {
        let parser = Parser::from_reader("````;\n````\n".as_bytes());
        assert_eq!("<pre><code class=\"language-;\"></code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_145() {
        let parser = Parser::from_reader("``` aa ```\nfoo\n".as_bytes());
        assert_eq!("<p><code>aa</code>\nfoo</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_146() {
        let parser = Parser::from_reader("~~~ aa ``` ~~~\nfoo\n~~~\n".as_bytes());
        assert_eq!("<pre><code class=\"language-aa\">foo\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Fenced code blocks
    fn test_example_147() {
        let parser = Parser::from_reader("```\n``` aaa\n```\n".as_bytes());
        assert_eq!("<pre><code>``` aaa\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_148() {
        let parser = Parser::from_reader("<table><tr><td>\n<pre>\n**Hello**,\n\n_world_.\n</pre>\n</td></tr></table>\n".as_bytes());
        assert_eq!("<table><tr><td>\n<pre>\n**Hello**,\n<p><em>world</em>.\n</pre></p>\n</td></tr></table>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_149() {
        let parser = Parser::from_reader("<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  </tr>\n</table>\n\nokay.\n".as_bytes());
        assert_eq!("<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  </tr>\n</table>\n<p>okay.</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_150() {
        let parser = Parser::from_reader(" <div>\n  *hello*\n         <foo><a>\n".as_bytes());
        assert_eq!(" <div>\n  *hello*\n         <foo><a>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_151() {
        let parser = Parser::from_reader("</div>\n*foo*\n".as_bytes());
        assert_eq!("</div>\n*foo*\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_152() {
        let parser = Parser::from_reader("<DIV CLASS=\"foo\">\n\n*Markdown*\n\n</DIV>\n".as_bytes());
        assert_eq!("<DIV CLASS=\"foo\">\n<p><em>Markdown</em></p>\n</DIV>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_153() {
        let parser = Parser::from_reader("<div id=\"foo\"\n  class=\"bar\">\n</div>\n".as_bytes());
        assert_eq!("<div id=\"foo\"\n  class=\"bar\">\n</div>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_154() {
        let parser = Parser::from_reader("<div id=\"foo\" class=\"bar\n  baz\">\n</div>\n".as_bytes());
        assert_eq!("<div id=\"foo\" class=\"bar\n  baz\">\n</div>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_155() {
        let parser = Parser::from_reader("<div>\n*foo*\n\n*bar*\n".as_bytes());
        assert_eq!("<div>\n*foo*\n<p><em>bar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_156() {
        let parser = Parser::from_reader("<div id=\"foo\"\n*hi*\n".as_bytes());
        assert_eq!("<div id=\"foo\"\n*hi*\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_157() {
        let parser = Parser::from_reader("<div class\nfoo\n".as_bytes());
        assert_eq!("<div class\nfoo\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_158() {
        let parser = Parser::from_reader("<div *???-&&&-<---\n*foo*\n".as_bytes());
        assert_eq!("<div *???-&&&-<---\n*foo*\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_159() {
        let parser = Parser::from_reader("<div><a href=\"bar\">*foo*</a></div>\n".as_bytes());
        assert_eq!("<div><a href=\"bar\">*foo*</a></div>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_160() {
        let parser = Parser::from_reader("<table><tr><td>\nfoo\n</td></tr></table>\n".as_bytes());
        assert_eq!("<table><tr><td>\nfoo\n</td></tr></table>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_161() {
        let parser = Parser::from_reader("<div></div>\n``` c\nint x = 33;\n```\n".as_bytes());
        assert_eq!("<div></div>\n``` c\nint x = 33;\n```\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_162() {
        let parser = Parser::from_reader("<a href=\"foo\">\n*bar*\n</a>\n".as_bytes());
        assert_eq!("<a href=\"foo\">\n*bar*\n</a>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_163() {
        let parser = Parser::from_reader("<Warning>\n*bar*\n</Warning>\n".as_bytes());
        assert_eq!("<Warning>\n*bar*\n</Warning>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_164() {
        let parser = Parser::from_reader("<i class=\"foo\">\n*bar*\n</i>\n".as_bytes());
        assert_eq!("<i class=\"foo\">\n*bar*\n</i>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_165() {
        let parser = Parser::from_reader("</ins>\n*bar*\n".as_bytes());
        assert_eq!("</ins>\n*bar*\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_166() {
        let parser = Parser::from_reader("<del>\n*foo*\n</del>\n".as_bytes());
        assert_eq!("<del>\n*foo*\n</del>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_167() {
        let parser = Parser::from_reader("<del>\n\n*foo*\n\n</del>\n".as_bytes());
        assert_eq!("<del>\n<p><em>foo</em></p>\n</del>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_168() {
        let parser = Parser::from_reader("<del>*foo*</del>\n".as_bytes());
        assert_eq!("<p><del><em>foo</em></del></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_169() {
        let parser = Parser::from_reader("<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ parseTags tags\n</code></pre>\nokay\n".as_bytes());
        assert_eq!("<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ parseTags tags\n</code></pre>\n<p>okay</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_170() {
        let parser = Parser::from_reader("<script type=\"text/javascript\">\n// JavaScript example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>\nokay\n".as_bytes());
        assert_eq!("<script type=\"text/javascript\">\n// JavaScript example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>\n<p>okay</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_171() {
        let parser = Parser::from_reader("<textarea>\n\n*foo*\n\n_bar_\n\n</textarea>\n".as_bytes());
        assert_eq!("<textarea>\n\n*foo*\n\n_bar_\n\n</textarea>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_172() {
        let parser = Parser::from_reader("<style\n  type=\"text/css\">\nh1 {color:red;}\n\np {color:blue;}\n</style>\nokay\n".as_bytes());
        assert_eq!("<style\n  type=\"text/css\">\nh1 {color:red;}\n\np {color:blue;}\n</style>\n<p>okay</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_173() {
        let parser = Parser::from_reader("<style\n  type=\"text/css\">\n\nfoo\n".as_bytes());
        assert_eq!("<style\n  type=\"text/css\">\n\nfoo\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_174() {
        let parser = Parser::from_reader("> <div>\n> foo\n\nbar\n".as_bytes());
        assert_eq!("<blockquote>\n<div>\nfoo\n</blockquote>\n<p>bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_175() {
        let parser = Parser::from_reader("- <div>\n- foo\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<div>\n</li>\n<li>foo</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_176() {
        let parser = Parser::from_reader("<style>p{color:red;}</style>\n*foo*\n".as_bytes());
        assert_eq!("<style>p{color:red;}</style>\n<p><em>foo</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_177() {
        let parser = Parser::from_reader("<!-- foo -->*bar*\n*baz*\n".as_bytes());
        assert_eq!("<!-- foo -->*bar*\n<p><em>baz</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_178() {
        let parser = Parser::from_reader("<script>\nfoo\n</script>1. *bar*\n".as_bytes());
        assert_eq!("<script>\nfoo\n</script>1. *bar*\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_179() {
        let parser = Parser::from_reader("<!-- Foo\n\nbar\n   baz -->\nokay\n".as_bytes());
        assert_eq!("<!-- Foo\n\nbar\n   baz -->\n<p>okay</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_180() {
        let parser = Parser::from_reader("<?php\n\n  echo '>';\n\n?>\nokay\n".as_bytes());
        assert_eq!("<?php\n\n  echo '>';\n\n?>\n<p>okay</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_181() {
        let parser = Parser::from_reader("<!DOCTYPE html>\n".as_bytes());
        assert_eq!("<!DOCTYPE html>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_182() {
        let parser = Parser::from_reader("<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then {\n    return 1;\n\n  } else {\n\n    return 0;\n  }\n}\n]]>\nokay\n".as_bytes());
        assert_eq!("<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then {\n    return 1;\n\n  } else {\n\n    return 0;\n  }\n}\n]]>\n<p>okay</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_183() {
        let parser = Parser::from_reader("  <!-- foo -->\n\n    <!-- foo -->\n".as_bytes());
        assert_eq!("  <!-- foo -->\n<pre><code>&lt;!-- foo --&gt;\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_184() {
        let parser = Parser::from_reader("  <div>\n\n    <div>\n".as_bytes());
        assert_eq!("  <div>\n<pre><code>&lt;div&gt;\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_185() {
        let parser = Parser::from_reader("Foo\n<div>\nbar\n</div>\n".as_bytes());
        assert_eq!("<p>Foo</p>\n<div>\nbar\n</div>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_186() {
        let parser = Parser::from_reader("<div>\nbar\n</div>\n*foo*\n".as_bytes());
        assert_eq!("<div>\nbar\n</div>\n*foo*\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_187() {
        let parser = Parser::from_reader("Foo\n<a href=\"bar\">\nbaz\n".as_bytes());
        assert_eq!("<p>Foo\n<a href=\"bar\">\nbaz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_188() {
        let parser = Parser::from_reader("<div>\n\n*Emphasized* text.\n\n</div>\n".as_bytes());
        assert_eq!("<div>\n<p><em>Emphasized</em> text.</p>\n</div>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_189() {
        let parser = Parser::from_reader("<div>\n*Emphasized* text.\n</div>\n".as_bytes());
        assert_eq!("<div>\n*Emphasized* text.\n</div>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_190() {
        let parser = Parser::from_reader("<table>\n\n<tr>\n\n<td>\nHi\n</td>\n\n</tr>\n\n</table>\n".as_bytes());
        assert_eq!("<table>\n<tr>\n<td>\nHi\n</td>\n</tr>\n</table>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// HTML blocks
    fn test_example_191() {
        let parser = Parser::from_reader("<table>\n\n  <tr>\n\n    <td>\n      Hi\n    </td>\n\n  </tr>\n\n</table>\n".as_bytes());
        assert_eq!("<table>\n  <tr>\n<pre><code>&lt;td&gt;\n  Hi\n&lt;/td&gt;\n</code></pre>\n  </tr>\n</table>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_192() {
        let parser = Parser::from_reader("[foo]: /url \"title\"\n\n[foo]\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_193() {
        let parser = Parser::from_reader("   [foo]: \n      /url  \n           'the title'  \n\n[foo]\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"the title\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_194() {
        let parser = Parser::from_reader("[Foo*bar\\]]:my_(url) 'title (with parens)'\n\n[Foo*bar\\]]\n".as_bytes());
        assert_eq!("<p><a href=\"my_(url)\" title=\"title (with parens)\">Foo*bar]</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_195() {
        let parser = Parser::from_reader("[Foo bar]:\n<my url>\n'title'\n\n[Foo bar]\n".as_bytes());
        assert_eq!("<p><a href=\"my%20url\" title=\"title\">Foo bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_196() {
        let parser = Parser::from_reader("[foo]: /url '\ntitle\nline1\nline2\n'\n\n[foo]\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"\ntitle\nline1\nline2\n\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_197() {
        let parser = Parser::from_reader("[foo]: /url 'title\n\nwith blank line'\n\n[foo]\n".as_bytes());
        assert_eq!("<p>[foo]: /url 'title</p>\n<p>with blank line'</p>\n<p>[foo]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_198() {
        let parser = Parser::from_reader("[foo]:\n/url\n\n[foo]\n".as_bytes());
        assert_eq!("<p><a href=\"/url\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_199() {
        let parser = Parser::from_reader("[foo]:\n\n[foo]\n".as_bytes());
        assert_eq!("<p>[foo]:</p>\n<p>[foo]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_200() {
        let parser = Parser::from_reader("[foo]: <>\n\n[foo]\n".as_bytes());
        assert_eq!("<p><a href=\"\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_201() {
        let parser = Parser::from_reader("[foo]: <bar>(baz)\n\n[foo]\n".as_bytes());
        assert_eq!("<p>[foo]: <bar>(baz)</p>\n<p>[foo]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_202() {
        let parser = Parser::from_reader("[foo]: /url\\bar\\*baz \"foo\\\"bar\\baz\"\n\n[foo]\n".as_bytes());
        assert_eq!("<p><a href=\"/url%5Cbar*baz\" title=\"foo&quot;bar\\baz\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_203() {
        let parser = Parser::from_reader("[foo]\n\n[foo]: url\n".as_bytes());
        assert_eq!("<p><a href=\"url\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_204() {
        let parser = Parser::from_reader("[foo]\n\n[foo]: first\n[foo]: second\n".as_bytes());
        assert_eq!("<p><a href=\"first\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_205() {
        let parser = Parser::from_reader("[FOO]: /url\n\n[Foo]\n".as_bytes());
        assert_eq!("<p><a href=\"/url\">Foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_206() {
        let parser = Parser::from_reader("[ΑΓΩ]: /φου\n\n[αγω]\n".as_bytes());
        assert_eq!("<p><a href=\"/%CF%86%CE%BF%CF%85\">αγω</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_207() {
        let parser = Parser::from_reader("[foo]: /url\n".as_bytes());
        assert_eq!("", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_208() {
        let parser = Parser::from_reader("[\nfoo\n]: /url\nbar\n".as_bytes());
        assert_eq!("<p>bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_209() {
        let parser = Parser::from_reader("[foo]: /url \"title\" ok\n".as_bytes());
        assert_eq!("<p>[foo]: /url &quot;title&quot; ok</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_210() {
        let parser = Parser::from_reader("[foo]: /url\n\"title\" ok\n".as_bytes());
        assert_eq!("<p>&quot;title&quot; ok</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_211() {
        let parser = Parser::from_reader("    [foo]: /url \"title\"\n\n[foo]\n".as_bytes());
        assert_eq!("<pre><code>[foo]: /url &quot;title&quot;\n</code></pre>\n<p>[foo]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_212() {
        let parser = Parser::from_reader("```\n[foo]: /url\n```\n\n[foo]\n".as_bytes());
        assert_eq!("<pre><code>[foo]: /url\n</code></pre>\n<p>[foo]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_213() {
        let parser = Parser::from_reader("Foo\n[bar]: /baz\n\n[bar]\n".as_bytes());
        assert_eq!("<p>Foo\n[bar]: /baz</p>\n<p>[bar]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_214() {
        let parser = Parser::from_reader("# [Foo]\n[foo]: /url\n> bar\n".as_bytes());
        assert_eq!("<h1><a href=\"/url\">Foo</a></h1>\n<blockquote>\n<p>bar</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_215() {
        let parser = Parser::from_reader("[foo]: /url\nbar\n===\n[foo]\n".as_bytes());
        assert_eq!("<h1>bar</h1>\n<p><a href=\"/url\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_216() {
        let parser = Parser::from_reader("[foo]: /url\n===\n[foo]\n".as_bytes());
        assert_eq!("<p>===\n<a href=\"/url\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_217() {
        let parser = Parser::from_reader("[foo]: /foo-url \"foo\"\n[bar]: /bar-url\n  \"bar\"\n[baz]: /baz-url\n\n[foo],\n[bar],\n[baz]\n".as_bytes());
        assert_eq!("<p><a href=\"/foo-url\" title=\"foo\">foo</a>,\n<a href=\"/bar-url\" title=\"bar\">bar</a>,\n<a href=\"/baz-url\">baz</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Link reference definitions
    fn test_example_218() {
        let parser = Parser::from_reader("[foo]\n\n> [foo]: /url\n".as_bytes());
        assert_eq!("<p><a href=\"/url\">foo</a></p>\n<blockquote>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Paragraphs
    fn test_example_219() {
        let parser = Parser::from_reader("aaa\n\nbbb\n".as_bytes());
        assert_eq!("<p>aaa</p>\n<p>bbb</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Paragraphs
    fn test_example_220() {
        let parser = Parser::from_reader("aaa\nbbb\n\nccc\nddd\n".as_bytes());
        assert_eq!("<p>aaa\nbbb</p>\n<p>ccc\nddd</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Paragraphs
    fn test_example_221() {
        let parser = Parser::from_reader("aaa\n\n\nbbb\n".as_bytes());
        assert_eq!("<p>aaa</p>\n<p>bbb</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Paragraphs
    fn test_example_222() {
        let parser = Parser::from_reader("  aaa\n bbb\n".as_bytes());
        assert_eq!("<p>aaa\nbbb</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Paragraphs
    fn test_example_223() {
        let parser = Parser::from_reader("aaa\n             bbb\n                                       ccc\n".as_bytes());
        assert_eq!("<p>aaa\nbbb\nccc</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Paragraphs
    fn test_example_224() {
        let parser = Parser::from_reader("   aaa\nbbb\n".as_bytes());
        assert_eq!("<p>aaa\nbbb</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Paragraphs
    fn test_example_225() {
        let parser = Parser::from_reader("    aaa\nbbb\n".as_bytes());
        assert_eq!("<pre><code>aaa\n</code></pre>\n<p>bbb</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Paragraphs
    fn test_example_226() {
        let parser = Parser::from_reader("aaa     \nbbb     \n".as_bytes());
        assert_eq!("<p>aaa<br />\nbbb</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Blank lines
    fn test_example_227() {
        let parser = Parser::from_reader("  \n\naaa\n  \n\n# aaa\n\n  \n".as_bytes());
        assert_eq!("<p>aaa</p>\n<h1>aaa</h1>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_228() {
        let parser = Parser::from_reader("> # Foo\n> bar\n> baz\n".as_bytes());
        assert_eq!("<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_229() {
        let parser = Parser::from_reader("># Foo\n>bar\n> baz\n".as_bytes());
        assert_eq!("<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_230() {
        let parser = Parser::from_reader("   > # Foo\n   > bar\n > baz\n".as_bytes());
        assert_eq!("<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_231() {
        let parser = Parser::from_reader("    > # Foo\n    > bar\n    > baz\n".as_bytes());
        assert_eq!("<pre><code>&gt; # Foo\n&gt; bar\n&gt; baz\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_232() {
        let parser = Parser::from_reader("> # Foo\n> bar\nbaz\n".as_bytes());
        assert_eq!("<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_233() {
        let parser = Parser::from_reader("> bar\nbaz\n> foo\n".as_bytes());
        assert_eq!("<blockquote>\n<p>bar\nbaz\nfoo</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_234() {
        let parser = Parser::from_reader("> foo\n---\n".as_bytes());
        assert_eq!("<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_235() {
        let parser = Parser::from_reader("> - foo\n- bar\n".as_bytes());
        assert_eq!("<blockquote>\n<ul>\n<li>foo</li>\n</ul>\n</blockquote>\n<ul>\n<li>bar</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_236() {
        let parser = Parser::from_reader(">     foo\n    bar\n".as_bytes());
        assert_eq!("<blockquote>\n<pre><code>foo\n</code></pre>\n</blockquote>\n<pre><code>bar\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_237() {
        let parser = Parser::from_reader("> ```\nfoo\n```\n".as_bytes());
        assert_eq!("<blockquote>\n<pre><code></code></pre>\n</blockquote>\n<p>foo</p>\n<pre><code></code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_238() {
        let parser = Parser::from_reader("> foo\n    - bar\n".as_bytes());
        assert_eq!("<blockquote>\n<p>foo\n- bar</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_239() {
        let parser = Parser::from_reader(">\n".as_bytes());
        assert_eq!("<blockquote>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_240() {
        let parser = Parser::from_reader(">\n>  \n> \n".as_bytes());
        assert_eq!("<blockquote>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_241() {
        let parser = Parser::from_reader(">\n> foo\n>  \n".as_bytes());
        assert_eq!("<blockquote>\n<p>foo</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_242() {
        let parser = Parser::from_reader("> foo\n\n> bar\n".as_bytes());
        assert_eq!("<blockquote>\n<p>foo</p>\n</blockquote>\n<blockquote>\n<p>bar</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_243() {
        let parser = Parser::from_reader("> foo\n> bar\n".as_bytes());
        assert_eq!("<blockquote>\n<p>foo\nbar</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_244() {
        let parser = Parser::from_reader("> foo\n>\n> bar\n".as_bytes());
        assert_eq!("<blockquote>\n<p>foo</p>\n<p>bar</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_245() {
        let parser = Parser::from_reader("foo\n> bar\n".as_bytes());
        assert_eq!("<p>foo</p>\n<blockquote>\n<p>bar</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_246() {
        let parser = Parser::from_reader("> aaa\n***\n> bbb\n".as_bytes());
        assert_eq!("<blockquote>\n<p>aaa</p>\n</blockquote>\n<hr />\n<blockquote>\n<p>bbb</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_247() {
        let parser = Parser::from_reader("> bar\nbaz\n".as_bytes());
        assert_eq!("<blockquote>\n<p>bar\nbaz</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_248() {
        let parser = Parser::from_reader("> bar\n\nbaz\n".as_bytes());
        assert_eq!("<blockquote>\n<p>bar</p>\n</blockquote>\n<p>baz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_249() {
        let parser = Parser::from_reader("> bar\n>\nbaz\n".as_bytes());
        assert_eq!("<blockquote>\n<p>bar</p>\n</blockquote>\n<p>baz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_250() {
        let parser = Parser::from_reader("> > > foo\nbar\n".as_bytes());
        assert_eq!("<blockquote>\n<blockquote>\n<blockquote>\n<p>foo\nbar</p>\n</blockquote>\n</blockquote>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_251() {
        let parser = Parser::from_reader(">>> foo\n> bar\n>>baz\n".as_bytes());
        assert_eq!("<blockquote>\n<blockquote>\n<blockquote>\n<p>foo\nbar\nbaz</p>\n</blockquote>\n</blockquote>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Block quotes
    fn test_example_252() {
        let parser = Parser::from_reader(">     code\n\n>    not code\n".as_bytes());
        assert_eq!("<blockquote>\n<pre><code>code\n</code></pre>\n</blockquote>\n<blockquote>\n<p>not code</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_253() {
        let parser = Parser::from_reader("A paragraph\nwith two lines.\n\n    indented code\n\n> A block quote.\n".as_bytes());
        assert_eq!("<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_254() {
        let parser = Parser::from_reader("1.  A paragraph\n    with two lines.\n\n        indented code\n\n    > A block quote.\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_255() {
        let parser = Parser::from_reader("- one\n\n two\n".as_bytes());
        assert_eq!("<ul>\n<li>one</li>\n</ul>\n<p>two</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_256() {
        let parser = Parser::from_reader("- one\n\n  two\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_257() {
        let parser = Parser::from_reader(" -    one\n\n     two\n".as_bytes());
        assert_eq!("<ul>\n<li>one</li>\n</ul>\n<pre><code> two\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_258() {
        let parser = Parser::from_reader(" -    one\n\n      two\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_259() {
        let parser = Parser::from_reader("   > > 1.  one\n>>\n>>     two\n".as_bytes());
        assert_eq!("<blockquote>\n<blockquote>\n<ol>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ol>\n</blockquote>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_260() {
        let parser = Parser::from_reader(">>- one\n>>\n  >  > two\n".as_bytes());
        assert_eq!("<blockquote>\n<blockquote>\n<ul>\n<li>one</li>\n</ul>\n<p>two</p>\n</blockquote>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_261() {
        let parser = Parser::from_reader("-one\n\n2.two\n".as_bytes());
        assert_eq!("<p>-one</p>\n<p>2.two</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_262() {
        let parser = Parser::from_reader("- foo\n\n\n  bar\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_263() {
        let parser = Parser::from_reader("1.  foo\n\n    ```\n    bar\n    ```\n\n    baz\n\n    > bam\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n<p>baz</p>\n<blockquote>\n<p>bam</p>\n</blockquote>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_264() {
        let parser = Parser::from_reader("- Foo\n\n      bar\n\n\n      baz\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>Foo</p>\n<pre><code>bar\n\n\nbaz\n</code></pre>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_265() {
        let parser = Parser::from_reader("123456789. ok\n".as_bytes());
        assert_eq!("<ol start=\"123456789\">\n<li>ok</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_266() {
        let parser = Parser::from_reader("1234567890. not ok\n".as_bytes());
        assert_eq!("<p>1234567890. not ok</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_267() {
        let parser = Parser::from_reader("0. ok\n".as_bytes());
        assert_eq!("<ol start=\"0\">\n<li>ok</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_268() {
        let parser = Parser::from_reader("003. ok\n".as_bytes());
        assert_eq!("<ol start=\"3\">\n<li>ok</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_269() {
        let parser = Parser::from_reader("-1. not ok\n".as_bytes());
        assert_eq!("<p>-1. not ok</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_270() {
        let parser = Parser::from_reader("- foo\n\n      bar\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_271() {
        let parser = Parser::from_reader("  10.  foo\n\n           bar\n".as_bytes());
        assert_eq!("<ol start=\"10\">\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_272() {
        let parser = Parser::from_reader("    indented code\n\nparagraph\n\n    more code\n".as_bytes());
        assert_eq!("<pre><code>indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_273() {
        let parser = Parser::from_reader("1.     indented code\n\n   paragraph\n\n       more code\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<pre><code>indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_274() {
        let parser = Parser::from_reader("1.      indented code\n\n   paragraph\n\n       more code\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<pre><code> indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_275() {
        let parser = Parser::from_reader("   foo\n\nbar\n".as_bytes());
        assert_eq!("<p>foo</p>\n<p>bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_276() {
        let parser = Parser::from_reader("-    foo\n\n  bar\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n</ul>\n<p>bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_277() {
        let parser = Parser::from_reader("-  foo\n\n   bar\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_278() {
        let parser = Parser::from_reader("-\n  foo\n-\n  ```\n  bar\n  ```\n-\n      baz\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n<li>\n<pre><code>bar\n</code></pre>\n</li>\n<li>\n<pre><code>baz\n</code></pre>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_279() {
        let parser = Parser::from_reader("-   \n  foo\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_280() {
        let parser = Parser::from_reader("-\n\n  foo\n".as_bytes());
        assert_eq!("<ul>\n<li></li>\n</ul>\n<p>foo</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_281() {
        let parser = Parser::from_reader("- foo\n-\n- bar\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_282() {
        let parser = Parser::from_reader("- foo\n-   \n- bar\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_283() {
        let parser = Parser::from_reader("1. foo\n2.\n3. bar\n".as_bytes());
        assert_eq!("<ol>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_284() {
        let parser = Parser::from_reader("*\n".as_bytes());
        assert_eq!("<ul>\n<li></li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_285() {
        let parser = Parser::from_reader("foo\n*\n\nfoo\n1.\n".as_bytes());
        assert_eq!("<p>foo\n*</p>\n<p>foo\n1.</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_286() {
        let parser = Parser::from_reader(" 1.  A paragraph\n     with two lines.\n\n         indented code\n\n     > A block quote.\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_287() {
        let parser = Parser::from_reader("  1.  A paragraph\n      with two lines.\n\n          indented code\n\n      > A block quote.\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_288() {
        let parser = Parser::from_reader("   1.  A paragraph\n       with two lines.\n\n           indented code\n\n       > A block quote.\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_289() {
        let parser = Parser::from_reader("    1.  A paragraph\n        with two lines.\n\n            indented code\n\n        > A block quote.\n".as_bytes());
        assert_eq!("<pre><code>1.  A paragraph\n    with two lines.\n\n        indented code\n\n    &gt; A block quote.\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_290() {
        let parser = Parser::from_reader("  1.  A paragraph\nwith two lines.\n\n          indented code\n\n      > A block quote.\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_291() {
        let parser = Parser::from_reader("  1.  A paragraph\n    with two lines.\n".as_bytes());
        assert_eq!("<ol>\n<li>A paragraph\nwith two lines.</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_292() {
        let parser = Parser::from_reader("> 1. > Blockquote\ncontinued here.\n".as_bytes());
        assert_eq!("<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_293() {
        let parser = Parser::from_reader("> 1. > Blockquote\n> continued here.\n".as_bytes());
        assert_eq!("<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_294() {
        let parser = Parser::from_reader("- foo\n  - bar\n    - baz\n      - boo\n".as_bytes());
        assert_eq!("<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz\n<ul>\n<li>boo</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_295() {
        let parser = Parser::from_reader("- foo\n - bar\n  - baz\n   - boo\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n<li>bar</li>\n<li>baz</li>\n<li>boo</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_296() {
        let parser = Parser::from_reader("10) foo\n    - bar\n".as_bytes());
        assert_eq!("<ol start=\"10\">\n<li>foo\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_297() {
        let parser = Parser::from_reader("10) foo\n   - bar\n".as_bytes());
        assert_eq!("<ol start=\"10\">\n<li>foo</li>\n</ol>\n<ul>\n<li>bar</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_298() {
        let parser = Parser::from_reader("- - foo\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<ul>\n<li>foo</li>\n</ul>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_299() {
        let parser = Parser::from_reader("1. - 2. foo\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<ul>\n<li>\n<ol start=\"2\">\n<li>foo</li>\n</ol>\n</li>\n</ul>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// List items
    fn test_example_300() {
        let parser = Parser::from_reader("- # Foo\n- Bar\n  ---\n  baz\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<h1>Foo</h1>\n</li>\n<li>\n<h2>Bar</h2>\nbaz</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_301() {
        let parser = Parser::from_reader("- foo\n- bar\n+ baz\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>\n<ul>\n<li>baz</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_302() {
        let parser = Parser::from_reader("1. foo\n2. bar\n3) baz\n".as_bytes());
        assert_eq!("<ol>\n<li>foo</li>\n<li>bar</li>\n</ol>\n<ol start=\"3\">\n<li>baz</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_303() {
        let parser = Parser::from_reader("Foo\n- bar\n- baz\n".as_bytes());
        assert_eq!("<p>Foo</p>\n<ul>\n<li>bar</li>\n<li>baz</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_304() {
        let parser = Parser::from_reader("The number of windows in my house is\n14.  The number of doors is 6.\n".as_bytes());
        assert_eq!("<p>The number of windows in my house is\n14.  The number of doors is 6.</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_305() {
        let parser = Parser::from_reader("The number of windows in my house is\n1.  The number of doors is 6.\n".as_bytes());
        assert_eq!("<p>The number of windows in my house is</p>\n<ol>\n<li>The number of doors is 6.</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_306() {
        let parser = Parser::from_reader("- foo\n\n- bar\n\n\n- baz\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>foo</p>\n</li>\n<li>\n<p>bar</p>\n</li>\n<li>\n<p>baz</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_307() {
        let parser = Parser::from_reader("- foo\n  - bar\n    - baz\n\n\n      bim\n".as_bytes());
        assert_eq!("<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>\n<p>baz</p>\n<p>bim</p>\n</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_308() {
        let parser = Parser::from_reader("- foo\n- bar\n\n<!-- -->\n\n- baz\n- bim\n".as_bytes());
        assert_eq!("<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>\n<!-- -->\n<ul>\n<li>baz</li>\n<li>bim</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_309() {
        let parser = Parser::from_reader("-   foo\n\n    notcode\n\n-   foo\n\n<!-- -->\n\n    code\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>foo</p>\n<p>notcode</p>\n</li>\n<li>\n<p>foo</p>\n</li>\n</ul>\n<!-- -->\n<pre><code>code\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_310() {
        let parser = Parser::from_reader("- a\n - b\n  - c\n   - d\n  - e\n - f\n- g\n".as_bytes());
        assert_eq!("<ul>\n<li>a</li>\n<li>b</li>\n<li>c</li>\n<li>d</li>\n<li>e</li>\n<li>f</li>\n<li>g</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_311() {
        let parser = Parser::from_reader("1. a\n\n  2. b\n\n   3. c\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n<li>\n<p>c</p>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_312() {
        let parser = Parser::from_reader("- a\n - b\n  - c\n   - d\n    - e\n".as_bytes());
        assert_eq!("<ul>\n<li>a</li>\n<li>b</li>\n<li>c</li>\n<li>d\n- e</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_313() {
        let parser = Parser::from_reader("1. a\n\n  2. b\n\n    3. c\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n</ol>\n<pre><code>3. c\n</code></pre>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_314() {
        let parser = Parser::from_reader("- a\n- b\n\n- c\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n<li>\n<p>c</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_315() {
        let parser = Parser::from_reader("* a\n*\n\n* c\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>a</p>\n</li>\n<li></li>\n<li>\n<p>c</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_316() {
        let parser = Parser::from_reader("- a\n- b\n\n  c\n- d\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n<p>c</p>\n</li>\n<li>\n<p>d</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_317() {
        let parser = Parser::from_reader("- a\n- b\n\n  [ref]: /url\n- d\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\n<li>\n<p>d</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_318() {
        let parser = Parser::from_reader("- a\n- ```\n  b\n\n\n  ```\n- c\n".as_bytes());
        assert_eq!("<ul>\n<li>a</li>\n<li>\n<pre><code>b\n\n\n</code></pre>\n</li>\n<li>c</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_319() {
        let parser = Parser::from_reader("- a\n  - b\n\n    c\n- d\n".as_bytes());
        assert_eq!("<ul>\n<li>a\n<ul>\n<li>\n<p>b</p>\n<p>c</p>\n</li>\n</ul>\n</li>\n<li>d</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_320() {
        let parser = Parser::from_reader("* a\n  > b\n  >\n* c\n".as_bytes());
        assert_eq!("<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\n</li>\n<li>c</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_321() {
        let parser = Parser::from_reader("- a\n  > b\n  ```\n  c\n  ```\n- d\n".as_bytes());
        assert_eq!("<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\n<pre><code>c\n</code></pre>\n</li>\n<li>d</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_322() {
        let parser = Parser::from_reader("- a\n".as_bytes());
        assert_eq!("<ul>\n<li>a</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_323() {
        let parser = Parser::from_reader("- a\n  - b\n".as_bytes());
        assert_eq!("<ul>\n<li>a\n<ul>\n<li>b</li>\n</ul>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_324() {
        let parser = Parser::from_reader("1. ```\n   foo\n   ```\n\n   bar\n".as_bytes());
        assert_eq!("<ol>\n<li>\n<pre><code>foo\n</code></pre>\n<p>bar</p>\n</li>\n</ol>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_325() {
        let parser = Parser::from_reader("* foo\n  * bar\n\n  baz\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\n<p>baz</p>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Lists
    fn test_example_326() {
        let parser = Parser::from_reader("- a\n  - b\n  - c\n\n- d\n  - e\n  - f\n".as_bytes());
        assert_eq!("<ul>\n<li>\n<p>a</p>\n<ul>\n<li>b</li>\n<li>c</li>\n</ul>\n</li>\n<li>\n<p>d</p>\n<ul>\n<li>e</li>\n<li>f</li>\n</ul>\n</li>\n</ul>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Inlines
    fn test_example_327() {
        let parser = Parser::from_reader("`hi`lo`\n".as_bytes());
        assert_eq!("<p><code>hi</code>lo`</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_328() {
        let parser = Parser::from_reader("`foo`\n".as_bytes());
        assert_eq!("<p><code>foo</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_329() {
        let parser = Parser::from_reader("`` foo ` bar ``\n".as_bytes());
        assert_eq!("<p><code>foo ` bar</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_330() {
        let parser = Parser::from_reader("` `` `\n".as_bytes());
        assert_eq!("<p><code>``</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_331() {
        let parser = Parser::from_reader("`  ``  `\n".as_bytes());
        assert_eq!("<p><code> `` </code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_332() {
        let parser = Parser::from_reader("` a`\n".as_bytes());
        assert_eq!("<p><code> a</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_333() {
        let parser = Parser::from_reader("`\u{a0}b\u{a0}`\n".as_bytes());
        assert_eq!("<p><code>\u{a0}b\u{a0}</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_334() {
        let parser = Parser::from_reader("`\u{a0}`\n`  `\n".as_bytes());
        assert_eq!("<p><code>\u{a0}</code>\n<code>  </code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_335() {
        let parser = Parser::from_reader("``\nfoo\nbar  \nbaz\n``\n".as_bytes());
        assert_eq!("<p><code>foo bar   baz</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_336() {
        let parser = Parser::from_reader("``\nfoo \n``\n".as_bytes());
        assert_eq!("<p><code>foo </code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_337() {
        let parser = Parser::from_reader("`foo   bar \nbaz`\n".as_bytes());
        assert_eq!("<p><code>foo   bar  baz</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_338() {
        let parser = Parser::from_reader("`foo\\`bar`\n".as_bytes());
        assert_eq!("<p><code>foo\\</code>bar`</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_339() {
        let parser = Parser::from_reader("``foo`bar``\n".as_bytes());
        assert_eq!("<p><code>foo`bar</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_340() {
        let parser = Parser::from_reader("` foo `` bar `\n".as_bytes());
        assert_eq!("<p><code>foo `` bar</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_341() {
        let parser = Parser::from_reader("*foo`*`\n".as_bytes());
        assert_eq!("<p>*foo<code>*</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_342() {
        let parser = Parser::from_reader("[not a `link](/foo`)\n".as_bytes());
        assert_eq!("<p>[not a <code>link](/foo</code>)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_343() {
        let parser = Parser::from_reader("`<a href=\"`\">`\n".as_bytes());
        assert_eq!("<p><code>&lt;a href=&quot;</code>&quot;&gt;`</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_344() {
        let parser = Parser::from_reader("<a href=\"`\">`\n".as_bytes());
        assert_eq!("<p><a href=\"`\">`</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_345() {
        let parser = Parser::from_reader("`<https://foo.bar.`baz>`\n".as_bytes());
        assert_eq!("<p><code>&lt;https://foo.bar.</code>baz&gt;`</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_346() {
        let parser = Parser::from_reader("<https://foo.bar.`baz>`\n".as_bytes());
        assert_eq!("<p><a href=\"https://foo.bar.%60baz\">https://foo.bar.`baz</a>`</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_347() {
        let parser = Parser::from_reader("```foo``\n".as_bytes());
        assert_eq!("<p>```foo``</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_348() {
        let parser = Parser::from_reader("`foo\n".as_bytes());
        assert_eq!("<p>`foo</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Code spans
    fn test_example_349() {
        let parser = Parser::from_reader("`foo``bar``\n".as_bytes());
        assert_eq!("<p>`foo<code>bar</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_350() {
        let parser = Parser::from_reader("*foo bar*\n".as_bytes());
        assert_eq!("<p><em>foo bar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_351() {
        let parser = Parser::from_reader("a * foo bar*\n".as_bytes());
        assert_eq!("<p>a * foo bar*</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_352() {
        let parser = Parser::from_reader("a*\"foo\"*\n".as_bytes());
        assert_eq!("<p>a*&quot;foo&quot;*</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_353() {
        let parser = Parser::from_reader("*\u{a0}a\u{a0}*\n".as_bytes());
        assert_eq!("<p>*\u{a0}a\u{a0}*</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_354() {
        let parser = Parser::from_reader("*$*alpha.\n\n*£*bravo.\n\n*€*charlie.\n".as_bytes());
        assert_eq!("<p>*$*alpha.</p>\n<p>*£*bravo.</p>\n<p>*€*charlie.</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_355() {
        let parser = Parser::from_reader("foo*bar*\n".as_bytes());
        assert_eq!("<p>foo<em>bar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_356() {
        let parser = Parser::from_reader("5*6*78\n".as_bytes());
        assert_eq!("<p>5<em>6</em>78</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_357() {
        let parser = Parser::from_reader("_foo bar_\n".as_bytes());
        assert_eq!("<p><em>foo bar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_358() {
        let parser = Parser::from_reader("_ foo bar_\n".as_bytes());
        assert_eq!("<p>_ foo bar_</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_359() {
        let parser = Parser::from_reader("a_\"foo\"_\n".as_bytes());
        assert_eq!("<p>a_&quot;foo&quot;_</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_360() {
        let parser = Parser::from_reader("foo_bar_\n".as_bytes());
        assert_eq!("<p>foo_bar_</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_361() {
        let parser = Parser::from_reader("5_6_78\n".as_bytes());
        assert_eq!("<p>5_6_78</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_362() {
        let parser = Parser::from_reader("пристаням_стремятся_\n".as_bytes());
        assert_eq!("<p>пристаням_стремятся_</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_363() {
        let parser = Parser::from_reader("aa_\"bb\"_cc\n".as_bytes());
        assert_eq!("<p>aa_&quot;bb&quot;_cc</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_364() {
        let parser = Parser::from_reader("foo-_(bar)_\n".as_bytes());
        assert_eq!("<p>foo-<em>(bar)</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_365() {
        let parser = Parser::from_reader("_foo*\n".as_bytes());
        assert_eq!("<p>_foo*</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_366() {
        let parser = Parser::from_reader("*foo bar *\n".as_bytes());
        assert_eq!("<p>*foo bar *</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_367() {
        let parser = Parser::from_reader("*foo bar\n*\n".as_bytes());
        assert_eq!("<p>*foo bar\n*</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_368() {
        let parser = Parser::from_reader("*(*foo)\n".as_bytes());
        assert_eq!("<p>*(*foo)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_369() {
        let parser = Parser::from_reader("*(*foo*)*\n".as_bytes());
        assert_eq!("<p><em>(<em>foo</em>)</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_370() {
        let parser = Parser::from_reader("*foo*bar\n".as_bytes());
        assert_eq!("<p><em>foo</em>bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_371() {
        let parser = Parser::from_reader("_foo bar _\n".as_bytes());
        assert_eq!("<p>_foo bar _</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_372() {
        let parser = Parser::from_reader("_(_foo)\n".as_bytes());
        assert_eq!("<p>_(_foo)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_373() {
        let parser = Parser::from_reader("_(_foo_)_\n".as_bytes());
        assert_eq!("<p><em>(<em>foo</em>)</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_374() {
        let parser = Parser::from_reader("_foo_bar\n".as_bytes());
        assert_eq!("<p>_foo_bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_375() {
        let parser = Parser::from_reader("_пристаням_стремятся\n".as_bytes());
        assert_eq!("<p>_пристаням_стремятся</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_376() {
        let parser = Parser::from_reader("_foo_bar_baz_\n".as_bytes());
        assert_eq!("<p><em>foo_bar_baz</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_377() {
        let parser = Parser::from_reader("_(bar)_.\n".as_bytes());
        assert_eq!("<p><em>(bar)</em>.</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_378() {
        let parser = Parser::from_reader("**foo bar**\n".as_bytes());
        assert_eq!("<p><strong>foo bar</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_379() {
        let parser = Parser::from_reader("** foo bar**\n".as_bytes());
        assert_eq!("<p>** foo bar**</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_380() {
        let parser = Parser::from_reader("a**\"foo\"**\n".as_bytes());
        assert_eq!("<p>a**&quot;foo&quot;**</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_381() {
        let parser = Parser::from_reader("foo**bar**\n".as_bytes());
        assert_eq!("<p>foo<strong>bar</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_382() {
        let parser = Parser::from_reader("__foo bar__\n".as_bytes());
        assert_eq!("<p><strong>foo bar</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_383() {
        let parser = Parser::from_reader("__ foo bar__\n".as_bytes());
        assert_eq!("<p>__ foo bar__</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_384() {
        let parser = Parser::from_reader("__\nfoo bar__\n".as_bytes());
        assert_eq!("<p>__\nfoo bar__</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_385() {
        let parser = Parser::from_reader("a__\"foo\"__\n".as_bytes());
        assert_eq!("<p>a__&quot;foo&quot;__</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_386() {
        let parser = Parser::from_reader("foo__bar__\n".as_bytes());
        assert_eq!("<p>foo__bar__</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_387() {
        let parser = Parser::from_reader("5__6__78\n".as_bytes());
        assert_eq!("<p>5__6__78</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_388() {
        let parser = Parser::from_reader("пристаням__стремятся__\n".as_bytes());
        assert_eq!("<p>пристаням__стремятся__</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_389() {
        let parser = Parser::from_reader("__foo, __bar__, baz__\n".as_bytes());
        assert_eq!("<p><strong>foo, <strong>bar</strong>, baz</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_390() {
        let parser = Parser::from_reader("foo-__(bar)__\n".as_bytes());
        assert_eq!("<p>foo-<strong>(bar)</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_391() {
        let parser = Parser::from_reader("**foo bar **\n".as_bytes());
        assert_eq!("<p>**foo bar **</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_392() {
        let parser = Parser::from_reader("**(**foo)\n".as_bytes());
        assert_eq!("<p>**(**foo)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_393() {
        let parser = Parser::from_reader("*(**foo**)*\n".as_bytes());
        assert_eq!("<p><em>(<strong>foo</strong>)</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_394() {
        let parser = Parser::from_reader("**Gomphocarpus (*Gomphocarpus physocarpus*, syn.\n*Asclepias physocarpa*)**\n".as_bytes());
        assert_eq!("<p><strong>Gomphocarpus (<em>Gomphocarpus physocarpus</em>, syn.\n<em>Asclepias physocarpa</em>)</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_395() {
        let parser = Parser::from_reader("**foo \"*bar*\" foo**\n".as_bytes());
        assert_eq!("<p><strong>foo &quot;<em>bar</em>&quot; foo</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_396() {
        let parser = Parser::from_reader("**foo**bar\n".as_bytes());
        assert_eq!("<p><strong>foo</strong>bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_397() {
        let parser = Parser::from_reader("__foo bar __\n".as_bytes());
        assert_eq!("<p>__foo bar __</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_398() {
        let parser = Parser::from_reader("__(__foo)\n".as_bytes());
        assert_eq!("<p>__(__foo)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_399() {
        let parser = Parser::from_reader("_(__foo__)_\n".as_bytes());
        assert_eq!("<p><em>(<strong>foo</strong>)</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_400() {
        let parser = Parser::from_reader("__foo__bar\n".as_bytes());
        assert_eq!("<p>__foo__bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_401() {
        let parser = Parser::from_reader("__пристаням__стремятся\n".as_bytes());
        assert_eq!("<p>__пристаням__стремятся</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_402() {
        let parser = Parser::from_reader("__foo__bar__baz__\n".as_bytes());
        assert_eq!("<p><strong>foo__bar__baz</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_403() {
        let parser = Parser::from_reader("__(bar)__.\n".as_bytes());
        assert_eq!("<p><strong>(bar)</strong>.</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_404() {
        let parser = Parser::from_reader("*foo [bar](/url)*\n".as_bytes());
        assert_eq!("<p><em>foo <a href=\"/url\">bar</a></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_405() {
        let parser = Parser::from_reader("*foo\nbar*\n".as_bytes());
        assert_eq!("<p><em>foo\nbar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_406() {
        let parser = Parser::from_reader("_foo __bar__ baz_\n".as_bytes());
        assert_eq!("<p><em>foo <strong>bar</strong> baz</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_407() {
        let parser = Parser::from_reader("_foo _bar_ baz_\n".as_bytes());
        assert_eq!("<p><em>foo <em>bar</em> baz</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_408() {
        let parser = Parser::from_reader("__foo_ bar_\n".as_bytes());
        assert_eq!("<p><em><em>foo</em> bar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_409() {
        let parser = Parser::from_reader("*foo *bar**\n".as_bytes());
        assert_eq!("<p><em>foo <em>bar</em></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_410() {
        let parser = Parser::from_reader("*foo **bar** baz*\n".as_bytes());
        assert_eq!("<p><em>foo <strong>bar</strong> baz</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_411() {
        let parser = Parser::from_reader("*foo**bar**baz*\n".as_bytes());
        assert_eq!("<p><em>foo<strong>bar</strong>baz</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_412() {
        let parser = Parser::from_reader("*foo**bar*\n".as_bytes());
        assert_eq!("<p><em>foo**bar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_413() {
        let parser = Parser::from_reader("***foo** bar*\n".as_bytes());
        assert_eq!("<p><em><strong>foo</strong> bar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_414() {
        let parser = Parser::from_reader("*foo **bar***\n".as_bytes());
        assert_eq!("<p><em>foo <strong>bar</strong></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_415() {
        let parser = Parser::from_reader("*foo**bar***\n".as_bytes());
        assert_eq!("<p><em>foo<strong>bar</strong></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_416() {
        let parser = Parser::from_reader("foo***bar***baz\n".as_bytes());
        assert_eq!("<p>foo<em><strong>bar</strong></em>baz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_417() {
        let parser = Parser::from_reader("foo******bar*********baz\n".as_bytes());
        assert_eq!("<p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_418() {
        let parser = Parser::from_reader("*foo **bar *baz* bim** bop*\n".as_bytes());
        assert_eq!("<p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_419() {
        let parser = Parser::from_reader("*foo [*bar*](/url)*\n".as_bytes());
        assert_eq!("<p><em>foo <a href=\"/url\"><em>bar</em></a></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_420() {
        let parser = Parser::from_reader("** is not an empty emphasis\n".as_bytes());
        assert_eq!("<p>** is not an empty emphasis</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_421() {
        let parser = Parser::from_reader("**** is not an empty strong emphasis\n".as_bytes());
        assert_eq!("<p>**** is not an empty strong emphasis</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_422() {
        let parser = Parser::from_reader("**foo [bar](/url)**\n".as_bytes());
        assert_eq!("<p><strong>foo <a href=\"/url\">bar</a></strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_423() {
        let parser = Parser::from_reader("**foo\nbar**\n".as_bytes());
        assert_eq!("<p><strong>foo\nbar</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_424() {
        let parser = Parser::from_reader("__foo _bar_ baz__\n".as_bytes());
        assert_eq!("<p><strong>foo <em>bar</em> baz</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_425() {
        let parser = Parser::from_reader("__foo __bar__ baz__\n".as_bytes());
        assert_eq!("<p><strong>foo <strong>bar</strong> baz</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_426() {
        let parser = Parser::from_reader("____foo__ bar__\n".as_bytes());
        assert_eq!("<p><strong><strong>foo</strong> bar</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_427() {
        let parser = Parser::from_reader("**foo **bar****\n".as_bytes());
        assert_eq!("<p><strong>foo <strong>bar</strong></strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_428() {
        let parser = Parser::from_reader("**foo *bar* baz**\n".as_bytes());
        assert_eq!("<p><strong>foo <em>bar</em> baz</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_429() {
        let parser = Parser::from_reader("**foo*bar*baz**\n".as_bytes());
        assert_eq!("<p><strong>foo<em>bar</em>baz</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_430() {
        let parser = Parser::from_reader("***foo* bar**\n".as_bytes());
        assert_eq!("<p><strong><em>foo</em> bar</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_431() {
        let parser = Parser::from_reader("**foo *bar***\n".as_bytes());
        assert_eq!("<p><strong>foo <em>bar</em></strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_432() {
        let parser = Parser::from_reader("**foo *bar **baz**\nbim* bop**\n".as_bytes());
        assert_eq!("<p><strong>foo <em>bar <strong>baz</strong>\nbim</em> bop</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_433() {
        let parser = Parser::from_reader("**foo [*bar*](/url)**\n".as_bytes());
        assert_eq!("<p><strong>foo <a href=\"/url\"><em>bar</em></a></strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_434() {
        let parser = Parser::from_reader("__ is not an empty emphasis\n".as_bytes());
        assert_eq!("<p>__ is not an empty emphasis</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_435() {
        let parser = Parser::from_reader("____ is not an empty strong emphasis\n".as_bytes());
        assert_eq!("<p>____ is not an empty strong emphasis</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_436() {
        let parser = Parser::from_reader("foo ***\n".as_bytes());
        assert_eq!("<p>foo ***</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_437() {
        let parser = Parser::from_reader("foo *\\**\n".as_bytes());
        assert_eq!("<p>foo <em>*</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_438() {
        let parser = Parser::from_reader("foo *_*\n".as_bytes());
        assert_eq!("<p>foo <em>_</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_439() {
        let parser = Parser::from_reader("foo *****\n".as_bytes());
        assert_eq!("<p>foo *****</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_440() {
        let parser = Parser::from_reader("foo **\\***\n".as_bytes());
        assert_eq!("<p>foo <strong>*</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_441() {
        let parser = Parser::from_reader("foo **_**\n".as_bytes());
        assert_eq!("<p>foo <strong>_</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_442() {
        let parser = Parser::from_reader("**foo*\n".as_bytes());
        assert_eq!("<p>*<em>foo</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_443() {
        let parser = Parser::from_reader("*foo**\n".as_bytes());
        assert_eq!("<p><em>foo</em>*</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_444() {
        let parser = Parser::from_reader("***foo**\n".as_bytes());
        assert_eq!("<p>*<strong>foo</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_445() {
        let parser = Parser::from_reader("****foo*\n".as_bytes());
        assert_eq!("<p>***<em>foo</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_446() {
        let parser = Parser::from_reader("**foo***\n".as_bytes());
        assert_eq!("<p><strong>foo</strong>*</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_447() {
        let parser = Parser::from_reader("*foo****\n".as_bytes());
        assert_eq!("<p><em>foo</em>***</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_448() {
        let parser = Parser::from_reader("foo ___\n".as_bytes());
        assert_eq!("<p>foo ___</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_449() {
        let parser = Parser::from_reader("foo _\\__\n".as_bytes());
        assert_eq!("<p>foo <em>_</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_450() {
        let parser = Parser::from_reader("foo _*_\n".as_bytes());
        assert_eq!("<p>foo <em>*</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_451() {
        let parser = Parser::from_reader("foo _____\n".as_bytes());
        assert_eq!("<p>foo _____</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_452() {
        let parser = Parser::from_reader("foo __\\___\n".as_bytes());
        assert_eq!("<p>foo <strong>_</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_453() {
        let parser = Parser::from_reader("foo __*__\n".as_bytes());
        assert_eq!("<p>foo <strong>*</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_454() {
        let parser = Parser::from_reader("__foo_\n".as_bytes());
        assert_eq!("<p>_<em>foo</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_455() {
        let parser = Parser::from_reader("_foo__\n".as_bytes());
        assert_eq!("<p><em>foo</em>_</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_456() {
        let parser = Parser::from_reader("___foo__\n".as_bytes());
        assert_eq!("<p>_<strong>foo</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_457() {
        let parser = Parser::from_reader("____foo_\n".as_bytes());
        assert_eq!("<p>___<em>foo</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_458() {
        let parser = Parser::from_reader("__foo___\n".as_bytes());
        assert_eq!("<p><strong>foo</strong>_</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_459() {
        let parser = Parser::from_reader("_foo____\n".as_bytes());
        assert_eq!("<p><em>foo</em>___</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_460() {
        let parser = Parser::from_reader("**foo**\n".as_bytes());
        assert_eq!("<p><strong>foo</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_461() {
        let parser = Parser::from_reader("*_foo_*\n".as_bytes());
        assert_eq!("<p><em><em>foo</em></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_462() {
        let parser = Parser::from_reader("__foo__\n".as_bytes());
        assert_eq!("<p><strong>foo</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_463() {
        let parser = Parser::from_reader("_*foo*_\n".as_bytes());
        assert_eq!("<p><em><em>foo</em></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_464() {
        let parser = Parser::from_reader("****foo****\n".as_bytes());
        assert_eq!("<p><strong><strong>foo</strong></strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_465() {
        let parser = Parser::from_reader("____foo____\n".as_bytes());
        assert_eq!("<p><strong><strong>foo</strong></strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_466() {
        let parser = Parser::from_reader("******foo******\n".as_bytes());
        assert_eq!("<p><strong><strong><strong>foo</strong></strong></strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_467() {
        let parser = Parser::from_reader("***foo***\n".as_bytes());
        assert_eq!("<p><em><strong>foo</strong></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_468() {
        let parser = Parser::from_reader("_____foo_____\n".as_bytes());
        assert_eq!("<p><em><strong><strong>foo</strong></strong></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_469() {
        let parser = Parser::from_reader("*foo _bar* baz_\n".as_bytes());
        assert_eq!("<p><em>foo _bar</em> baz_</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_470() {
        let parser = Parser::from_reader("*foo __bar *baz bim__ bam*\n".as_bytes());
        assert_eq!("<p><em>foo <strong>bar *baz bim</strong> bam</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_471() {
        let parser = Parser::from_reader("**foo **bar baz**\n".as_bytes());
        assert_eq!("<p>**foo <strong>bar baz</strong></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_472() {
        let parser = Parser::from_reader("*foo *bar baz*\n".as_bytes());
        assert_eq!("<p>*foo <em>bar baz</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_473() {
        let parser = Parser::from_reader("*[bar*](/url)\n".as_bytes());
        assert_eq!("<p>*<a href=\"/url\">bar*</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_474() {
        let parser = Parser::from_reader("_foo [bar_](/url)\n".as_bytes());
        assert_eq!("<p>_foo <a href=\"/url\">bar_</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_475() {
        let parser = Parser::from_reader("*<img src=\"foo\" title=\"*\"/>\n".as_bytes());
        assert_eq!("<p>*<img src=\"foo\" title=\"*\"/></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_476() {
        let parser = Parser::from_reader("**<a href=\"**\">\n".as_bytes());
        assert_eq!("<p>**<a href=\"**\"></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_477() {
        let parser = Parser::from_reader("__<a href=\"__\">\n".as_bytes());
        assert_eq!("<p>__<a href=\"__\"></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_478() {
        let parser = Parser::from_reader("*a `*`*\n".as_bytes());
        assert_eq!("<p><em>a <code>*</code></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_479() {
        let parser = Parser::from_reader("_a `_`_\n".as_bytes());
        assert_eq!("<p><em>a <code>_</code></em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_480() {
        let parser = Parser::from_reader("**a<https://foo.bar/?q=**>\n".as_bytes());
        assert_eq!("<p>**a<a href=\"https://foo.bar/?q=**\">https://foo.bar/?q=**</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Emphasis and strong emphasis
    fn test_example_481() {
        let parser = Parser::from_reader("__a<https://foo.bar/?q=__>\n".as_bytes());
        assert_eq!("<p>__a<a href=\"https://foo.bar/?q=__\">https://foo.bar/?q=__</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_482() {
        let parser = Parser::from_reader("[link](/uri \"title\")\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\" title=\"title\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_483() {
        let parser = Parser::from_reader("[link](/uri)\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_484() {
        let parser = Parser::from_reader("[](./target.md)\n".as_bytes());
        assert_eq!("<p><a href=\"./target.md\"></a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_485() {
        let parser = Parser::from_reader("[link]()\n".as_bytes());
        assert_eq!("<p><a href=\"\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_486() {
        let parser = Parser::from_reader("[link](<>)\n".as_bytes());
        assert_eq!("<p><a href=\"\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_487() {
        let parser = Parser::from_reader("[]()\n".as_bytes());
        assert_eq!("<p><a href=\"\"></a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_488() {
        let parser = Parser::from_reader("[link](/my uri)\n".as_bytes());
        assert_eq!("<p>[link](/my uri)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_489() {
        let parser = Parser::from_reader("[link](</my uri>)\n".as_bytes());
        assert_eq!("<p><a href=\"/my%20uri\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_490() {
        let parser = Parser::from_reader("[link](foo\nbar)\n".as_bytes());
        assert_eq!("<p>[link](foo\nbar)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_491() {
        let parser = Parser::from_reader("[link](<foo\nbar>)\n".as_bytes());
        assert_eq!("<p>[link](<foo\nbar>)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_492() {
        let parser = Parser::from_reader("[a](<b)c>)\n".as_bytes());
        assert_eq!("<p><a href=\"b)c\">a</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_493() {
        let parser = Parser::from_reader("[link](<foo\\>)\n".as_bytes());
        assert_eq!("<p>[link](&lt;foo&gt;)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_494() {
        let parser = Parser::from_reader("[a](<b)c\n[a](<b)c>\n[a](<b>c)\n".as_bytes());
        assert_eq!("<p>[a](&lt;b)c\n[a](&lt;b)c&gt;\n[a](<b>c)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_495() {
        let parser = Parser::from_reader("[link](\\(foo\\))\n".as_bytes());
        assert_eq!("<p><a href=\"(foo)\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_496() {
        let parser = Parser::from_reader("[link](foo(and(bar)))\n".as_bytes());
        assert_eq!("<p><a href=\"foo(and(bar))\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_497() {
        let parser = Parser::from_reader("[link](foo(and(bar))\n".as_bytes());
        assert_eq!("<p>[link](foo(and(bar))</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_498() {
        let parser = Parser::from_reader("[link](foo\\(and\\(bar\\))\n".as_bytes());
        assert_eq!("<p><a href=\"foo(and(bar)\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_499() {
        let parser = Parser::from_reader("[link](<foo(and(bar)>)\n".as_bytes());
        assert_eq!("<p><a href=\"foo(and(bar)\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_500() {
        let parser = Parser::from_reader("[link](foo\\)\\:)\n".as_bytes());
        assert_eq!("<p><a href=\"foo):\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_501() {
        let parser = Parser::from_reader("[link](#fragment)\n\n[link](https://example.com#fragment)\n\n[link](https://example.com?foo=3#frag)\n".as_bytes());
        assert_eq!("<p><a href=\"#fragment\">link</a></p>\n<p><a href=\"https://example.com#fragment\">link</a></p>\n<p><a href=\"https://example.com?foo=3#frag\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_502() {
        let parser = Parser::from_reader("[link](foo\\bar)\n".as_bytes());
        assert_eq!("<p><a href=\"foo%5Cbar\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_503() {
        let parser = Parser::from_reader("[link](foo%20b&auml;)\n".as_bytes());
        assert_eq!("<p><a href=\"foo%20b%C3%A4\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_504() {
        let parser = Parser::from_reader("[link](\"title\")\n".as_bytes());
        assert_eq!("<p><a href=\"%22title%22\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_505() {
        let parser = Parser::from_reader("[link](/url \"title\")\n[link](/url 'title')\n[link](/url (title))\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\">link</a>\n<a href=\"/url\" title=\"title\">link</a>\n<a href=\"/url\" title=\"title\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_506() {
        let parser = Parser::from_reader("[link](/url \"title \\\"&quot;\")\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title &quot;&quot;\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_507() {
        let parser = Parser::from_reader("[link](/url\u{a0}\"title\")\n".as_bytes());
        assert_eq!("<p><a href=\"/url%C2%A0%22title%22\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_508() {
        let parser = Parser::from_reader("[link](/url \"title \"and\" title\")\n".as_bytes());
        assert_eq!("<p>[link](/url &quot;title &quot;and&quot; title&quot;)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_509() {
        let parser = Parser::from_reader("[link](/url 'title \"and\" title')\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title &quot;and&quot; title\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_510() {
        let parser = Parser::from_reader("[link](   /uri\n  \"title\"  )\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\" title=\"title\">link</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_511() {
        let parser = Parser::from_reader("[link] (/uri)\n".as_bytes());
        assert_eq!("<p>[link] (/uri)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_512() {
        let parser = Parser::from_reader("[link [foo [bar]]](/uri)\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">link [foo [bar]]</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_513() {
        let parser = Parser::from_reader("[link] bar](/uri)\n".as_bytes());
        assert_eq!("<p>[link] bar](/uri)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_514() {
        let parser = Parser::from_reader("[link [bar](/uri)\n".as_bytes());
        assert_eq!("<p>[link <a href=\"/uri\">bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_515() {
        let parser = Parser::from_reader("[link \\[bar](/uri)\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">link [bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_516() {
        let parser = Parser::from_reader("[link *foo **bar** `#`*](/uri)\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_517() {
        let parser = Parser::from_reader("[![moon](moon.jpg)](/uri)\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\"><img src=\"moon.jpg\" alt=\"moon\" /></a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_518() {
        let parser = Parser::from_reader("[foo [bar](/uri)](/uri)\n".as_bytes());
        assert_eq!("<p>[foo <a href=\"/uri\">bar</a>](/uri)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_519() {
        let parser = Parser::from_reader("[foo *[bar [baz](/uri)](/uri)*](/uri)\n".as_bytes());
        assert_eq!("<p>[foo <em>[bar <a href=\"/uri\">baz</a>](/uri)</em>](/uri)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_520() {
        let parser = Parser::from_reader("![[[foo](uri1)](uri2)](uri3)\n".as_bytes());
        assert_eq!("<p><img src=\"uri3\" alt=\"[foo](uri2)\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_521() {
        let parser = Parser::from_reader("*[foo*](/uri)\n".as_bytes());
        assert_eq!("<p>*<a href=\"/uri\">foo*</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_522() {
        let parser = Parser::from_reader("[foo *bar](baz*)\n".as_bytes());
        assert_eq!("<p><a href=\"baz*\">foo *bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_523() {
        let parser = Parser::from_reader("*foo [bar* baz]\n".as_bytes());
        assert_eq!("<p><em>foo [bar</em> baz]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_524() {
        let parser = Parser::from_reader("[foo <bar attr=\"](baz)\">\n".as_bytes());
        assert_eq!("<p>[foo <bar attr=\"](baz)\"></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_525() {
        let parser = Parser::from_reader("[foo`](/uri)`\n".as_bytes());
        assert_eq!("<p>[foo<code>](/uri)</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_526() {
        let parser = Parser::from_reader("[foo<https://example.com/?search=](uri)>\n".as_bytes());
        assert_eq!("<p>[foo<a href=\"https://example.com/?search=%5D(uri)\">https://example.com/?search=](uri)</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_527() {
        let parser = Parser::from_reader("[foo][bar]\n\n[bar]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_528() {
        let parser = Parser::from_reader("[link [foo [bar]]][ref]\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">link [foo [bar]]</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_529() {
        let parser = Parser::from_reader("[link \\[bar][ref]\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">link [bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_530() {
        let parser = Parser::from_reader("[link *foo **bar** `#`*][ref]\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_531() {
        let parser = Parser::from_reader("[![moon](moon.jpg)][ref]\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\"><img src=\"moon.jpg\" alt=\"moon\" /></a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_532() {
        let parser = Parser::from_reader("[foo [bar](/uri)][ref]\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p>[foo <a href=\"/uri\">bar</a>]<a href=\"/uri\">ref</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_533() {
        let parser = Parser::from_reader("[foo *bar [baz][ref]*][ref]\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p>[foo <em>bar <a href=\"/uri\">baz</a></em>]<a href=\"/uri\">ref</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_534() {
        let parser = Parser::from_reader("*[foo*][ref]\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p>*<a href=\"/uri\">foo*</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_535() {
        let parser = Parser::from_reader("[foo *bar][ref]*\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">foo *bar</a>*</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_536() {
        let parser = Parser::from_reader("[foo <bar attr=\"][ref]\">\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p>[foo <bar attr=\"][ref]\"></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_537() {
        let parser = Parser::from_reader("[foo`][ref]`\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p>[foo<code>][ref]</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_538() {
        let parser = Parser::from_reader("[foo<https://example.com/?search=][ref]>\n\n[ref]: /uri\n".as_bytes());
        assert_eq!("<p>[foo<a href=\"https://example.com/?search=%5D%5Bref%5D\">https://example.com/?search=][ref]</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_539() {
        let parser = Parser::from_reader("[foo][BaR]\n\n[bar]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_540() {
        let parser = Parser::from_reader("[ẞ]\n\n[SS]: /url\n".as_bytes());
        assert_eq!("<p><a href=\"/url\">ẞ</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_541() {
        let parser = Parser::from_reader("[Foo\n  bar]: /url\n\n[Baz][Foo bar]\n".as_bytes());
        assert_eq!("<p><a href=\"/url\">Baz</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_542() {
        let parser = Parser::from_reader("[foo] [bar]\n\n[bar]: /url \"title\"\n".as_bytes());
        assert_eq!("<p>[foo] <a href=\"/url\" title=\"title\">bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_543() {
        let parser = Parser::from_reader("[foo]\n[bar]\n\n[bar]: /url \"title\"\n".as_bytes());
        assert_eq!("<p>[foo]\n<a href=\"/url\" title=\"title\">bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_544() {
        let parser = Parser::from_reader("[foo]: /url1\n\n[foo]: /url2\n\n[bar][foo]\n".as_bytes());
        assert_eq!("<p><a href=\"/url1\">bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_545() {
        let parser = Parser::from_reader("[bar][foo\\!]\n\n[foo!]: /url\n".as_bytes());
        assert_eq!("<p>[bar][foo!]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_546() {
        let parser = Parser::from_reader("[foo][ref[]\n\n[ref[]: /uri\n".as_bytes());
        assert_eq!("<p>[foo][ref[]</p>\n<p>[ref[]: /uri</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_547() {
        let parser = Parser::from_reader("[foo][ref[bar]]\n\n[ref[bar]]: /uri\n".as_bytes());
        assert_eq!("<p>[foo][ref[bar]]</p>\n<p>[ref[bar]]: /uri</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_548() {
        let parser = Parser::from_reader("[[[foo]]]\n\n[[[foo]]]: /url\n".as_bytes());
        assert_eq!("<p>[[[foo]]]</p>\n<p>[[[foo]]]: /url</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_549() {
        let parser = Parser::from_reader("[foo][ref\\[]\n\n[ref\\[]: /uri\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_550() {
        let parser = Parser::from_reader("[bar\\\\]: /uri\n\n[bar\\\\]\n".as_bytes());
        assert_eq!("<p><a href=\"/uri\">bar\\</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_551() {
        let parser = Parser::from_reader("[]\n\n[]: /uri\n".as_bytes());
        assert_eq!("<p>[]</p>\n<p>[]: /uri</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_552() {
        let parser = Parser::from_reader("[\n ]\n\n[\n ]: /uri\n".as_bytes());
        assert_eq!("<p>[\n]</p>\n<p>[\n]: /uri</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_553() {
        let parser = Parser::from_reader("[foo][]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_554() {
        let parser = Parser::from_reader("[*foo* bar][]\n\n[*foo* bar]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\"><em>foo</em> bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_555() {
        let parser = Parser::from_reader("[Foo][]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\">Foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_556() {
        let parser = Parser::from_reader("[foo] \n[]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\">foo</a>\n[]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_557() {
        let parser = Parser::from_reader("[foo]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_558() {
        let parser = Parser::from_reader("[*foo* bar]\n\n[*foo* bar]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\"><em>foo</em> bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_559() {
        let parser = Parser::from_reader("[[*foo* bar]]\n\n[*foo* bar]: /url \"title\"\n".as_bytes());
        assert_eq!("<p>[<a href=\"/url\" title=\"title\"><em>foo</em> bar</a>]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_560() {
        let parser = Parser::from_reader("[[bar [foo]\n\n[foo]: /url\n".as_bytes());
        assert_eq!("<p>[[bar <a href=\"/url\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_561() {
        let parser = Parser::from_reader("[Foo]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><a href=\"/url\" title=\"title\">Foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_562() {
        let parser = Parser::from_reader("[foo] bar\n\n[foo]: /url\n".as_bytes());
        assert_eq!("<p><a href=\"/url\">foo</a> bar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_563() {
        let parser = Parser::from_reader("\\[foo]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p>[foo]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_564() {
        let parser = Parser::from_reader("[foo*]: /url\n\n*[foo*]\n".as_bytes());
        assert_eq!("<p>*<a href=\"/url\">foo*</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_565() {
        let parser = Parser::from_reader("[foo][bar]\n\n[foo]: /url1\n[bar]: /url2\n".as_bytes());
        assert_eq!("<p><a href=\"/url2\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_566() {
        let parser = Parser::from_reader("[foo][]\n\n[foo]: /url1\n".as_bytes());
        assert_eq!("<p><a href=\"/url1\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_567() {
        let parser = Parser::from_reader("[foo]()\n\n[foo]: /url1\n".as_bytes());
        assert_eq!("<p><a href=\"\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_568() {
        let parser = Parser::from_reader("[foo](not a link)\n\n[foo]: /url1\n".as_bytes());
        assert_eq!("<p><a href=\"/url1\">foo</a>(not a link)</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_569() {
        let parser = Parser::from_reader("[foo][bar][baz]\n\n[baz]: /url\n".as_bytes());
        assert_eq!("<p>[foo]<a href=\"/url\">bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_570() {
        let parser = Parser::from_reader("[foo][bar][baz]\n\n[baz]: /url1\n[bar]: /url2\n".as_bytes());
        assert_eq!("<p><a href=\"/url2\">foo</a><a href=\"/url1\">baz</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Links
    fn test_example_571() {
        let parser = Parser::from_reader("[foo][bar][baz]\n\n[baz]: /url1\n[foo]: /url2\n".as_bytes());
        assert_eq!("<p>[foo]<a href=\"/url1\">bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_572() {
        let parser = Parser::from_reader("![foo](/url \"title\")\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_573() {
        let parser = Parser::from_reader("![foo *bar*]\n\n[foo *bar*]: train.jpg \"train & tracks\"\n".as_bytes());
        assert_eq!("<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_574() {
        let parser = Parser::from_reader("![foo ![bar](/url)](/url2)\n".as_bytes());
        assert_eq!("<p><img src=\"/url2\" alt=\"foo bar\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_575() {
        let parser = Parser::from_reader("![foo [bar](/url)](/url2)\n".as_bytes());
        assert_eq!("<p><img src=\"/url2\" alt=\"foo bar\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_576() {
        let parser = Parser::from_reader("![foo *bar*][]\n\n[foo *bar*]: train.jpg \"train & tracks\"\n".as_bytes());
        assert_eq!("<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_577() {
        let parser = Parser::from_reader("![foo *bar*][foobar]\n\n[FOOBAR]: train.jpg \"train & tracks\"\n".as_bytes());
        assert_eq!("<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_578() {
        let parser = Parser::from_reader("![foo](train.jpg)\n".as_bytes());
        assert_eq!("<p><img src=\"train.jpg\" alt=\"foo\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_579() {
        let parser = Parser::from_reader("My ![foo bar](/path/to/train.jpg  \"title\"   )\n".as_bytes());
        assert_eq!("<p>My <img src=\"/path/to/train.jpg\" alt=\"foo bar\" title=\"title\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_580() {
        let parser = Parser::from_reader("![foo](<url>)\n".as_bytes());
        assert_eq!("<p><img src=\"url\" alt=\"foo\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_581() {
        let parser = Parser::from_reader("![](/url)\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_582() {
        let parser = Parser::from_reader("![foo][bar]\n\n[bar]: /url\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"foo\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_583() {
        let parser = Parser::from_reader("![foo][bar]\n\n[BAR]: /url\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"foo\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_584() {
        let parser = Parser::from_reader("![foo][]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_585() {
        let parser = Parser::from_reader("![*foo* bar][]\n\n[*foo* bar]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"foo bar\" title=\"title\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_586() {
        let parser = Parser::from_reader("![Foo][]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"Foo\" title=\"title\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_587() {
        let parser = Parser::from_reader("![foo] \n[]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"foo\" title=\"title\" />\n[]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_588() {
        let parser = Parser::from_reader("![foo]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_589() {
        let parser = Parser::from_reader("![*foo* bar]\n\n[*foo* bar]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"foo bar\" title=\"title\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_590() {
        let parser = Parser::from_reader("![[foo]]\n\n[[foo]]: /url \"title\"\n".as_bytes());
        assert_eq!("<p>![[foo]]</p>\n<p>[[foo]]: /url &quot;title&quot;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_591() {
        let parser = Parser::from_reader("![Foo]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p><img src=\"/url\" alt=\"Foo\" title=\"title\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_592() {
        let parser = Parser::from_reader("!\\[foo]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p>![foo]</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Images
    fn test_example_593() {
        let parser = Parser::from_reader("\\![foo]\n\n[foo]: /url \"title\"\n".as_bytes());
        assert_eq!("<p>!<a href=\"/url\" title=\"title\">foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_594() {
        let parser = Parser::from_reader("<http://foo.bar.baz>\n".as_bytes());
        assert_eq!("<p><a href=\"http://foo.bar.baz\">http://foo.bar.baz</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_595() {
        let parser = Parser::from_reader("<https://foo.bar.baz/test?q=hello&id=22&boolean>\n".as_bytes());
        assert_eq!("<p><a href=\"https://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean\">https://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_596() {
        let parser = Parser::from_reader("<irc://foo.bar:2233/baz>\n".as_bytes());
        assert_eq!("<p><a href=\"irc://foo.bar:2233/baz\">irc://foo.bar:2233/baz</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_597() {
        let parser = Parser::from_reader("<MAILTO:FOO@BAR.BAZ>\n".as_bytes());
        assert_eq!("<p><a href=\"MAILTO:FOO@BAR.BAZ\">MAILTO:FOO@BAR.BAZ</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_598() {
        let parser = Parser::from_reader("<a+b+c:d>\n".as_bytes());
        assert_eq!("<p><a href=\"a+b+c:d\">a+b+c:d</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_599() {
        let parser = Parser::from_reader("<made-up-scheme://foo,bar>\n".as_bytes());
        assert_eq!("<p><a href=\"made-up-scheme://foo,bar\">made-up-scheme://foo,bar</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_600() {
        let parser = Parser::from_reader("<https://../>\n".as_bytes());
        assert_eq!("<p><a href=\"https://../\">https://../</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_601() {
        let parser = Parser::from_reader("<localhost:5001/foo>\n".as_bytes());
        assert_eq!("<p><a href=\"localhost:5001/foo\">localhost:5001/foo</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_602() {
        let parser = Parser::from_reader("<https://foo.bar/baz bim>\n".as_bytes());
        assert_eq!("<p>&lt;https://foo.bar/baz bim&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_603() {
        let parser = Parser::from_reader("<https://example.com/\\[\\>\n".as_bytes());
        assert_eq!("<p><a href=\"https://example.com/%5C%5B%5C\">https://example.com/\\[\\</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_604() {
        let parser = Parser::from_reader("<foo@bar.example.com>\n".as_bytes());
        assert_eq!("<p><a href=\"mailto:foo@bar.example.com\">foo@bar.example.com</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_605() {
        let parser = Parser::from_reader("<foo+special@Bar.baz-bar0.com>\n".as_bytes());
        assert_eq!("<p><a href=\"mailto:foo+special@Bar.baz-bar0.com\">foo+special@Bar.baz-bar0.com</a></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_606() {
        let parser = Parser::from_reader("<foo\\+@bar.example.com>\n".as_bytes());
        assert_eq!("<p>&lt;foo+@bar.example.com&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_607() {
        let parser = Parser::from_reader("<>\n".as_bytes());
        assert_eq!("<p>&lt;&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_608() {
        let parser = Parser::from_reader("< https://foo.bar >\n".as_bytes());
        assert_eq!("<p>&lt; https://foo.bar &gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_609() {
        let parser = Parser::from_reader("<m:abc>\n".as_bytes());
        assert_eq!("<p>&lt;m:abc&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_610() {
        let parser = Parser::from_reader("<foo.bar.baz>\n".as_bytes());
        assert_eq!("<p>&lt;foo.bar.baz&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_611() {
        let parser = Parser::from_reader("https://example.com\n".as_bytes());
        assert_eq!("<p>https://example.com</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Autolinks
    fn test_example_612() {
        let parser = Parser::from_reader("foo@bar.example.com\n".as_bytes());
        assert_eq!("<p>foo@bar.example.com</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_613() {
        let parser = Parser::from_reader("<a><bab><c2c>\n".as_bytes());
        assert_eq!("<p><a><bab><c2c></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_614() {
        let parser = Parser::from_reader("<a/><b2/>\n".as_bytes());
        assert_eq!("<p><a/><b2/></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_615() {
        let parser = Parser::from_reader("<a  /><b2\ndata=\"foo\" >\n".as_bytes());
        assert_eq!("<p><a  /><b2\ndata=\"foo\" ></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_616() {
        let parser = Parser::from_reader("<a foo=\"bar\" bam = 'baz <em>\"</em>'\n_boolean zoop:33=zoop:33 />\n".as_bytes());
        assert_eq!("<p><a foo=\"bar\" bam = 'baz <em>\"</em>'\n_boolean zoop:33=zoop:33 /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_617() {
        let parser = Parser::from_reader("Foo <responsive-image src=\"foo.jpg\" />\n".as_bytes());
        assert_eq!("<p>Foo <responsive-image src=\"foo.jpg\" /></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_618() {
        let parser = Parser::from_reader("<33> <__>\n".as_bytes());
        assert_eq!("<p>&lt;33&gt; &lt;__&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_619() {
        let parser = Parser::from_reader("<a h*#ref=\"hi\">\n".as_bytes());
        assert_eq!("<p>&lt;a h*#ref=&quot;hi&quot;&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_620() {
        let parser = Parser::from_reader("<a href=\"hi'> <a href=hi'>\n".as_bytes());
        assert_eq!("<p>&lt;a href=&quot;hi'&gt; &lt;a href=hi'&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_621() {
        let parser = Parser::from_reader("< a><\nfoo><bar/ >\n<foo bar=baz\nbim!bop />\n".as_bytes());
        assert_eq!("<p>&lt; a&gt;&lt;\nfoo&gt;&lt;bar/ &gt;\n&lt;foo bar=baz\nbim!bop /&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_622() {
        let parser = Parser::from_reader("<a href='bar'title=title>\n".as_bytes());
        assert_eq!("<p>&lt;a href='bar'title=title&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_623() {
        let parser = Parser::from_reader("</a></foo >\n".as_bytes());
        assert_eq!("<p></a></foo ></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_624() {
        let parser = Parser::from_reader("</a href=\"foo\">\n".as_bytes());
        assert_eq!("<p>&lt;/a href=&quot;foo&quot;&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_625() {
        let parser = Parser::from_reader("foo <!-- this is a --\ncomment - with hyphens -->\n".as_bytes());
        assert_eq!("<p>foo <!-- this is a --\ncomment - with hyphens --></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_626() {
        let parser = Parser::from_reader("foo <!--> foo -->\n\nfoo <!---> foo -->\n".as_bytes());
        assert_eq!("<p>foo <!--> foo --&gt;</p>\n<p>foo <!---> foo --&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_627() {
        let parser = Parser::from_reader("foo <?php echo $a; ?>\n".as_bytes());
        assert_eq!("<p>foo <?php echo $a; ?></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_628() {
        let parser = Parser::from_reader("foo <!ELEMENT br EMPTY>\n".as_bytes());
        assert_eq!("<p>foo <!ELEMENT br EMPTY></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_629() {
        let parser = Parser::from_reader("foo <![CDATA[>&<]]>\n".as_bytes());
        assert_eq!("<p>foo <![CDATA[>&<]]></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_630() {
        let parser = Parser::from_reader("foo <a href=\"&ouml;\">\n".as_bytes());
        assert_eq!("<p>foo <a href=\"&ouml;\"></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_631() {
        let parser = Parser::from_reader("foo <a href=\"\\*\">\n".as_bytes());
        assert_eq!("<p>foo <a href=\"\\*\"></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Raw HTML
    fn test_example_632() {
        let parser = Parser::from_reader("<a href=\"\\\"\">\n".as_bytes());
        assert_eq!("<p>&lt;a href=&quot;&quot;&quot;&gt;</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_633() {
        let parser = Parser::from_reader("foo  \nbaz\n".as_bytes());
        assert_eq!("<p>foo<br />\nbaz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_634() {
        let parser = Parser::from_reader("foo\\\nbaz\n".as_bytes());
        assert_eq!("<p>foo<br />\nbaz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_635() {
        let parser = Parser::from_reader("foo       \nbaz\n".as_bytes());
        assert_eq!("<p>foo<br />\nbaz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_636() {
        let parser = Parser::from_reader("foo  \n     bar\n".as_bytes());
        assert_eq!("<p>foo<br />\nbar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_637() {
        let parser = Parser::from_reader("foo\\\n     bar\n".as_bytes());
        assert_eq!("<p>foo<br />\nbar</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_638() {
        let parser = Parser::from_reader("*foo  \nbar*\n".as_bytes());
        assert_eq!("<p><em>foo<br />\nbar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_639() {
        let parser = Parser::from_reader("*foo\\\nbar*\n".as_bytes());
        assert_eq!("<p><em>foo<br />\nbar</em></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_640() {
        let parser = Parser::from_reader("`code  \nspan`\n".as_bytes());
        assert_eq!("<p><code>code   span</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_641() {
        let parser = Parser::from_reader("`code\\\nspan`\n".as_bytes());
        assert_eq!("<p><code>code\\ span</code></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_642() {
        let parser = Parser::from_reader("<a href=\"foo  \nbar\">\n".as_bytes());
        assert_eq!("<p><a href=\"foo  \nbar\"></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_643() {
        let parser = Parser::from_reader("<a href=\"foo\\\nbar\">\n".as_bytes());
        assert_eq!("<p><a href=\"foo\\\nbar\"></p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_644() {
        let parser = Parser::from_reader("foo\\\n".as_bytes());
        assert_eq!("<p>foo\\</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_645() {
        let parser = Parser::from_reader("foo  \n".as_bytes());
        assert_eq!("<p>foo</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_646() {
        let parser = Parser::from_reader("### foo\\\n".as_bytes());
        assert_eq!("<h3>foo\\</h3>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Hard line breaks
    fn test_example_647() {
        let parser = Parser::from_reader("### foo  \n".as_bytes());
        assert_eq!("<h3>foo</h3>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Soft line breaks
    fn test_example_648() {
        let parser = Parser::from_reader("foo\nbaz\n".as_bytes());
        assert_eq!("<p>foo\nbaz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Soft line breaks
    fn test_example_649() {
        let parser = Parser::from_reader("foo \n baz\n".as_bytes());
        assert_eq!("<p>foo\nbaz</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Textual content
    fn test_example_650() {
        let parser = Parser::from_reader("hello $.;'there\n".as_bytes());
        assert_eq!("<p>hello $.;'there</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Textual content
    fn test_example_651() {
        let parser = Parser::from_reader("Foo χρῆν\n".as_bytes());
        assert_eq!("<p>Foo χρῆν</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

    #[test]
    /// Textual content
    fn test_example_652() {
        let parser = Parser::from_reader("Multiple     spaces\n".as_bytes());
        assert_eq!("<p>Multiple     spaces</p>\n", parser.parse_to_string().unwrap_or("".into()));
    }

}
