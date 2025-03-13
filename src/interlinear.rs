use bible_reference_parser::{book_chapter_verse::BookChapterVerse, passage_segments::chapter_verse::ChapterVerse};

/// Okay, but what about when Greek words are out of order?
/// Maybe all BSB words are in order
#[derive(Clone, Debug)]
pub enum TranslatedWord {
    /// " [This is the] record ": Text translated into segments
    Text(Vec<TranslatedTextSegment>),
    /// " - ": Not directly translated
    Omitted,
    /// " . . . ": Part of previous word
    Earlier,
    /// " vvv ": Part of upcoming word
    Later,
}

#[derive(Clone, Debug)]
pub enum TranslatedTextSegment {
    /// "record" in " [This is the] record "
    Word(String),
    /// "This is the" in " [This is the] record "
    Grammar(String),
}

/**
This should be a mix of language, parsing, and strong's number
*/
#[derive(Clone, Debug)]
pub enum Language {
    Hebrew,
    Greek,
    Aramaic,
}

#[derive(Clone, Debug)]
pub struct InterlinearVerseEntry {
    verse: BookChapterVerse,
    words: Vec<InterlinearWordEntry>,
}

/// Taken from `BSB Translation Tables - xlsx` at https://berean.bible/downloads.htm
/// - HTML Markup seems to use `|` instead of `"`
#[derive(Clone, Debug)]
pub struct InterlinearWordEntry {

    /**
    Excel Column: `"Verse"`
    */
    verse: BookChapterVerse,

    /**
    Excel Column: `"Language"`
    */
    language: Language,

    /**
    Excel Column: `"WLC / Nestle Base TR RP WH NE NA SBL"`
    https://biblehub.com/interlinear/
    ### Hebrew:
    - **WLC**: Westminster Leningrad Codex
    ### Greek:
    - **TR:** Schrivener’s Textus Receptus 1896
    - **RP:** Byzantine Majority Text
    - **WH:** Westcott and Hort
    - **NE:** Nestle 1904
    - **NA:** NA27
    - **SBL:** SBLGNT
    */
    text_1: String,

    /**
    Excel Column: `"WLC / Nestle Base {TR} ⧼RP⧽ (WH) 〈NE〉 [NA] ‹SBL› [[ECM]]"`
    - It seems to be exactly like column 1, but it is for words not contained in the base text
    */
    text_2: String,

    /**
    Excel Column: `"Translit"`
    */
    transliteration: String,

    /**
    Excel Column: `"Parsing"`
    I can probably merge this with Language, because parsing depends on language
    */
    parsing_code: String,

    /**
    Excel Column: `"Parsing"`
    */
    parsing: String,

    /**
    Excel Column: `"Str Heb"`
    */
    strongs_hebrew: Option<u32>,

    /**
    Excel Column: `"Str Grk"`
    */
    strongs_greek: Option<u32>,

    /**
    Excel Column: `"Hdg"`
    Example values:
    ```text
    ""
    "<p class=|hdg|>The Genealogy of Jesus"
    ```
    which is really `"The Genealogy of Jesus"`
    */
    heading: Option<String>,

    /**
    Excel Column: `"Crossref"`
    - This has HTML markup
    Example NT values
    ```text
    ""
    "<br /><span class=|cross|>(<a href =|../ruth/4.htm#18|>Ruth 4:18–22</a>; <a href =|../luke/3.htm#23|>Luke 3:23–38</a>)</span>"
    ```
    which is really `["Ruth 4:18–22", "Luke 3:23–38"]`
    */
    crossref: Option<Vec<String>>,

    /**
    Excel Column: `"Par"`
    - This has HTML markup
    All NT values
    ```text
    ""
    "<br />"
    "<p class=|indent1stlinered|>"
    "<p class=|indent1stline|>"
    "<p class=|indent1|>"
    "<p class=|indent2|>"
    "<p class=|indentred1|>"
    "<p class=|indentred2|>"
    "<p class=|list1stline|>"
    "<p class=|list1|>"
    "<p class=|red|>"
    "<p class=|reg|>"
    "<p class=|reg|><div class=|inscrip|>"
    "<p class=|reg|><span class=|red|>"
    "<p class=|selah|>"
    "<p class=|tab1stlinered|>"
    "<p class=|tab1stline|>"
    "<span class=|red|>"
    ```
    */
    paragraph: Option<String>,

    /**
    Excel Column: `"“"`
    ```text
    ``
    ` ‘`
    `(`
    `[“]`
    `[“] `
    `‘`
    `“`
    `“ `
    `“‘`
    ```
    */
    start_quote: Option<String>,

    /**
    Excel Column: `" BSB version "`
    Examples NT values
    ```
    ` [This is the] record `
    ```
    where "record" corresponds to the Greek word and "[This is the]" captures the form/context/usage of the word
    */
    english: TranslatedWord,

    /**
    Excel Column: `"pnc"`
    All NT Values
    ```text
    ``
    ` )— `
    ` — `
    ` —  `
    `!`
    `! `
    `!</span>`
    `!’`
    `!”`
    `!”</span>).`
    `!”’`
    `)`
    `),`
    `).`
    `,`
    `, `
    `,’`
    `,”`
    `,”</span>`
    `.`
    `. )`
    `.)`
    `...`
    `.’`
    `.’”`
    `.”`
    `.”</span>`
    `.”’`
    `:`
    `;`
    `?`
    `?’`
    `?”`
    `?”</span>`
    `—`
    `’`
    `’ ...`
    `’</span>?`
    `’?`
    `”`
    `” `
    `”),`
    `”;`
    ````
    */
    puncutation: Option<String>,

    /**
    Excel Column: `"”"`
    All NT Values
    ```text
    ``
    `?`
    `’`
    `’ ”`
    `”`
    `”</span>`
    ```
    */
    end_quote: Option<String>,

    /**
    Excel Column: `"footnotes"`
    - This has HTML markup
    Example NT values
    ```text
    ``
    `Greek <i>Aram</i>, a variant of <i>Ram</i>; also in verse 4; see 1 Chronicles 2:9–10.`
    `Greek <i>Am&#333;s</i>, a variant spelling of Amon; twice in this verse; see 1 Chronicles 3:14.`
    `<i>Jesus</i> means <i>The LORD saves</i>.`
    `Literally <i>they will call His name Immanuel</i>; Isaiah 7:14 (see also DSS)`
    `See Isaiah 7:14, Isaiah 8:8, and Isaiah 8:10.`
    `Literally <i>he did not know her</i>`
    `Or <i>as it rose</i>`
    `Deuteronomy 6:16`
    `That is, the Ten Cities`
    `Exodus 20:14; Deuteronomy 5:18`
    `BYZ and TR <i>he sent two of his disciples</i>`
    `BYZ and TR include <i><span class=|fnv|>28</span>So the Scripture was fulfilled that says, “And He was numbered with the transgressors.”</i> See Isaiah 53:12 and Luke 22:37.`
    ```
    */
    footnotes: Option<String>,

    /**
    Excel Column: `"End text"`
    All NT values
    ```text
    ``
    ` `
    ` — `
    `),`
    `).`
    `</div>`
    `</span>`
    `?`
    `?”`
    `?”</span>`
    `End text`
    `[’’]`
    `[’’]</span>`
    `—`
    `’`
    `’</span>`
    `’</span>?”`
    `’</span>”`
    `’?`
    `’”`
    `’”</span>`
    `”`
    `” </span>`
    `”</span>`
    `”?`
    `”’`
    `”’</span>`
    ```
    */
    end_text: Option<String>,
}


#[derive(Clone, Debug)]
pub struct RawInterlinearEntry {
    /**
    Excel Column: `"Heb Sort"`
    */
    hebrew_sort: u32,

    /**
    Excel Column: `"Greek Sort"`
    */
    greek_sort: u32,

    /**
    Excel Column: `"BSB Sort"`
    */
    bsb_sort: u32,

    /**
    Excel Column: `"Verse"`
    */
    verse_id: u32,

    /**
    Excel Column: `"Language"`
    */
    language: String,

    /**
    Excel Column: `"WLC / Nestle Base TR RP WH NE NA SBL"`
    */
    text_1: Option<String>,

    /**
    Excel Column: `"WLC / Nestle Base {TR} ⧼RP⧽ (WH) 〈NE〉 [NA] ‹SBL› [[ECM]]"`
    */
    text_2: Option<String>,

    /**
    Excel Column: `"Translit"`
    */
    transliteration: Option<String>,

    /**
    Excel Column: `"Parsing"`
    */
    parsing_code: Option<String>,

    /**
    Excel Column: `"Parsing"`
    */
    parsing: Option<String>,

    /**
    Excel Column: `"Str Heb"`
    */
    strongs_hebrew: Option<u32>,

    /**
    Excel Column: `"Str Grk"`
    */
    strongs_greek: Option<u32>,

    /**
    Excel Column: `"Verse"`
    */
    verse: Option<String>,

    /**
    Excel Column: `"Hdg"`
    */
    heading: Option<String>,

    /**
    Excel Column: `"Crossref"`
    */
    crossref: Option<Vec<String>>,

    /**
    Excel Column: `"Par"`
    ```
    */
    paragraph: Option<String>,

    /**
    Excel Column: `"“"`
    */
    start_quote: Option<String>,

    /**
    Excel Column: `" BSB version "`
    */
    english: Option<String>,

    /**
    Excel Column: `"pnc"`
    */
    puncutation: Option<String>,

    /**
    Excel Column: `"”"`
    */
    end_quote: Option<String>,

    /**
    Excel Column: `"footnotes"`
    */
    footnotes: Option<String>,

    /**
    Excel Column: `"End text"`
    */
    end_text: Option<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn idk() {


    }
}
